use egui::RichText;

use crate::model::vm_config::{HugepagesConfig, MemoryBackingConfig, VMConfig};

pub struct MemoryPanel;

impl MemoryPanel {
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.heading(RichText::new("💾 内存配置").size(18.0));
        ui.separator();
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label(RichText::new("当前内存设置").strong());
            ui.add_space(5.0);

            egui::Grid::new("memory_current_grid").num_columns(2).spacing([10.0, 8.0]).show(
                ui,
                |ui| {
                    ui.label("内存大小:");
                    ui.add(egui::Slider::new(&mut config.general.memory.value, 1..=128));
                    ui.end_row();

                    ui.label("内存单位:");
                    let mut unit = config.general.memory.unit.clone().unwrap_or_default();
                    egui::ComboBox::from_id_source("memory_unit_panel")
                        .selected_text(&unit)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut unit, "KiB".to_string(), "KiB");
                            ui.selectable_value(&mut unit, "MiB".to_string(), "MiB");
                            ui.selectable_value(&mut unit, "GiB".to_string(), "GiB");
                            ui.selectable_value(&mut unit, "TiB".to_string(), "TiB");
                        });
                    config.general.memory.unit = Some(unit);
                    ui.end_row();
                },
            );
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label(RichText::new("内存后端配置").strong());
            ui.add_space(5.0);

            let mut has_backing = config.memory_backing.is_some();
            if ui.checkbox(&mut has_backing, "启用内存后端配置").changed() {
                if has_backing {
                    config.memory_backing = Some(MemoryBackingConfig::default());
                } else {
                    config.memory_backing = None;
                }
            }

            if let Some(ref mut backing) = config.memory_backing {
                ui.add_space(5.0);

                let mut has_hugepages = backing.hugepages.is_some();
                if ui.checkbox(&mut has_hugepages, "使用大页内存").changed() {
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
                    egui::Grid::new("hugepages_grid").num_columns(2).spacing([10.0, 8.0]).show(
                        ui,
                        |ui| {
                            ui.label("页大小:");
                            let mut size = hp.size.clone().unwrap_or_default();
                            ui.text_edit_singleline(&mut size);
                            hp.size = Some(size);
                            ui.end_row();

                            ui.label("单位:");
                            let mut unit = hp.unit.clone().unwrap_or_default();
                            egui::ComboBox::from_id_source("hugepages_unit")
                                .selected_text(&unit)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut unit, "KiB".to_string(), "KiB");
                                    ui.selectable_value(&mut unit, "MiB".to_string(), "MiB");
                                    ui.selectable_value(&mut unit, "GiB".to_string(), "GiB");
                                });
                            hp.unit = Some(unit);
                            ui.end_row();
                        },
                    );
                }

                ui.add_space(5.0);

                let mut nosharepages = backing.nosharepages.is_some();
                if ui.checkbox(&mut nosharepages, "禁用内存共享页").changed() {
                    backing.nosharepages = if nosharepages { Some(()) } else { None };
                }

                let mut locked = backing.locked.is_some();
                if ui.checkbox(&mut locked, "锁定内存").changed() {
                    backing.locked = if locked { Some(()) } else { None };
                }
            }
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label(RichText::new("内存调优").strong());
            ui.add_space(5.0);

            let mut nosharepages = config.memory.nosharepages.unwrap_or(false);
            if ui.checkbox(&mut nosharepages, "禁用内存共享页").changed() {
                config.memory.nosharepages = Some(nosharepages);
            }

            let mut locked = config.memory.locked.unwrap_or(false);
            if ui.checkbox(&mut locked, "锁定内存 (mlock)").changed() {
                config.memory.locked = Some(locked);
            }
        });
    }
}
