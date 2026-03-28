use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FibreChannelVMIDConfig {
    #[serde(rename = "@id")]
    pub id: String,
}
