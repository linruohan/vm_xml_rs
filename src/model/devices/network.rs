use serde::{Deserialize, Serialize};

use super::{AddressConfig, AliasConfig, BootOrderConfig};

/// MAC 地址配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacAddress {
    #[serde(rename = "@address")]
    pub address: String,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub mac_type: Option<String>,
    #[serde(rename = "@currentAddress", skip_serializing_if = "Option::is_none")]
    pub current_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceSource {
    #[serde(rename = "@bridge", skip_serializing_if = "Option::is_none")]
    pub bridge: Option<String>,
    #[serde(rename = "@network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(rename = "@dev", skip_serializing_if = "Option::is_none")]
    pub dev: Option<String>,
    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceModel {
    #[serde(rename = "@type")]
    pub model_type: String,
}

/// 带宽配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound: Option<DirectionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound: Option<DirectionConfig>,
}

/// 方向带宽配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectionConfig {
    #[serde(rename = "@average", skip_serializing_if = "Option::is_none")]
    pub average: Option<u64>,
    #[serde(rename = "@peak", skip_serializing_if = "Option::is_none")]
    pub peak: Option<u64>,
    #[serde(rename = "@burst", skip_serializing_if = "Option::is_none")]
    pub burst: Option<u64>,
}

/// 虚拟端口配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualPortConfig {
    #[serde(rename = "@type")]
    pub port_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<VirtualPortParameters>,
}

/// 虚拟端口参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualPortParameters {
    #[serde(rename = "@interfaceid", skip_serializing_if = "Option::is_none")]
    pub interfaceid: Option<String>,
    #[serde(rename = "@profileid", skip_serializing_if = "Option::is_none")]
    pub profileid: Option<String>,
    #[serde(rename = "@instanceid", skip_serializing_if = "Option::is_none")]
    pub instanceid: Option<String>,
}

/// 链接状态配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkConfig {
    #[serde(rename = "@state")]
    pub state: String,
}

/// ROM 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RomConfig {
    #[serde(rename = "@bar")]
    pub bar: String,
    #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

/// ACPI 索引配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpiConfig {
    #[serde(rename = "@index")]
    pub index: u32,
}

/// 后端配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    #[serde(rename = "@tap", skip_serializing_if = "Option::is_none")]
    pub tap: Option<String>,
    #[serde(rename = "@vhost", skip_serializing_if = "Option::is_none")]
    pub vhost: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub backend_type: Option<String>,
    #[serde(rename = "@logFile", skip_serializing_if = "Option::is_none")]
    pub log_file: Option<String>,
    #[serde(rename = "@hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "@fqdn", skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
}

/// 驱动主机卸载配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverHostOffload {
    #[serde(rename = "@csum", skip_serializing_if = "Option::is_none")]
    pub csum: Option<String>,
    #[serde(rename = "@gso", skip_serializing_if = "Option::is_none")]
    pub gso: Option<String>,
    #[serde(rename = "@tso4", skip_serializing_if = "Option::is_none")]
    pub tso4: Option<String>,
    #[serde(rename = "@tso6", skip_serializing_if = "Option::is_none")]
    pub tso6: Option<String>,
    #[serde(rename = "@ecn", skip_serializing_if = "Option::is_none")]
    pub ecn: Option<String>,
    #[serde(rename = "@ufo", skip_serializing_if = "Option::is_none")]
    pub ufo: Option<String>,
    #[serde(rename = "@mrg_rxbuf", skip_serializing_if = "Option::is_none")]
    pub mrg_rxbuf: Option<String>,
}

/// 驱动访客卸载配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverGuestOffload {
    #[serde(rename = "@csum", skip_serializing_if = "Option::is_none")]
    pub csum: Option<String>,
    #[serde(rename = "@tso4", skip_serializing_if = "Option::is_none")]
    pub tso4: Option<String>,
    #[serde(rename = "@tso6", skip_serializing_if = "Option::is_none")]
    pub tso6: Option<String>,
    #[serde(rename = "@ecn", skip_serializing_if = "Option::is_none")]
    pub ecn: Option<String>,
    #[serde(rename = "@ufo", skip_serializing_if = "Option::is_none")]
    pub ufo: Option<String>,
}

/// 驱动配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverConfig {
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@txmode", skip_serializing_if = "Option::is_none")]
    pub txmode: Option<String>,
    #[serde(rename = "@ioeventfd", skip_serializing_if = "Option::is_none")]
    pub ioeventfd: Option<String>,
    #[serde(rename = "@event_idx", skip_serializing_if = "Option::is_none")]
    pub event_idx: Option<String>,
    #[serde(rename = "@queues", skip_serializing_if = "Option::is_none")]
    pub queues: Option<u32>,
    #[serde(rename = "@rx_queue_size", skip_serializing_if = "Option::is_none")]
    pub rx_queue_size: Option<u32>,
    #[serde(rename = "@tx_queue_size", skip_serializing_if = "Option::is_none")]
    pub tx_queue_size: Option<u32>,
    #[serde(rename = "@rss", skip_serializing_if = "Option::is_none")]
    pub rss: Option<String>,
    #[serde(rename = "@rss_hash_report", skip_serializing_if = "Option::is_none")]
    pub rss_hash_report: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<DriverHostOffload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest: Option<DriverGuestOffload>,
}

/// 调优配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuneConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sndbuf: Option<u64>,
}

/// 访客设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestConfig {
    #[serde(rename = "@dev")]
    pub dev: String,
}

/// VLAN 标签配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VlanTag {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@nativeMode", skip_serializing_if = "Option::is_none")]
    pub native_mode: Option<String>,
}

/// VLAN 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VlanConfig {
    #[serde(rename = "@trunk", skip_serializing_if = "Option::is_none")]
    pub trunk: Option<String>,
    #[serde(rename = "tag")]
    pub tags: Vec<VlanTag>,
}

/// 端口配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortConfig {
    #[serde(rename = "@isolated")]
    pub isolated: String,
}

/// IP 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpConfig {
    #[serde(rename = "@family")]
    pub family: String,
    #[serde(rename = "@address")]
    pub address: String,
    #[serde(rename = "@prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<u32>,
}

/// 端口范围配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortRangeConfig {
    #[serde(rename = "@start")]
    pub start: u32,
    #[serde(rename = "@end", skip_serializing_if = "Option::is_none")]
    pub end: Option<u32>,
    #[serde(rename = "@to", skip_serializing_if = "Option::is_none")]
    pub to: Option<u32>,
    #[serde(rename = "@exclude", skip_serializing_if = "Option::is_none")]
    pub exclude: Option<String>,
}

/// 端口转发配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortForwardConfig {
    #[serde(rename = "@proto")]
    pub proto: String,
    #[serde(rename = "@address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "@dev", skip_serializing_if = "Option::is_none")]
    pub dev: Option<String>,
    #[serde(rename = "range")]
    pub ranges: Vec<PortRangeConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceConfig {
    #[serde(rename = "@type")]
    pub interface_type: String,
    #[serde(rename = "@trustGuestRxFilters", skip_serializing_if = "Option::is_none")]
    pub trust_guest_rx_filters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<MacAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<InterfaceSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<InterfaceModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<AliasConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot: Option<BootOrderConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<BandwidthConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtualport: Option<VirtualPortConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<LinkConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// ROM 配置 (rom bar='on' file='/path/to/rom.bin')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rom: Option<RomConfig>,
    /// ACPI 索引配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acpi: Option<AcpiConfig>,
    /// 后端配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<BackendConfig>,
    /// 驱动配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<DriverConfig>,
    /// 调优配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tune: Option<TuneConfig>,
    /// 访客设备名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest: Option<GuestConfig>,
    /// 端口组
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portgroup: Option<String>,
    /// VLAN 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<VlanConfig>,
    /// 端口隔离配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<PortConfig>,
    /// IP 配置 (用于 user/passt 类型)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<IpConfig>,
    /// 端口转发配置 (用于 passt 后端)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_forward: Option<Vec<PortForwardConfig>>,
}
