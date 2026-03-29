use egui::{Align, Layout};

use crate::{
    model::{DiskThrottleGroupConfig, ThrottleConfig, VMConfig},
    panels::utils::*,
};

/// 磁盘 I/O 限流组配置面板
pub struct DiskThrottleGroupPanel;

impl DiskThrottleGroupPanel {
    /// 显示磁盘 I/O 限流组配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "⏱", "磁盘 I/O 限流组配置");

        let card_width = 380.0;
        let spacing = 8.0;

        ui.with_layout(Layout::left_to_right(Align::TOP).with_main_wrap(true), |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);

            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "限流组设置", Some("🎛"), colors, |ui| {
                        let mut has_throttle = config.disk_throttle_group.is_some();
                        if checkbox(ui, &mut has_throttle, "启用磁盘限流组") {
                            if has_throttle {
                                config.disk_throttle_group = Some(DiskThrottleGroupConfig {
                                    name: "default".to_string(),
                                    throttle: None,
                                });
                            } else {
                                config.disk_throttle_group = None;
                            }
                        }

                        if let Some(ref mut throttle_group) = config.disk_throttle_group {
                            ui.add_space(5.0);
                            grid(ui, "group_name_grid", 2, |ui| {
                                ui.label("组名称:");
                                ui.text_edit_singleline(&mut throttle_group.name);
                                ui.end_row();
                            });

                            ui.add_space(5.0);
                            ui.collapsing("限流配置", |ui| {
                                if throttle_group.throttle.is_none() {
                                    throttle_group.throttle = Some(ThrottleConfig::default());
                                }
                                if let Some(ref mut throttle) = throttle_group.throttle {
                                    grid(ui, "group_throttle_grid", 2, |ui| {
                                        ui.label("读字节/秒:");
                                        let mut read_bytes = throttle.read_bytes_sec.unwrap_or(0);
                                        ui.add(egui::DragValue::new(&mut read_bytes));
                                        throttle.read_bytes_sec =
                                            if read_bytes > 0 { Some(read_bytes) } else { None };
                                        ui.end_row();

                                        ui.label("写字节/秒:");
                                        let mut write_bytes = throttle.write_bytes_sec.unwrap_or(0);
                                        ui.add(egui::DragValue::new(&mut write_bytes));
                                        throttle.write_bytes_sec =
                                            if write_bytes > 0 { Some(write_bytes) } else { None };
                                        ui.end_row();

                                        ui.label("读 IOPS/秒:");
                                        let mut read_iops = throttle.read_iops_sec.unwrap_or(0);
                                        ui.add(egui::DragValue::new(&mut read_iops));
                                        throttle.read_iops_sec =
                                            if read_iops > 0 { Some(read_iops) } else { None };
                                        ui.end_row();

                                        ui.label("写 IOPS/秒:");
                                        let mut write_iops = throttle.write_iops_sec.unwrap_or(0);
                                        ui.add(egui::DragValue::new(&mut write_iops));
                                        throttle.write_iops_sec =
                                            if write_iops > 0 { Some(write_iops) } else { None };
                                        ui.end_row();
                                    });
                                }
                            });
                        }
                    });
                },
            );
        });
    }
}
