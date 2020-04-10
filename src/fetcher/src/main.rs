extern crate config;
extern crate postgres;

mod settings;

use postgres::{Connection, TlsMode};
use settings::Settings;

struct ToFetch {
    feed_id: i64,
    url: String,
}

fn main() {
    let settings: Settings = Settings::new().unwrap();

    let conn: Connection = Connection::connect(settings.db_connection_string(), TlsMode::None)
        .expect(
            format!(
                "Failed to connect to db {}",
                settings.redacted_db_connection_string(),
            )
            .as_ref(),
        );

    let rows = &conn
        .query(
            "SELECT feeds.id, feeds.url
FROM fetch_queue,
     feeds
WHERE fetch_queue.feed = feeds.id
ORDER BY fetch_queue.id;",
            &[],
        )
        .unwrap();

    let fetch_rows = rows.into_iter().map(|row| ToFetch {
        feed_id: row.get(0),
        url: row.get(1),
    });

    for f in fetch_rows {
        println!("fetch: {} {}", f.feed_id, f.url)
    }
}
