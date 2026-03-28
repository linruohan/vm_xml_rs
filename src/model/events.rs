use serde::{Deserialize, Serialize};

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
