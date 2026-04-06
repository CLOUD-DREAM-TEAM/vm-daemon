use serde::Deserialize;
use std::sync::Arc;
use tracing::{error, info};

use crate::models::*;

#[derive(Debug, Deserialize)]
struct OrchestratorResponse {
    per_vm_cpu: f32,
    per_vm_ram: f32,
}

pub fn spawn(config: Arc<Config>, shared_simulation_settings: SharedSimulationSettings) {
    let _ = tokio::spawn(async move {
        let client = reqwest::Client::new();

        let simulation_url = config.orchestrator_url.join("/simulation").unwrap();
        let mut failed_polls_count = 0;
        let failure_threshold = 10;

        info!("Orchestrator poller started with URL: {}", simulation_url);

        loop {
            match client.get(simulation_url.as_str()).send().await {
                Ok(response) => {
                    let status = response.status();
                    let body = response.text().await;
                    if body.is_err() {
                        failed_polls_count += 1;
                        error!("Failed to read poll response body: {}", body.err().unwrap());
                        error!(
                            "Poll iteration failed, {}/{}",
                            failed_polls_count, failure_threshold
                        );
                        continue;
                    }

                    let orchestrator_response =
                        serde_json::from_str::<OrchestratorResponse>(&body.unwrap());
                    if orchestrator_response.is_err() {
                        failed_polls_count += 1;
                        error!(
                            "Failed to parse poll response body: {}",
                            orchestrator_response.err().unwrap()
                        );
                        error!(
                            "Poll iteration failed, {}/{}",
                            failed_polls_count, failure_threshold
                        );
                        continue;
                    }
                    debug!("Poll response [{}]: {:?}", status, orchestrator_response);

                    let orchestrator_response = orchestrator_response.unwrap();
                    let mut simulation_settings = shared_simulation_settings.write().await;
                    simulation_settings.cpu_to_simulate = Some(orchestrator_response.per_vm_cpu);
                    simulation_settings.ram_to_simulate = Some(orchestrator_response.per_vm_ram);

                    info!(
                        "Updated simulation settings: CPU = {}%, RAM = {}%",
                        orchestrator_response.per_vm_cpu, orchestrator_response.per_vm_ram
                    );
                    failed_polls_count = 0;
                }
                Err(e) => {
                    failed_polls_count += 1;
                    error!("Poll request failed: {}", e);
                    error!(
                        "Poll iteration failed, {}/{}",
                        failed_polls_count, failure_threshold
                    );
                }
            }

            if failed_polls_count >= failure_threshold {
                error!("Shutting down due to repeated poll failures.");
                std::process::exit(1);
            }

            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    });
}
