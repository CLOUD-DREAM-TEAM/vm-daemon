use crate::models::response::{ApiResponse, construct_api_response};
use rocket::http::Status;

#[get("/test")]
pub async fn test() -> ApiResponse {
    construct_api_response(true, "🦀", Status::Ok)
}
