use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialConfig {
    #[serde(rename = "@type")]
    pub serial_type: String,
    #[serde(rename = "@port", skip_serializing_if = "Option::is_none")]
    pub port: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<SerialTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialTarget {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(rename = "@port", skip_serializing_if = "Option::is_none")]
    pub port: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleConfig {
    #[serde(rename = "@type")]
    pub console_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<ConsoleTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleTarget {
    #[serde(rename = "@type")]
    pub target_type: String,
    #[serde(rename = "@port", skip_serializing_if = "Option::is_none")]
    pub port: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelConfig {
    #[serde(rename = "@type")]
    pub parallel_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<ParallelTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelTarget {
    #[serde(rename = "@port")]
    pub port: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    #[serde(rename = "@type")]
    pub channel_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<ChannelTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelTarget {
    #[serde(rename = "@type")]
    pub target_type: String,
    #[serde(rename = "@name")]
    pub name: String,
}
