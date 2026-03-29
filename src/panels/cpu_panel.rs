use egui::{Align, Layout, RichText};

use crate::{
    model::{CPUFeatureConfig, CPUModel, CPUTopology, VMConfig},
    panels::utils::*,
};

pub struct CPUPanel;

impl CPUPanel {
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "🖥", "CPU 配置");

        // 卡片宽度和间距配置
        let card_width = 380.0;
        let spacing = 8.0;

        // 使用流式布局实现卡片自动换行
        ui.with_layout(Layout::left_to_right(Align::TOP).with_main_wrap(true), |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);

            // CPU 拓扑结构卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "CPU 拓扑结构", Some("🔧"), colors, |ui| {
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
                            if ui.add(egui::Slider::new(&mut dies_val, 1..=4).text("个")).changed()
                            {
                                topology.dies = Some(dies_val);
                            }
                            ui.end_row();

                            ui.label("Clusters:");
                            let mut clusters_val = topology.clusters.unwrap_or(1);
                            if ui
                                .add(egui::Slider::new(&mut clusters_val, 1..=4).text("个"))
                                .changed()
                            {
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
                            RichText::new(format!("总 vCPU 数：{}", total_vcpus))
                                .strong()
                                .color(colors.info),
                        );

                        config.cpu.topology = Some(topology);
                    });
                },
            );

            // CPU 模式与匹配卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "CPU 模式与匹配", Some("⚙"), colors, |ui| {
                        grid(ui, "cpu_mode_grid", 2, |ui| {
                            // CPU 模式
                            ui.label("CPU 模式:");
                            let cpu_mode = config
                                .cpu
                                .mode
                                .get_or_insert_with(|| "host-passthrough".to_string());
                            egui::ComboBox::from_id_source("cpu_mode")
                                .selected_text(cpu_mode.as_str())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(cpu_mode, "custom".to_string(), "custom");
                                    ui.selectable_value(
                                        cpu_mode,
                                        "host-model".to_string(),
                                        "host-model",
                                    );
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
                            let cpu_match =
                                config.cpu.match_.get_or_insert_with(|| "exact".to_string());
                            egui::ComboBox::from_id_source("cpu_match")
                                .selected_text(cpu_match.as_str())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(cpu_match, "custom".to_string(), "custom");
                                    ui.selectable_value(cpu_match, "exact".to_string(), "exact");
                                    ui.selectable_value(cpu_match, "strict".to_string(), "strict");
                                    ui.selectable_value(
                                        cpu_match,
                                        "minimum".to_string(),
                                        "minimum",
                                    );
                                });
                            ui.end_row();

                            // migratable 属性
                            ui.label("可迁移性:");
                            let migratable =
                                config.cpu.migratable.get_or_insert_with(|| "off".to_string());
                            egui::ComboBox::from_id_source("cpu_migratable")
                                .selected_text(migratable.as_str())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(migratable, "on".to_string(), "on");
                                    ui.selectable_value(migratable, "off".to_string(), "off");
                                });
                            ui.end_row();
                        });
                    });
                },
            );

            // CPU 型号卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "CPU 型号", Some("🏷"), colors, |ui| {
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
                                let fallback =
                                    model.fallback.get_or_insert_with(|| "allow".to_string());
                                egui::ComboBox::from_id_source("cpu_fallback")
                                    .selected_text(fallback.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(fallback, "allow".to_string(), "allow");
                                        ui.selectable_value(
                                            fallback,
                                            "forbid".to_string(),
                                            "forbid",
                                        );
                                    });
                                ui.end_row();

                                ui.label("Vendor ID:");
                                let vendor_id =
                                    model.vendor_id.get_or_insert_with(|| "".to_string());
                                ui.text_edit_singleline(vendor_id);
                                ui.end_row();
                            });
                        }
                    });
                },
            );

            // CPU 厂商卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "CPU 厂商", Some("🏢"), colors, |ui| {
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
                                        ui.selectable_value(
                                            vendor,
                                            "AMD".to_string(),
                                            "AMD (EPYC)",
                                        );
                                    });
                                ui.end_row();
                            });
                        }
                    });
                },
            );

            // CPU 特性卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "CPU 特性", Some("⚡"), colors, |ui| {
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
                                        egui::ComboBox::from_id_source(format!(
                                            "cpu_feature_name_{}",
                                            i
                                        ))
                                        .selected_text(&feature.name)
                                        .width(150.0)
                                        .show_ui(
                                            ui,
                                            |ui| {
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
                                                    "svm",
                                                    "avic",
                                                    "npt",
                                                    "nrip",
                                                    "tsc_scale",
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
                                                    ui.selectable_value(
                                                        &mut feature.name,
                                                        f.to_string(),
                                                        f,
                                                    );
                                                }
                                            },
                                        );

                                        // 策略
                                        egui::ComboBox::from_id_source(format!(
                                            "cpu_feature_policy_{}",
                                            i
                                        ))
                                        .selected_text(&feature.policy)
                                        .width(100.0)
                                        .show_ui(
                                            ui,
                                            |ui| {
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
                                            },
                                        );

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
                },
            );

            // CPU Cache 卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "CPU Cache", Some("💾"), colors, |ui| {
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
                                            egui::ComboBox::from_id_source(format!(
                                                "cache_level_{}",
                                                i
                                            ))
                                            .selected_text(format!("L{}", level))
                                            .width(80.0)
                                            .show_ui(
                                                ui,
                                                |ui| {
                                                    if ui
                                                        .selectable_label(*level == 1, "L1")
                                                        .clicked()
                                                    {
                                                        *level = 1;
                                                    }
                                                    if ui
                                                        .selectable_label(*level == 2, "L2")
                                                        .clicked()
                                                    {
                                                        *level = 2;
                                                    }
                                                    if ui
                                                        .selectable_label(*level == 3, "L3")
                                                        .clicked()
                                                    {
                                                        *level = 3;
                                                    }
                                                },
                                            );

                                            // Mode
                                            let mode = cache
                                                .mode
                                                .get_or_insert_with(|| "emulate".to_string());
                                            egui::ComboBox::from_id_source(format!(
                                                "cache_mode_{}",
                                                i
                                            ))
                                            .selected_text(mode.as_str())
                                            .width(100.0)
                                            .show_ui(
                                                ui,
                                                |ui| {
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
                                                },
                                            );

                                            if delete_button(ui, None) {
                                                to_remove = Some(i);
                                            }
                                        });

                                        ui.add_space(5.0);

                                        // 关联度和策略
                                        grid(ui, format!("cache_assoc_grid_{}", i), 2, |ui| {
                                            ui.label("关联度:");
                                            let associativity = cache
                                                .associativity
                                                .get_or_insert_with(|| "".to_string());
                                            egui::ComboBox::from_id_source(format!(
                                                "cache_assoc_{}",
                                                i
                                            ))
                                            .selected_text(associativity.as_str())
                                            .show_ui(
                                                ui,
                                                |ui| {
                                                    ui.selectable_value(
                                                        associativity,
                                                        "".to_string(),
                                                        "默认",
                                                    );
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
                                                },
                                            );
                                            ui.end_row();

                                            ui.label("写策略:");
                                            let policy =
                                                cache.policy.get_or_insert_with(|| "".to_string());
                                            egui::ComboBox::from_id_source(format!(
                                                "cache_policy_{}",
                                                i
                                            ))
                                            .selected_text(policy.as_str())
                                            .show_ui(
                                                ui,
                                                |ui| {
                                                    ui.selectable_value(
                                                        policy,
                                                        "".to_string(),
                                                        "默认",
                                                    );
                                                    ui.selectable_value(
                                                        policy,
                                                        "none".to_string(),
                                                        "none",
                                                    );
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
                                                },
                                            );
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
                },
            );
        });
    }
}
