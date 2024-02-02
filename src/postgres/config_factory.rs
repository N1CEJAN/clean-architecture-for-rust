use deadpool_postgres::Config;

pub struct ConfigFactory;

impl ConfigFactory {
    pub fn create(&self) -> Config {
        let mut cfg = Config::new();
        cfg.host = std::env::var("PG_HOST").ok();
        cfg.dbname = std::env::var("PG_DBNAME").ok();
        cfg.user = std::env::var("PG_USER").ok();
        cfg.password = std::env::var("PG_PASSWORD").ok();
        cfg
    }
}
