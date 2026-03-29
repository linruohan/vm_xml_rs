use serde::{Deserialize, Serialize};

/// 锁租约配置（用于共享存储锁）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseConfig {
    /// 锁空间名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lockspace: Option<String>,

    /// 锁键
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// 目标配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<LeaseTarget>,
}

/// 租约目标配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseTarget {
    /// 文件路径
    #[serde(rename = "@path")]
    pub path: String,

    /// 偏移量
    #[serde(rename = "@offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
}
