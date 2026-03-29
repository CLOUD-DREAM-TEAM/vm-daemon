#[macro_use]
extern crate rocket;

extern crate tracing;
use tracing::{error, info};

use rocket::fairing::{self};
use rocket::http::Status;
use rocket::response::status::NoContent;

use rocket::Request;

mod endpoints;

mod models;
pub use models::*;

pub struct CORS;
#[rocket::async_trait]
impl fairing::Fairing for CORS {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "CORS",
            kind: fairing::Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _: &'r rocket::Request<'_>, res: &mut rocket::Response<'r>) {
        res.set_header(rocket::http::Header::new(
            "Access-Control-Allow-Origin",
            "*",
        ));
        res.set_header(rocket::http::Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, PATCH, DELETE, OPTIONS",
        ));
        res.set_header(rocket::http::Header::new(
            "Access-Control-Allow-Headers",
            "*",
        ));
    }
}

// ============================================================================
// Main Entry Points
// ============================================================================

fn main() {
    let result = start();

    info!("Rocket: deorbit.");

    if let Some(err) = result.err() {
        error!("Error: {:?}", err);
    }
}

#[tokio::main]
async fn start() -> Result<(), Box<rocket::Error>> {
    rocket::build()
        .attach(CORS)
        .mount(
            "/",
            routes![
                options_preflight,
                endpoints::test::test,
            ],
        )
        .register("/", catchers![not_found])
        .launch()
        .await
        .map(|_| ())
        .map_err(Box::new)
}

// ============================================================================
// Route Handlers
// ============================================================================

#[options("/<_..>")]
fn options_preflight() -> NoContent {
    NoContent
}

#[catch(404)]
fn not_found(req: &Request) -> ApiResponse {
    info!("Request ended up in 404 catcher with request: {:?}", req);

    ApiResponse {
        status: Status::NotFound,
        content_type: rocket::http::ContentType::Text,
        message: "404: Not found.".to_string(),
    }
}
