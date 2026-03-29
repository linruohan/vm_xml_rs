use egui::{Align, Layout};

use crate::{
    model::{
        memory::{HugepagesConfig, PageConfig},
        memory_backing::{MemoryAccess, MemoryAllocation, MemorySource},
        MemoryBackingConfig, VMConfig,
    },
    panels::utils::*,
};

/// Memory Backing 配置面板
pub struct MemoryBackingPanel;

impl MemoryBackingPanel {
    /// 显示 Memory Backing 配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "📊", "内存后端配置");

        // 卡片宽度和间距配置
        let card_width = 380.0;
        let spacing = 8.0;

        // 使用流式布局实现卡片自动换行
        ui.with_layout(Layout::left_to_right(Align::TOP).with_main_wrap(true), |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);

            // 基本设置卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "基本设置", Some("⚙"), colors, |ui| {
                        let mut has_memory_backing = config.memory_backing.is_some();
                        if checkbox(ui, &mut has_memory_backing, "启用内存后端配置") {
                            if has_memory_backing {
                                config.memory_backing = Some(MemoryBackingConfig::default());
                            } else {
                                config.memory_backing = None;
                            }
                        }

                        if let Some(ref mut memory_backing) = config.memory_backing {
                            ui.add_space(5.0);

                            // nosharepages
                            let mut nosharepages = memory_backing.nosharepages.is_some();
                            if checkbox(ui, &mut nosharepages, "禁用共享页面 (KSM)") {
                                if nosharepages {
                                    memory_backing.nosharepages = Some(());
                                } else {
                                    memory_backing.nosharepages = None;
                                }
                            }

                            ui.add_space(5.0);

                            // locked
                            let mut locked = memory_backing.locked.is_some();
                            if checkbox(ui, &mut locked, "锁定内存 (禁止交换)") {
                                if locked {
                                    memory_backing.locked = Some(());
                                } else {
                                    memory_backing.locked = None;
                                }
                            }

                            ui.add_space(5.0);

                            // discard
                            let mut discard = memory_backing.discard.is_some();
                            if checkbox(ui, &mut discard, "Discard (关机时丢弃内存内容)") {
                                if discard {
                                    memory_backing.discard = Some(());
                                } else {
                                    memory_backing.discard = None;
                                }
                            }
                        }
                    });
                },
            );

            // 内存源配置卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "内存源配置", Some("🗄"), colors, |ui| {
                        if let Some(ref mut memory_backing) = config.memory_backing {
                            let mut has_source = memory_backing.source.is_some();
                            if checkbox(ui, &mut has_source, "启用内存源配置") {
                                if has_source {
                                    memory_backing.source =
                                        Some(MemorySource { source_type: "anonymous".to_string() });
                                } else {
                                    memory_backing.source = None;
                                }
                            }

                            if let Some(ref mut source) = memory_backing.source {
                                ui.add_space(5.0);
                                let source_type = &mut source.source_type;
                                egui::ComboBox::from_id_source("memory_source_type")
                                    .selected_text(source_type.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            source_type,
                                            "anonymous".to_string(),
                                            "anonymous (匿名内存)",
                                        );
                                        ui.selectable_value(
                                            source_type,
                                            "file".to_string(),
                                            "file (文件-backed)",
                                        );
                                        ui.selectable_value(
                                            source_type,
                                            "memfd".to_string(),
                                            "memfd (内存文件描述符)",
                                        );
                                    });
                            }
                        }
                    });
                },
            );

            // 内存访问模式卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "内存访问模式", Some("🔓"), colors, |ui| {
                        if let Some(ref mut memory_backing) = config.memory_backing {
                            let mut has_access = memory_backing.access.is_some();
                            if checkbox(ui, &mut has_access, "启用访问模式配置") {
                                if has_access {
                                    memory_backing.access =
                                        Some(MemoryAccess { mode: "shared".to_string() });
                                } else {
                                    memory_backing.access = None;
                                }
                            }

                            if let Some(ref mut access) = memory_backing.access {
                                ui.add_space(5.0);
                                let mode = &mut access.mode;
                                egui::ComboBox::from_id_source("memory_access_mode")
                                    .selected_text(mode.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            mode,
                                            "shared".to_string(),
                                            "shared (共享)",
                                        );
                                        ui.selectable_value(
                                            mode,
                                            "private".to_string(),
                                            "private (私有)",
                                        );
                                    });
                            }
                        }
                    });
                },
            );

            // 内存分配配置卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "内存分配配置", Some("📦"), colors, |ui| {
                        if let Some(ref mut memory_backing) = config.memory_backing {
                            let mut has_allocation = memory_backing.allocation.is_some();
                            if checkbox(ui, &mut has_allocation, "启用内存分配配置") {
                                if has_allocation {
                                    memory_backing.allocation = Some(MemoryAllocation {
                                        mode: "immediate".to_string(),
                                        threads: None,
                                    });
                                } else {
                                    memory_backing.allocation = None;
                                }
                            }

                            if let Some(ref mut allocation) = memory_backing.allocation {
                                ui.add_space(5.0);
                                grid(ui, "memory_allocation_grid", 2, |ui| {
                                    ui.label("分配模式:");
                                    let mode = &mut allocation.mode;
                                    egui::ComboBox::from_id_source("memory_allocation_mode")
                                        .selected_text(mode.as_str())
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                mode,
                                                "immediate".to_string(),
                                                "immediate (立即分配)",
                                            );
                                            ui.selectable_value(
                                                mode,
                                                "ondemand".to_string(),
                                                "ondemand (按需分配)",
                                            );
                                        });
                                    ui.end_row();

                                    ui.label("线程数:");
                                    let threads = allocation.threads.get_or_insert(1);
                                    ui.add(egui::Slider::new(threads, 1..=256).text("个"));
                                    ui.end_row();
                                });
                            }
                        }
                    });
                },
            );

            // 大页配置卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "大页配置 (Hugepages)", Some("📄"), colors, |ui| {
                        if let Some(ref mut memory_backing) = config.memory_backing {
                            if memory_backing.hugepages.is_none() {
                                memory_backing.hugepages = Some(HugepagesConfig {
                                    size: None,
                                    unit: None,
                                    nodeset: None,
                                    page: None,
                                });
                            }

                            if let Some(ref mut hugepages) = memory_backing.hugepages {
                                let mut has_pages = hugepages.page.is_some();
                                if checkbox(ui, &mut has_pages, "启用大页配置") {
                                    if has_pages {
                                        hugepages.page = Some(vec![PageConfig {
                                            size: "2".to_string(),
                                            unit: Some("M".to_string()),
                                            nodeset: None,
                                        }]);
                                    } else {
                                        hugepages.page = None;
                                    }
                                }

                                if let Some(ref mut page_list) = hugepages.page {
                                    ui.horizontal(|ui| {
                                        if add_button(ui, "➕ 添加大页", colors) {
                                            page_list.push(PageConfig {
                                                size: "2".to_string(),
                                                unit: Some("M".to_string()),
                                                nodeset: None,
                                            });
                                        }
                                    });

                                    let mut to_remove = None;
                                    for (i, page) in page_list.iter_mut().enumerate() {
                                        ui.push_id(i, |ui| {
                                            inner_group(ui, colors, |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.label(format!("大页 {}", i + 1));
                                                    if delete_button(ui, None) {
                                                        to_remove = Some(i);
                                                    }
                                                });

                                                grid(ui, format!("hugepage_grid_{}", i), 2, |ui| {
                                                    ui.label("大小:");
                                                    ui.text_edit_singleline(&mut page.size);
                                                    ui.end_row();

                                                    ui.label("单位:");
                                                    let unit = page
                                                        .unit
                                                        .get_or_insert_with(|| "M".to_string());
                                                    egui::ComboBox::from_id_source(format!(
                                                        "hugepage_unit_{}",
                                                        i
                                                    ))
                                                    .selected_text(unit.as_str())
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(
                                                            unit,
                                                            "K".to_string(),
                                                            "K (KiB)",
                                                        );
                                                        ui.selectable_value(
                                                            unit,
                                                            "M".to_string(),
                                                            "M (MiB)",
                                                        );
                                                        ui.selectable_value(
                                                            unit,
                                                            "G".to_string(),
                                                            "G (GiB)",
                                                        );
                                                    });
                                                    ui.end_row();

                                                    ui.label("节点集:");
                                                    let nodeset = page
                                                        .nodeset
                                                        .get_or_insert_with(|| "".to_string());
                                                    ui.text_edit_singleline(nodeset);
                                                    ui.end_row();
                                                });
                                            });
                                            ui.add_space(5.0);
                                        });
                                    }

                                    if let Some(idx) = to_remove {
                                        page_list.remove(idx);
                                    }
                                }
                            }
                        }
                    });
                },
            );
        });
    }
}
