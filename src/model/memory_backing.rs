use serde::{Deserialize, Serialize};

use crate::model::HugepagesConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryBackingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hugepages: Option<HugepagesConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nosharepages: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<MemorySource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<MemoryAccess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation: Option<MemoryAllocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discard: Option<()>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySource {
    #[serde(rename = "@type")]
    pub source_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAccess {
    #[serde(rename = "@mode")]
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAllocation {
    #[serde(rename = "@mode")]
    pub mode: String,
    #[serde(rename = "@threads", skip_serializing_if = "Option::is_none")]
    pub threads: Option<u32>,
}
