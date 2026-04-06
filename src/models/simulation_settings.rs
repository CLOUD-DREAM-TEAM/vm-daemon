use std::sync::Arc;
use tokio::sync::RwLock;

pub type SharedSimulationSettings = Arc<RwLock<SimulationSettings>>;

#[derive(Debug, Clone, Default)]
pub struct SimulationSettings {
    pub cpu_to_simulate: Option<f32>,
    pub ram_to_simulate: Option<f32>,
}
