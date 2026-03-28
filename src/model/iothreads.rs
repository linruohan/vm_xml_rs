use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IOThreadsConfig {
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iothreadids: Option<Vec<IOThread>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaultiothread: Option<DefaultIOThread>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOThread {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@thread_pool_min", skip_serializing_if = "Option::is_none")]
    pub thread_pool_min: Option<u32>,
    #[serde(rename = "@thread_pool_max", skip_serializing_if = "Option::is_none")]
    pub thread_pool_max: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<PollConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollConfig {
    #[serde(rename = "@max")]
    pub max: u32,
    #[serde(rename = "@grow", skip_serializing_if = "Option::is_none")]
    pub grow: Option<u32>,
    #[serde(rename = "@shrink", skip_serializing_if = "Option::is_none")]
    pub shrink: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultIOThread {
    #[serde(rename = "@thread_pool_min", skip_serializing_if = "Option::is_none")]
    pub thread_pool_min: Option<u32>,
    #[serde(rename = "@thread_pool_max", skip_serializing_if = "Option::is_none")]
    pub thread_pool_max: Option<u32>,
}
