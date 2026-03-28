use serde::{Deserialize, Serialize};

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
