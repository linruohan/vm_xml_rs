use crate::{
    model::{SecurityLabelConfig, VMConfig},
    panels::utils::*,
};

/// 安全标签配置面板
pub struct SecurityLabelPanel;

impl SecurityLabelPanel {
    /// 显示安全标签配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        panel_header(ui, "🔒", "安全标签配置");

        card_group(ui, "安全设置", None, |ui| {
            let mut has_security = config.security_label.is_some();
            if checkbox(ui, &mut has_security, "启用安全标签") {
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
                grid(ui, "security_label_grid", 2, |ui| {
                    ui.label("类型:");
                    ui.text_edit_singleline(&mut security.label_type);
                    ui.end_row();

                    ui.label("模型:");
                    ui.text_edit_singleline(&mut security.model);
                    ui.end_row();

                    ui.label("标签:");
                    ui.text_edit_singleline(security.label.get_or_insert_with(String::new));
                    ui.end_row();
                });
            }
        });
    }
}
