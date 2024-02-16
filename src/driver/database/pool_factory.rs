use deadpool_postgres::{Pool, Runtime};
use tokio_postgres::NoTls;
use tokio_postgres_migration::Migration;

use crate::driver::database::config_factory::ConfigFactory;

pub struct PoolFactory {
    config_factory: ConfigFactory,
}

const SCRIPTS_UP: [(&str, &str); 1] = [
    ("0001_create-users", include_str!("../../../migrations/0001_create-users_up.sql")),
];

impl PoolFactory {
    pub fn new(config_factory: ConfigFactory) -> PoolFactory {
        PoolFactory { config_factory }
    }

    pub async fn create(&mut self) -> Pool {
        let pool: Pool = self
            .config_factory
            .create()
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .expect("could not create postgres connection pool");
        self.migrate(&pool).await;
        pool
    }

    async fn migrate(&mut self, pool: &Pool) {
        let mut client = pool.get().await.expect("could not get postgres client");
        let migration = Migration::new("migrations".to_string());
        migration
            .up(&mut **client, &SCRIPTS_UP)
            .await
            .expect("couldn't run migrations");
    }
}
