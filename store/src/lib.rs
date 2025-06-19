use diesel::prelude::*;
pub mod schema;
pub struct Store {
    conn: PgConnection,
}

impl Default for Store {
    fn default() -> Self {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            panic!("DATABASE_URL environment variable is not set");
        });
        let conn = PgConnection::establish(&database_url).unwrap_or_else(|_| {
            panic!("Error connecting to the database");
        });
        Self { conn }
    }
}

impl Store {
    pub fn create_user(&self) {
        print!("Creating user in the store...");
    }
    pub fn create_website(&self) -> String {
        String::from("value")
    }
}
