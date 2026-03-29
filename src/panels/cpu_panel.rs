use egui::RichText;

use crate::{
    model::{CPUFeatureConfig, CPUModel, CPUTopology, VMConfig},
    panels::utils::*,
};

pub struct CPUPanel;

impl CPUPanel {
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "🖥", "CPU 配置");

        card_group(ui, "CPU 拓扑结构", None, colors, |ui| {
            let mut topology = config.cpu.topology.take().unwrap_or(CPUTopology {
                sockets: 1,
                dies: Some(1),
                clusters: None,
                cores: 2,
                threads: 1,
            });

            grid(ui, "cpu_topology_grid", 2, |ui| {
                ui.label("Sockets:");
                ui.add(egui::Slider::new(&mut topology.sockets, 1..=8).text("个"));
                ui.end_row();

                ui.label("Dies:");
                let mut dies_val = topology.dies.unwrap_or(1);
                if ui.add(egui::Slider::new(&mut dies_val, 1..=4).text("个")).changed() {
                    topology.dies = Some(dies_val);
                }
                ui.end_row();

                ui.label("Clusters:");
                let mut clusters_val = topology.clusters.unwrap_or(1);
                if ui.add(egui::Slider::new(&mut clusters_val, 1..=4).text("个")).changed() {
                    topology.clusters = Some(clusters_val);
                }
                ui.end_row();

                ui.label("Cores:");
                ui.add(egui::Slider::new(&mut topology.cores, 1..=64).text("核"));
                ui.end_row();

                ui.label("Threads:");
                ui.add(egui::Slider::new(&mut topology.threads, 1..=8).text("线程"));
                ui.end_row();
            });

            let total_vcpus = topology.sockets
                * topology.dies.unwrap_or(1)
                * topology.clusters.unwrap_or(1)
                * topology.cores
                * topology.threads;
            ui.label(
                RichText::new(format!("总 vCPU 数：{}", total_vcpus)).strong().color(colors.info),
            );

            config.cpu.topology = Some(topology);
        });

        ui.add_space(8.0);

        card_group(ui, "CPU 模式与匹配", None, colors, |ui| {
            grid(ui, "cpu_mode_grid", 2, |ui| {
                // CPU 模式
                ui.label("CPU 模式:");
                let cpu_mode =
                    config.cpu.mode.get_or_insert_with(|| "host-passthrough".to_string());
                egui::ComboBox::from_id_source("cpu_mode")
                    .selected_text(cpu_mode.as_str())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(cpu_mode, "custom".to_string(), "custom");
                        ui.selectable_value(cpu_mode, "host-model".to_string(), "host-model");
                        ui.selectable_value(
                            cpu_mode,
                            "host-passthrough".to_string(),
                            "host-passthrough",
                        );
                        ui.selectable_value(cpu_mode, "maximum".to_string(), "maximum");
                    });
                ui.end_row();

                // match 属性
                ui.label("匹配方式:");
                let cpu_match = config.cpu.match_.get_or_insert_with(|| "exact".to_string());
                egui::ComboBox::from_id_source("cpu_match")
                    .selected_text(cpu_match.as_str())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(cpu_match, "custom".to_string(), "custom");
                        ui.selectable_value(cpu_match, "exact".to_string(), "exact");
                        ui.selectable_value(cpu_match, "strict".to_string(), "strict");
                        ui.selectable_value(cpu_match, "minimum".to_string(), "minimum");
                    });
                ui.end_row();

                // migratable 属性
                ui.label("可迁移性:");
                let migratable = config.cpu.migratable.get_or_insert_with(|| "off".to_string());
                egui::ComboBox::from_id_source("cpu_migratable")
                    .selected_text(migratable.as_str())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(migratable, "on".to_string(), "on");
                        ui.selectable_value(migratable, "off".to_string(), "off");
                    });
                ui.end_row();
            });
        });

        ui.add_space(8.0);

        card_group(ui, "CPU 型号", None, colors, |ui| {
            let mut has_model = config.cpu.model.is_some();
            if checkbox(ui, &mut has_model, "指定 CPU 型号") {
                if has_model {
                    config.cpu.model = Some(CPUModel {
                        fallback: Some("allow".to_string()),
                        vendor_id: None,
                        name: "qemu64".to_string(),
                    });
                } else {
                    config.cpu.model = None;
                }
            }

            if let Some(ref mut model) = config.cpu.model {
                ui.add_space(5.0);
                grid(ui, "cpu_model_grid", 2, |ui| {
                    ui.label("型号名称:");
                    egui::ComboBox::from_id_source("cpu_model_name")
                        .selected_text(&model.name)
                        .show_ui(ui, |ui| {
                            let models = [
                                "qemu64",
                                "qemu32",
                                "host",
                                "kvm64",
                                "kvm32",
                                "Skylake-Client",
                                "Skylake-Server",
                                "Cascadelake-Server",
                                "Icelake-Client",
                                "Icelake-Server",
                                "Broadwell",
                                "Haswell",
                                "IvyBridge",
                                "SandyBridge",
                                "Westmere",
                                "Nehalem",
                            ];
                            for m in models {
                                ui.selectable_value(&mut model.name, m.to_string(), m);
                            }
                        });
                    ui.end_row();

                    ui.label("回退策略:");
                    let fallback = model.fallback.get_or_insert_with(|| "allow".to_string());
                    egui::ComboBox::from_id_source("cpu_fallback")
                        .selected_text(fallback.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(fallback, "allow".to_string(), "allow");
                            ui.selectable_value(fallback, "forbid".to_string(), "forbid");
                        });
                    ui.end_row();

                    ui.label("Vendor ID:");
                    let vendor_id = model.vendor_id.get_or_insert_with(|| "".to_string());
                    ui.text_edit_singleline(vendor_id);
                    ui.end_row();
                });
            }
        });

        ui.add_space(8.0);

        card_group(ui, "CPU 厂商", None, colors, |ui| {
            let mut has_vendor = config.cpu.vendor.is_some();
            if checkbox(ui, &mut has_vendor, "指定 CPU 厂商") {
                if has_vendor {
                    config.cpu.vendor = Some("Intel".to_string());
                } else {
                    config.cpu.vendor = None;
                }
            }

            if let Some(ref mut vendor) = config.cpu.vendor {
                ui.add_space(5.0);
                grid(ui, "cpu_vendor_grid", 2, |ui| {
                    ui.label("厂商名称:");
                    egui::ComboBox::from_id_source("cpu_vendor")
                        .selected_text(vendor.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(vendor, "Intel".to_string(), "Intel");
                            ui.selectable_value(vendor, "AMD".to_string(), "AMD");
                            ui.selectable_value(vendor, "IBM".to_string(), "IBM");
                            ui.selectable_value(vendor, "Cavium".to_string(), "Cavium");
                            ui.selectable_value(vendor, "AMD".to_string(), "AMD (EPYC)");
                        });
                    ui.end_row();
                });
            }
        });

        ui.add_space(8.0);

        card_group(ui, "CPU 特性", None, colors, |ui| {
            if config.cpu.feature.is_none() {
                config.cpu.feature = Some(Vec::new());
            }

            if let Some(ref mut feature_list) = config.cpu.feature {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加特性", colors) {
                        feature_list.push(CPUFeatureConfig {
                            policy: "require".to_string(),
                            name: "apic".to_string(),
                        });
                    }
                });

                let mut to_remove = None;
                for (i, feature) in feature_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}. ", i + 1));

                            // 特性名称
                            egui::ComboBox::from_id_source(format!("cpu_feature_name_{}", i))
                                .selected_text(&feature.name)
                                .width(150.0)
                                .show_ui(ui, |ui| {
                                    let features = [
                                        "apic",
                                        "pae",
                                        "pse",
                                        "pse36",
                                        "msr",
                                        "mtrr",
                                        "mmx",
                                        "fxsr",
                                        "sse",
                                        "sse2",
                                        "sse3",
                                        "ssse3",
                                        "sse4_1",
                                        "sse4_2",
                                        "x2apic",
                                        "popcnt",
                                        "aes",
                                        "xsave",
                                        "xsaveopt",
                                        "avx",
                                        "avx2",
                                        "f16c",
                                        "fma",
                                        "bmi1",
                                        "bmi2",
                                        "rdseed",
                                        "adx",
                                        "smap",
                                        "smep",
                                        "invpcid",
                                        "rdtscp",
                                        "la57",
                                        "pdpe1gb",
                                        // AMD 特性
                                        "svm",
                                        "avic",
                                        "npt",
                                        "nrip",
                                        "tsc_scale",
                                        // 其他
                                        "vmx",
                                        "ept",
                                        "vpid",
                                        "vme",
                                        "de",
                                        "psn",
                                        "ds",
                                        "acpi",
                                        "ss",
                                        "ht",
                                        "tm",
                                        "ia64",
                                        "cdt",
                                    ];
                                    for f in features {
                                        ui.selectable_value(&mut feature.name, f.to_string(), f);
                                    }
                                });

                            // 策略
                            egui::ComboBox::from_id_source(format!("cpu_feature_policy_{}", i))
                                .selected_text(&feature.policy)
                                .width(100.0)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut feature.policy,
                                        "force".to_string(),
                                        "force",
                                    );
                                    ui.selectable_value(
                                        &mut feature.policy,
                                        "require".to_string(),
                                        "require",
                                    );
                                    ui.selectable_value(
                                        &mut feature.policy,
                                        "optional".to_string(),
                                        "optional",
                                    );
                                    ui.selectable_value(
                                        &mut feature.policy,
                                        "disable".to_string(),
                                        "disable",
                                    );
                                    ui.selectable_value(
                                        &mut feature.policy,
                                        "forbid".to_string(),
                                        "forbid",
                                    );
                                });

                            if delete_button(ui, None) {
                                to_remove = Some(i);
                            }
                        });
                    });
                }

                if let Some(idx) = to_remove {
                    feature_list.remove(idx);
                }
            }
        });

        ui.add_space(8.0);

        card_group(ui, "CPU Cache", None, colors, |ui| {
            if config.cpu.cache.is_none() {
                config.cpu.cache = Some(Vec::new());
            }

            if let Some(ref mut cache_list) = config.cpu.cache {
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

                let mut to_remove = None;
                for (i, cache) in cache_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        inner_group(ui, colors, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}. ", i + 1));

                                // Cache Level
                                let level = cache.level.get_or_insert(3);
                                egui::ComboBox::from_id_source(format!("cache_level_{}", i))
                                    .selected_text(format!("L{}", level))
                                    .width(80.0)
                                    .show_ui(ui, |ui| {
                                        if ui.selectable_label(*level == 1, "L1").clicked() {
                                            *level = 1;
                                        }
                                        if ui.selectable_label(*level == 2, "L2").clicked() {
                                            *level = 2;
                                        }
                                        if ui.selectable_label(*level == 3, "L3").clicked() {
                                            *level = 3;
                                        }
                                    });

                                // Mode
                                let mode = cache.mode.get_or_insert_with(|| "emulate".to_string());
                                egui::ComboBox::from_id_source(format!("cache_mode_{}", i))
                                    .selected_text(mode.as_str())
                                    .width(100.0)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(mode, "emulate".to_string(), "emulate");
                                        ui.selectable_value(
                                            mode,
                                            "passthrough".to_string(),
                                            "passthrough",
                                        );
                                        ui.selectable_value(mode, "disable".to_string(), "disable");
                                    });

                                if delete_button(ui, None) {
                                    to_remove = Some(i);
                                }
                            });

                            ui.add_space(5.0);

                            // 关联度和策略
                            grid(ui, format!("cache_assoc_grid_{}", i), 2, |ui| {
                                ui.label("关联度:");
                                let associativity =
                                    cache.associativity.get_or_insert_with(|| "".to_string());
                                egui::ComboBox::from_id_source(format!("cache_assoc_{}", i))
                                    .selected_text(associativity.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(associativity, "".to_string(), "默认");
                                        ui.selectable_value(
                                            associativity,
                                            "none".to_string(),
                                            "none",
                                        );
                                        ui.selectable_value(
                                            associativity,
                                            "direct".to_string(),
                                            "direct",
                                        );
                                        ui.selectable_value(
                                            associativity,
                                            "full".to_string(),
                                            "full",
                                        );
                                    });
                                ui.end_row();

                                ui.label("写策略:");
                                let policy = cache.policy.get_or_insert_with(|| "".to_string());
                                egui::ComboBox::from_id_source(format!("cache_policy_{}", i))
                                    .selected_text(policy.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(policy, "".to_string(), "默认");
                                        ui.selectable_value(policy, "none".to_string(), "none");
                                        ui.selectable_value(
                                            policy,
                                            "writeback".to_string(),
                                            "writeback",
                                        );
                                        ui.selectable_value(
                                            policy,
                                            "writethrough".to_string(),
                                            "writethrough",
                                        );
                                    });
                                ui.end_row();
                            });

                            ui.add_space(5.0);

                            // 缓存大小和行大小
                            grid(ui, format!("cache_size_grid_{}", i), 2, |ui| {
                                ui.label("缓存大小:");
                                ui.horizontal(|ui| {
                                    let size_val =
                                        if let Some(ref mut s) = cache.size { s.value } else { 0 };
                                    let mut size_value = size_val;
                                    if ui
                                        .add(egui::DragValue::new(&mut size_value).prefix("大小: "))
                                        .changed()
                                    {
                                        if size_value > 0 {
                                            cache.size =
                                                Some(crate::model::cpu_tuning::CacheSizeConfig {
                                                    unit: Some("KiB".to_string()),
                                                    value: size_value,
                                                });
                                        } else {
                                            cache.size = None;
                                        }
                                    }
                                    if let Some(ref mut s) = cache.size {
                                        egui::ComboBox::from_id_source(format!(
                                            "cache_size_unit_{}",
                                            i
                                        ))
                                        .selected_text(s.unit.as_deref().unwrap_or("KiB"))
                                        .show_ui(
                                            ui,
                                            |ui| {
                                                if ui
                                                    .selectable_label(
                                                        s.unit.as_deref().unwrap_or("KiB") == "KiB",
                                                        "KiB",
                                                    )
                                                    .changed()
                                                {
                                                    s.unit = Some("KiB".to_string());
                                                }
                                                if ui
                                                    .selectable_label(
                                                        s.unit.as_deref().unwrap_or("KiB") == "MiB",
                                                        "MiB",
                                                    )
                                                    .changed()
                                                {
                                                    s.unit = Some("MiB".to_string());
                                                }
                                                if ui
                                                    .selectable_label(
                                                        s.unit.as_deref().unwrap_or("KiB") == "GiB",
                                                        "GiB",
                                                    )
                                                    .changed()
                                                {
                                                    s.unit = Some("GiB".to_string());
                                                }
                                            },
                                        );
                                    }
                                });
                                ui.end_row();

                                ui.label("缓存行大小:");
                                ui.horizontal(|ui| {
                                    let line_val =
                                        if let Some(ref mut l) = cache.line { l.value } else { 0 };
                                    let mut line_value = line_val;
                                    if ui
                                        .add(
                                            egui::DragValue::new(&mut line_value)
                                                .prefix("行大小: "),
                                        )
                                        .changed()
                                    {
                                        if line_value > 0 {
                                            cache.line =
                                                Some(crate::model::cpu_tuning::CacheLineConfig {
                                                    unit: Some("B".to_string()),
                                                    value: line_value,
                                                });
                                        } else {
                                            cache.line = None;
                                        }
                                    }
                                    if let Some(ref mut l) = cache.line {
                                        egui::ComboBox::from_id_source(format!(
                                            "cache_line_unit_{}",
                                            i
                                        ))
                                        .selected_text(l.unit.as_deref().unwrap_or("B"))
                                        .show_ui(
                                            ui,
                                            |ui| {
                                                if ui
                                                    .selectable_label(
                                                        l.unit.as_deref().unwrap_or("B") == "B",
                                                        "B",
                                                    )
                                                    .changed()
                                                {
                                                    l.unit = Some("B".to_string());
                                                }
                                                if ui
                                                    .selectable_label(
                                                        l.unit.as_deref().unwrap_or("B") == "KiB",
                                                        "KiB",
                                                    )
                                                    .changed()
                                                {
                                                    l.unit = Some("KiB".to_string());
                                                }
                                            },
                                        );
                                    }
                                });
                                ui.end_row();
                            });
                        });
                        ui.add_space(5.0);
                    });
                }

                if let Some(idx) = to_remove {
                    cache_list.remove(idx);
                }
            }
        });

        ui.add_space(8.0);

        card_group(ui, "最大物理地址位数", None, colors, |ui| {
            let mut has_maxphysaddr = config.cpu.maxphysaddr.is_some();
            if checkbox(ui, &mut has_maxphysaddr, "启用最大物理地址限制") {
                if has_maxphysaddr {
                    config.cpu.maxphysaddr = Some(crate::model::CPUMaxPhysAddr {
                        mode: "emulate".to_string(),
                        bits: Some(42),
                        limit: None,
                    });
                } else {
                    config.cpu.maxphysaddr = None;
                }
            }

            if let Some(ref mut maxphysaddr) = config.cpu.maxphysaddr {
                ui.add_space(5.0);
                grid(ui, "maxphysaddr_grid", 2, |ui| {
                    ui.label("模式:");
                    egui::ComboBox::from_id_source("maxphysaddr_mode")
                        .selected_text(&maxphysaddr.mode)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut maxphysaddr.mode,
                                "emulate".to_string(),
                                "emulate",
                            );
                            ui.selectable_value(
                                &mut maxphysaddr.mode,
                                "passthrough".to_string(),
                                "passthrough",
                            );
                        });
                    ui.end_row();

                    ui.label("地址位数:");
                    let bits = maxphysaddr.bits.get_or_insert(42);
                    ui.add(egui::Slider::new(bits, 32..=52).text("bits"));
                    ui.end_row();

                    ui.label("限制位数:");
                    let limit = maxphysaddr.limit.get_or_insert(39);
                    ui.add(egui::Slider::new(limit, 32..=52).text("bits"));
                    ui.end_row();
                });
            }
        });

        ui.add_space(8.0);

        card_group(ui, "CPU NUMA 配置", None, colors, |ui| {
            let mut has_numa = config.cpu.numa.is_some();
            if checkbox(ui, &mut has_numa, "启用 CPU NUMA 配置") {
                if has_numa {
                    config.cpu.numa = Some(crate::model::CPUNUMAConfig {
                        cell: Some(vec![crate::model::CPUNUMACell {
                            id: 0,
                            cpus: "0-3".to_string(),
                            memory: 4194304,
                            unit: Some("KiB".to_string()),
                            mem_access: None,
                            discard: None,
                            distances: None,
                        }]),
                        interconnects: None,
                    });
                } else {
                    config.cpu.numa = None;
                }
            }

            if let Some(ref mut numa) = config.cpu.numa {
                ui.add_space(5.0);

                // Cell 配置
                ui.label("NUMA Cell:");
                if numa.cell.is_none() {
                    numa.cell = Some(Vec::new());
                }
                if let Some(ref mut cell_list) = numa.cell {
                    ui.horizontal(|ui| {
                        if add_button(ui, "➕ 添加 Cell", colors) {
                            cell_list.push(crate::model::CPUNUMACell {
                                id: cell_list.len() as u32,
                                cpus: "".to_string(),
                                memory: 0,
                                unit: Some("KiB".to_string()),
                                mem_access: None,
                                discard: None,
                                distances: None,
                            });
                        }
                    });

                    let mut to_remove = None;
                    for (i, cell) in cell_list.iter_mut().enumerate() {
                        ui.push_id(i, |ui| {
                            egui::Frame::group(ui.style())
                                .inner_margin(egui::Margin::same(8.0))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("Cell {}", i));
                                        if delete_button(ui, None) {
                                            to_remove = Some(i);
                                        }
                                    });

                                    grid(ui, format!("cell_grid_{}", i), 2, |ui| {
                                        ui.label("ID:");
                                        ui.add(egui::Slider::new(&mut cell.id, 0..=255));
                                        ui.end_row();

                                        ui.label("CPUs:");
                                        ui.text_edit_singleline(&mut cell.cpus);
                                        ui.end_row();

                                        ui.label("内存:");
                                        ui.add(
                                            egui::Slider::new(&mut cell.memory, 0..=67108864)
                                                .text("KiB"),
                                        );
                                        ui.end_row();

                                        ui.label("单位:");
                                        let unit =
                                            cell.unit.get_or_insert_with(|| "KiB".to_string());
                                        egui::ComboBox::from_id_source(format!("cell_unit_{}", i))
                                            .selected_text(unit.as_str())
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(unit, "KiB".to_string(), "KiB");
                                                ui.selectable_value(unit, "MiB".to_string(), "MiB");
                                                ui.selectable_value(unit, "GiB".to_string(), "GiB");
                                            });
                                        ui.end_row();

                                        ui.label("内存访问:");
                                        let mem_access =
                                            cell.mem_access.get_or_insert_with(|| "".to_string());
                                        egui::ComboBox::from_id_source(format!(
                                            "cell_mem_access_{}",
                                            i
                                        ))
                                        .selected_text(mem_access.as_str())
                                        .show_ui(
                                            ui,
                                            |ui| {
                                                ui.selectable_value(
                                                    mem_access,
                                                    "".to_string(),
                                                    "无",
                                                );
                                                ui.selectable_value(
                                                    mem_access,
                                                    "local".to_string(),
                                                    "local",
                                                );
                                                ui.selectable_value(
                                                    mem_access,
                                                    "remote".to_string(),
                                                    "remote",
                                                );
                                            },
                                        );
                                        ui.end_row();

                                        ui.label("Discard:");
                                        let discard =
                                            cell.discard.get_or_insert_with(|| "".to_string());
                                        egui::ComboBox::from_id_source(format!(
                                            "cell_discard_{}",
                                            i
                                        ))
                                        .selected_text(discard.as_str())
                                        .show_ui(
                                            ui,
                                            |ui| {
                                                ui.selectable_value(discard, "".to_string(), "无");
                                                ui.selectable_value(
                                                    discard,
                                                    "enable".to_string(),
                                                    "enable",
                                                );
                                                ui.selectable_value(
                                                    discard,
                                                    "disable".to_string(),
                                                    "disable",
                                                );
                                            },
                                        );
                                        ui.end_row();
                                    });

                                    ui.add_space(5.0);

                                    // Distances 配置
                                    ui.collapsing("NUMA 距离 (Distances)", |ui| {
                                        if cell.distances.is_none() {
                                            cell.distances = Some(crate::model::CPUNUMADistances {
                                                sibling: None,
                                            });
                                        }
                                        if let Some(ref mut distances) = cell.distances {
                                            if distances.sibling.is_none() {
                                                distances.sibling = Some(Vec::new());
                                            }
                                            if let Some(ref mut sibling_list) = distances.sibling {
                                                ui.horizontal(|ui| {
                                                    if add_button(ui, "➕ 添加距离", colors) {
                                                        sibling_list.push(
                                                            crate::model::CPUNUMASibling {
                                                                id: sibling_list.len() as u32,
                                                                value: 20,
                                                            },
                                                        );
                                                    }
                                                });

                                                let mut to_remove = None;
                                                for (j, sibling) in
                                                    sibling_list.iter_mut().enumerate()
                                                {
                                                    ui.horizontal(|ui| {
                                                        ui.label(format!("距离 {}:", j + 1));
                                                        ui.add(
                                                            egui::Slider::new(
                                                                &mut sibling.id,
                                                                0..=255,
                                                            )
                                                            .text("target cell id"),
                                                        );
                                                        ui.add(
                                                            egui::Slider::new(
                                                                &mut sibling.value,
                                                                10..=40,
                                                            )
                                                            .text("value"),
                                                        );
                                                        if delete_button(ui, None) {
                                                            to_remove = Some(j);
                                                        }
                                                    });
                                                }
                                                if let Some(idx) = to_remove {
                                                    sibling_list.remove(idx);
                                                }
                                            }
                                        }
                                    });
                                });
                            ui.add_space(5.0);
                        });
                    }
                    if let Some(idx) = to_remove {
                        cell_list.remove(idx);
                    }
                }

                ui.add_space(5.0);

                // Interconnects 配置
                ui.collapsing("互联配置 (Interconnects)", |ui| {
                    if numa.interconnects.is_none() {
                        numa.interconnects = Some(crate::model::CPUNUMAInterconnects {
                            latency: None,
                            bandwidth: None,
                        });
                    }
                    if let Some(ref mut interconnects) = numa.interconnects {
                        ui.collapsing("Latency", |ui| {
                            if interconnects.latency.is_none() {
                                interconnects.latency = Some(Vec::new());
                            }
                            if let Some(ref mut latency_list) = interconnects.latency {
                                ui.horizontal(|ui| {
                                    if add_button(ui, "➕ 添加 Latency", colors) {
                                        latency_list.push(crate::model::CPUNUMALatency {
                                            initiator: 0,
                                            target: 0,
                                            type_: "latency".to_string(),
                                            value: 0,
                                            cache: None,
                                        });
                                    }
                                });

                                let mut to_remove = None;
                                for (i, latency) in latency_list.iter_mut().enumerate() {
                                    ui.push_id(i, |ui| {
                                        inner_group(ui, colors, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(format!("Latency {}:", i + 1));
                                                if delete_button(ui, None) {
                                                    to_remove = Some(i);
                                                }
                                            });

                                            grid(ui, format!("latency_grid_{}", i), 3, |ui| {
                                                ui.label("Initiator:");
                                                ui.add(
                                                    egui::Slider::new(
                                                        &mut latency.initiator,
                                                        0..=255,
                                                    )
                                                    .text("node"),
                                                );
                                                ui.end_row();

                                                ui.label("Target:");
                                                ui.add(
                                                    egui::Slider::new(&mut latency.target, 0..=255)
                                                        .text("node"),
                                                );
                                                ui.end_row();

                                                ui.label("Value:");
                                                ui.add(
                                                    egui::Slider::new(
                                                        &mut latency.value,
                                                        0..=65535,
                                                    )
                                                    .text("ns"),
                                                );
                                                ui.end_row();

                                                ui.label("类型:");
                                                egui::ComboBox::from_id_source(format!(
                                                    "latency_type_{}",
                                                    i
                                                ))
                                                .selected_text(&latency.type_)
                                                .show_ui(ui, |ui| {
                                                    ui.selectable_value(
                                                        &mut latency.type_,
                                                        "access".to_string(),
                                                        "access",
                                                    );
                                                    ui.selectable_value(
                                                        &mut latency.type_,
                                                        "read".to_string(),
                                                        "read",
                                                    );
                                                    ui.selectable_value(
                                                        &mut latency.type_,
                                                        "write".to_string(),
                                                        "write",
                                                    );
                                                });
                                                ui.end_row();

                                                ui.label("Cache:");
                                                let cache_val = latency.cache.get_or_insert(0);
                                                ui.add(
                                                    egui::Slider::new(cache_val, 0..=3)
                                                        .text("level"),
                                                );
                                                ui.end_row();
                                            });
                                        });
                                        ui.add_space(5.0);
                                    });
                                }
                                if let Some(idx) = to_remove {
                                    latency_list.remove(idx);
                                }
                            }
                        });

                        ui.add_space(5.0);

                        ui.collapsing("Bandwidth", |ui| {
                            if interconnects.bandwidth.is_none() {
                                interconnects.bandwidth = Some(Vec::new());
                            }
                            if let Some(ref mut bandwidth_list) = interconnects.bandwidth {
                                ui.horizontal(|ui| {
                                    if add_button(ui, "➕ 添加 Bandwidth", colors) {
                                        bandwidth_list.push(crate::model::CPUNUMABandwidth {
                                            initiator: 0,
                                            target: 0,
                                            type_: "bandwidth".to_string(),
                                            value: 0,
                                            unit: Some("KiB/s".to_string()),
                                        });
                                    }
                                });

                                let mut to_remove = None;
                                for (i, bandwidth) in bandwidth_list.iter_mut().enumerate() {
                                    ui.push_id(i, |ui| {
                                        inner_group(ui, colors, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(format!("Bandwidth {}:", i + 1));
                                                if delete_button(ui, None) {
                                                    to_remove = Some(i);
                                                }
                                            });

                                            grid(ui, format!("bandwidth_grid_{}", i), 3, |ui| {
                                                ui.label("Initiator:");
                                                ui.add(
                                                    egui::Slider::new(
                                                        &mut bandwidth.initiator,
                                                        0..=255,
                                                    )
                                                    .text("node"),
                                                );
                                                ui.end_row();

                                                ui.label("Target:");
                                                ui.add(
                                                    egui::Slider::new(
                                                        &mut bandwidth.target,
                                                        0..=255,
                                                    )
                                                    .text("node"),
                                                );
                                                ui.end_row();

                                                ui.label("Value:");
                                                ui.add(
                                                    egui::Slider::new(
                                                        &mut bandwidth.value,
                                                        0..=65535,
                                                    )
                                                    .text("value"),
                                                );
                                                ui.end_row();

                                                ui.label("类型:");
                                                egui::ComboBox::from_id_source(format!(
                                                    "bandwidth_type_{}",
                                                    i
                                                ))
                                                .selected_text(&bandwidth.type_)
                                                .show_ui(ui, |ui| {
                                                    ui.selectable_value(
                                                        &mut bandwidth.type_,
                                                        "access".to_string(),
                                                        "access",
                                                    );
                                                    ui.selectable_value(
                                                        &mut bandwidth.type_,
                                                        "read".to_string(),
                                                        "read",
                                                    );
                                                    ui.selectable_value(
                                                        &mut bandwidth.type_,
                                                        "write".to_string(),
                                                        "write",
                                                    );
                                                });
                                                ui.end_row();

                                                ui.label("单位:");
                                                let unit = bandwidth
                                                    .unit
                                                    .get_or_insert_with(|| "KiB/s".to_string());
                                                egui::ComboBox::from_id_source(format!(
                                                    "bandwidth_unit_{}",
                                                    i
                                                ))
                                                .selected_text(unit.as_str())
                                                .show_ui(ui, |ui| {
                                                    ui.selectable_value(
                                                        unit,
                                                        "KiB/s".to_string(),
                                                        "KiB/s",
                                                    );
                                                    ui.selectable_value(
                                                        unit,
                                                        "MiB/s".to_string(),
                                                        "MiB/s",
                                                    );
                                                    ui.selectable_value(
                                                        unit,
                                                        "GiB/s".to_string(),
                                                        "GiB/s",
                                                    );
                                                });
                                                ui.end_row();
                                            });
                                        });
                                        ui.add_space(5.0);
                                    });
                                }
                                if let Some(idx) = to_remove {
                                    bandwidth_list.remove(idx);
                                }
                            }
                        });
                    }
                });
            }
        });

        ui.add_space(8.0);

        card_group(ui, "废弃特性配置 (Deprecated Features)", None, colors, |ui| {
            ui.label(
                RichText::new("仅适用于 S390 架构 (Since 11.0.0)")
                    .small()
                    .color(colors.text_secondary),
            );

            let mut has_deprecated = config.cpu.deprecated_features.is_some();
            if checkbox(ui, &mut has_deprecated, "启用废弃特性配置") {
                if has_deprecated {
                    config.cpu.deprecated_features = Some(crate::model::DeprecatedFeaturesConfig {
                        state: "off".to_string(),
                        feature: None,
                    });
                } else {
                    config.cpu.deprecated_features = None;
                }
            }

            if let Some(ref mut deprecated) = config.cpu.deprecated_features {
                ui.add_space(5.0);
                grid(ui, "deprecated_features_grid", 2, |ui| {
                    ui.label("状态:");
                    egui::ComboBox::from_id_source("deprecated_features_state")
                        .selected_text(&deprecated.state)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut deprecated.state,
                                "on".to_string(),
                                "on (启用)",
                            );
                            ui.selectable_value(
                                &mut deprecated.state,
                                "off".to_string(),
                                "off (禁用)",
                            );
                        });
                    ui.end_row();
                });

                ui.add_space(5.0);

                // 特性列表
                if deprecated.feature.is_none() {
                    deprecated.feature = Some(Vec::new());
                }
                if let Some(ref mut feature_list) = deprecated.feature {
                    ui.horizontal(|ui| {
                        if add_button(ui, "➕ 添加特性", colors) {
                            feature_list.push(CPUFeatureConfig {
                                policy: "require".to_string(),
                                name: "".to_string(),
                            });
                        }
                    });

                    let mut to_remove = None;
                    for (i, feature) in feature_list.iter_mut().enumerate() {
                        ui.push_id(i, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}. ", i + 1));

                                // 特性名称
                                ui.text_edit_singleline(&mut feature.name);

                                // 策略
                                egui::ComboBox::from_id_source(format!(
                                    "deprecated_feature_policy_{}",
                                    i
                                ))
                                .selected_text(&feature.policy)
                                .width(100.0)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut feature.policy,
                                        "force".to_string(),
                                        "force",
                                    );
                                    ui.selectable_value(
                                        &mut feature.policy,
                                        "require".to_string(),
                                        "require",
                                    );
                                    ui.selectable_value(
                                        &mut feature.policy,
                                        "optional".to_string(),
                                        "optional",
                                    );
                                    ui.selectable_value(
                                        &mut feature.policy,
                                        "disable".to_string(),
                                        "disable",
                                    );
                                    ui.selectable_value(
                                        &mut feature.policy,
                                        "forbid".to_string(),
                                        "forbid",
                                    );
                                });

                                if delete_button(ui, None) {
                                    to_remove = Some(i);
                                }
                            });
                        });
                    }

                    if let Some(idx) = to_remove {
                        feature_list.remove(idx);
                    }
                }
            }
        });
    }
}
