use actix_web::{Result, web::{Data, Json}};
use actix_web::HttpResponse;
use log::debug;

use crate::api::auth::token::JsonWebToken;
use crate::api::auth::token::RefreshToken;
use crate::api::error::ApiError;
use crate::business::auth::request::LoginUserRequest;
use crate::business::auth::service::AuthService;

pub async fn login(
    auth_service: Data<AuthService>,
    json: Json<LoginUserRequest>,
) -> Result<HttpResponse, ApiError> {
    debug!("auth/handler.login() with inputs: {:?}", json);
    let user_dto = auth_service.login(json.into_inner()).await?;
    let new_refresh_token = RefreshToken::new(user_dto.latest_token()
        .expect("if no token had been created, auth_service would have failed"));
    let access_token = JsonWebToken::new(user_dto.username());
    Ok(HttpResponse::Ok()
        .cookie(new_refresh_token.cookie().clone())
        .json(Json(access_token.key().to_owned())))
}

pub async fn refresh(
    auth_service: Data<AuthService>,
    refresh_token: RefreshToken<'_>,
) -> Result<HttpResponse, ApiError> {
    debug!("auth/handler.refresh() with inputs: refresh_token={:?}", refresh_token);
    let user_dto = auth_service.refresh(refresh_token.key().as_ref()).await?;
    let new_refresh_token = RefreshToken::new(user_dto.latest_token()
        .expect("if no token had been created, auth_service would have failed"));
    let access_token = JsonWebToken::new(user_dto.username());
    Ok(HttpResponse::Ok()
        .cookie(new_refresh_token.cookie().clone())
        .json(Json(access_token.key().to_owned())))
}

pub async fn logout(
    auth_service: Data<AuthService>,
    refresh_token: RefreshToken<'_>,
) -> Result<HttpResponse, ApiError> {
    debug!("auth/handler.logout() with inputs: refresh_token={:?}", refresh_token);
    auth_service.logout(refresh_token.key().as_ref()).await?;
    Ok(HttpResponse::Ok().finish())
}
