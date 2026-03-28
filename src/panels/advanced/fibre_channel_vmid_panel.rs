use egui::RichText;

use crate::model::vm_config::{FibreChannelVMIDConfig, VMConfig};

/// 光纤通道虚拟机标识配置面板
pub struct FibreChannelVMIDPanel;

impl FibreChannelVMIDPanel {
    /// 显示光纤通道虚拟机标识配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.group(|ui| {
            ui.label(RichText::new("光纤通道虚拟机标识").strong());
            ui.add_space(5.0);

            let mut has_fc_vmid = config.fibre_channel_vmid.is_some();
            if ui.checkbox(&mut has_fc_vmid, "启用光纤通道 VMID").changed() {
                if has_fc_vmid {
                    config.fibre_channel_vmid = Some(FibreChannelVMIDConfig {
                        id: "00000000-0000-0000-0000-000000000000".to_string(),
                    });
                } else {
                    config.fibre_channel_vmid = None;
                }
            }

            if let Some(ref mut vmid) = config.fibre_channel_vmid {
                ui.label("VMID:");
                ui.text_edit_singleline(&mut vmid.id);
            }
        });
    }
}
