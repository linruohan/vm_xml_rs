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

        card_group(ui, "PMU 设置", None, colors, |ui| {
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
                                ui.selectable_value(&mut pmu.state, "on".to_string(), "on");
                                ui.selectable_value(&mut pmu.state, "off".to_string(), "off");
                            });
                        ui.end_row();
                    });
                }

                ui.add_space(5.0);
                ui.collapsing("监控事件", |ui| {
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
                });
            }
        });
    }
}
