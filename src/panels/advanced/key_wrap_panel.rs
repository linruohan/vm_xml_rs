use egui::RichText;

use crate::model::vm_config::{KeyWrapConfig, MasterKeyConfig, VMConfig};

/// 密钥包装配置面板
pub struct KeyWrapPanel;

impl KeyWrapPanel {
    /// 显示密钥包装配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.group(|ui| {
            ui.label(RichText::new("密钥包装配置").strong());
            ui.add_space(5.0);

            let mut has_key_wrap = config.key_wrap.is_some();
            if ui.checkbox(&mut has_key_wrap, "启用密钥包装").changed() {
                if has_key_wrap {
                    config.key_wrap = Some(KeyWrapConfig::default());
                } else {
                    config.key_wrap = None;
                }
            }

            if let Some(ref mut key_wrap) = config.key_wrap {
                ui.collapsing("主密钥配置", |ui| {
                    if key_wrap.master_key.is_none() {
                        key_wrap.master_key = Some(MasterKeyConfig {
                            key_type: "default".to_string(),
                            uri: "".to_string(),
                        });
                    }
                    if let Some(ref mut master_key) = key_wrap.master_key {
                        egui::Grid::new("master_key_grid")
                            .num_columns(2)
                            .spacing([10.0, 8.0])
                            .show(ui, |ui| {
                                ui.label("类型:");
                                ui.text_edit_singleline(&mut master_key.key_type);
                                ui.end_row();

                                ui.label("URI:");
                                ui.text_edit_singleline(&mut master_key.uri);
                                ui.end_row();
                            });
                    }
                });
            }
        });
    }
}
