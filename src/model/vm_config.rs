use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VMConfig {
    pub general: GeneralConfig,
    pub os_booting: OSBootingConfig,
    pub cpu: CPUConfig,
    pub memory: MemoryConfig,
    pub devices: DevicesConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smbios: Option<SMBIOSConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iothreads: Option<IOThreadsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_tuning: Option<CPUTuningConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_backing: Option<MemoryBackingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_tuning: Option<MemoryTuningConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numa: Option<NUMAConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockio_tuning: Option<BlockIOTuningConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_partitioning: Option<ResourcePartitioningConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fibre_channel_vmid: Option<FibreChannelVMIDConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<EventsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_management: Option<PowerManagementConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_throttle_group: Option<DiskThrottleGroupConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypervisor_features: Option<HypervisorFeaturesConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_keeping: Option<TimeKeepingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_monitoring: Option<PerformanceMonitoringConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_label: Option<SecurityLabelConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_wrap: Option<KeyWrapConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_security: Option<LaunchSecurityConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(rename = "@type")]
    pub vm_type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwuuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataConfig>,
    pub vcpu: VCPUInfo,
    pub memory: MemoryInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_memory: Option<MemoryInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_memory: Option<MemoryInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<OSSystem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<Vec<VCPUConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootloader: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootloader_args: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VCPUConfig {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@enabled")]
    pub enabled: String,
    #[serde(rename = "@hotpluggable")]
    pub hotpluggable: String,
    #[serde(rename = "@order", skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataConfig {
    #[serde(flatten)]
    pub entries: Vec<MetadataEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataEntry {
    #[serde(rename = "@xmlns")]
    pub xmlns: String,
    #[serde(rename = "$")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VCPUInfo {
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<String>,
    #[serde(rename = "@cpuset", skip_serializing_if = "Option::is_none")]
    pub cpuset: Option<String>,
    #[serde(rename = "@current", skip_serializing_if = "Option::is_none")]
    pub current: Option<u32>,
    #[serde(rename = "$value")]
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "@slots", skip_serializing_if = "Option::is_none")]
    pub slots: Option<u32>,
    #[serde(rename = "@dumpCore", skip_serializing_if = "Option::is_none")]
    pub dump_core: Option<String>,
    #[serde(rename = "$value")]
    pub value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSSystem {
    #[serde(rename = "@type")]
    pub os_type: String,
    #[serde(rename = "@arch", skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(rename = "@machine", skip_serializing_if = "Option::is_none")]
    pub machine: Option<String>,
    #[serde(rename = "@firmware", skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loader: Option<LoaderConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nvram: Option<Vec<NVRAMConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub varstore: Option<VarStoreConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot: Option<Vec<BootConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootmenu: Option<BootMenuConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smbios: Option<SMBIOSModeConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bios: Option<BIOSConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initrd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmdline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shim: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initarg: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initenv: Option<Vec<InitEnvConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initdir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inituser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initgroup: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idmap: Option<IdMapConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acpi: Option<ACPIConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<Vec<FeatureConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoaderConfig {
    #[serde(rename = "@readonly", skip_serializing_if = "Option::is_none")]
    pub readonly: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub loader_type: Option<String>,
    #[serde(rename = "@secure", skip_serializing_if = "Option::is_none")]
    pub secure: Option<String>,
    #[serde(rename = "@stateless", skip_serializing_if = "Option::is_none")]
    pub stateless: Option<String>,
    #[serde(rename = "@format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "$value")]
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NVRAMConfig {
    #[serde(rename = "@template", skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(rename = "@templateFormat", skip_serializing_if = "Option::is_none")]
    pub template_format: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub nvram_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<NVSourceConfig>,
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NVSourceConfig {
    #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(rename = "@dev", skip_serializing_if = "Option::is_none")]
    pub dev: Option<String>,
    #[serde(rename = "@protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<HostConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<AuthConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostConfig {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@port", skip_serializing_if = "Option::is_none")]
    pub port: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    #[serde(rename = "@username")]
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<SecretConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretConfig {
    #[serde(rename = "@type")]
    pub secret_type: String,
    #[serde(rename = "@usage")]
    pub usage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarStoreConfig {
    #[serde(rename = "@path")]
    pub path: String,
    #[serde(rename = "@template", skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootConfig {
    #[serde(rename = "@dev")]
    pub dev: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootMenuConfig {
    #[serde(rename = "@enable")]
    pub enable: String,
    #[serde(rename = "@timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMBIOSModeConfig {
    #[serde(rename = "@mode")]
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BIOSConfig {
    #[serde(rename = "@useserial", skip_serializing_if = "Option::is_none")]
    pub useserial: Option<String>,
    #[serde(rename = "@rebootTimeout", skip_serializing_if = "Option::is_none")]
    pub reboot_timeout: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitEnvConfig {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdMapConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<IdMapEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<IdMapEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdMapEntry {
    #[serde(rename = "@start")]
    pub start: u32,
    #[serde(rename = "@target")]
    pub target: u32,
    #[serde(rename = "@count")]
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACPIConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Vec<ACITableConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACITableConfig {
    #[serde(rename = "@type")]
    pub table_type: String,
    #[serde(rename = "$")]
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureConfig {
    #[serde(rename = "@enabled")]
    pub enabled: String,
    #[serde(rename = "@name")]
    pub name: String,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            vm_type: "kvm".to_string(),
            name: "new-vm".to_string(),
            uuid: Some(Uuid::new_v4().to_string()),
            hwuuid: None,
            genid: None,
            description: None,
            title: None,
            metadata: None,
            vcpu: VCPUInfo {
                placement: Some("static".to_string()),
                cpuset: None,
                current: None,
                count: 2,
            },
            memory: MemoryInfo {
                unit: Some("GiB".to_string()),
                slots: None,
                dump_core: None,
                value: 4,
            },
            max_memory: None,
            current_memory: None,
            os: Some(OSSystem {
                os_type: "hvm".to_string(),
                arch: Some("x86_64".to_string()),
                machine: Some("q35".to_string()),
                firmware: None,
                loader: None,
                nvram: None,
                varstore: None,
                boot: Some(vec![BootConfig { dev: "hd".to_string() }]),
                bootmenu: None,
                smbios: None,
                bios: None,
                kernel: None,
                initrd: None,
                cmdline: None,
                shim: None,
                dtb: None,
                init: None,
                initarg: None,
                initenv: None,
                initdir: None,
                inituser: None,
                initgroup: None,
                idmap: None,
                acpi: None,
                feature: None,
            }),
            vcpus: None,
            bootloader: None,
            bootloader_args: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OSBootingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_menu: Option<BootMenuConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CPUConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology: Option<CPUTopology>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<CPUModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUTopology {
    #[serde(rename = "@sockets")]
    pub sockets: u32,
    #[serde(rename = "@dies")]
    pub dies: u32,
    #[serde(rename = "@cores")]
    pub cores: u32,
    #[serde(rename = "@threads")]
    pub threads: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUModel {
    #[serde(rename = "@fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<String>,
    #[serde(rename = "$value")]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hugepages: Option<HugepagesConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nosharepages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HugepagesConfig {
    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "@nodeset", skip_serializing_if = "Option::is_none")]
    pub nodeset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<Vec<PageConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageConfig {
    #[serde(rename = "@size")]
    pub size: String,
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "@nodeset", skip_serializing_if = "Option::is_none")]
    pub nodeset: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DevicesConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emulator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics: Option<Vec<GraphicsConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Vec<VideoConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<Vec<DiskConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface: Option<Vec<InterfaceConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<Vec<SerialConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console: Option<ConsoleConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Vec<InputConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpm: Option<TPMConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memballoon: Option<MemballoonConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysinfo: Option<SysInfoConfig>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskSource {
    #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(rename = "@dev", skip_serializing_if = "Option::is_none")]
    pub dev: Option<String>,
    #[serde(rename = "@protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskTarget {
    #[serde(rename = "@dev")]
    pub dev: String,
    #[serde(rename = "@bus", skip_serializing_if = "Option::is_none")]
    pub bus: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceConfig {
    #[serde(rename = "@type")]
    pub interface_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<MacAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<InterfaceSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<InterfaceModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacAddress {
    #[serde(rename = "@address")]
    pub address: String,
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
pub struct InputConfig {
    #[serde(rename = "@type")]
    pub input_type: String,
    #[serde(rename = "@bus", skip_serializing_if = "Option::is_none")]
    pub bus: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TPMConfig {
    #[serde(rename = "@model")]
    pub model: String,
    pub backend: TPMBackend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TPMBackend {
    #[serde(rename = "@type")]
    pub backend_type: String,
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemballoonConfig {
    #[serde(rename = "@model")]
    pub model: String,
    #[serde(rename = "@autodeflate", skip_serializing_if = "Option::is_none")]
    pub autodeflate: Option<String>,
    #[serde(rename = "@period", skip_serializing_if = "Option::is_none")]
    pub period: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SMBIOSConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SMBIOSSystem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bios: Option<SMBIOSBios>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_board: Option<SMBIOSBaseBoard>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SMBIOSSystem {
    #[serde(rename = "@manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "@product", skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "@serial", skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(rename = "@uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(rename = "@sku", skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "@family", skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SMBIOSBios {
    #[serde(rename = "@vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "@date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SMBIOSBaseBoard {
    #[serde(rename = "@manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "@product", skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "@serial", skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(rename = "@asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "@location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IOThreadsConfig {
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iothreadids: Option<Vec<IOThread>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaultiothread: Option<DefaultIOThread>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOThread {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@thread_pool_min", skip_serializing_if = "Option::is_none")]
    pub thread_pool_min: Option<u32>,
    #[serde(rename = "@thread_pool_max", skip_serializing_if = "Option::is_none")]
    pub thread_pool_max: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<PollConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollConfig {
    #[serde(rename = "@max")]
    pub max: u32,
    #[serde(rename = "@grow", skip_serializing_if = "Option::is_none")]
    pub grow: Option<u32>,
    #[serde(rename = "@shrink", skip_serializing_if = "Option::is_none")]
    pub shrink: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultIOThread {
    #[serde(rename = "@thread_pool_min", skip_serializing_if = "Option::is_none")]
    pub thread_pool_min: Option<u32>,
    #[serde(rename = "@thread_pool_max", skip_serializing_if = "Option::is_none")]
    pub thread_pool_max: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CPUTuningConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpupin: Option<Vec<VCPUPin>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emulatorpin: Option<EmulatorPin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iothreadpin: Option<Vec<IOThreadPin>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_quota: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emulator_period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emulator_quota: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iothread_period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iothread_quota: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpusched: Option<Vec<VCpuschedConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iothreadsched: Option<Vec<IOThreadschedConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emulatorsched: Option<EmulatorschedConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cachetune: Option<Vec<CachetuneConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memorytune: Option<Vec<MemorytuneConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VCPUPin {
    #[serde(rename = "@vcpu")]
    pub vcpu: u32,
    #[serde(rename = "@cpuset")]
    pub cpuset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulatorPin {
    #[serde(rename = "@cpuset")]
    pub cpuset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOThreadPin {
    #[serde(rename = "@iothread")]
    pub iothread: u32,
    #[serde(rename = "@cpuset")]
    pub cpuset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VCpuschedConfig {
    #[serde(rename = "@vcpus")]
    pub vcpus: String,
    #[serde(rename = "@scheduler")]
    pub scheduler: String,
    #[serde(rename = "@priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOThreadschedConfig {
    #[serde(rename = "@iothreads")]
    pub iothreads: String,
    #[serde(rename = "@scheduler")]
    pub scheduler: String,
    #[serde(rename = "@priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulatorschedConfig {
    #[serde(rename = "@scheduler")]
    pub scheduler: String,
    #[serde(rename = "@priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachetuneConfig {
    #[serde(rename = "@vcpus")]
    pub vcpus: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<Vec<CacheConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<Vec<MonitorConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@level")]
    pub level: u32,
    #[serde(rename = "@type")]
    pub cache_type: String,
    #[serde(rename = "@size")]
    pub size: u32,
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfig {
    #[serde(rename = "@level")]
    pub level: u32,
    #[serde(rename = "@vcpus")]
    pub vcpus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorytuneConfig {
    #[serde(rename = "@vcpus")]
    pub vcpus: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<Vec<NodeConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@bandwidth")]
    pub bandwidth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryBackingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hugepages: Option<HugepagesConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nosharepages: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<MemorySource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<MemoryAccess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation: Option<MemoryAllocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discard: Option<()>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySource {
    #[serde(rename = "@type")]
    pub source_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAccess {
    #[serde(rename = "@mode")]
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAllocation {
    #[serde(rename = "@mode")]
    pub mode: String,
    #[serde(rename = "@threads", skip_serializing_if = "Option::is_none")]
    pub threads: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NUMAConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell: Option<Vec<NUMACell>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NUMACell {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@cpus")]
    pub cpus: String,
    #[serde(rename = "@memory")]
    pub memory: u64,
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memnode: Option<Vec<MemNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemNode {
    #[serde(rename = "@cellid")]
    pub cellid: u32,
    #[serde(rename = "@mode")]
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryTuningConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_hard_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantee: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlockIOTuningConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_weight: Option<Vec<DeviceWeight>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle: Option<ThrottleConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceWeight {
    #[serde(rename = "@dev")]
    pub dev: String,
    #[serde(rename = "@weight")]
    pub weight: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThrottleConfig {
    #[serde(rename = "@read_bytes_sec", skip_serializing_if = "Option::is_none")]
    pub read_bytes_sec: Option<u64>,
    #[serde(rename = "@write_bytes_sec", skip_serializing_if = "Option::is_none")]
    pub write_bytes_sec: Option<u64>,
    #[serde(rename = "@read_iops_sec", skip_serializing_if = "Option::is_none")]
    pub read_iops_sec: Option<u64>,
    #[serde(rename = "@write_iops_sec", skip_serializing_if = "Option::is_none")]
    pub write_iops_sec: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcePartitioningConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memnode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FibreChannelVMIDConfig {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_poweroff: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_reboot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_crash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_lockfailure: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PowerManagementConfig {
    pub suspend_to_disk: bool,
    pub suspend_to_ram: bool,
    pub autoboot: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiskThrottleGroupConfig {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle: Option<ThrottleConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HypervisorFeaturesConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<Vec<FeatureConfig>>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMonitoringConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmu: Option<PMUConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<EventConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PMUConfig {
    #[serde(rename = "@state")]
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventConfig {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityLabelConfig {
    #[serde(rename = "@type")]
    pub label_type: String,
    #[serde(rename = "@model")]
    pub model: String,
    #[serde(rename = "@relabel", skip_serializing_if = "Option::is_none")]
    pub relabel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeyWrapConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_key: Option<MasterKeyConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterKeyConfig {
    #[serde(rename = "@type")]
    pub key_type: String,
    #[serde(rename = "@uri")]
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LaunchSecurityConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seclabel: Option<SecurityLabelConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpm: Option<TPMConfig>,
}

impl VMConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
