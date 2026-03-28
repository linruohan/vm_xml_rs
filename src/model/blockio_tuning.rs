use serde::{Deserialize, Serialize};

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
