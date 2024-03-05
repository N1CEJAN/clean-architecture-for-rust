use actix_web::web::{Data, Json};
use actix_web::HttpResponse;

use crate::api::auth::token::JsonWebToken;
use crate::api::auth::token::RefreshToken;
use crate::api::error::ApiError;
use crate::business::auth::request::{LoginUserRequest, RegisterUserRequest};
use crate::business::auth::service::AuthService;

pub async fn register(
    auth_service: Data<AuthService>,
    json: Json<RegisterUserRequest>,
) -> actix_web::Result<HttpResponse, ApiError> {
    let api_request = json.into_inner();
    auth_service.register(api_request).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn login(
    auth_service: Data<AuthService>,
    json: Json<LoginUserRequest>,
) -> actix_web::Result<HttpResponse, ApiError> {
    let user_dto = auth_service.login(json.into_inner()).await?;
    // if no token had been created, auth_service would have failed
    let refresh_token = RefreshToken::new(user_dto.latest_token().unwrap().key());
    let access_token = JsonWebToken::new(user_dto.username());
    Ok(HttpResponse::Ok()
        .cookie(refresh_token.cookie().clone())
        .json(Json(access_token.key().to_string().clone())))
}

pub async fn refresh<'a>(
    auth_service: Data<AuthService>,
    old_refresh_token: RefreshToken<'a>,
) -> actix_web::Result<HttpResponse, ApiError> {
    let user_dto = auth_service.refresh(old_refresh_token.key()).await?;
    // if no token had been created, auth_service would have failed
    let refresh_token = RefreshToken::new(user_dto.latest_token().unwrap().key());
    let access_token = JsonWebToken::new(user_dto.username());
    Ok(HttpResponse::Ok()
        .cookie(refresh_token.cookie().clone())
        .json(Json(access_token.key().to_string().clone())))
}

pub async fn logout<'a>(
    auth_service: Data<AuthService>,
    refresh_token: RefreshToken<'a>,
) -> actix_web::Result<HttpResponse, ApiError> {
    auth_service.logout(refresh_token.key()).await?;
    Ok(HttpResponse::Ok().finish())
}
