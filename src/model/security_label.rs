use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityLabelConfig {
    #[serde(rename = "@type")]
    pub label_type: String,
    #[serde(rename = "@model")]
    pub model: String,
    #[serde(rename = "@relabel", skip_serializing_if = "Option::is_none")]
    pub relabel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baselabel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imagelabel: Option<String>,
}
