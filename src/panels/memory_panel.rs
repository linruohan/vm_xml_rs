use egui::{Align, Layout};

use crate::{
    model::{HugepagesConfig, MemoryBackingConfig, MemoryInfo, VMConfig},
    panels::utils::*,
};

pub struct MemoryPanel;

impl MemoryPanel {
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "💾", "内存配置");

        // 卡片宽度和间距配置
        let card_width = 380.0;
        let spacing = 8.0;

        // 使用流式布局实现卡片自动换行
        ui.with_layout(Layout::left_to_right(Align::TOP).with_main_wrap(true), |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);

            // 当前内存设置卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "当前内存设置", Some("📊"), colors, |ui| {
                        grid(ui, "memory_current_grid", 2, |ui| {
                            ui.label("内存大小:");
                            ui.add(
                                egui::Slider::new(&mut config.general.memory.value, 1..=128)
                                    .text("单位"),
                            );
                            ui.end_row();

                            ui.label("内存单位:");
                            let unit =
                                config.general.memory.unit.get_or_insert_with(|| "MiB".to_string());
                            egui::ComboBox::from_id_source("memory_unit_panel")
                                .selected_text(unit.as_str())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(unit, "KiB".to_string(), "KiB");
                                    ui.selectable_value(unit, "MiB".to_string(), "MiB");
                                    ui.selectable_value(unit, "GiB".to_string(), "GiB");
                                    ui.selectable_value(unit, "TiB".to_string(), "TiB");
                                });
                            ui.end_row();
                        });
                    });
                },
            );

            // 最大内存卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "最大内存 (热插拔)", Some("📈"), colors, |ui| {
                        let mut has_max_memory = config.general.max_memory.is_some();
                        if checkbox(ui, &mut has_max_memory, "启用最大内存限制") {
                            if has_max_memory {
                                config.general.max_memory = Some(MemoryInfo {
                                    unit: Some("GiB".to_string()),
                                    slots: Some(16),
                                    dump_core: None,
                                    value: 64,
                                });
                            } else {
                                config.general.max_memory = None;
                            }
                        }

                        if let Some(ref mut max_mem) = config.general.max_memory {
                            ui.add_space(5.0);
                            grid(ui, "max_memory_grid", 2, |ui| {
                                ui.label("最大内存:");
                                ui.add(egui::Slider::new(&mut max_mem.value, 1..=256).text("GiB"));
                                ui.end_row();

                                ui.label("内存插槽数:");
                                let slots = max_mem.slots.get_or_insert(16);
                                ui.add(egui::Slider::new(slots, 1..=32).text("个"));
                                ui.end_row();

                                ui.label("单位:");
                                let unit = max_mem.unit.get_or_insert_with(|| "GiB".to_string());
                                egui::ComboBox::from_id_source("max_memory_unit")
                                    .selected_text(unit.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(unit, "KiB".to_string(), "KiB");
                                        ui.selectable_value(unit, "MiB".to_string(), "MiB");
                                        ui.selectable_value(unit, "GiB".to_string(), "GiB");
                                        ui.selectable_value(unit, "TiB".to_string(), "TiB");
                                    });
                                ui.end_row();
                            });
                        }
                    });
                },
            );

            // 当前内存 (动态气球) 卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "当前内存 (动态气球)", Some("🎈"), colors, |ui| {
                        let mut has_current_memory = config.general.current_memory.is_some();
                        if checkbox(ui, &mut has_current_memory, "启用当前内存配置") {
                            if has_current_memory {
                                config.general.current_memory = Some(MemoryInfo {
                                    unit: Some("GiB".to_string()),
                                    slots: None,
                                    dump_core: None,
                                    value: config.general.memory.value,
                                });
                            } else {
                                config.general.current_memory = None;
                            }
                        }

                        if let Some(ref mut current_mem) = config.general.current_memory {
                            ui.add_space(5.0);
                            grid(ui, "current_memory_grid", 2, |ui| {
                                ui.label("当前内存:");
                                ui.add(
                                    egui::Slider::new(&mut current_mem.value, 1..=128).text("GiB"),
                                );
                                ui.end_row();

                                ui.label("单位:");
                                let unit =
                                    current_mem.unit.get_or_insert_with(|| "GiB".to_string());
                                egui::ComboBox::from_id_source("current_memory_unit")
                                    .selected_text(unit.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(unit, "KiB".to_string(), "KiB");
                                        ui.selectable_value(unit, "MiB".to_string(), "MiB");
                                        ui.selectable_value(unit, "GiB".to_string(), "GiB");
                                        ui.selectable_value(unit, "TiB".to_string(), "TiB");
                                    });
                                ui.end_row();
                            });
                        }
                    });
                },
            );

            // 内存后端配置卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "内存后端配置", Some("🗄"), colors, |ui| {
                        let mut has_backing = config.memory_backing.is_some();
                        if checkbox(ui, &mut has_backing, "启用内存后端配置") {
                            if has_backing {
                                config.memory_backing = Some(MemoryBackingConfig::default());
                            } else {
                                config.memory_backing = None;
                            }
                        }

                        if let Some(ref mut backing) = config.memory_backing {
                            ui.add_space(5.0);

                            // 大页内存配置
                            let mut has_hugepages = backing.hugepages.is_some();
                            if checkbox(ui, &mut has_hugepages, "使用大页内存") {
                                if has_hugepages {
                                    backing.hugepages = Some(HugepagesConfig {
                                        size: Some("2".to_string()),
                                        unit: Some("MiB".to_string()),
                                        nodeset: None,
                                        page: None,
                                    });
                                } else {
                                    backing.hugepages = None;
                                }
                            }

                            if let Some(ref mut hp) = backing.hugepages {
                                ui.add_space(5.0);
                                grid(ui, "hugepages_grid", 2, |ui| {
                                    ui.label("页大小:");
                                    let mut size = hp.size.clone().unwrap_or_default();
                                    if ui.text_edit_singleline(&mut size).changed() {
                                        hp.size = Some(size);
                                    }
                                    ui.end_row();

                                    ui.label("单位:");
                                    let unit = hp.unit.get_or_insert_with(|| "MiB".to_string());
                                    egui::ComboBox::from_id_source("hugepages_unit")
                                        .selected_text(unit.as_str())
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(unit, "KiB".to_string(), "KiB");
                                            ui.selectable_value(unit, "MiB".to_string(), "MiB");
                                            ui.selectable_value(unit, "GiB".to_string(), "GiB");
                                        });
                                    ui.end_row();

                                    ui.label("NUMA nodeset:");
                                    let nodeset = hp.nodeset.get_or_insert_with(|| "".to_string());
                                    ui.text_edit_singleline(nodeset);
                                    ui.end_row();
                                });
                            }

                            ui.add_space(5.0);

                            // 内存共享页
                            let mut nosharepages = backing.nosharepages.is_some();
                            if checkbox(ui, &mut nosharepages, "禁用内存共享页 (KSM)") {
                                backing.nosharepages = if nosharepages { Some(()) } else { None };
                            }

                            // 锁定内存
                            let mut locked = backing.locked.is_some();
                            if checkbox(ui, &mut locked, "锁定内存 (mlock)") {
                                backing.locked = if locked { Some(()) } else { None };
                            }

                            ui.add_space(5.0);

                            // 内存后端源类型
                            ui.label("内存后端源:");
                            let source_type = backing.source.get_or_insert_with(|| {
                                crate::model::MemorySource { source_type: "anonymous".to_string() }
                            });
                            egui::ComboBox::from_id_source("memory_source_type")
                                .selected_text(&source_type.source_type)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut source_type.source_type,
                                        "anonymous".to_string(),
                                        "anonymous (默认)",
                                    );
                                    ui.selectable_value(
                                        &mut source_type.source_type,
                                        "file".to_string(),
                                        "file",
                                    );
                                    ui.selectable_value(
                                        &mut source_type.source_type,
                                        "memfd".to_string(),
                                        "memfd",
                                    );
                                });

                            ui.add_space(5.0);

                            // 内存访问模式
                            ui.label("内存访问模式:");
                            let access_mode = backing.access.get_or_insert_with(|| {
                                crate::model::MemoryAccess { mode: "shared".to_string() }
                            });
                            egui::ComboBox::from_id_source("memory_access_mode")
                                .selected_text(&access_mode.mode)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut access_mode.mode,
                                        "shared".to_string(),
                                        "shared",
                                    );
                                    ui.selectable_value(
                                        &mut access_mode.mode,
                                        "private".to_string(),
                                        "private",
                                    );
                                });

                            ui.add_space(5.0);

                            // 内存分配模式
                            ui.label("内存分配模式:");
                            let alloc_mode = backing.allocation.get_or_insert_with(|| {
                                crate::model::MemoryAllocation {
                                    mode: "ondemand".to_string(),
                                    threads: None,
                                }
                            });
                            egui::ComboBox::from_id_source("memory_alloc_mode")
                                .selected_text(&alloc_mode.mode)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut alloc_mode.mode,
                                        "immediate".to_string(),
                                        "immediate (立即分配)",
                                    );
                                    ui.selectable_value(
                                        &mut alloc_mode.mode,
                                        "ondemand".to_string(),
                                        "ondemand (按需分配)",
                                    );
                                });

                            if alloc_mode.mode == "immediate" {
                                ui.add_space(5.0);
                                ui.label("分配线程数:");
                                let threads = alloc_mode.threads.get_or_insert(4);
                                ui.add(egui::Slider::new(threads, 1..=64).text("个"));
                            }

                            ui.add_space(5.0);

                            // 内存回收
                            let mut discard = backing.discard.is_some();
                            if checkbox(ui, &mut discard, "启用内存回收 (discard)") {
                                backing.discard = if discard { Some(()) } else { None };
                            }
                        }
                    });
                },
            );

            // 内存调优卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "内存调优", Some("🔧"), colors, |ui| {
                        let mut nosharepages = config.memory.nosharepages.unwrap_or(false);
                        if checkbox(ui, &mut nosharepages, "禁用内存共享页") {
                            config.memory.nosharepages = Some(nosharepages);
                        }

                        let mut locked = config.memory.locked.unwrap_or(false);
                        if checkbox(ui, &mut locked, "锁定内存 (mlock)") {
                            config.memory.locked = Some(locked);
                        }
                    });
                },
            );
        });
    }
}
