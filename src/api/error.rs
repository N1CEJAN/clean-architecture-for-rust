use crate::business::error::BusinessError;

#[derive(Debug)]
pub struct ApiError(BusinessError);

impl From<BusinessError> for ApiError {
    fn from(error: BusinessError) -> ApiError {
        ApiError(error)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::BadRequest().json(&self.0)
    }
}
