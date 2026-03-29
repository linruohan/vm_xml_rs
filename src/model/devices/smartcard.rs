use serde::{Deserialize, Serialize};

use super::AddressConfig;

/// 智能卡设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartcardConfig {
    /// 模式：host, host-certificates, passthrough
    #[serde(rename = "@mode")]
    pub mode: String,
    /// 类型（仅 passthrough 模式有效）：tcp, spicevmc
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub smartcard_type: Option<String>,
    /// 证书列表（仅 host-certificates 模式）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Vec<String>>,
    /// 证书数据库路径（仅 host-certificates 模式）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// 源配置（仅 passthrough 模式）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SmartcardSource>,
    /// 协议配置（仅 passthrough 模式）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<SmartcardProtocol>,
    /// 地址配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressConfig>,
}

/// 智能卡源配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartcardSource {
    /// 模式：bind, connect
    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 主机地址
    #[serde(rename = "@host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// 服务/端口
    #[serde(rename = "@service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

/// 智能卡协议配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartcardProtocol {
    /// 协议类型：raw
    #[serde(rename = "@type")]
    pub protocol_type: String,
}
