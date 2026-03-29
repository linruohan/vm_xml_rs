use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimeKeepingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock: Option<ClockConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtc: Option<RTCConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClockConfig {
    #[serde(rename = "@offset")]
    pub offset: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer: Option<Vec<TimerConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerConfig {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@present", skip_serializing_if = "Option::is_none")]
    pub present: Option<String>,
    #[serde(rename = "@frequency", skip_serializing_if = "Option::is_none")]
    pub frequency: Option<u32>,
    #[serde(rename = "@tickpolicy", skip_serializing_if = "Option::is_none")]
    pub tickpolicy: Option<String>,
    #[serde(rename = "@track", skip_serializing_if = "Option::is_none")]
    pub track: Option<String>,
    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catchup: Option<CatchupConfig>,
}

/// Timer catchup 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatchupConfig {
    #[serde(rename = "@threshold", skip_serializing_if = "Option::is_none")]
    pub threshold: Option<u32>,
    #[serde(rename = "@slew", skip_serializing_if = "Option::is_none")]
    pub slew: Option<u32>,
    #[serde(rename = "@limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RTCConfig {
    #[serde(rename = "@tickpolicy", skip_serializing_if = "Option::is_none")]
    pub tickpolicy: Option<String>,
    #[serde(rename = "@base", skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(rename = "@offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
}
