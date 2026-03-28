use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMonitoringConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmu: Option<PMUConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<EventConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PMUConfig {
    #[serde(rename = "@state")]
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventConfig {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}
