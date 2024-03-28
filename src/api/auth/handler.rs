use actix_web::HttpResponse;
use actix_web::web::{Data, Json};
use log::debug;

use crate::api::auth::token::JsonWebToken;
use crate::api::auth::token::RefreshToken;
use crate::api::error::ApiError;
use crate::business::auth::request::{LoginUserRequest, RegisterUserRequest};
use crate::business::auth::service::AuthService;

pub async fn register(
    auth_service: Data<AuthService>,
    json: Json<RegisterUserRequest>,
) -> actix_web::Result<HttpResponse, ApiError> {
    debug!("auth/handler.register() with parameters: {:?}", json);
    let api_request = json.into_inner();
    auth_service.register(api_request).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn login(
    auth_service: Data<AuthService>,
    json: Json<LoginUserRequest>,
) -> actix_web::Result<HttpResponse, ApiError> {
    debug!("auth/handler.login() with parameters: {:?}", json);
    let user_dto = auth_service.login(json.into_inner()).await?;
    let refresh_token = RefreshToken::new(user_dto.latest_token()
        .expect("if no token had been created, auth_service would have failed"));
    let access_token = JsonWebToken::new(user_dto.username());
    Ok(HttpResponse::Ok()
        .cookie(refresh_token.cookie().clone())
        .json(Json(access_token.key().to_string().clone())))
}

pub async fn refresh<'a>(
    auth_service: Data<AuthService>,
    old_refresh_token: RefreshToken<'a>,
) -> actix_web::Result<HttpResponse, ApiError> {
    debug!("auth/handler.refresh() with refresh token: {:?}", old_refresh_token);
    let user_dto = auth_service.refresh(old_refresh_token.key().as_ref()).await?;
    let refresh_token = RefreshToken::new(user_dto.latest_token()
        .expect("if no token had been created, auth_service would have failed"));
    let access_token = JsonWebToken::new(user_dto.username());
    Ok(HttpResponse::Ok()
        .cookie(refresh_token.cookie().clone())
        .json(Json(access_token.key().to_string().clone())))
}

pub async fn logout<'a>(
    auth_service: Data<AuthService>,
    refresh_token: RefreshToken<'a>,
) -> actix_web::Result<HttpResponse, ApiError> {
    debug!("auth/handler.logout() with refresh token: {:?}", refresh_token);
    auth_service.logout(refresh_token.key().as_ref()).await?;
    Ok(HttpResponse::Ok().finish())
}
