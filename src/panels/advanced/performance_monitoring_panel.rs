use egui::{Align, Layout};

use crate::{
    model::{EventConfig, PMUConfig, PerformanceMonitoringConfig, VMConfig},
    panels::utils::*,
};

/// 性能监控相关事件配置面板
pub struct PerformanceMonitoringPanel;

impl PerformanceMonitoringPanel {
    /// 显示性能监控相关事件配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "📈", "性能监控配置");

        // 卡片宽度和间距配置
        let card_width = 380.0;
        let spacing = 8.0;

        // 使用流式布局实现卡片自动换行
        ui.with_layout(Layout::left_to_right(Align::TOP).with_main_wrap(true), |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);

            // PMU 设置卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "PMU 设置", Some("🔧"), colors, |ui| {
                        let mut has_perf = config.performance_monitoring.is_some();
                        if checkbox(ui, &mut has_perf, "启用性能监控") {
                            if has_perf {
                                config.performance_monitoring = Some(PerformanceMonitoringConfig {
                                    pmu: Some(PMUConfig { state: "on".to_string() }),
                                    events: None,
                                });
                            } else {
                                config.performance_monitoring = None;
                            }
                        }

                        if let Some(ref mut perf) = config.performance_monitoring {
                            if let Some(ref mut pmu) = perf.pmu {
                                grid(ui, "pmu_grid", 2, |ui| {
                                    ui.label("PMU 状态:");
                                    egui::ComboBox::from_id_source("pmu_state")
                                        .selected_text(&pmu.state)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut pmu.state,
                                                "on".to_string(),
                                                "on",
                                            );
                                            ui.selectable_value(
                                                &mut pmu.state,
                                                "off".to_string(),
                                                "off",
                                            );
                                        });
                                    ui.end_row();
                                });
                            }
                        }
                    });
                },
            );

            // 监控事件卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "监控事件", Some("📊"), colors, |ui| {
                        if let Some(ref mut perf) = config.performance_monitoring {
                            if perf.events.is_none() {
                                perf.events = Some(Vec::new());
                            }
                            if let Some(ref mut event_list) = perf.events {
                                ui.horizontal(|ui| {
                                    if add_button(ui, "➕ 添加事件", colors) {
                                        event_list.push(EventConfig {
                                            name: "cpu-cycles".to_string(),
                                            count: Some(1),
                                        });
                                    }
                                });

                                let mut to_remove = None;
                                for (i, event) in event_list.iter_mut().enumerate() {
                                    ui.push_id(i, |ui| {
                                        inner_group(ui, colors, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label("事件名称:");
                                                ui.text_edit_singleline(&mut event.name);
                                                ui.label("计数:");
                                                let mut count = event.count.unwrap_or(1);
                                                ui.add(egui::DragValue::new(&mut count));
                                                event.count = Some(count);
                                                if delete_button(ui, None) {
                                                    to_remove = Some(i);
                                                }
                                            });
                                        });
                                        ui.add_space(5.0);
                                    });
                                }
                                if let Some(idx) = to_remove {
                                    event_list.remove(idx);
                                }
                            }
                        }
                    });
                },
            );
        });
    }
}
