use crate::business::error::BusinessError;

#[derive(Debug)]
pub struct ApiError {
    message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::BadRequest().json(&self.message)
    }
}

impl From<&dyn ToString> for ApiError {
    fn from(error: &dyn ToString) -> Self {
        ApiError {
            message: error.to_string(),
        }
    }
}

impl From<BusinessError> for ApiError {
    fn from(error: BusinessError) -> Self {
        ApiError::from(&error as &dyn ToString)
    }
}
