use crate::{
    model::{
        CPUTuningConfig, CachetuneConfig, EmulatorPin, EmulatorschedConfig, IOThreadPin,
        IOThreadschedConfig, MemorytuneConfig, MonitorConfig, NodeConfig, VCPUPin, VCpuschedConfig,
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

                // 基础 CPU 带宽参数
                grid(ui, "cpu_bw_grid", 2, |ui| {
                    ui.label("Shares:");
                    let shares = tuning.shares.get_or_insert(0);
                    ui.add(egui::Slider::new(shares, 0..=1000000).text("shares"));
                    ui.end_row();

                    ui.label("Period:");
                    let period = tuning.period.get_or_insert(0);
                    ui.add(egui::Slider::new(period, 0..=1000000000).text("ns"));
                    ui.end_row();

                    ui.label("Quota:");
                    let quota = tuning.quota.get_or_insert(-1);
                    ui.add(egui::Slider::new(quota, -1..=1000000000).text("ns"));
                    ui.end_row();

                    ui.label("全局 Period:");
                    let global_period = tuning.global_period.get_or_insert(0);
                    ui.add(egui::Slider::new(global_period, 0..=1000000000).text("ns"));
                    ui.end_row();

                    ui.label("全局 Quota:");
                    let global_quota = tuning.global_quota.get_or_insert(-1);
                    ui.add(egui::Slider::new(global_quota, -1..=1000000000).text("ns"));
                    ui.end_row();

                    ui.label("模拟器 Period:");
                    let emulator_period = tuning.emulator_period.get_or_insert(0);
                    ui.add(egui::Slider::new(emulator_period, 0..=1000000000).text("ns"));
                    ui.end_row();

                    ui.label("模拟器 Quota:");
                    let emulator_quota = tuning.emulator_quota.get_or_insert(-1);
                    ui.add(egui::Slider::new(emulator_quota, -1..=1000000000).text("ns"));
                    ui.end_row();

                    ui.label("IO 线程 Period:");
                    let iothread_period = tuning.iothread_period.get_or_insert(0);
                    ui.add(egui::Slider::new(iothread_period, 0..=1000000000).text("ns"));
                    ui.end_row();

                    ui.label("IO 线程 Quota:");
                    let iothread_quota = tuning.iothread_quota.get_or_insert(-1);
                    ui.add(egui::Slider::new(iothread_quota, -1..=1000000000).text("ns"));
                    ui.end_row();
                });

                ui.add_space(10.0);

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

                ui.collapsing("IO 线程绑定", |ui| {
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
                });

                ui.add_space(10.0);

                // 调度器配置
                ui.collapsing("vCPU 调度器", |ui| {
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
                                        egui::ComboBox::from_id_source(format!("vcpu_sched_{}", i))
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
                                        ui.add(egui::DragValue::new(priority).clamp_range(0..=99));
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
                });

                ui.collapsing("IO 线程调度器", |ui| {
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
                                        egui::ComboBox::from_id_source(format!("io_sched_{}", i))
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
                                        ui.add(egui::DragValue::new(priority).clamp_range(0..=99));
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
                });

                ui.collapsing("模拟器调度器", |ui| {
                    let mut has_emu_sched = tuning.emulatorsched.is_some();
                    if checkbox(ui, &mut has_emu_sched, "启用模拟器调度器") {
                        if has_emu_sched {
                            tuning.emulatorsched = Some(EmulatorschedConfig {
                                scheduler: "fifo".to_string(),
                                priority: Some(0),
                            });
                        } else {
                            tuning.emulatorsched = None;
                        }
                    }

                    if let Some(ref mut emu_sched) = tuning.emulatorsched {
                        ui.add_space(5.0);
                        grid(ui, "emu_sched_grid", 2, |ui| {
                            ui.label("调度策略:");
                            egui::ComboBox::from_id_source("emu_scheduler")
                                .selected_text(&emu_sched.scheduler)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut emu_sched.scheduler,
                                        "fifo".to_string(),
                                        "fifo",
                                    );
                                    ui.selectable_value(
                                        &mut emu_sched.scheduler,
                                        "rr".to_string(),
                                        "rr",
                                    );
                                    ui.selectable_value(
                                        &mut emu_sched.scheduler,
                                        "other".to_string(),
                                        "other",
                                    );
                                });
                            ui.end_row();

                            ui.label("优先级:");
                            let priority = emu_sched.priority.get_or_insert(0);
                            ui.add(egui::DragValue::new(priority).clamp_range(0..=99));
                            ui.end_row();
                        });
                    }
                });

                ui.add_space(10.0);

                // CacheTune 配置
                ui.collapsing("Cache 调优 (Cachetune)", |ui| {
                    if tuning.cachetune.is_none() {
                        tuning.cachetune = Some(Vec::new());
                    }
                    if let Some(ref mut cache_tune_list) = tuning.cachetune {
                        ui.horizontal(|ui| {
                            if add_button(ui, "➕ 添加 Cache 调优", colors) {
                                cache_tune_list.push(CachetuneConfig {
                                    vcpus: "0".to_string(),
                                    cache: None,
                                    monitor: None,
                                });
                            }
                        });

                        let mut to_remove = None;
                        for (i, cache_tune) in cache_tune_list.iter_mut().enumerate() {
                            ui.push_id(i, |ui| {
                                inner_group(ui, colors, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("Cache 调优 {}", i + 1));
                                        ui.label("vCPUs:");
                                        ui.text_edit_singleline(&mut cache_tune.vcpus);
                                        if delete_button(ui, None) {
                                            to_remove = Some(i);
                                        }
                                    });

                                    ui.add_space(5.0);

                                    // Cache 配置
                                    ui.collapsing("Cache 配置", |ui| {
                                        if cache_tune.cache.is_none() {
                                            cache_tune.cache = Some(Vec::new());
                                        }
                                        if let Some(ref mut cache_list) = cache_tune.cache {
                                            ui.horizontal(|ui| {
                                                if add_button(ui, "➕ 添加 Cache", colors) {
                                                    cache_list.push(crate::model::CacheConfig {
                                                        level: Some(3),
                                                        mode: Some("emulate".to_string()),
                                                        associativity: None,
                                                        policy: None,
                                                        size: None,
                                                        line: None,
                                                    });
                                                }
                                            });

                                            let mut cache_to_remove = None;
                                            for (j, cache) in cache_list.iter_mut().enumerate() {
                                                ui.horizontal(|ui| {
                                                    ui.label(format!("Cache L{}", j + 1));
                                                    ui.label("Level:");
                                                    let level = cache.level.get_or_insert(3);
                                                    egui::ComboBox::from_id_source(format!(
                                                        "cache_level_{}_{}",
                                                        i, j
                                                    ))
                                                    .selected_text(format!("L{}", level))
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(level, 1, "L1");
                                                        ui.selectable_value(level, 2, "L2");
                                                        ui.selectable_value(level, 3, "L3");
                                                    });
                                                    ui.label("模式:");
                                                    let mode =
                                                        cache.mode.get_or_insert_with(|| {
                                                            "emulate".to_string()
                                                        });
                                                    egui::ComboBox::from_id_source(format!(
                                                        "cache_mode_{}_{}",
                                                        i, j
                                                    ))
                                                    .selected_text(mode.as_str())
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(
                                                            mode,
                                                            "emulate".to_string(),
                                                            "emulate",
                                                        );
                                                        ui.selectable_value(
                                                            mode,
                                                            "passthrough".to_string(),
                                                            "passthrough",
                                                        );
                                                        ui.selectable_value(
                                                            mode,
                                                            "disable".to_string(),
                                                            "disable",
                                                        );
                                                    });
                                                    if delete_button(ui, None) {
                                                        cache_to_remove = Some(j);
                                                    }
                                                });
                                            }
                                            if let Some(idx) = cache_to_remove {
                                                cache_list.remove(idx);
                                            }
                                        }
                                    });

                                    // Monitor 配置
                                    ui.collapsing("Monitor 配置", |ui| {
                                        if cache_tune.monitor.is_none() {
                                            cache_tune.monitor = Some(Vec::new());
                                        }
                                        if let Some(ref mut monitor_list) = cache_tune.monitor {
                                            ui.horizontal(|ui| {
                                                if add_button(ui, "➕ 添加 Monitor", colors) {
                                                    monitor_list.push(MonitorConfig {
                                                        level: 3,
                                                        vcpus: "0".to_string(),
                                                    });
                                                }
                                            });

                                            let mut monitor_to_remove = None;
                                            for (j, monitor) in monitor_list.iter_mut().enumerate()
                                            {
                                                ui.horizontal(|ui| {
                                                    ui.label(format!("Monitor {}", j + 1));
                                                    ui.label("Level:");
                                                    ui.add(
                                                        egui::DragValue::new(&mut monitor.level)
                                                            .clamp_range(1..=3),
                                                    );
                                                    ui.label("vCPUs:");
                                                    ui.text_edit_singleline(&mut monitor.vcpus);
                                                    if delete_button(ui, None) {
                                                        monitor_to_remove = Some(j);
                                                    }
                                                });
                                            }
                                            if let Some(idx) = monitor_to_remove {
                                                monitor_list.remove(idx);
                                            }
                                        }
                                    });
                                });
                                ui.add_space(5.0);
                            });
                        }
                        if let Some(idx) = to_remove {
                            cache_tune_list.remove(idx);
                        }
                    }
                });

                // MemoryTune 配置
                ui.collapsing("内存带宽调优 (MemoryTune)", |ui| {
                    if tuning.memorytune.is_none() {
                        tuning.memorytune = Some(Vec::new());
                    }
                    if let Some(ref mut mem_tune_list) = tuning.memorytune {
                        ui.horizontal(|ui| {
                            if add_button(ui, "➕ 添加内存带宽调优", colors) {
                                mem_tune_list
                                    .push(MemorytuneConfig { vcpus: "0".to_string(), node: None });
                            }
                        });

                        let mut to_remove = None;
                        for (i, mem_tune) in mem_tune_list.iter_mut().enumerate() {
                            ui.push_id(i, |ui| {
                                inner_group(ui, colors, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("内存带宽调优 {}", i + 1));
                                        ui.label("vCPUs:");
                                        ui.text_edit_singleline(&mut mem_tune.vcpus);
                                        if delete_button(ui, None) {
                                            to_remove = Some(i);
                                        }
                                    });

                                    ui.add_space(5.0);

                                    // Node 配置
                                    if mem_tune.node.is_none() {
                                        mem_tune.node = Some(Vec::new());
                                    }
                                    if let Some(ref mut node_list) = mem_tune.node {
                                        ui.horizontal(|ui| {
                                            if add_button(ui, "➕ 添加节点", colors) {
                                                node_list.push(NodeConfig {
                                                    id: node_list.len() as u32,
                                                    bandwidth: 100,
                                                });
                                            }
                                        });

                                        let mut node_to_remove = None;
                                        for (j, node) in node_list.iter_mut().enumerate() {
                                            ui.horizontal(|ui| {
                                                ui.label(format!("节点 {}", j + 1));
                                                ui.label("ID:");
                                                ui.add(egui::DragValue::new(&mut node.id));
                                                ui.label("带宽:");
                                                ui.add(
                                                    egui::DragValue::new(&mut node.bandwidth)
                                                        .clamp_range(0..=100)
                                                        .suffix("%"),
                                                );
                                                if delete_button(ui, None) {
                                                    node_to_remove = Some(j);
                                                }
                                            });
                                        }
                                        if let Some(idx) = node_to_remove {
                                            node_list.remove(idx);
                                        }
                                    }
                                });
                                ui.add_space(5.0);
                            });
                        }
                        if let Some(idx) = to_remove {
                            mem_tune_list.remove(idx);
                        }
                    }
                });
            }
        });
    }
}
