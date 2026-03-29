use serde::{Deserialize, Serialize};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associativity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<CacheSizeConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<CacheLineConfig>,
}

/// CPU Cache 大小配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSizeConfig {
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "$value")]
    pub value: u32,
}

/// CPU Cache 行大小配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheLineConfig {
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "$value")]
    pub value: u32,
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
