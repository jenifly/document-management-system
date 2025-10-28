use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};
use std::sync::Arc;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<DbPool>,
    pub config: Arc<crate::config::Config>,
}

pub fn create_pool(database_url: &str) -> Result<DbPool, diesel::r2d2::PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(10)
        .build(manager)
}

impl AppState {
    pub fn new(pool: DbPool, config: crate::config::Config) -> Self {
        Self {
            pool: Arc::new(pool),
            config: Arc::new(config),
        }
    }

    pub fn get_connection(&self) -> Result<DbConnection, diesel::r2d2::PoolError> {
        self.pool.get()
    }
}

