use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::Serialize;
use rocket::Response;
use rocket::Request;
use tracing::debug;

#[derive(Debug)]
pub struct ApiResponse {
    pub status: Status,
    pub content_type: rocket::http::ContentType,
    pub message: String,
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let body = std::io::Cursor::new(self.message);
        Response::build()
            .header(rocket::http::ContentType::Text)
            .status(self.status)
            .streamed_body(body)
            .ok()
    }
}

#[derive(Serialize)]
pub struct RequestResult {
    pub success: bool,
    // Can be displayed on the FE!
    pub message: String,
}

pub fn construct_api_response(success: bool, message: &str, status: Status) -> ApiResponse {
    let request_result = RequestResult {
        success,
        message: message.to_string(),
    };
    let api_response = ApiResponse {
        status,
        content_type: rocket::http::ContentType::JSON,
        message: serde_json::to_string(&request_result).unwrap(),
    };
    debug!("API response: {:?}", api_response);
    api_response
}
