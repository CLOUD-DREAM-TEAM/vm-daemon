use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MetricReport {
    pub vm_id: String,
    pub cpu_percent: f32,
    pub ram_percent: f32,
}
