use std::sync::Arc;
use tracing::{error, info};

use crate::models::*;

pub fn spawn(config: Arc<Config>, shared_simulation_settings: SharedSimulationSettings) {
    let _ = tokio::spawn(async move {
        let client = reqwest::Client::new();

        let mut metric_url = config.orchestrator_url.clone();
        metric_url.set_port(Some(config.logs_port)).unwrap();

        info!("Metric reporter started with URL: {:?}", metric_url);

        loop {
            if shared_simulation_settings
                .read()
                .await
                .cpu_to_simulate
                .is_none()
                || shared_simulation_settings
                    .read()
                    .await
                    .ram_to_simulate
                    .is_none()
            {
                info!(
                    "Simulation settings not yet received from orchestrator, skipping metric report."
                );
                tokio::time::sleep(std::time::Duration::from_secs(config.vm_report_interval)).await;
                continue;
            }

            let metrics = MetricReport {
                vm_id: config.vm_id.clone(),
                cpu_percent: shared_simulation_settings
                    .read()
                    .await
                    .cpu_to_simulate
                    .unwrap(),
                ram_percent: shared_simulation_settings
                    .read()
                    .await
                    .ram_to_simulate
                    .unwrap(),
            };

            match client.post(metric_url.as_str()).json(&metrics).send().await {
                Ok(response) => {
                    info!("Metric report sent, response status: {}", response.status());
                }
                Err(e) => {
                    error!("Failed to send metric report: {}", e);
                }
            }

            tokio::time::sleep(std::time::Duration::from_secs(config.vm_report_interval)).await;
        }
    });
}
