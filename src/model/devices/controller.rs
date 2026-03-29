use serde::{Deserialize, Serialize};

use super::AddressConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerConfig {
    #[serde(rename = "@type")]
    pub controller_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vectors: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_grant_frames: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_event_channels: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<ControllerDriver>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master: Option<ControllerMaster>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_elem: Option<ControllerModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<ControllerTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcihole64: Option<ControllerPCIhole64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hotplug: Option<ControllerHotplug>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerDriver {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub driver_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd_per_lun: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sectors: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ioeventfd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iothread: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iothreads: Option<Vec<ControllerIOThread>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerIOThread {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<ControllerQueue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerQueue {
    #[serde(rename = "@id")]
    pub id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerMaster {
    #[serde(rename = "@address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startport: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerModel {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerTarget {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(rename = "@chassisNr", skip_serializing_if = "Option::is_none")]
    pub chassis_nr: Option<u32>,
    #[serde(rename = "@chassis", skip_serializing_if = "Option::is_none")]
    pub chassis: Option<u32>,
    #[serde(rename = "@port", skip_serializing_if = "Option::is_none")]
    pub port: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerHotplug {
    #[serde(rename = "@enabled")]
    pub enabled: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerPCIhole64 {
    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<u64>,
}
