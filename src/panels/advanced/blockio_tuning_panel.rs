use crate::{
    model::{BlockIOTuningConfig, DeviceWeight, ThrottleConfig, VMConfig},
    panels::utils::*,
};

/// 块设备 I/O 调优配置面板
pub struct BlockIOTuningPanel;

impl BlockIOTuningPanel {
    /// 显示块设备 I/O 调优配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "💽", "块设备 I/O 调优");

        card_group(ui, "I/O 权重设置", None, colors, |ui| {
            let mut has_tuning = config.blockio_tuning.is_some();
            if checkbox(ui, &mut has_tuning, "启用块设备 I/O 调优") {
                if has_tuning {
                    config.blockio_tuning = Some(BlockIOTuningConfig::default());
                } else {
                    config.blockio_tuning = None;
                }
            }

            if let Some(ref mut tuning) = config.blockio_tuning {
                ui.add_space(5.0);
                ui.label("全局权重:");
                let mut weight = tuning.weight.unwrap_or(0);
                ui.add(egui::Slider::new(&mut weight, 0..=1000).text(""));
                tuning.weight = if weight > 0 { Some(weight) } else { None };

                ui.add_space(5.0);
                ui.collapsing("设备权重", |ui| {
                    if tuning.device_weight.is_none() {
                        tuning.device_weight = Some(Vec::new());
                    }
                    if let Some(ref mut device_list) = tuning.device_weight {
                        ui.horizontal(|ui| {
                            if add_button(ui, "➕ 添加设备权重", colors) {
                                device_list
                                    .push(DeviceWeight { dev: "sda".to_string(), weight: 500 });
                            }
                        });

                        let mut to_remove = None;
                        for (i, device) in device_list.iter_mut().enumerate() {
                            ui.push_id(i, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("设备:");
                                    ui.text_edit_singleline(&mut device.dev);
                                    ui.label("权重:");
                                    ui.add(
                                        egui::Slider::new(&mut device.weight, 1..=1000).text(""),
                                    );
                                    if delete_button(ui, None) {
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

                ui.add_space(5.0);
                ui.collapsing("I/O 限流", |ui| {
                    if tuning.throttle.is_none() {
                        tuning.throttle = Some(ThrottleConfig::default());
                    }
                    if let Some(ref mut throttle) = tuning.throttle {
                        grid(ui, "throttle_grid", 2, |ui| {
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
    }
}
