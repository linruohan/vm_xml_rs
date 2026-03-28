use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SMBIOSConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SMBIOSSystem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bios: Option<SMBIOSBios>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_board: Option<SMBIOSBaseBoard>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SMBIOSSystem {
    #[serde(rename = "@manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "@product", skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "@serial", skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(rename = "@uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(rename = "@sku", skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "@family", skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SMBIOSBios {
    #[serde(rename = "@vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "@date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SMBIOSBaseBoard {
    #[serde(rename = "@manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "@product", skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "@serial", skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(rename = "@asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "@location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
