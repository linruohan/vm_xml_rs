use crate::{
    model::{EventsConfig, VMConfig},
    panels::utils::*,
};

/// 系统事件配置面板
pub struct EventsPanel;

const EVENT_ACTIONS: &[&str] = &["destroy", "preserve", "restart", "shutdown", "pause"];

impl EventsPanel {
    /// 显示系统事件配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        panel_header(ui, "📅", "系统事件配置");

        card_group(ui, "事件处理策略", None, |ui| {
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
                                ui.selectable_value(on_poweroff, action.to_string(), *action);
                            }
                        });
                    ui.end_row();

                    // 重启事件
                    ui.label("重启时:");
                    let on_reboot = events.on_reboot.get_or_insert_with(|| "destroy".to_string());
                    egui::ComboBox::from_id_source("on_reboot")
                        .selected_text(on_reboot.as_str())
                        .show_ui(ui, |ui| {
                            for action in EVENT_ACTIONS {
                                ui.selectable_value(on_reboot, action.to_string(), *action);
                            }
                        });
                    ui.end_row();

                    // 崩溃事件
                    ui.label("崩溃时:");
                    let on_crash = events.on_crash.get_or_insert_with(|| "destroy".to_string());
                    egui::ComboBox::from_id_source("on_crash")
                        .selected_text(on_crash.as_str())
                        .show_ui(ui, |ui| {
                            for action in EVENT_ACTIONS {
                                ui.selectable_value(on_crash, action.to_string(), *action);
                            }
                        });
                    ui.end_row();

                    // 锁定失败事件
                    ui.label("锁定失败时:");
                    let on_lockfailure =
                        events.on_lockfailure.get_or_insert_with(|| "destroy".to_string());
                    egui::ComboBox::from_id_source("on_lockfailure")
                        .selected_text(on_lockfailure.as_str())
                        .show_ui(ui, |ui| {
                            for action in &["destroy", "pause", "restart"] {
                                ui.selectable_value(on_lockfailure, action.to_string(), *action);
                            }
                        });
                    ui.end_row();
                });
            }
        });
    }
}
