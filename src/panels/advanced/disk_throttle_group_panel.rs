use egui::RichText;

use crate::model::{DiskThrottleGroupConfig, ThrottleConfig, VMConfig};

/// 磁盘 I/O 限流组配置面板
pub struct DiskThrottleGroupPanel;

impl DiskThrottleGroupPanel {
    /// 显示磁盘 I/O 限流组配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.group(|ui| {
            ui.label(RichText::new("磁盘 I/O 限流组配置").strong());
            ui.add_space(5.0);

            let mut has_throttle = config.disk_throttle_group.is_some();
            if ui.checkbox(&mut has_throttle, "启用磁盘限流组").changed() {
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
                ui.label("组名称:");
                ui.text_edit_singleline(&mut throttle_group.name);

                ui.collapsing("限流配置", |ui| {
                    if throttle_group.throttle.is_none() {
                        throttle_group.throttle = Some(ThrottleConfig::default());
                    }
                    if let Some(ref mut throttle) = throttle_group.throttle {
                        egui::Grid::new("group_throttle_grid")
                            .num_columns(2)
                            .spacing([10.0, 8.0])
                            .show(ui, |ui| {
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

                                ui.label("读IOPS/秒:");
                                let mut read_iops = throttle.read_iops_sec.unwrap_or(0);
                                ui.add(egui::DragValue::new(&mut read_iops));
                                throttle.read_iops_sec =
                                    if read_iops > 0 { Some(read_iops) } else { None };
                                ui.end_row();

                                ui.label("写IOPS/秒:");
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
    }
}
