use actix_web::{
    HttpResponse,
    Result, web::{Data, Json, Path},
};
use uuid::Uuid;

use crate::api::error::ApiError;
use crate::business::user::request::DeleteUserRequest;
use crate::business::user::service::UserService;
use crate::core::user::UserDto;

pub async fn index(
    user_service: Data<UserService>
) -> Result<Json<Vec<UserDto>>, ApiError> {
    let list_of_user = user_service.index().await?;
    Ok(Json(list_of_user))
}

pub async fn show(
    user_service: Data<UserService>,
    params: Path<Uuid>,
) -> Result<Json<Option<UserDto>>, ApiError> {
    let user_id = params.into_inner();
    let option = user_service.show(user_id).await?;
    Ok(Json(option))
}

pub async fn delete(
    user_service: Data<UserService>,
    json: Json<DeleteUserRequest>,
) -> Result<HttpResponse, ApiError> {
    let request = json.into_inner();
    user_service.delete(request).await?;
    Ok(HttpResponse::NoContent().finish())
}
