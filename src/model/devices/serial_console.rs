use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialConfig {
    #[serde(rename = "@type")]
    pub serial_type: String,
    #[serde(rename = "@port", skip_serializing_if = "Option::is_none")]
    pub port: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SerialSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<SerialTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<SerialLog>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialSource {
    #[serde(rename = "@path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "@host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "@port", skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(rename = "@service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "@channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialLog {
    #[serde(rename = "@file")]
    pub file: String,
    #[serde(rename = "@append", skip_serializing_if = "Option::is_none")]
    pub append: Option<String>,
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
    pub source: Option<ConsoleSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<ConsoleTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<ConsoleLog>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleSource {
    #[serde(rename = "@path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "@host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "@port", skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(rename = "@service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "@channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleLog {
    #[serde(rename = "@file")]
    pub file: String,
    #[serde(rename = "@append", skip_serializing_if = "Option::is_none")]
    pub append: Option<String>,
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
    pub source: Option<ParallelSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<ParallelTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelSource {
    #[serde(rename = "@path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelTarget {
    #[serde(rename = "@port")]
    pub port: u32,
    #[serde(rename = "@path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
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
