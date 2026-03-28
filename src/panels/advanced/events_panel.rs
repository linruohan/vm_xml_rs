use egui::RichText;

use crate::model::vm_config::{EventsConfig, VMConfig};

/// 系统事件配置面板
pub struct EventsPanel;

impl EventsPanel {
    /// 显示系统事件配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.group(|ui| {
            ui.label(RichText::new("系统事件配置").strong());
            ui.add_space(5.0);

            let mut has_events = config.events.is_some();
            if ui.checkbox(&mut has_events, "启用事件配置").changed() {
                if has_events {
                    config.events = Some(EventsConfig::default());
                } else {
                    config.events = None;
                }
            }

            if let Some(ref mut events) = config.events {
                egui::Grid::new("events_grid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                    ui.label("关机时:");
                    let mut on_poweroff = events.on_poweroff.clone().unwrap_or_default();
                    egui::ComboBox::from_id_source("on_poweroff")
                        .selected_text(&on_poweroff)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut on_poweroff, "destroy".to_string(), "destroy");
                            ui.selectable_value(
                                &mut on_poweroff,
                                "preserve".to_string(),
                                "preserve",
                            );
                            ui.selectable_value(&mut on_poweroff, "restart".to_string(), "restart");
                            ui.selectable_value(
                                &mut on_poweroff,
                                "shutdown".to_string(),
                                "shutdown",
                            );
                        });
                    events.on_poweroff =
                        if on_poweroff.is_empty() { None } else { Some(on_poweroff) };
                    ui.end_row();

                    ui.label("重启时:");
                    let mut on_reboot = events.on_reboot.clone().unwrap_or_default();
                    egui::ComboBox::from_id_source("on_reboot").selected_text(&on_reboot).show_ui(
                        ui,
                        |ui| {
                            ui.selectable_value(&mut on_reboot, "destroy".to_string(), "destroy");
                            ui.selectable_value(&mut on_reboot, "preserve".to_string(), "preserve");
                            ui.selectable_value(&mut on_reboot, "restart".to_string(), "restart");
                            ui.selectable_value(&mut on_reboot, "shutdown".to_string(), "shutdown");
                        },
                    );
                    events.on_reboot = if on_reboot.is_empty() { None } else { Some(on_reboot) };
                    ui.end_row();

                    ui.label("崩溃时:");
                    let mut on_crash = events.on_crash.clone().unwrap_or_default();
                    egui::ComboBox::from_id_source("on_crash").selected_text(&on_crash).show_ui(
                        ui,
                        |ui| {
                            ui.selectable_value(&mut on_crash, "destroy".to_string(), "destroy");
                            ui.selectable_value(&mut on_crash, "preserve".to_string(), "preserve");
                            ui.selectable_value(&mut on_crash, "restart".to_string(), "restart");
                            ui.selectable_value(&mut on_crash, "shutdown".to_string(), "shutdown");
                        },
                    );
                    events.on_crash = if on_crash.is_empty() { None } else { Some(on_crash) };
                    ui.end_row();

                    ui.label("锁定失败时:");
                    let mut on_lockfailure = events.on_lockfailure.clone().unwrap_or_default();
                    egui::ComboBox::from_id_source("on_lockfailure")
                        .selected_text(&on_lockfailure)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut on_lockfailure,
                                "destroy".to_string(),
                                "destroy",
                            );
                            ui.selectable_value(&mut on_lockfailure, "pause".to_string(), "pause");
                            ui.selectable_value(
                                &mut on_lockfailure,
                                "restart".to_string(),
                                "restart",
                            );
                        });
                    events.on_lockfailure =
                        if on_lockfailure.is_empty() { None } else { Some(on_lockfailure) };
                    ui.end_row();
                });
            }
        });
    }
}
