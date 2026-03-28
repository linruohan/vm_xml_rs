use serde::{Deserialize, Serialize};

use crate::model::ThrottleConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiskThrottleGroupConfig {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle: Option<ThrottleConfig>,
}
