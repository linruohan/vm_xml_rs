use crate::{
    model::{FeatureConfig, HypervisorFeaturesConfig, VMConfig},
    panels::utils::*,
};

/// Hypervisor 特性配置面板
pub struct HypervisorFeaturesPanel;

impl HypervisorFeaturesPanel {
    /// 显示 Hypervisor 特性配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "🛡", "Hypervisor 特性配置");

        card_group(ui, "特性设置", None, colors, |ui| {
            let mut has_features = config.hypervisor_features.is_some();
            if checkbox(ui, &mut has_features, "启用 Hypervisor 特性") {
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
                    ui.horizontal(|ui| {
                        if add_button(ui, "➕ 添加特性", colors) {
                            feature_list.push(FeatureConfig {
                                enabled: "yes".to_string(),
                                name: "acpi".to_string(),
                            });
                        }
                    });

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
                                if delete_button(ui, None) {
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
