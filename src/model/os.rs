use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OSBootingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_menu: Option<BootMenuConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootMenuConfig {
    #[serde(rename = "@enable")]
    pub enable: String,
    #[serde(rename = "@timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
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

impl Default for OSSystem {
    fn default() -> Self {
        Self {
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
        }
    }
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
