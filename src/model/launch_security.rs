use serde::{Deserialize, Serialize};

use crate::model::{SecurityLabelConfig, TPMConfig};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LaunchSecurityConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seclabel: Option<SecurityLabelConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpm: Option<TPMConfig>,
}
