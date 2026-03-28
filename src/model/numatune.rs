use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NUMATuneConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<NUMAMemory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memnode: Option<Vec<NUMAMemNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NUMAMemory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NUMAMemNode {
    #[serde(rename = "@cellid")]
    pub cellid: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodeset: Option<String>,
}
