use serde::{Deserialize, Serialize};

/// NVRAM 设备配置（非易失性 RAM，用于存储 UEFI 变量）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NVRAMConfig {
    /// NVRAM 模板文件路径
    #[serde(rename = "@template", skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// NVRAM 存储文件路径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nvram_source: Option<String>,
}
