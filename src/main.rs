#[macro_use]
extern crate rocket;

extern crate tracing;
use tracing::{error, info};

use rocket::fairing::{self};
use rocket::http::Status;
use rocket::response::status::NoContent;

use rocket::Request;
use std::sync::Arc;

mod endpoints;
mod metric_reporter;
mod orchestrator_poller;

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
    tracing_subscriber::fmt::init();

    let config_result = check_envs_and_parse_config();

    if config_result.is_err() {
        error!(
            "Environment variable check failed: {}",
            config_result.err().unwrap()
        );
    } else {
        info!("All required environment variables are set.");

        let result = start(config_result.unwrap());

        info!("Rocket: deorbit.");

        if let Some(err) = result.err() {
            error!("Error: {:?}", err);
        }
    }
}

fn check_envs_and_parse_config() -> Result<Config, String> {
    let mut vm_id = std::env::var("VM_ID").unwrap_or("".to_string());
    if vm_id.is_empty() {
        vm_id = std::env::var("NOVA_SERVER_ID").unwrap_or("".to_string());
    }
    if vm_id.is_empty() {
        return Err("Neither VM_ID nor NOVA_SERVER_ID env vars are set.".to_string());
    }

    let orchestrator_url = std::env::var("ORCHESTRATOR_URL")
        .map_err(|_| "ORCHESTRATOR_URL env var is not set.".to_string())?;
    let orchestrator_url = reqwest::Url::parse(&orchestrator_url)
        .map_err(|_| "ORCHESTRATOR_URL env var is not a valid URL.".to_string())?;

    let logs_port =
        std::env::var("LOGS_PORT").map_err(|_| "LOGS_PORT env var is not set.".to_string())?;
    let logs_port = logs_port
        .parse()
        .map_err(|_| "LOGS_PORT env var is not a valid u16.".to_string())?;

    let vm_report_interval = std::env::var("VM_REPORT_INTERVAL")
        .map_err(|_| "VM_REPORT_INTERVAL env var is not set.".to_string())?;
    let vm_report_interval = vm_report_interval
        .parse()
        .map_err(|_| "VM_REPORT_INTERVAL env var is not a valid u16.".to_string())?;

    Ok(Config {
        vm_id,
        orchestrator_url,
        logs_port,
        vm_report_interval,
    })
}

#[tokio::main]
async fn start(config: Config) -> Result<(), Box<rocket::Error>> {
    let config = Arc::new(config);
    let shared_simulation_settings = SharedSimulationSettings::default();

    orchestrator_poller::spawn(config.clone(), shared_simulation_settings.clone());
    metric_reporter::spawn(config.clone(), shared_simulation_settings.clone());

    rocket::build()
        .attach(CORS)
        .manage(config)
        .manage(shared_simulation_settings)
        .mount("/", routes![options_preflight, endpoints::test::test,])
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
