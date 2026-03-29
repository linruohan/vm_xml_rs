use serde::{Deserialize, Serialize};

/// 输入设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
    #[serde(rename = "@type")]
    pub input_type: String,
    #[serde(rename = "@bus", skip_serializing_if = "Option::is_none")]
    pub bus: Option<String>,
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<InputSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<InputDriver>,
}

/// 输入设备源配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputSource {
    #[serde(rename = "@dev", skip_serializing_if = "Option::is_none")]
    pub dev: Option<String>,
    #[serde(rename = "@grab", skip_serializing_if = "Option::is_none")]
    pub grab: Option<String>,
    #[serde(rename = "@repeat", skip_serializing_if = "Option::is_none")]
    pub repeat: Option<String>,
    #[serde(rename = "@grabToggle", skip_serializing_if = "Option::is_none")]
    pub grab_toggle: Option<String>,
}

/// 输入设备驱动配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputDriver {
    #[serde(rename = "@queues", skip_serializing_if = "Option::is_none")]
    pub queues: Option<u32>,
    #[serde(rename = "@ioeventfd", skip_serializing_if = "Option::is_none")]
    pub ioeventfd: Option<String>,
    #[serde(rename = "@event_idx", skip_serializing_if = "Option::is_none")]
    pub event_idx: Option<String>,
}

/// 声音设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundConfig {
    #[serde(rename = "@model")]
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<SoundCodec>,
}

/// 声音设备编解码器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundCodec {
    #[serde(rename = "@type")]
    pub codec_type: String,
    #[serde(rename = "@input-type", skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(rename = "@output-type", skip_serializing_if = "Option::is_none")]
    pub output_type: Option<String>,
}

// ChannelConfig 和 ChannelTarget 已移至 common.rs

/// Watchdog 设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchdogConfig {
    #[serde(rename = "@model")]
    pub model: String,
    #[serde(rename = "@action")]
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<crate::model::devices::AddressConfig>,
}

/// RNG (随机数生成器) 设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<RngBackend>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<RngSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<RngRate>,
}

/// RNG 后端配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngBackend {
    #[serde(rename = "@model")]
    pub model: String,
    #[serde(rename = "@type")]
    pub rng_type: String,
    #[serde(rename = "@device", skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
}

/// RNG size 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngSize {
    #[serde(rename = "$")]
    pub value: u64,
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// RNG rate 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngRate {
    #[serde(rename = "$")]
    pub value: u64,
    #[serde(rename = "@period", skip_serializing_if = "Option::is_none")]
    pub period: Option<u64>,
    #[serde(rename = "@bytes", skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
}

/// TPM 设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TPMConfig {
    #[serde(rename = "@model")]
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<TPMBackend>,
}

/// TPM 后端配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TPMBackend {
    #[serde(rename = "@type")]
    pub backend_type: String,
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "@device", skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(rename = "@model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// 内存气球设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemballoonConfig {
    #[serde(rename = "@model")]
    pub model: String,
    #[serde(rename = "@autodeflate", skip_serializing_if = "Option::is_none")]
    pub autodeflate: Option<String>,
    #[serde(rename = "@period", skip_serializing_if = "Option::is_none")]
    pub period: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<MemballoonStats>,
}

/// 内存气球统计配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemballoonStats {
    #[serde(rename = "@period", skip_serializing_if = "Option::is_none")]
    pub period: Option<u32>,
}
