use serde::{Deserialize, Serialize};

use super::common::{AddressConfig, AliasConfig, BootOrderConfig};

/// USB 重定向设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirdevConfig {
    /// USB 总线类型（必须为 usb）
    #[serde(rename = "@bus")]
    pub bus: String,
    /// 重定向类型
    #[serde(rename = "@type")]
    pub redir_type: String, // spicevmc, tcp

    /// 源配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<RedirdevSource>,

    /// 协议配置（仅用于 passthrough 模式）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<RedirdevProtocol>,

    /// 地址配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressConfig>,

    /// 启动顺序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot: Option<BootOrderConfig>,

    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<AliasConfig>,
}

/// USB 重定向设备源配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirdevSource {
    /// 连接模式（bind, connect）
    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// 主机地址
    #[serde(rename = "@host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// 服务端口
    #[serde(rename = "@service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

/// USB 重定向设备协议配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirdevProtocol {
    /// 协议类型
    #[serde(rename = "@type")]
    pub protocol_type: String, // raw, etc.
}

/// USB 重定向过滤器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirfilterConfig {
    /// USB 设备过滤规则列表
    #[serde(rename = "usbdev", default)]
    pub usb_devices: Vec<UsbDevFilter>,
}

/// USB 设备过滤器规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDevFilter {
    /// 是否允许
    #[serde(rename = "@allow")]
    pub allow: String, // yes, no

    /// USB 设备类代码
    #[serde(rename = "@class", skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,

    /// Vendor ID
    #[serde(rename = "@vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    /// Product ID
    #[serde(rename = "@product", skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    /// 设备版本号（bcdDevice 字段）
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
