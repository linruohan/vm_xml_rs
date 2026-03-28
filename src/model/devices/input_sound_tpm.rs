use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
    #[serde(rename = "@type")]
    pub input_type: String,
    #[serde(rename = "@bus", skip_serializing_if = "Option::is_none")]
    pub bus: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundConfig {
    #[serde(rename = "@model")]
    pub model: String,
}

// ChannelConfig 和 ChannelTarget 已移至 common.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchdogConfig {
    #[serde(rename = "@model")]
    pub model: String,
    #[serde(rename = "@action")]
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<RngBackend>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngBackend {
    #[serde(rename = "@model")]
    pub model: String,
    #[serde(rename = "@type")]
    pub rng_type: String,
    #[serde(rename = "@device")]
    pub device: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TPMConfig {
    #[serde(rename = "@model")]
    pub model: String,
    pub backend: TPMBackend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TPMBackend {
    #[serde(rename = "@type")]
    pub backend_type: String,
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemballoonConfig {
    #[serde(rename = "@model")]
    pub model: String,
    #[serde(rename = "@autodeflate", skip_serializing_if = "Option::is_none")]
    pub autodeflate: Option<String>,
    #[serde(rename = "@period", skip_serializing_if = "Option::is_none")]
    pub period: Option<u32>,
}
