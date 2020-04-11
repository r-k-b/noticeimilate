extern crate config;
extern crate postgres;
extern crate postgres_types;

use postgres::{Client, NoTls, Row};
use postgres_types::{FromSql, ToSql};
use settings::Settings;

#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "feed_type")]
enum FeedType {
    #[postgres(name = "rss")]
    Rss,
    #[postgres(name = "atom")]
    Atom,
}

struct Feed {
    id: i64,
    feed_type: FeedType,
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
            "SELECT feeds.id, feeds.url, feeds.feed_type
FROM feeds
ORDER BY feeds.id;",
            &[],
        )
        .unwrap();

    let fetch_rows = rows.into_iter().map(|row| Feed {
        id: row.get(0),
        url: row.get(1),
        feed_type: row.get(2),
    });

    for f in fetch_rows {
        println!("fetch: {} {} {:?}", f.id, f.url, f.feed_type)
    }
}
