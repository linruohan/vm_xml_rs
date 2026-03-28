use serde::{Deserialize, Serialize};

use crate::model::FeatureConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HypervisorFeaturesConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<Vec<FeatureConfig>>,
}
