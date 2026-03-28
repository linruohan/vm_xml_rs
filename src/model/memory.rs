use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VCPUInfo {
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<String>,
    #[serde(rename = "@cpuset", skip_serializing_if = "Option::is_none")]
    pub cpuset: Option<String>,
    #[serde(rename = "@current", skip_serializing_if = "Option::is_none")]
    pub current: Option<u32>,
    #[serde(rename = "$value")]
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "@slots", skip_serializing_if = "Option::is_none")]
    pub slots: Option<u32>,
    #[serde(rename = "@dumpCore", skip_serializing_if = "Option::is_none")]
    pub dump_core: Option<String>,
    #[serde(rename = "$value")]
    pub value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hugepages: Option<HugepagesConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nosharepages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HugepagesConfig {
    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "@nodeset", skip_serializing_if = "Option::is_none")]
    pub nodeset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<Vec<PageConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageConfig {
    #[serde(rename = "@size")]
    pub size: String,
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "@nodeset", skip_serializing_if = "Option::is_none")]
    pub nodeset: Option<String>,
}
