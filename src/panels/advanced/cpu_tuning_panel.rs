use crate::{
    model::{CPUTuningConfig, EmulatorPin, VCPUPin, VMConfig},
    panels::utils::*,
};

/// CPU 调优配置面板
pub struct CPUTuningPanel;

impl CPUTuningPanel {
    /// 显示 CPU 调优配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "⚡", "CPU 调优");

        card_group(ui, "CPU 绑定设置", None, colors, |ui| {
            let mut has_tuning = config.cpu_tuning.is_some();
            if checkbox(ui, &mut has_tuning, "启用 CPU 调优") {
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
                        ui.horizontal(|ui| {
                            if add_button(ui, "➕ 添加 vCPU 绑定", colors) {
                                pin_list.push(VCPUPin {
                                    vcpu: pin_list.len() as u32,
                                    cpuset: "0".to_string(),
                                });
                            }
                        });

                        let mut to_remove = None;
                        for (i, pin) in pin_list.iter_mut().enumerate() {
                            ui.push_id(i, |ui| {
                                inner_group(ui, colors, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("vCPU {}", pin.vcpu));
                                        ui.label("绑定到 CPU:");
                                        ui.text_edit_singleline(&mut pin.cpuset);
                                        if delete_button(ui, None) {
                                            to_remove = Some(i);
                                        }
                                    });
                                });
                                ui.add_space(5.0);
                            });
                        }
                        if let Some(idx) = to_remove {
                            pin_list.remove(idx);
                        }
                    }
                });

                ui.collapsing("模拟器绑定", |ui| {
                    let mut has_emu_pin = tuning.emulatorpin.is_some();
                    if checkbox(ui, &mut has_emu_pin, "启用模拟器绑定") {
                        if has_emu_pin {
                            tuning.emulatorpin = Some(EmulatorPin { cpuset: "0".to_string() });
                        } else {
                            tuning.emulatorpin = None;
                        }
                    }

                    if let Some(ref mut emu_pin) = tuning.emulatorpin {
                        ui.add_space(5.0);
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
