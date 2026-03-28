use crate::{
    model::{PowerManagementConfig, VMConfig},
    panels::utils::*,
};

/// 电源管理策略配置面板
pub struct PowerManagementPanel;

impl PowerManagementPanel {
    /// 显示电源管理策略配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "🔋", "电源管理策略");

        card_group(ui, "电源选项", None, colors, |ui| {
            let mut has_power = config.power_management.is_some();
            if checkbox(ui, &mut has_power, "启用电源管理") {
                if has_power {
                    config.power_management = Some(PowerManagementConfig::default());
                } else {
                    config.power_management = None;
                }
            }

            if let Some(ref mut power) = config.power_management {
                ui.add_space(5.0);
                let mut suspend_to_disk = power.suspend_to_disk;
                if checkbox(ui, &mut suspend_to_disk, "支持挂起到磁盘") {
                    power.suspend_to_disk = suspend_to_disk;
                }

                let mut suspend_to_ram = power.suspend_to_ram;
                if checkbox(ui, &mut suspend_to_ram, "支持挂起到内存") {
                    power.suspend_to_ram = suspend_to_ram;
                }

                let mut autoboot = power.autoboot;
                if checkbox(ui, &mut autoboot, "自动启动") {
                    power.autoboot = autoboot;
                }
            }
        });
    }
}
