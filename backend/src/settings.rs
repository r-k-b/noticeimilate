use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Database {
    database: String,
    host: String,
    password: String,
    port: i16,
    username: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    db: Database,
}

impl Settings {
    pub fn db_connection_string(&self) -> String {
        format!(
            "postgresql://{un}:{pw}@{host}:{port}/{db}",
            un = self.db.username,
            pw = self.db.password,
            host = self.db.host,
            port = self.db.port,
            db = self.db.database,
        )
    }

    pub fn redacted_db_connection_string(&self) -> String {
        format!(
            "postgresql://{un}:{pw}@{host}:{port}",
            un = self.db.username,
            pw = "[REDACTED]",
            host = self.db.host,
            port = self.db.port
        )
    }

    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("config/Default.toml"))?;
        s.merge(File::with_name("../secrets/fetcher.toml"))?;
        // s.merge(File::with_name("../secrets/replenisher.toml"))?;
        s.merge(Environment::with_prefix("FETCHER"))?;
        // s.merge(Environment::with_prefix("REPLENISHER"))?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
