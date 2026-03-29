use serde::{Deserialize, Serialize};

use super::AddressConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsConfig {
    #[serde(rename = "@type")]
    pub graphics_type: String,
    #[serde(rename = "@port", skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(rename = "@autoport", skip_serializing_if = "Option::is_none")]
    pub autoport: Option<String>,
    #[serde(rename = "@listen", skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_type: Option<ListenConfig>,
    #[serde(rename = "@passwd", skip_serializing_if = "Option::is_none")]
    pub passwd: Option<String>,
    #[serde(rename = "@keymap", skip_serializing_if = "Option::is_none")]
    pub keymap: Option<String>,
    #[serde(rename = "@sharePolicy", skip_serializing_if = "Option::is_none")]
    pub share_policy: Option<String>,
    #[serde(rename = "@defaultMode", skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<String>,
    #[serde(rename = "@connected", skip_serializing_if = "Option::is_none")]
    pub connected: Option<String>,
    #[serde(rename = "@passwdValidTo", skip_serializing_if = "Option::is_none")]
    pub passwd_valid_to: Option<String>,
    #[serde(rename = "@powerControl", skip_serializing_if = "Option::is_none")]
    pub power_control: Option<String>,
    #[serde(rename = "@wait", skip_serializing_if = "Option::is_none")]
    pub wait: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gl: Option<GlConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Vec<ChannelPolicyConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming: Option<StreamingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clipboard: Option<ClipboardConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouse: Option<MouseConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filetransfer: Option<FileTransferConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<GraphicsAudioConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlConfig {
    #[serde(rename = "@enable")]
    pub enable: String,
    #[serde(rename = "@rendernode", skip_serializing_if = "Option::is_none")]
    pub rendernode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPolicyConfig {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@mode")]
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    #[serde(rename = "@compression")]
    pub compression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingConfig {
    #[serde(rename = "@mode")]
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardConfig {
    #[serde(rename = "@copypaste")]
    pub copypaste: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseConfig {
    #[serde(rename = "@mode")]
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTransferConfig {
    #[serde(rename = "@enable")]
    pub enable: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsAudioConfig {
    #[serde(rename = "@id")]
    pub id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenConfig {
    #[serde(rename = "@type")]
    pub listen_type: String,
    #[serde(rename = "@address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub video_type: Option<String>,
    #[serde(rename = "@primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,
    pub model: VideoModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration: Option<AccelerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<VideoDriverConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<ResolutionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccelerationConfig {
    #[serde(rename = "@accel3d", skip_serializing_if = "Option::is_none")]
    pub accel3d: Option<String>,
    #[serde(rename = "@accel2d", skip_serializing_if = "Option::is_none")]
    pub accel2d: Option<String>,
    #[serde(rename = "@rendernode", skip_serializing_if = "Option::is_none")]
    pub rendernode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoDriverConfig {
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@ioeventfd", skip_serializing_if = "Option::is_none")]
    pub ioeventfd: Option<String>,
    #[serde(rename = "@event_idx", skip_serializing_if = "Option::is_none")]
    pub event_idx: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionConfig {
    #[serde(rename = "@x")]
    pub x: u32,
    #[serde(rename = "@y")]
    pub y: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoModel {
    #[serde(rename = "@type")]
    pub model_type: String,
    #[serde(rename = "@vram", skip_serializing_if = "Option::is_none")]
    pub vram: Option<u32>,
    #[serde(rename = "@heads", skip_serializing_if = "Option::is_none")]
    pub heads: Option<u32>,
    #[serde(rename = "@primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,
    #[serde(rename = "@ram", skip_serializing_if = "Option::is_none")]
    pub ram: Option<u32>,
    #[serde(rename = "@vgamem", skip_serializing_if = "Option::is_none")]
    pub vgamem: Option<u32>,
    #[serde(rename = "@vram64", skip_serializing_if = "Option::is_none")]
    pub vram64: Option<u32>,
    #[serde(rename = "@blob", skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
    #[serde(rename = "@edid", skip_serializing_if = "Option::is_none")]
    pub edid: Option<String>,
}
