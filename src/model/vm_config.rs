use serde::{Deserialize, Serialize};

use crate::model::{
    devices::DevicesConfig, BlockIOTuningConfig, CPUConfig, CPUTuningConfig,
    DiskThrottleGroupConfig, EventsConfig, FibreChannelVMIDConfig, GeneralConfig,
    HypervisorFeaturesConfig, IOThreadsConfig, KeyWrapConfig, LaunchSecurityConfig,
    MemoryBackingConfig, MemoryConfig, MemoryTuningConfig, NUMAConfig, NUMATuneConfig,
    OSBootingConfig, PerformanceMonitoringConfig, PowerManagementConfig,
    ResourcePartitioningConfig, SMBIOSConfig, SecurityLabelConfig, TimeKeepingConfig,
};

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
    pub numatune: Option<NUMATuneConfig>,
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

impl VMConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
