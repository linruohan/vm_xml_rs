use egui::RichText;

use crate::model::vm_config::{HypervisorFeaturesConfig, VMConfig};

/// Hypervisor 特性配置面板
pub struct HypervisorFeaturesPanel;

impl HypervisorFeaturesPanel {
    /// 显示 Hypervisor 特性配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.group(|ui| {
            ui.label(RichText::new("Hypervisor 特性配置").strong());
            ui.add_space(5.0);

            let mut has_features = config.hypervisor_features.is_some();
            if ui.checkbox(&mut has_features, "启用 Hypervisor 特性").changed() {
                if has_features {
                    config.hypervisor_features = Some(HypervisorFeaturesConfig::default());
                } else {
                    config.hypervisor_features = None;
                }
            }

            if let Some(ref mut features) = config.hypervisor_features {
                if features.feature.is_none() {
                    features.feature = Some(Vec::new());
                }
                if let Some(ref mut feature_list) = features.feature {
                    if ui.button("➕ 添加特性").clicked() {
                        feature_list.push(crate::model::vm_config::FeatureConfig {
                            enabled: "yes".to_string(),
                            name: "acpi".to_string(),
                        });
                    }

                    let mut to_remove = None;
                    for (i, feature) in feature_list.iter_mut().enumerate() {
                        ui.push_id(i, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("名称:");
                                ui.text_edit_singleline(&mut feature.name);
                                ui.label("启用:");
                                egui::ComboBox::from_id_source(format!("feature_enabled_{}", i))
                                    .selected_text(&feature.enabled)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut feature.enabled,
                                            "yes".to_string(),
                                            "yes",
                                        );
                                        ui.selectable_value(
                                            &mut feature.enabled,
                                            "no".to_string(),
                                            "no",
                                        );
                                    });
                                if ui.button("🗑️").clicked() {
                                    to_remove = Some(i);
                                }
                            });
                        });
                    }
                    if let Some(idx) = to_remove {
                        feature_list.remove(idx);
                    }
                }
            }
        });
    }
}
