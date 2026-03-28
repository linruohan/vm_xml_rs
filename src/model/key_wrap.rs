use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeyWrapConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_key: Option<MasterKeyConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterKeyConfig {
    #[serde(rename = "@type")]
    pub key_type: String,
    #[serde(rename = "@uri")]
    pub uri: String,
}
