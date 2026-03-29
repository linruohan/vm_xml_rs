//! 应用错误类型定义
//!
//! 提供统一的错误处理机制，区分不同类型的错误

use thiserror::Error;

/// 应用错误类型
#[derive(Debug, Error)]
pub enum AppError {
    /// XML 生成错误
    #[error("XML 生成失败：{0}")]
    XmlGeneration(String),

    /// 文件操作错误
    #[error("文件操作失败：{0}")]
    FileOperation(#[from] std::io::Error),

    /// 配置验证错误
    #[error("配置验证失败：{0}")]
    Validation(String),

    /// JSON 序列化/反序列化错误
    #[error("JSON 处理失败：{0}")]
    JsonError(#[from] serde_json::Error),

    /// XML 解析错误
    #[error("XML 解析失败：{0}")]
    XmlParseError(String),

    /// UI 错误
    #[error("UI 操作失败：{0}")]
    UiError(String),
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::XmlGeneration(s)
    }
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, AppError>;
