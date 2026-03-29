use egui::RichText;

use crate::{
    model::{SecurityLabelConfig, VMConfig},
    panels::utils::*,
};

/// 安全标签配置面板
pub struct SecurityLabelPanel;

impl SecurityLabelPanel {
    /// 显示安全标签配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "🔒", "安全标签配置");

        card_group(ui, "安全设置", None, colors, |ui| {
            let mut has_security = config.security_label.is_some();
            if checkbox(ui, &mut has_security, "启用安全标签") {
                if has_security {
                    config.security_label = Some(SecurityLabelConfig {
                        label_type: "dynamic".to_string(),
                        model: "selinux".to_string(),
                        relabel: None,
                        label: None,
                        baselabel: None,
                        imagelabel: None,
                    });
                } else {
                    config.security_label = None;
                }
            }

            if let Some(ref mut security) = config.security_label {
                grid(ui, "security_grid", 2, |ui| {
                    // 类型
                    ui.label("类型:");
                    let label_type = &mut security.label_type;
                    egui::ComboBox::from_id_source("seclabel_type")
                        .selected_text(label_type.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                label_type,
                                "dynamic".to_string(),
                                "dynamic (动态)",
                            );
                            ui.selectable_value(label_type, "static".to_string(), "static (静态)");
                            ui.selectable_value(label_type, "none".to_string(), "none (无)");
                        });
                    ui.end_row();

                    // 模型
                    ui.label("模型:");
                    let model = &mut security.model;
                    egui::ComboBox::from_id_source("seclabel_model")
                        .selected_text(model.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(model, "selinux".to_string(), "SELinux");
                            ui.selectable_value(model, "apparmor".to_string(), "AppArmor");
                            ui.selectable_value(model, "dac".to_string(), "DAC");
                        });
                    ui.end_row();

                    // 重标记
                    ui.label("重标记:");
                    let relabel = security.relabel.get_or_insert_with(|| "".to_string());
                    egui::ComboBox::from_id_source("seclabel_relabel")
                        .selected_text(relabel.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(relabel, "".to_string(), "默认");
                            ui.selectable_value(relabel, "yes".to_string(), "是");
                            ui.selectable_value(relabel, "no".to_string(), "否");
                        });
                    ui.end_row();
                });

                ui.add_space(5.0);

                // 安全标签
                ui.horizontal(|ui| {
                    ui.label("安全标签:");
                    let label = security.label.get_or_insert_with(|| "".to_string());
                    ui.text_edit_singleline(label);
                });

                ui.add_space(5.0);

                // 基础标签
                ui.horizontal(|ui| {
                    ui.label("基础标签:");
                    let baselabel = security.baselabel.get_or_insert_with(|| "".to_string());
                    ui.text_edit_singleline(baselabel);
                    ui.label(
                        RichText::new("(可选，动态标签时使用)")
                            .small()
                            .color(colors.text_secondary),
                    );
                });

                ui.add_space(5.0);

                // 镜像标签（只读，输出用）
                ui.horizontal(|ui| {
                    ui.label("镜像标签:");
                    let imagelabel = security.imagelabel.get_or_insert_with(|| "".to_string());
                    ui.text_edit_singleline(imagelabel);
                    ui.label(RichText::new("(只读，输出用)").small().color(colors.text_secondary));
                });
            }
        });
    }
}
