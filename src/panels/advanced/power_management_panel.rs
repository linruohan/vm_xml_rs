use egui::RichText;

use crate::model::{PowerManagementConfig, VMConfig};

/// 电源管理策略配置面板
pub struct PowerManagementPanel;

impl PowerManagementPanel {
    /// 显示电源管理策略配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.group(|ui| {
            ui.label(RichText::new("电源管理策略").strong());
            ui.add_space(5.0);

            let mut has_power = config.power_management.is_some();
            if ui.checkbox(&mut has_power, "启用电源管理").changed() {
                if has_power {
                    config.power_management = Some(PowerManagementConfig::default());
                } else {
                    config.power_management = None;
                }
            }

            if let Some(ref mut power) = config.power_management {
                ui.checkbox(&mut power.suspend_to_disk, "支持挂起到磁盘");
                ui.checkbox(&mut power.suspend_to_ram, "支持挂起到内存");
                ui.checkbox(&mut power.autoboot, "自动启动");
            }
        });
    }
}
