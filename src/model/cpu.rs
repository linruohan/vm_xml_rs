use serde::{Deserialize, Serialize};

use crate::model::CacheConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CPUConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology: Option<CPUTopology>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<CPUModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<Vec<CPUFeatureConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<Vec<CacheConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxphysaddr: Option<CPUMaxPhysAddr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numa: Option<CPUNUMAConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migratable: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_features: Option<DeprecatedFeaturesConfig>,
}

/// CPU 废弃特性配置 (11.0.0, S390 guests)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeprecatedFeaturesConfig {
    #[serde(rename = "@state")]
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<Vec<CPUFeatureConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUTopology {
    #[serde(rename = "@sockets")]
    pub sockets: u32,
    #[serde(rename = "@dies", skip_serializing_if = "Option::is_none")]
    pub dies: Option<u32>,
    #[serde(rename = "@clusters", skip_serializing_if = "Option::is_none")]
    pub clusters: Option<u32>,
    #[serde(rename = "@cores")]
    pub cores: u32,
    #[serde(rename = "@threads")]
    pub threads: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUModel {
    #[serde(rename = "@fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<String>,
    #[serde(rename = "@vendor_id", skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<String>,
    #[serde(rename = "$value")]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUFeatureConfig {
    #[serde(rename = "@policy")]
    pub policy: String,
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUMaxPhysAddr {
    #[serde(rename = "@mode")]
    pub mode: String,
    #[serde(rename = "@bits", skip_serializing_if = "Option::is_none")]
    pub bits: Option<u32>,
    #[serde(rename = "@limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CPUNUMAConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell: Option<Vec<CPUNUMACell>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnects: Option<CPUNUMAInterconnects>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUNUMACell {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@cpus")]
    pub cpus: String,
    #[serde(rename = "@memory")]
    pub memory: u64,
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distances: Option<CPUNUMADistances>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUNUMADistances {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sibling: Option<Vec<CPUNUMASibling>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUNUMASibling {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@value")]
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUNUMAInterconnects {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<Vec<CPUNUMALatency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<Vec<CPUNUMABandwidth>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUNUMALatency {
    #[serde(rename = "@initiator")]
    pub initiator: u32,
    #[serde(rename = "@target")]
    pub target: u32,
    #[serde(rename = "@type")]
    pub type_: String,
    #[serde(rename = "@value")]
    pub value: u32,
    #[serde(rename = "@cache", skip_serializing_if = "Option::is_none")]
    pub cache: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUNUMABandwidth {
    #[serde(rename = "@initiator")]
    pub initiator: u32,
    #[serde(rename = "@target")]
    pub target: u32,
    #[serde(rename = "@type")]
    pub type_: String,
    #[serde(rename = "@value")]
    pub value: u32,
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
