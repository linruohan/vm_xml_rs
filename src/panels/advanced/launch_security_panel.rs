use crate::{
    model::{LaunchSecurityConfig, SecurityLabelConfig, TPMBackend, TPMConfig, VMConfig},
    panels::utils::*,
};

/// 启动安全配置面板
pub struct LaunchSecurityPanel;

impl LaunchSecurityPanel {
    /// 显示启动安全配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        panel_header(ui, "🛡️", "启动安全配置");

        card_group(ui, "启动安全设置", None, |ui| {
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
                ui.collapsing("安全标签", |ui| {
                    if launch_security.seclabel.is_none() {
                        launch_security.seclabel = Some(SecurityLabelConfig {
                            label_type: "default".to_string(),
                            model: "selinux".to_string(),
                            relabel: None,
                            label: None,
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
                ui.collapsing("TPM 配置", |ui| {
                    if launch_security.tpm.is_none() {
                        launch_security.tpm = Some(TPMConfig {
                            model: "tpm-tis".to_string(),
                            backend: TPMBackend {
                                backend_type: "emulator".to_string(),
                                version: Some("2.0".to_string()),
                            },
                        });
                    }
                    if let Some(ref mut tpm) = launch_security.tpm {
                        grid(ui, "tpm_grid", 2, |ui| {
                            ui.label("模型:");
                            ui.text_edit_singleline(&mut tpm.model);
                            ui.end_row();

                            ui.label("后端类型:");
                            ui.text_edit_singleline(&mut tpm.backend.backend_type);
                            ui.end_row();

                            ui.label("版本:");
                            if let Some(ref mut version) = &mut tpm.backend.version {
                                ui.text_edit_singleline(version);
                            } else {
                                let mut empty = String::new();
                                ui.text_edit_singleline(&mut empty);
                            }
                            ui.end_row();
                        });
                    }
                });
            }
        });
    }
}
