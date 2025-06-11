use crate::config::Config;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub struct Store {
    pub conn: PgConnection,
}

impl Default for Store {
    fn default() -> Self {
        let config = Config::default();
            let conn = PgConnection::establish(&config.db_url)
            .expect("Failed to connect to DB");
        Self { conn }
    }
}
