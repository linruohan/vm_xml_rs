use egui::RichText;

use crate::model::vm_config::{SecurityLabelConfig, VMConfig};

/// 安全标签配置面板
pub struct SecurityLabelPanel;

impl SecurityLabelPanel {
    /// 显示安全标签配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.group(|ui| {
            ui.label(RichText::new("安全标签配置").strong());
            ui.add_space(5.0);

            let mut has_security = config.security_label.is_some();
            if ui.checkbox(&mut has_security, "启用安全标签").changed() {
                if has_security {
                    config.security_label = Some(SecurityLabelConfig {
                        label_type: "default".to_string(),
                        model: "selinux".to_string(),
                        relabel: None,
                        label: None,
                    });
                } else {
                    config.security_label = None;
                }
            }

            if let Some(ref mut security) = config.security_label {
                egui::Grid::new("security_label_grid").num_columns(2).spacing([10.0, 8.0]).show(
                    ui,
                    |ui| {
                        ui.label("类型:");
                        ui.text_edit_singleline(&mut security.label_type);
                        ui.end_row();

                        ui.label("模型:");
                        ui.text_edit_singleline(&mut security.model);
                        ui.end_row();

                        ui.label("标签:");
                        if let Some(ref mut label) = &mut security.label {
                            ui.text_edit_singleline(label);
                        } else {
                            ui.text_edit_singleline(&mut String::new());
                        }
                        ui.end_row();
                    },
                );
            }
        });
    }
}
