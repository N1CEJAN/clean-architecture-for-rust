use std::sync::Arc;

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::{Data, get, post, scope};

use crate::api::auth::handler as auth_handler;
use crate::api::user::handler as user_handler;
use crate::business::auth::service::AuthService;
use crate::business::user::repository::UserRepository;
use crate::business::user::service::UserService;
use crate::driver::dao::token::TokenDao;
use crate::driver::dao::user::UserDao;
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
    let pool = pool_factory.create().await;
    let pool_adapter = Arc::new(PoolAdapter::new(pool));
    let user_dao = UserDao::new(pool_adapter.clone());
    let token_dao = TokenDao::new(pool_adapter.clone());
    let user_repository = Arc::new(UserRepository::new(user_dao, token_dao));
    let user_service = Arc::new(UserService::new(user_repository.clone()));
    let auth_service = Arc::new(AuthService::new(user_repository.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(user_service.clone()))
            .app_data(Data::from(auth_service.clone()))
            .wrap(Logger::default())
            .service(scope("")
                .route("/register", post().to(auth_handler::register))
                .route("/login", post().to(auth_handler::login))
                .route("/refresh", get().to(auth_handler::refresh))
                .route("/logout", post().to(auth_handler::logout))
                .service(scope("/users")
                    .route("", get().to(user_handler::index))
                    .route("/{id}", get().to(user_handler::show))
                    .route("/delete", post().to(user_handler::delete))))
    })
    .bind(&address)?
    .run()
    .await
}
