use actix_web::{
    HttpResponse,
    Result, web::{Data, Json, Path},
};
use log::debug;
use uuid::Uuid;

use crate::api::auth::token::JsonWebToken;
use crate::api::error::ApiError;
use crate::business::user::request::DeleteUserRequest;
use crate::business::user::request::RegisterUserRequest;
use crate::business::user::service::UserService;
use crate::core::user::UserDto;

pub async fn index(user_service: Data<UserService>) -> Result<Json<Vec<UserDto>>, ApiError> {
    debug!("user/handler.index()");
    let list_of_user = user_service.index().await?;
    Ok(Json(list_of_user))
}

pub async fn protected_index(
    user_service: Data<UserService>,
    jwt: JsonWebToken,
) -> Result<Json<Vec<UserDto>>, ApiError> {
    debug!("user/handler.protected_index() with inputs: username={:?}", jwt.username());
    let list_of_user = user_service.index().await?;
    Ok(Json(list_of_user))
}

pub async fn show(
    user_service: Data<UserService>,
    params: Path<Uuid>,
) -> Result<Json<Option<UserDto>>, ApiError> {
    debug!("user/handler.show() with inputs: params={:?}", params);
    let option = user_service.show(params.into_inner()).await?;
    Ok(Json(option))
}

pub async fn register(
    user_service: Data<UserService>,
    json: Json<RegisterUserRequest>,
) -> Result<HttpResponse, ApiError> {
    debug!("user/handler.register() with inputs: json={:?}", json);
    user_service.register(json.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn delete(
    user_service: Data<UserService>,
    json: Json<DeleteUserRequest>,
) -> Result<HttpResponse, ApiError> {
    debug!("user/handler.delete() with inputs: json={:?}", json);
    user_service.delete(json.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
