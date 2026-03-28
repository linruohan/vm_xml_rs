use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsConfig {
    #[serde(rename = "@type")]
    pub graphics_type: String,
    #[serde(rename = "@port", skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(rename = "@autoport", skip_serializing_if = "Option::is_none")]
    pub autoport: Option<String>,
    #[serde(rename = "@listen", skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_type: Option<ListenConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenConfig {
    #[serde(rename = "@type")]
    pub listen_type: String,
    #[serde(rename = "@address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub video_type: Option<String>,
    pub model: VideoModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoModel {
    #[serde(rename = "@type")]
    pub model_type: String,
    #[serde(rename = "@vram", skip_serializing_if = "Option::is_none")]
    pub vram: Option<u32>,
    #[serde(rename = "@heads", skip_serializing_if = "Option::is_none")]
    pub heads: Option<u32>,
    #[serde(rename = "@primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,
}
