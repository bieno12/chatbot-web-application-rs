use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub mod conversation;
pub mod message;

pub use conversation::Conversation;
pub use message::Message;

lazy_static::lazy_static! {
    static ref DATABASE : Database = {
        println!("opened db at {}", std::env::current_dir().unwrap().to_string_lossy());
        Database::new("./chat_app.db").unwrap()
    };
}
pub struct Database {
    pool: Pool<SqliteConnectionManager>,
}

pub fn db() -> &'static Database {
    &*DATABASE
}

impl Database {
    pub fn new(database_path: &str) -> Result<Self, r2d2::Error> {
        let manager = SqliteConnectionManager::file(database_path);
        let pool = Pool::builder().build(manager)?;
        Ok(Self { pool })
    }

    pub fn get_connection(&self) -> r2d2::PooledConnection<SqliteConnectionManager> {
        self.pool.get().unwrap()
    }
}
