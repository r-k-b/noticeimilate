extern crate config;
extern crate postgres;

use postgres::{Client, NoTls, Row};
use settings::Settings;

struct ToFetch {
    feed_id: i64,
    url: String,
}

fn main() {
    let settings: Settings = Settings::new().unwrap();

    let mut connection: Client = Client::connect(settings.db_connection_string().as_ref(), NoTls)
        .expect(
            format!(
                "Failed to connect to db {}",
                settings.redacted_db_connection_string(),
            )
            .as_ref(),
        );

    let rows: Vec<postgres::Row> = connection
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
