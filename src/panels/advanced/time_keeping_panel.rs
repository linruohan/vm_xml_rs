use egui::{Align, Layout};

use crate::{
    model::{ClockConfig, RTCConfig, TimeKeepingConfig, TimerConfig, VMConfig},
    panels::utils::*,
};

/// 时间同步机制配置面板
pub struct TimeKeepingPanel;

impl TimeKeepingPanel {
    /// 显示时间同步机制配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "⏰", "时间同步机制配置");

        // 卡片宽度和间距配置
        let card_width = 380.0;
        let spacing = 8.0;

        // 使用流式布局实现卡片自动换行
        ui.with_layout(Layout::left_to_right(Align::TOP).with_main_wrap(true), |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);

            // 时钟设置卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "时钟设置", Some("🕐"), colors, |ui| {
                        let mut has_time = config.time_keeping.is_some();
                        if checkbox(ui, &mut has_time, "启用时间同步配置") {
                            if has_time {
                                config.time_keeping = Some(TimeKeepingConfig {
                                    clock: Some(ClockConfig {
                                        offset: "utc".to_string(),
                                        timer: None,
                                    }),
                                    rtc: None,
                                });
                            } else {
                                config.time_keeping = None;
                            }
                        }

                        if let Some(ref mut time) = config.time_keeping {
                            if let Some(ref mut clock) = time.clock {
                                ui.add_space(5.0);
                                grid(ui, "clock_offset_grid", 2, |ui| {
                                    ui.label("时钟偏移:");
                                    egui::ComboBox::from_id_source("clock_offset")
                                        .selected_text(&clock.offset)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut clock.offset,
                                                "utc".to_string(),
                                                "utc",
                                            );
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
                                    ui.end_row();
                                });
                            }
                        }
                    });
                },
            );

            // 定时器配置卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "定时器配置", Some("⏲"), colors, |ui| {
                        if let Some(ref mut time) = config.time_keeping {
                            if let Some(ref mut clock) = time.clock {
                                if clock.timer.is_none() {
                                    clock.timer = Some(Vec::new());
                                }
                                if let Some(ref mut timer_list) = clock.timer {
                                    ui.horizontal(|ui| {
                                        if add_button(ui, "➕ 添加定时器", colors) {
                                            timer_list.push(TimerConfig {
                                                name: "rtc".to_string(),
                                                present: Some("yes".to_string()),
                                                frequency: None,
                                                tickpolicy: None,
                                                track: None,
                                                mode: None,
                                                catchup: None,
                                            });
                                        }
                                    });

                                    let mut to_remove = None;
                                    for (i, timer) in timer_list.iter_mut().enumerate() {
                                        ui.push_id(i, |ui| {
                                            inner_group(ui, colors, |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.label(format!("定时器 {}", i + 1));
                                                    ui.label("名称:");
                                                    ui.text_edit_singleline(&mut timer.name);
                                                    if delete_button(ui, None) {
                                                        to_remove = Some(i);
                                                    }
                                                });
                                            });
                                            ui.add_space(5.0);
                                        });
                                    }
                                    if let Some(idx) = to_remove {
                                        timer_list.remove(idx);
                                    }
                                }
                            }
                        }
                    });
                },
            );

            // RTC 配置卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "RTC 配置", Some("🔧"), colors, |ui| {
                        if let Some(ref mut time) = config.time_keeping {
                            if time.rtc.is_none() {
                                time.rtc = Some(RTCConfig::default());
                            }
                            if let Some(ref mut rtc) = time.rtc {
                                grid(ui, "rtc_grid", 2, |ui| {
                                    ui.label("tickpolicy:");
                                    let tickpolicy =
                                        rtc.tickpolicy.get_or_insert_with(|| "catchup".to_string());
                                    egui::ComboBox::from_id_source("rtc_tickpolicy")
                                        .selected_text(tickpolicy.as_str())
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                tickpolicy,
                                                "catchup".to_string(),
                                                "catchup",
                                            );
                                            ui.selectable_value(
                                                tickpolicy,
                                                "delay".to_string(),
                                                "delay",
                                            );
                                            ui.selectable_value(
                                                tickpolicy,
                                                "none".to_string(),
                                                "none",
                                            );
                                        });
                                    ui.end_row();

                                    ui.label("base:");
                                    let base = rtc.base.get_or_insert_with(|| "utc".to_string());
                                    egui::ComboBox::from_id_source("rtc_base")
                                        .selected_text(base.as_str())
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(base, "utc".to_string(), "utc");
                                            ui.selectable_value(
                                                base,
                                                "localtime".to_string(),
                                                "localtime",
                                            );
                                        });
                                    ui.end_row();
                                });
                            }
                        }
                    });
                },
            );
        });
    }
}
