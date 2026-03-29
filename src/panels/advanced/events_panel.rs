use egui::{Align, Layout};

use crate::{
    model::{EventsConfig, VMConfig},
    panels::utils::*,
};

/// 系统事件配置面板
pub struct EventsPanel;

const EVENT_ACTIONS: &[&str] = &["destroy", "preserve", "restart", "shutdown", "pause", "coredump"];

impl EventsPanel {
    /// 显示系统事件配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "📅", "系统事件配置");

        // 卡片宽度和间距配置
        let card_width = 380.0;
        let spacing = 8.0;

        // 使用流式布局实现卡片自动换行
        ui.with_layout(Layout::left_to_right(Align::TOP).with_main_wrap(true), |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);

            // 事件处理策略卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "事件处理策略", Some("⚙"), colors, |ui| {
                        let mut has_events = config.events.is_some();
                        if checkbox(ui, &mut has_events, "启用事件配置") {
                            if has_events {
                                config.events = Some(EventsConfig::default());
                            } else {
                                config.events = None;
                            }
                        }

                        if let Some(ref mut events) = config.events {
                            ui.add_space(5.0);
                            grid(ui, "events_grid", 2, |ui| {
                                // 关机事件
                                ui.label("关机时:");
                                let on_poweroff =
                                    events.on_poweroff.get_or_insert_with(|| "destroy".to_string());
                                egui::ComboBox::from_id_source("on_poweroff")
                                    .selected_text(on_poweroff.as_str())
                                    .show_ui(ui, |ui| {
                                        for action in EVENT_ACTIONS {
                                            ui.selectable_value(
                                                on_poweroff,
                                                action.to_string(),
                                                *action,
                                            );
                                        }
                                    });
                                ui.end_row();

                                // 重启事件
                                ui.label("重启时:");
                                let on_reboot =
                                    events.on_reboot.get_or_insert_with(|| "destroy".to_string());
                                egui::ComboBox::from_id_source("on_reboot")
                                    .selected_text(on_reboot.as_str())
                                    .show_ui(ui, |ui| {
                                        for action in EVENT_ACTIONS {
                                            ui.selectable_value(
                                                on_reboot,
                                                action.to_string(),
                                                *action,
                                            );
                                        }
                                    });
                                ui.end_row();

                                // 崩溃事件
                                ui.label("崩溃时:");
                                let on_crash =
                                    events.on_crash.get_or_insert_with(|| "destroy".to_string());
                                egui::ComboBox::from_id_source("on_crash")
                                    .selected_text(on_crash.as_str())
                                    .show_ui(ui, |ui| {
                                        for action in EVENT_ACTIONS {
                                            ui.selectable_value(
                                                on_crash,
                                                action.to_string(),
                                                *action,
                                            );
                                        }
                                    });
                                ui.end_row();

                                // 锁定失败事件
                                ui.label("锁定失败时:");
                                let on_lockfailure = events
                                    .on_lockfailure
                                    .get_or_insert_with(|| "destroy".to_string());
                                egui::ComboBox::from_id_source("on_lockfailure")
                                    .selected_text(on_lockfailure.as_str())
                                    .show_ui(ui, |ui| {
                                        for action in &["destroy", "pause", "restart", "coredump"] {
                                            ui.selectable_value(
                                                on_lockfailure,
                                                action.to_string(),
                                                *action,
                                            );
                                        }
                                    });
                                ui.end_row();
                            });
                        }
                    });
                },
            );
        });
    }
}
