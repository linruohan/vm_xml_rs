use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NUMAConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell: Option<Vec<NUMACell>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NUMACell {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@cpus")]
    pub cpus: String,
    #[serde(rename = "@memory")]
    pub memory: u64,
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memnode: Option<Vec<MemNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemNode {
    #[serde(rename = "@cellid")]
    pub cellid: u32,
    #[serde(rename = "@mode")]
    pub mode: String,
}
