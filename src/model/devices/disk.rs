use serde::{Deserialize, Serialize};

use super::{AddressConfig, AliasConfig, BootOrderConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskConfig {
    #[serde(rename = "@type")]
    pub disk_type: String,
    #[serde(rename = "@device")]
    pub device: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<DiskDriver>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<DiskSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<DiskTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<GeometryConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockio: Option<BlockIOConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iotune: Option<IOtuneConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backenddomain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttlefilters: Option<Vec<ThrottleFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<AliasConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot: Option<BootOrderConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shareable: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transient: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wwn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlockIOConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_block_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_block_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discard_granularity: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IOtuneConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes_sec: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_bytes_sec: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_bytes_sec: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_iops_sec: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_iops_sec: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_iops_sec: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThrottleFilter {
    #[serde(rename = "@group")]
    pub group: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometryConfig {
    #[serde(rename = "@cyls")]
    pub cyls: u32,
    #[serde(rename = "@heads")]
    pub heads: u32,
    #[serde(rename = "@secs")]
    pub secs: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    #[serde(rename = "@type")]
    pub encryption_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<EncryptionSecret>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionSecret {
    #[serde(rename = "@type")]
    pub secret_type: String,
    #[serde(rename = "@usage")]
    pub usage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskDriver {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub driver_type: String,
    #[serde(rename = "@cache", skip_serializing_if = "Option::is_none")]
    pub cache: Option<String>,
    #[serde(rename = "@io", skip_serializing_if = "Option::is_none")]
    pub io: Option<String>,
    #[serde(rename = "@ioeventfd", skip_serializing_if = "Option::is_none")]
    pub ioeventfd: Option<String>,
    #[serde(rename = "@event_idx", skip_serializing_if = "Option::is_none")]
    pub event_idx: Option<String>,
    #[serde(rename = "@queues", skip_serializing_if = "Option::is_none")]
    pub queues: Option<u32>,
    #[serde(rename = "@queue_size", skip_serializing_if = "Option::is_none")]
    pub queue_size: Option<u32>,
    #[serde(rename = "@iothread", skip_serializing_if = "Option::is_none")]
    pub iothread: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iothreads: Option<Vec<DiskIOThread>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<DiskStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_histogram: Option<Vec<LatencyHistogramConfig>>,
    #[serde(rename = "@discard_no_unref", skip_serializing_if = "Option::is_none")]
    pub discard_no_unref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskIOThread {
    #[serde(rename = "@id")]
    pub id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskStatistics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<Vec<StatisticInterval>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_histogram: Option<Vec<LatencyHistogramConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticInterval {
    #[serde(rename = "@interval")]
    pub interval: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyHistogramConfig {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub histogram_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<Vec<HistogramBin>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramBin {
    #[serde(rename = "@start")]
    pub start: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskSource {
    #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(rename = "@dev", skip_serializing_if = "Option::is_none")]
    pub dev: Option<String>,
    #[serde(rename = "@protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@startupPolicy", skip_serializing_if = "Option::is_none")]
    pub startup_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<DiskSourceHost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<DiskAuth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seclabel: Option<Vec<DiskSecLabel>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskSourceHost {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@port", skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskAuth {
    #[serde(rename = "@type")]
    pub auth_type: String,
    #[serde(rename = "@username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<DiskSecret>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskSecret {
    #[serde(rename = "@type")]
    pub secret_type: String,
    #[serde(rename = "@usage")]
    pub usage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskSecLabel {
    #[serde(rename = "@type")]
    pub sec_type: String,
    #[serde(rename = "@model")]
    pub model: String,
    #[serde(rename = "@relabel", skip_serializing_if = "Option::is_none")]
    pub relabel: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskTarget {
    #[serde(rename = "@dev")]
    pub dev: String,
    #[serde(rename = "@bus", skip_serializing_if = "Option::is_none")]
    pub bus: Option<String>,
    #[serde(rename = "@tray", skip_serializing_if = "Option::is_none")]
    pub tray: Option<String>,
    #[serde(rename = "@rotation_rate", skip_serializing_if = "Option::is_none")]
    pub rotation_rate: Option<u32>,
}
