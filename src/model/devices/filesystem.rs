use serde::{Deserialize, Serialize};

use crate::model::IdMapConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemConfig {
    #[serde(rename = "@type")]
    pub fs_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessmode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multidevs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fmode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dmode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<FilesystemDriver>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<FilesystemBinary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<FilesystemSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<FilesystemTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idmap: Option<IdMapConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_hard_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_soft_limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemTarget {
    #[serde(rename = "@dir")]
    pub dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemDriver {
    #[serde(rename = "@type")]
    pub driver_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrpolicy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemBinary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xattr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<FilesystemCache>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<FilesystemSandbox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<FilesystemLock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_pool: Option<FilesystemThreadPool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemCache {
    #[serde(rename = "@mode")]
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemSandbox {
    #[serde(rename = "@mode")]
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemLock {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flock: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemThreadPool {
    #[serde(rename = "@size")]
    pub size: u32,
}
