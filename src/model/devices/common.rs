use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DevicesConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emulator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics: Option<Vec<crate::model::GraphicsConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Vec<crate::model::VideoConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<Vec<crate::model::DiskConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface: Option<Vec<crate::model::InterfaceConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<Vec<crate::model::SerialConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel: Option<Vec<crate::model::ParallelConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console: Option<crate::model::ConsoleConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Vec<crate::model::InputConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<Vec<crate::model::SoundConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Vec<crate::model::ChannelConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchdog: Option<crate::model::WatchdogConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rng: Option<Vec<crate::model::RngConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpm: Option<crate::model::TPMConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memballoon: Option<crate::model::MemballoonConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysinfo: Option<SysInfoConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesystem: Option<Vec<crate::model::FilesystemConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<Vec<crate::model::ControllerConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub: Option<Vec<crate::model::HubConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub panic: Option<crate::model::PanicConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shmem: Option<Vec<crate::model::ShmemConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_device: Option<Vec<crate::model::MemoryDeviceConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iommu: Option<crate::model::IommuConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsock: Option<crate::model::VsockConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto: Option<crate::model::CryptoConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstore: Option<crate::model::PstoreConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<crate::model::AudioConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostdev: Option<Vec<crate::model::HostdevConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysInfoConfig {
    #[serde(rename = "@type")]
    pub sysinfo_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bios: Option<SMBIOSBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SMBIOSBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_board: Option<Vec<SMBIOSBlock>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chassis: Option<SMBIOSBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oem_strings: Option<Vec<OemString>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<Vec<SysInfoEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMBIOSBlock {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<Vec<SMBIOSEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMBIOSEntry {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OemString {
    #[serde(rename = "$")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysInfoEntry {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(rename = "$", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressConfig {
    #[serde(rename = "@type")]
    pub address_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bus: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multifunction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cssid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devno: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iobase: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irq: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasConfig {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootOrderConfig {
    #[serde(rename = "@order")]
    pub order: u32,
}
