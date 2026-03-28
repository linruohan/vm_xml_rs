use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PowerManagementConfig {
    pub suspend_to_disk: bool,
    pub suspend_to_ram: bool,
    pub autoboot: bool,
}
