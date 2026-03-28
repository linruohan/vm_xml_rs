use egui::RichText;

use crate::model::{ClockConfig, RTCConfig, TimeKeepingConfig, TimerConfig, VMConfig};

/// 时间同步机制配置面板
pub struct TimeKeepingPanel;

impl TimeKeepingPanel {
    /// 显示时间同步机制配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.group(|ui| {
            ui.label(RichText::new("时间同步机制配置").strong());
            ui.add_space(5.0);

            let mut has_time = config.time_keeping.is_some();
            if ui.checkbox(&mut has_time, "启用时间同步配置").changed() {
                if has_time {
                    config.time_keeping = Some(TimeKeepingConfig {
                        clock: Some(ClockConfig { offset: "utc".to_string(), timer: None }),
                        rtc: None,
                    });
                } else {
                    config.time_keeping = None;
                }
            }

            if let Some(ref mut time) = config.time_keeping {
                if let Some(ref mut clock) = time.clock {
                    ui.label("时钟偏移:");
                    egui::ComboBox::from_id_source("clock_offset")
                        .selected_text(&clock.offset)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut clock.offset, "utc".to_string(), "utc");
                            ui.selectable_value(
                                &mut clock.offset,
                                "localtime".to_string(),
                                "localtime",
                            );
                            ui.selectable_value(
                                &mut clock.offset,
                                "timezone".to_string(),
                                "timezone",
                            );
                            ui.selectable_value(
                                &mut clock.offset,
                                "variable".to_string(),
                                "variable",
                            );
                        });

                    ui.collapsing("定时器配置", |ui| {
                        if clock.timer.is_none() {
                            clock.timer = Some(Vec::new());
                        }
                        if let Some(ref mut timer_list) = clock.timer {
                            if ui.button("➕ 添加定时器").clicked() {
                                timer_list.push(TimerConfig {
                                    name: "rtc".to_string(),
                                    present: Some("yes".to_string()),
                                    frequency: None,
                                    tickpolicy: None,
                                });
                            }

                            let mut to_remove = None;
                            for (i, timer) in timer_list.iter_mut().enumerate() {
                                ui.push_id(i, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label("名称:");
                                        ui.text_edit_singleline(&mut timer.name);
                                        if ui.button("🗑️").clicked() {
                                            to_remove = Some(i);
                                        }
                                    });
                                });
                            }
                            if let Some(idx) = to_remove {
                                timer_list.remove(idx);
                            }
                        }
                    });
                }

                ui.collapsing("RTC 配置", |ui| {
                    if time.rtc.is_none() {
                        time.rtc = Some(RTCConfig::default());
                    }
                    if let Some(ref mut rtc) = time.rtc {
                        egui::Grid::new("rtc_grid").num_columns(2).spacing([10.0, 8.0]).show(
                            ui,
                            |ui| {
                                ui.label("tickpolicy:");
                                let mut tickpolicy = rtc.tickpolicy.clone().unwrap_or_default();
                                egui::ComboBox::from_id_source("rtc_tickpolicy")
                                    .selected_text(&tickpolicy)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut tickpolicy,
                                            "catchup".to_string(),
                                            "catchup",
                                        );
                                        ui.selectable_value(
                                            &mut tickpolicy,
                                            "delay".to_string(),
                                            "delay",
                                        );
                                        ui.selectable_value(
                                            &mut tickpolicy,
                                            "none".to_string(),
                                            "none",
                                        );
                                    });
                                rtc.tickpolicy =
                                    if tickpolicy.is_empty() { None } else { Some(tickpolicy) };
                                ui.end_row();

                                ui.label("base:");
                                let mut base = rtc.base.clone().unwrap_or_default();
                                egui::ComboBox::from_id_source("rtc_base")
                                    .selected_text(&base)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut base, "utc".to_string(), "utc");
                                        ui.selectable_value(
                                            &mut base,
                                            "localtime".to_string(),
                                            "localtime",
                                        );
                                    });
                                rtc.base = if base.is_empty() { None } else { Some(base) };
                                ui.end_row();
                            },
                        );
                    }
                });
            }
        });
    }
}
