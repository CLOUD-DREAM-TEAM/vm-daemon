pub struct Config {
    pub vm_id: String,
    pub orchestrator_url: reqwest::Url,
    pub logs_port: u16,
    pub vm_report_interval: u64,
}
