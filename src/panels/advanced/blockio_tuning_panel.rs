use egui::RichText;

use crate::model::{BlockIOTuningConfig, DeviceWeight, ThrottleConfig, VMConfig};

/// 块设备 I/O 调优配置面板
pub struct BlockIOTuningPanel;

impl BlockIOTuningPanel {
    /// 显示块设备 I/O 调优配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.group(|ui| {
            ui.label(RichText::new("块设备 I/O 调优").strong());
            ui.add_space(5.0);

            let mut has_tuning = config.blockio_tuning.is_some();
            if ui.checkbox(&mut has_tuning, "启用块设备 I/O 调优").changed() {
                if has_tuning {
                    config.blockio_tuning = Some(BlockIOTuningConfig::default());
                } else {
                    config.blockio_tuning = None;
                }
            }

            if let Some(ref mut tuning) = config.blockio_tuning {
                ui.label("全局权重:");
                let mut weight = tuning.weight.unwrap_or(0);
                ui.add(egui::Slider::new(&mut weight, 0..=1000));
                tuning.weight = if weight > 0 { Some(weight) } else { None };

                ui.collapsing("设备权重", |ui| {
                    if tuning.device_weight.is_none() {
                        tuning.device_weight = Some(Vec::new());
                    }
                    if let Some(ref mut device_list) = tuning.device_weight {
                        if ui.button("➕ 添加设备权重").clicked() {
                            device_list.push(DeviceWeight { dev: "sda".to_string(), weight: 500 });
                        }

                        let mut to_remove = None;
                        for (i, device) in device_list.iter_mut().enumerate() {
                            ui.push_id(i, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("设备:");
                                    ui.text_edit_singleline(&mut device.dev);
                                    ui.label("权重:");
                                    ui.add(egui::Slider::new(&mut device.weight, 1..=1000));
                                    if ui.button("🗑️").clicked() {
                                        to_remove = Some(i);
                                    }
                                });
                            });
                        }
                        if let Some(idx) = to_remove {
                            device_list.remove(idx);
                        }
                    }
                });

                ui.collapsing("I/O 限流", |ui| {
                    if tuning.throttle.is_none() {
                        tuning.throttle = Some(ThrottleConfig::default());
                    }
                    if let Some(ref mut throttle) = tuning.throttle {
                        egui::Grid::new("throttle_grid").num_columns(2).spacing([10.0, 8.0]).show(
                            ui,
                            |ui| {
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
                            },
                        );
                    }
                });
            }
        });
    }
}
