use egui::{Align, Layout};

use crate::{
    model::{FibreChannelVMIDConfig, VMConfig},
    panels::utils::*,
};

/// 光纤通道虚拟机标识配置面板
pub struct FibreChannelVMIDPanel;

impl FibreChannelVMIDPanel {
    /// 显示光纤通道虚拟机标识配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "🔗", "光纤通道 VMID 配置");

        let card_width = 380.0;
        let spacing = 8.0;

        ui.with_layout(Layout::left_to_right(Align::TOP).with_main_wrap(true), |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);

            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "VMID 设置", Some("🏷"), colors, |ui| {
                        let mut has_fc_vmid = config.fibre_channel_vmid.is_some();
                        if checkbox(ui, &mut has_fc_vmid, "启用光纤通道 VMID") {
                            if has_fc_vmid {
                                config.fibre_channel_vmid = Some(FibreChannelVMIDConfig {
                                    id: "00000000-0000-0000-0000-000000000000".to_string(),
                                });
                            } else {
                                config.fibre_channel_vmid = None;
                            }
                        }

                        if let Some(ref mut vmid) = config.fibre_channel_vmid {
                            ui.add_space(5.0);
                            grid(ui, "vmid_grid", 2, |ui| {
                                ui.label("VMID:");
                                ui.text_edit_singleline(&mut vmid.id);
                                ui.end_row();
                            });
                        }
                    });
                },
            );
        });
    }
}
