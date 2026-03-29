use serde::{Deserialize, Serialize};

use super::AddressConfig;

/// 主机设备直通配置（Host Device Assignment）
/// 支持 USB、PCI、SCSI、SCSI Host、MDEV 等设备类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostdevConfig {
    /// 设备模式，固定为 "subsystem"
    #[serde(rename = "@mode")]
    pub mode: String,
    /// 设备类型：usb, pci, scsi, scsi_host, mdev
    #[serde(rename = "@type")]
    pub device_type: String,
    /// 是否由 libvirt 自动管理设备分离/附加（仅 PCI 有效）
    #[serde(rename = "@managed", skip_serializing_if = "Option::is_none")]
    pub managed: Option<String>,
    /// 设备模型（仅 mdev 有效）：vfio-pci, vfio-ccw, vfio-ap
    #[serde(rename = "@model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// rawio 属性（仅 scsi 有效）
    #[serde(rename = "@rawio", skip_serializing_if = "Option::is_none")]
    pub rawio: Option<String>,
    /// 是否作为显示设备（仅 mdev/vgpu 有效）
    #[serde(rename = "@display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    /// 是否启用 ramfb（仅 mdev/vgpu 有效）
    #[serde(rename = "@ramfb", skip_serializing_if = "Option::is_none")]
    pub ramfb: Option<String>,
    /// 源设备描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<HostdevSource>,
    /// 引导顺序配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot: Option<super::BootOrderConfig>,
    /// ROM 配置（仅 PCI 有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rom: Option<HostdevROM>,
    /// 设备地址配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressConfig>,
    /// 驱动配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<HostdevDriver>,
    /// 是否只读（仅 SCSI 有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<()>,
    /// 是否可共享（仅 SCSI 有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shareable: Option<()>,
    /// ACPI Generic Initiators 配置（仅 NVIDIA MIG 场景）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acpi: Option<HostdevACPI>,
}

/// 主机设备源配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostdevSource {
    /// 启动策略（仅 USB 有效）：mandatory, requisite, optional
    #[serde(rename = "@startupPolicy", skip_serializing_if = "Option::is_none")]
    pub startup_policy: Option<String>,
    /// guestReset 属性（仅 USB 有效）：on, off, uninitialized
    #[serde(rename = "@guestReset", skip_serializing_if = "Option::is_none")]
    pub guest_reset: Option<String>,
    /// writeFiltering 属性（仅 PCI/Xen 有效）
    #[serde(rename = "@writeFiltering", skip_serializing_if = "Option::is_none")]
    pub write_filtering: Option<String>,
    /// 协议类型（仅 SCSI 有效）：iscsi
    #[serde(rename = "@protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// 网络存储名称（仅 SCSI/iscsi 有效）
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// vhost_scsi WWPN（仅 SCSI_HOST 有效）
    #[serde(rename = "@wwpn", skip_serializing_if = "Option::is_none")]
    pub wwpn: Option<String>,
    /// USB Vendor ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<HostdevVendor>,
    /// USB Product ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<HostdevProduct>,
    /// 设备地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<HostdevAddress>,
    /// SCSI 适配器
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter: Option<HostdevAdapter>,
    /// SCSI 总线地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scsi_address: Option<HostdevSCSIAddress>,
    /// 主机配置（网络存储）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<HostdevHost>,
    /// 认证配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<HostdevAuth>,
    /// 发起者配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<HostdevInitiator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostdevVendor {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostdevProduct {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostdevAddress {
    /// USB 设备地址
    #[serde(rename = "@bus", skip_serializing_if = "Option::is_none")]
    pub bus: Option<String>,
    #[serde(rename = "@device", skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(rename = "@port", skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// PCI 设备地址
    #[serde(rename = "@domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "@slot", skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(rename = "@function", skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    /// MDEV UUID
    #[serde(rename = "@uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostdevAdapter {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostdevSCSIAddress {
    #[serde(rename = "@bus")]
    pub bus: String,
    #[serde(rename = "@target")]
    pub target: String,
    #[serde(rename = "@unit")]
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostdevHost {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@port", skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostdevAuth {
    #[serde(rename = "@username")]
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<HostdevSecret>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostdevSecret {
    #[serde(rename = "@type")]
    pub secret_type: String,
    #[serde(rename = "@usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    #[serde(rename = "@uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostdevInitiator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iqn: Option<HostdevIQN>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostdevIQN {
    #[serde(rename = "@name")]
    pub name: String,
}

/// ROM 配置（PCI 设备）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostdevROM {
    #[serde(rename = "@bar", skip_serializing_if = "Option::is_none")]
    pub bar: Option<String>,
    #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(rename = "@enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
}

/// 驱动配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostdevDriver {
    #[serde(rename = "@model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// iommufd 后端配置（12.1.0+）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iommufd: Option<HostdevIOMMUFD>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostdevIOMMUFD {
    #[serde(rename = "@fd")]
    pub fd: u32,
}

/// ACPI Generic Initiators 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostdevACPI {
    #[serde(rename = "@nodeset")]
    pub nodeset: String,
}
