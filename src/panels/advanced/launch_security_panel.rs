use egui::{Align, Layout};

use crate::{
    model::{LaunchSecurityConfig, SecurityLabelConfig, TPMBackend, TPMConfig, VMConfig},
    panels::utils::*,
};

/// 启动安全配置面板
pub struct LaunchSecurityPanel;

impl LaunchSecurityPanel {
    /// 显示启动安全配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "🛡️", "启动安全配置");

        // 卡片宽度和间距配置
        let card_width = 380.0;
        let spacing = 8.0;

        // 使用流式布局实现卡片自动换行
        ui.with_layout(Layout::left_to_right(Align::TOP).with_main_wrap(true), |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);

            // 启动安全设置卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "启动安全设置", Some("🔒"), colors, |ui| {
                        let mut has_launch_security = config.launch_security.is_some();
                        if checkbox(ui, &mut has_launch_security, "启用启动安全") {
                            if has_launch_security {
                                config.launch_security = Some(LaunchSecurityConfig::default());
                            } else {
                                config.launch_security = None;
                            }
                        }

                        if let Some(ref mut launch_security) = config.launch_security {
                            ui.add_space(5.0);

                            // 安全标签
                            ui.collapsing("安全标签", |ui| {
                                if launch_security.seclabel.is_none() {
                                    launch_security.seclabel = Some(SecurityLabelConfig {
                                        label_type: "default".to_string(),
                                        model: "selinux".to_string(),
                                        relabel: None,
                                        label: None,
                                        baselabel: None,
                                        imagelabel: None,
                                    });
                                }
                                if let Some(ref mut seclabel) = launch_security.seclabel {
                                    grid(ui, "seclabel_grid", 2, |ui| {
                                        ui.label("类型:");
                                        ui.text_edit_singleline(&mut seclabel.label_type);
                                        ui.end_row();

                                        ui.label("模型:");
                                        ui.text_edit_singleline(&mut seclabel.model);
                                        ui.end_row();
                                    });
                                }
                            });

                            ui.add_space(5.0);

                            // TPM 配置
                            ui.collapsing("TPM 配置", |ui| {
                                if launch_security.tpm.is_none() {
                                    launch_security.tpm = Some(TPMConfig {
                                        model: "tpm-tis".to_string(),
                                        backend: Some(TPMBackend {
                                            backend_type: "emulator".to_string(),
                                            version: Some("2.0".to_string()),
                                            device: None,
                                            model: None,
                                        }),
                                    });
                                }
                                if let Some(ref mut tpm) = launch_security.tpm {
                                    grid(ui, "tpm_grid", 2, |ui| {
                                        ui.label("模型:");
                                        ui.text_edit_singleline(&mut tpm.model);
                                        ui.end_row();

                                        ui.label("后端类型:");
                                        if let Some(ref mut backend) = tpm.backend {
                                            ui.text_edit_singleline(&mut backend.backend_type);
                                            ui.end_row();

                                            ui.label("版本:");
                                            if let Some(ref mut version) = &mut backend.version {
                                                ui.text_edit_singleline(version);
                                            } else {
                                                let mut empty = String::new();
                                                ui.text_edit_singleline(&mut empty);
                                            }
                                            ui.end_row();
                                        }
                                    });
                                }
                            });
                        }
                    });
                },
            );
        });
    }
}
