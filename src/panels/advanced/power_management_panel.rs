use egui::{Align, Layout};

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

        // 卡片宽度和间距配置
        let card_width = 380.0;
        let spacing = 8.0;

        // 使用流式布局实现卡片自动换行
        ui.with_layout(Layout::left_to_right(Align::TOP).with_main_wrap(true), |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);

            // 电源选项卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "电源选项", Some("⚡"), colors, |ui| {
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

                            ui.add_space(5.0);

                            let mut suspend_to_ram = power.suspend_to_ram;
                            if checkbox(ui, &mut suspend_to_ram, "支持挂起到内存") {
                                power.suspend_to_ram = suspend_to_ram;
                            }

                            ui.add_space(5.0);

                            let mut autoboot = power.autoboot;
                            if checkbox(ui, &mut autoboot, "自动启动") {
                                power.autoboot = autoboot;
                            }
                        }
                    });
                },
            );
        });
    }
}
