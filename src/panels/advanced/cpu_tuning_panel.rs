use egui::{Align, Layout};

use crate::{
    model::{
        CPUTuningConfig, EmulatorPin, IOThreadPin, IOThreadschedConfig, VCPUPin, VCpuschedConfig,
        VMConfig,
    },
    panels::utils::*,
};

/// CPU 调优配置面板
pub struct CPUTuningPanel;

impl CPUTuningPanel {
    /// 显示 CPU 调优配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "⚡", "CPU 调优");

        // 卡片宽度和间距配置
        let card_width = 380.0;
        let spacing = 8.0;

        // 使用流式布局实现卡片自动换行
        ui.with_layout(Layout::left_to_right(Align::TOP).with_main_wrap(true), |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);

            // CPU 带宽设置卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "CPU 带宽设置", Some("📊"), colors, |ui| {
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
                            grid(ui, "cpu_bw_grid", 2, |ui| {
                                ui.label("Shares:");
                                let shares = tuning.shares.get_or_insert(0);
                                ui.add(egui::Slider::new(shares, 0..=1_000_000).text("shares"));
                                ui.end_row();

                                ui.label("Period:");
                                let period = tuning.period.get_or_insert(0);
                                ui.add(egui::Slider::new(period, 0..=1_000_000_000).text("ns"));
                                ui.end_row();

                                ui.label("Quota:");
                                let quota = tuning.quota.get_or_insert(-1);
                                ui.add(egui::Slider::new(quota, -1..=1_000_000_000).text("ns"));
                                ui.end_row();

                                ui.label("全局 Period:");
                                let global_period = tuning.global_period.get_or_insert(0);
                                ui.add(
                                    egui::Slider::new(global_period, 0..=1_000_000_000).text("ns"),
                                );
                                ui.end_row();

                                ui.label("全局 Quota:");
                                let global_quota = tuning.global_quota.get_or_insert(-1);
                                ui.add(
                                    egui::Slider::new(global_quota, -1..=1_000_000_000).text("ns"),
                                );
                                ui.end_row();
                            });
                        }
                    });
                },
            );

            // vCPU 绑定卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "vCPU 绑定", Some("🔗"), colors, |ui| {
                        if let Some(ref mut tuning) = config.cpu_tuning {
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
                        }
                    });
                },
            );

            // 模拟器绑定卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "模拟器绑定", Some("🎭"), colors, |ui| {
                        if let Some(ref mut tuning) = config.cpu_tuning {
                            let mut has_emu_pin = tuning.emulatorpin.is_some();
                            if checkbox(ui, &mut has_emu_pin, "启用模拟器绑定") {
                                if has_emu_pin {
                                    tuning.emulatorpin =
                                        Some(EmulatorPin { cpuset: "0".to_string() });
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
                        }
                    });
                },
            );

            // IO 线程绑定卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "IO 线程绑定", Some("💾"), colors, |ui| {
                        if let Some(ref mut tuning) = config.cpu_tuning {
                            let mut has_iothread_pin = tuning.iothreadpin.is_some();
                            if checkbox(ui, &mut has_iothread_pin, "启用 IO 线程绑定") {
                                if has_iothread_pin {
                                    tuning.iothreadpin = Some(Vec::new());
                                } else {
                                    tuning.iothreadpin = None;
                                }
                            }

                            if let Some(ref mut pin_list) = tuning.iothreadpin {
                                ui.horizontal(|ui| {
                                    if add_button(ui, "➕ 添加 IO 线程绑定", colors) {
                                        pin_list.push(IOThreadPin {
                                            iothread: pin_list.len() as u32,
                                            cpuset: "0".to_string(),
                                        });
                                    }
                                });

                                let mut to_remove = None;
                                for (i, pin) in pin_list.iter_mut().enumerate() {
                                    ui.push_id(i, |ui| {
                                        inner_group(ui, colors, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(format!("IO 线程 {}", pin.iothread));
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
                        }
                    });
                },
            );

            // vCPU 调度器卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "vCPU 调度器", Some("📅"), colors, |ui| {
                        if let Some(ref mut tuning) = config.cpu_tuning {
                            if tuning.vcpusched.is_none() {
                                tuning.vcpusched = Some(Vec::new());
                            }
                            if let Some(ref mut sched_list) = tuning.vcpusched {
                                ui.horizontal(|ui| {
                                    if add_button(ui, "➕ 添加调度器", colors) {
                                        sched_list.push(VCpuschedConfig {
                                            vcpus: "0".to_string(),
                                            scheduler: "fifo".to_string(),
                                            priority: Some(0),
                                        });
                                    }
                                });

                                let mut to_remove = None;
                                for (i, sched) in sched_list.iter_mut().enumerate() {
                                    ui.push_id(i, |ui| {
                                        inner_group(ui, colors, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(format!("调度器 {}", i + 1));
                                                ui.label("vCPUs:");
                                                ui.text_edit_singleline(&mut sched.vcpus);
                                                ui.label("调度策略:");
                                                egui::ComboBox::from_id_source(format!(
                                                    "vcpu_sched_{}",
                                                    i
                                                ))
                                                .selected_text(&sched.scheduler)
                                                .show_ui(ui, |ui| {
                                                    ui.selectable_value(
                                                        &mut sched.scheduler,
                                                        "fifo".to_string(),
                                                        "fifo",
                                                    );
                                                    ui.selectable_value(
                                                        &mut sched.scheduler,
                                                        "rr".to_string(),
                                                        "rr",
                                                    );
                                                    ui.selectable_value(
                                                        &mut sched.scheduler,
                                                        "other".to_string(),
                                                        "other",
                                                    );
                                                    ui.selectable_value(
                                                        &mut sched.scheduler,
                                                        "batch".to_string(),
                                                        "batch",
                                                    );
                                                    ui.selectable_value(
                                                        &mut sched.scheduler,
                                                        "idle".to_string(),
                                                        "idle",
                                                    );
                                                });
                                                ui.label("优先级:");
                                                let priority = sched.priority.get_or_insert(0);
                                                ui.add(
                                                    egui::DragValue::new(priority)
                                                        .clamp_range(0..=99),
                                                );
                                                if delete_button(ui, None) {
                                                    to_remove = Some(i);
                                                }
                                            });
                                        });
                                        ui.add_space(5.0);
                                    });
                                }
                                if let Some(idx) = to_remove {
                                    sched_list.remove(idx);
                                }
                            }
                        }
                    });
                },
            );

            // IO 线程调度器卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "IO 线程调度器", Some("📅"), colors, |ui| {
                        if let Some(ref mut tuning) = config.cpu_tuning {
                            if tuning.iothreadsched.is_none() {
                                tuning.iothreadsched = Some(Vec::new());
                            }
                            if let Some(ref mut sched_list) = tuning.iothreadsched {
                                ui.horizontal(|ui| {
                                    if add_button(ui, "➕ 添加 IO 调度器", colors) {
                                        sched_list.push(IOThreadschedConfig {
                                            iothreads: "0".to_string(),
                                            scheduler: "fifo".to_string(),
                                            priority: Some(0),
                                        });
                                    }
                                });

                                let mut to_remove = None;
                                for (i, sched) in sched_list.iter_mut().enumerate() {
                                    ui.push_id(i, |ui| {
                                        inner_group(ui, colors, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(format!("IO 调度器 {}", i + 1));
                                                ui.label("IO 线程:");
                                                ui.text_edit_singleline(&mut sched.iothreads);
                                                ui.label("调度策略:");
                                                egui::ComboBox::from_id_source(format!(
                                                    "io_sched_{}",
                                                    i
                                                ))
                                                .selected_text(&sched.scheduler)
                                                .show_ui(ui, |ui| {
                                                    ui.selectable_value(
                                                        &mut sched.scheduler,
                                                        "fifo".to_string(),
                                                        "fifo",
                                                    );
                                                    ui.selectable_value(
                                                        &mut sched.scheduler,
                                                        "rr".to_string(),
                                                        "rr",
                                                    );
                                                    ui.selectable_value(
                                                        &mut sched.scheduler,
                                                        "other".to_string(),
                                                        "other",
                                                    );
                                                });
                                                ui.label("优先级:");
                                                let priority = sched.priority.get_or_insert(0);
                                                ui.add(
                                                    egui::DragValue::new(priority)
                                                        .clamp_range(0..=99),
                                                );
                                                if delete_button(ui, None) {
                                                    to_remove = Some(i);
                                                }
                                            });
                                        });
                                        ui.add_space(5.0);
                                    });
                                }
                                if let Some(idx) = to_remove {
                                    sched_list.remove(idx);
                                }
                            }
                        }
                    });
                },
            );
        });
    }
}
