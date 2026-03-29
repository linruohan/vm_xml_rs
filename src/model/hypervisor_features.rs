use serde::{Deserialize, Serialize};

use crate::model::FeatureConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HypervisorFeaturesConfig {
    /// 基础特性（简单开关）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pae: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acpi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viridian: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privnet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pvspinlock: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmport: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmcoreinfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub htm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested_hv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccf_assist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub async_teardown: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ras: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ps2: Option<String>,

    /// Hyper-V 特性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperv: Option<HyperVConfig>,

    /// KVM 特性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kvm: Option<KvmFeaturesConfig>,

    /// Xen 特性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xen: Option<XenFeaturesConfig>,

    /// GIC 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gic: Option<GicConfig>,

    /// SMM 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smm: Option<SmmConfig>,

    /// IOAPIC 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ioapic: Option<IoapicConfig>,

    /// HPT 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hpt: Option<HptConfig>,

    /// MSRs 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msrs: Option<MsrsConfig>,

    /// CFPC 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfpc: Option<CfpcConfig>,

    /// SBBC 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sbbc: Option<SbbcConfig>,

    /// IBS 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ibs: Option<IbsConfig>,

    /// TCG 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcg: Option<TcgConfig>,

    /// AIA 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aia: Option<AiaConfig>,

    /// 通用特性列表（用于未专门建模的特性）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<Vec<FeatureConfig>>,
}

/// Hyper-V 特性配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperVConfig {
    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relaxed: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vapic: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spinlocks: Option<SpinlocksConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpindex: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synic: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stimer: Option<StimerConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<VendorIdConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequencies: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reenlightenment: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tlbflush: Option<TlbflushConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipi: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avic: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evmcs: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emsr_bitmap: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xmm_input: Option<FeatureState>,
}

impl Default for HyperVConfig {
    fn default() -> Self {
        Self {
            mode: Some("custom".to_string()),
            relaxed: None,
            vapic: None,
            spinlocks: None,
            vpindex: None,
            runtime: None,
            synic: None,
            stimer: None,
            reset: None,
            vendor_id: None,
            frequencies: None,
            reenlightenment: None,
            tlbflush: None,
            ipi: None,
            avic: None,
            evmcs: None,
            emsr_bitmap: None,
            xmm_input: None,
        }
    }
}

/// KVM 特性配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KvmFeaturesConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint_dedicated: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_control: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pv_ipi: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dirty_ring: Option<DirtyRingConfig>,
}

/// Xen 特性配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct XenFeaturesConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e820_host: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough: Option<PassthroughConfig>,
}

/// 特性状态配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureState {
    #[serde(rename = "@state")]
    pub state: String,
}

/// Spinlocks 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinlocksConfig {
    #[serde(rename = "@state")]
    pub state: String,
    #[serde(rename = "@retries", skip_serializing_if = "Option::is_none")]
    pub retries: Option<u32>,
}

/// Stimer 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StimerConfig {
    #[serde(rename = "@state")]
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct: Option<FeatureState>,
}

/// Vendor ID 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorIdConfig {
    #[serde(rename = "@state")]
    pub state: String,
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// TLBFlush 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlbflushConfig {
    #[serde(rename = "@state")]
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct: Option<FeatureState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended: Option<FeatureState>,
}

/// Dirty Ring 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirtyRingConfig {
    #[serde(rename = "@state")]
    pub state: String,
    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
}

/// Passthrough 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassthroughConfig {
    #[serde(rename = "@state")]
    pub state: String,
    #[serde(rename = "@mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// GIC 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GicConfig {
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// SMM 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmmConfig {
    #[serde(rename = "@state")]
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tseg: Option<TsegConfig>,
}

/// TSEG 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsegConfig {
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "$value")]
    pub value: u32,
}

/// IOAPIC 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoapicConfig {
    #[serde(rename = "@driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
}

/// HPT 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HptConfig {
    #[serde(rename = "@resizing", skip_serializing_if = "Option::is_none")]
    pub resizing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxpagesize: Option<MaxPageSizeConfig>,
}

/// Max Page Size 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxPageSizeConfig {
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "$value")]
    pub value: u32,
}

/// MSRs 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsrsConfig {
    #[serde(rename = "@unknown", skip_serializing_if = "Option::is_none")]
    pub unknown: Option<String>,
}

/// CFPC 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfpcConfig {
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// SBBC 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbbcConfig {
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// IBS 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IbsConfig {
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// TCG 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcgConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb_cache: Option<TbCacheConfig>,
}

/// TB Cache 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TbCacheConfig {
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "$value")]
    pub value: u32,
}

/// AIA 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiaConfig {
    #[serde(rename = "@value")]
    pub value: String,
}
