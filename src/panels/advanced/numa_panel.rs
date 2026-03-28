use egui::RichText;

use crate::model::vm_config::{NUMACell, NUMAConfig, VMConfig};

/// NUMA 配置面板
pub struct NUMAPanel;

impl NUMAPanel {
    /// 显示 NUMA 配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.group(|ui| {
            ui.label(RichText::new("NUMA 配置").strong());
            ui.add_space(5.0);

            let mut has_numa = config.numa.is_some();
            if ui.checkbox(&mut has_numa, "启用 NUMA 配置").changed() {
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
                    if ui.button("➕ 添加 NUMA 节点").clicked() {
                        let new_id = cell_list.len() as u32;
                        cell_list.push(NUMACell {
                            id: new_id,
                            cpus: format!("{}-{}", new_id * 2, new_id * 2 + 1),
                            memory: 4 * 1024 * 1024,
                            unit: Some("KiB".to_string()),
                            memnode: None,
                        });
                    }

                    let mut to_remove = None;
                    for (i, cell) in cell_list.iter_mut().enumerate() {
                        ui.push_id(i, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("NUMA 节点 {}:", cell.id));
                                if ui.button("🗑️ 删除").clicked() {
                                    to_remove = Some(i);
                                }
                            });

                            egui::Grid::new(format!("numa_grid_{}", i))
                                .num_columns(2)
                                .spacing([10.0, 8.0])
                                .show(ui, |ui| {
                                    ui.label("节点 ID:");
                                    ui.add(egui::Slider::new(&mut cell.id, 0..=16));
                                    ui.end_row();

                                    ui.label("CPU 范围:");
                                    ui.text_edit_singleline(&mut cell.cpus);
                                    ui.end_row();

                                    ui.label("内存:");
                                    ui.add(egui::DragValue::new(&mut cell.memory));
                                    ui.end_row();

                                    ui.label("内存单位:");
                                    let mut unit = cell.unit.clone().unwrap_or_default();
                                    egui::ComboBox::from_id_source(format!("numa_unit_{}", i))
                                        .selected_text(&unit)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut unit,
                                                "KiB".to_string(),
                                                "KiB",
                                            );
                                            ui.selectable_value(
                                                &mut unit,
                                                "MiB".to_string(),
                                                "MiB",
                                            );
                                            ui.selectable_value(
                                                &mut unit,
                                                "GiB".to_string(),
                                                "GiB",
                                            );
                                        });
                                    cell.unit = Some(unit);
                                    ui.end_row();
                                });
                        });
                    }

                    if let Some(idx) = to_remove {
                        cell_list.remove(idx);
                    }
                }
            }
        });
    }
}
