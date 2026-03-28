use egui::RichText;

use crate::model::vm_config::{CPUTuningConfig, EmulatorPin, VCPUPin, VMConfig};

/// CPU 调优配置面板
pub struct CPUTuningPanel;

impl CPUTuningPanel {
    /// 显示 CPU 调优配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.group(|ui| {
            ui.label(RichText::new("CPU 调优").strong());
            ui.add_space(5.0);

            let mut has_tuning = config.cpu_tuning.is_some();
            if ui.checkbox(&mut has_tuning, "启用 CPU 调优").changed() {
                if has_tuning {
                    config.cpu_tuning = Some(CPUTuningConfig::default());
                } else {
                    config.cpu_tuning = None;
                }
            }

            if let Some(ref mut tuning) = config.cpu_tuning {
                ui.add_space(5.0);

                ui.collapsing("vCPU 绑定", |ui| {
                    if tuning.vcpupin.is_none() {
                        tuning.vcpupin = Some(Vec::new());
                    }
                    if let Some(ref mut pin_list) = tuning.vcpupin {
                        if ui.button("➕ 添加 vCPU 绑定").clicked() {
                            pin_list.push(VCPUPin {
                                vcpu: pin_list.len() as u32,
                                cpuset: "0".to_string(),
                            });
                        }

                        let mut to_remove = None;
                        for (i, pin) in pin_list.iter_mut().enumerate() {
                            ui.push_id(i, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(format!("vCPU {}:", pin.vcpu));
                                    ui.label("绑定到 CPU:");
                                    ui.text_edit_singleline(&mut pin.cpuset);
                                    if ui.button("🗑️").clicked() {
                                        to_remove = Some(i);
                                    }
                                });
                            });
                        }
                        if let Some(idx) = to_remove {
                            pin_list.remove(idx);
                        }
                    }
                });

                ui.collapsing("模拟器绑定", |ui| {
                    let mut has_emu_pin = tuning.emulatorpin.is_some();
                    if ui.checkbox(&mut has_emu_pin, "启用模拟器绑定").changed() {
                        if has_emu_pin {
                            tuning.emulatorpin = Some(EmulatorPin { cpuset: "0".to_string() });
                        } else {
                            tuning.emulatorpin = None;
                        }
                    }

                    if let Some(ref mut emu_pin) = tuning.emulatorpin {
                        ui.horizontal(|ui| {
                            ui.label("CPU 集合:");
                            ui.text_edit_singleline(&mut emu_pin.cpuset);
                        });
                    }
                });
            }
        });
    }
}
