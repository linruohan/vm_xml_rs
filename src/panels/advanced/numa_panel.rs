use egui::{Align, Layout};

use crate::{
    model::{NUMACell, NUMAConfig, VMConfig},
    panels::utils::*,
};

/// NUMA 配置面板
pub struct NUMAPanel;

impl NUMAPanel {
    /// 显示 NUMA 配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "🔢", "NUMA 配置");

        // 卡片宽度和间距配置
        let card_width = 380.0;
        let spacing = 8.0;

        // 使用流式布局实现卡片自动换行
        ui.with_layout(Layout::left_to_right(Align::TOP).with_main_wrap(true), |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);

            // NUMA 节点配置卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "NUMA 节点配置", Some("🔧"), colors, |ui| {
                        let mut has_numa = config.numa.is_some();
                        if checkbox(ui, &mut has_numa, "启用 NUMA 配置") {
                            if has_numa {
                                config.numa = Some(NUMAConfig {
                                    cell: Some(vec![NUMACell {
                                        id: 0,
                                        cpus: "0-1".to_string(),
                                        memory: 4 * 1024 * 1024,
                                        unit: Some("KiB".to_string()),
                                        memnode: None,
                                    }]),
                                });
                            } else {
                                config.numa = None;
                            }
                        }

                        if let Some(ref mut numa_config) = config.numa {
                            if let Some(ref mut cell_list) = numa_config.cell {
                                ui.horizontal(|ui| {
                                    if add_button(ui, "➕ 添加 NUMA 节点", colors) {
                                        let new_id = cell_list.len() as u32;
                                        cell_list.push(NUMACell {
                                            id: new_id,
                                            cpus: format!("{}-{}", new_id * 2, new_id * 2 + 1),
                                            memory: 4 * 1024 * 1024,
                                            unit: Some("KiB".to_string()),
                                            memnode: None,
                                        });
                                    }
                                });

                                let mut to_remove = None;
                                for (i, cell) in cell_list.iter_mut().enumerate() {
                                    ui.push_id(i, |ui| {
                                        inner_group(ui, colors, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(format!("NUMA 节点 {}", cell.id));
                                                if delete_button(ui, None) {
                                                    to_remove = Some(i);
                                                }
                                            });

                                            grid(ui, format!("numa_grid_{}", i), 2, |ui| {
                                                ui.label("节点 ID:");
                                                ui.add(
                                                    egui::Slider::new(&mut cell.id, 0..=16)
                                                        .text(""),
                                                );
                                                ui.end_row();

                                                ui.label("CPU 范围:");
                                                ui.text_edit_singleline(&mut cell.cpus);
                                                ui.end_row();

                                                ui.label("内存:");
                                                ui.add(
                                                    egui::DragValue::new(&mut cell.memory)
                                                        .prefix("KiB: "),
                                                );
                                                ui.end_row();

                                                ui.label("内存单位:");
                                                let unit = cell
                                                    .unit
                                                    .get_or_insert_with(|| "KiB".to_string());
                                                egui::ComboBox::from_id_source(format!(
                                                    "numa_unit_{}",
                                                    i
                                                ))
                                                .selected_text(unit.as_str())
                                                .show_ui(ui, |ui| {
                                                    ui.selectable_value(
                                                        unit,
                                                        "KiB".to_string(),
                                                        "KiB",
                                                    );
                                                    ui.selectable_value(
                                                        unit,
                                                        "MiB".to_string(),
                                                        "MiB",
                                                    );
                                                    ui.selectable_value(
                                                        unit,
                                                        "GiB".to_string(),
                                                        "GiB",
                                                    );
                                                });
                                                ui.end_row();
                                            });
                                        });
                                        ui.add_space(5.0);
                                    });
                                }

                                if let Some(idx) = to_remove {
                                    cell_list.remove(idx);
                                }
                            }
                        }
                    });
                },
            );
        });
    }
}
