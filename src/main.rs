use std::sync::Arc;

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::{Data, get, post, scope};

use crate::api::controller::user;
use crate::business::service::user::UserService;
use crate::driver::dao::user::RawUserDao;
use crate::driver::database::config_factory::ConfigFactory;
use crate::driver::database::pool_adapter::PoolAdapter;
use crate::driver::database::pool_factory::PoolFactory;

mod api;
mod business;
mod core;
mod driver;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let address = std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".into());

    let mut pool_factory = PoolFactory::new(ConfigFactory);
    let pool = Arc::new(pool_factory.create().await);
    let pool_adapter = Arc::new(PoolAdapter::new(pool));
    let user_dao = Arc::new(RawUserDao::new(pool_adapter));
    let user_service = Arc::new(UserService::new(user_dao));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(user_service.clone()))
            .wrap(Logger::default())
            .service(
                scope("/users")
                    .route("", get().to(user::index))
                    .route("/{id}", get().to(user::show))
                    .route("/create", post().to(user::create))
                    .route("/delete", post().to(user::delete)),
            )
    })
    .bind(&address)?
    .run()
    .await
}
