use actix_web::{
    HttpResponse,
    Result, web::{Data, Json, Path},
};
use uuid::Uuid;

use crate::api::error::ApiError;
use crate::api::request::user::CreateUserRequest;
use crate::api::request::user::DeleteUserRequest;
use crate::business::service::user::UserService;
use crate::core::user::UserDto;

pub async fn index(
    user_service: Data<UserService>
) -> Result<Json<Vec<UserDto>>, ApiError> {
    let list_of_user = user_service.index().await
        .map_err(|error| ApiError::from(error))?;
    Ok(Json(list_of_user))
}

pub async fn show(
    user_service: Data<UserService>,
    params: Path<Uuid>,
) -> Result<Json<Option<UserDto>>, ApiError> {
    let user_id = params.into_inner();
    let option = user_service.show(user_id).await
        .map_err(|error| ApiError::from(error))?;
    Ok(Json(option))
}

pub async fn create(
    user_service: Data<UserService>,
    json: Json<CreateUserRequest>,
) -> Result<Json<UserDto>, ApiError> {
    let request = json.into_inner();
    let user = user_service.create(request).await
        .map_err(|error| ApiError::from(error))?;
    Ok(Json(user))
}

pub async fn delete(
    user_service: Data<UserService>,
    json: Json<DeleteUserRequest>,
) -> Result<HttpResponse, ApiError> {
    let request = json.into_inner();
    user_service.delete(request).await
        .map_err(|error| ApiError::from(error))?;
    Ok(HttpResponse::NoContent().finish())
}
