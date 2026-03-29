use serde::{Deserialize, Serialize};

use super::AddressConfig;

/// Hub 设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubConfig {
    #[serde(rename = "@type")]
    pub hub_type: String,
}

/// Panic 设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanicConfig {
    #[serde(rename = "@model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressConfig>,
}

/// 共享内存设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShmemConfig {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<ShmemModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<SizeConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<ShmemServer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msi: Option<MsiConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShmemModel {
    #[serde(rename = "@type")]
    pub model_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShmemServer {
    #[serde(rename = "@path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsiConfig {
    #[serde(rename = "@vectors", skip_serializing_if = "Option::is_none")]
    pub vectors: Option<u32>,
    #[serde(rename = "@ioeventfd", skip_serializing_if = "Option::is_none")]
    pub ioeventfd: Option<String>,
}

/// 内存设备配置 (dimm/nvdimm)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDeviceConfig {
    #[serde(rename = "@model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "@access", skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[serde(rename = "@discard", skip_serializing_if = "Option::is_none")]
    pub discard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<MemoryDeviceSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<MemoryDeviceTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDeviceSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagesize: Option<SizeConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodemask: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDeviceTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<SizeConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<LabelConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelConfig {
    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// IOMMU 设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IommuConfig {
    #[serde(rename = "@model")]
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<IommuDriver>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IommuDriver {
    #[serde(rename = "@intremap", skip_serializing_if = "Option::is_none")]
    pub intremap: Option<String>,
    #[serde(rename = "@caching_mode", skip_serializing_if = "Option::is_none")]
    pub caching_mode: Option<String>,
    #[serde(rename = "@api_mode", skip_serializing_if = "Option::is_none")]
    pub api_mode: Option<String>,
    #[serde(rename = "@ats", skip_serializing_if = "Option::is_none")]
    pub ats: Option<String>,
    #[serde(rename = "@aw_bits", skip_serializing_if = "Option::is_none")]
    pub aw_bits: Option<String>,
    #[serde(rename = "@snoop_wb", skip_serializing_if = "Option::is_none")]
    pub snoop_wb: Option<String>,
    #[serde(rename = "@x2apic_scale", skip_serializing_if = "Option::is_none")]
    pub x2apic_scale: Option<String>,
}

/// Vsock 设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VsockConfig {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<VsockSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VsockSource {
    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "@path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "@cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<u32>,
    #[serde(rename = "@auto", skip_serializing_if = "Option::is_none")]
    pub auto: Option<String>,
}

/// Crypto 设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoConfig {
    #[serde(rename = "@type")]
    pub crypto_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<CryptoBackend>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoBackend {
    #[serde(rename = "@type")]
    pub backend_type: String,
    #[serde(rename = "@node", skip_serializing_if = "Option::is_none")]
    pub node: Option<u32>,
}

/// Pstore 设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PstoreConfig {
    #[serde(rename = "@path")]
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<SizeConfig>,
}

/// Audio 设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<AudioStream>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<AudioStream>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioStream {
    #[serde(rename = "@type")]
    pub stream_type: String,
    #[serde(rename = "@server", skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@device", skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(rename = "@format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "@global", skip_serializing_if = "Option::is_none")]
    pub global: Option<String>,
}

/// Size 配置（用于多个设备类型）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeConfig {
    #[serde(rename = "$", skip_serializing_if = "Option::is_none")]
    pub value: Option<u64>,
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
