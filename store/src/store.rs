use crate::config::Config;
use diesel::prelude::*;
pub struct Store {
    pub conn: PgConnection,
}

impl Store {
    pub fn new() -> Result<Self, ConnectionError> {
        let config = Config::default();
        let conn = PgConnection::establish(&config.database_url)?;
        Ok(Self { conn })
    }
}
