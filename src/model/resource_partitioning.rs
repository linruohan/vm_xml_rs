use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcePartitioningConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memnode: Option<String>,
}
