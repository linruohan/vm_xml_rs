use serde::{Deserialize, Serialize};

use super::{AddressConfig, AliasConfig, BootOrderConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacAddress {
    #[serde(rename = "@address")]
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceSource {
    #[serde(rename = "@bridge", skip_serializing_if = "Option::is_none")]
    pub bridge: Option<String>,
    #[serde(rename = "@network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(rename = "@dev", skip_serializing_if = "Option::is_none")]
    pub dev: Option<String>,
    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceModel {
    #[serde(rename = "@type")]
    pub model_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceConfig {
    #[serde(rename = "@type")]
    pub interface_type: String,
    #[serde(rename = "@trustGuestRxFilters", skip_serializing_if = "Option::is_none")]
    pub trust_guest_rx_filters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<MacAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<InterfaceSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<InterfaceModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<AliasConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot: Option<BootOrderConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressConfig>,
}
