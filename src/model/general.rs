use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::{MemoryInfo, MetadataConfig, OSSystem, VCPUConfig, VCPUInfo};

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

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            vm_type: "kvm".to_string(),
            name: "vm0".to_string(),
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
            os: Some(OSSystem::default()),
            vcpus: None,
            bootloader: None,
            bootloader_args: None,
        }
    }
}
