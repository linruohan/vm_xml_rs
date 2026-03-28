use crate::{
    model::{HugepagesConfig, MemoryBackingConfig, VMConfig},
    panels::utils::*,
};

pub struct MemoryPanel;

impl MemoryPanel {
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        panel_header(ui, "💾", "内存配置");

        card_group(ui, "当前内存设置", None, |ui| {
            grid(ui, "memory_current_grid", 2, |ui| {
                ui.label("内存大小:");
                ui.add(egui::Slider::new(&mut config.general.memory.value, 1..=128).text("单位"));
                ui.end_row();

                ui.label("内存单位:");
                let unit = config.general.memory.unit.get_or_insert_with(|| "MiB".to_string());
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

        ui.add_space(8.0);

        card_group(ui, "内存后端配置", None, |ui| {
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
                    });
                }

                ui.add_space(5.0);

                let mut nosharepages = backing.nosharepages.is_some();
                if checkbox(ui, &mut nosharepages, "禁用内存共享页") {
                    backing.nosharepages = if nosharepages { Some(()) } else { None };
                }

                let mut locked = backing.locked.is_some();
                if checkbox(ui, &mut locked, "锁定内存") {
                    backing.locked = if locked { Some(()) } else { None };
                }
            }
        });

        ui.add_space(8.0);

        card_group(ui, "内存调优", None, |ui| {
            let mut nosharepages = config.memory.nosharepages.unwrap_or(false);
            if checkbox(ui, &mut nosharepages, "禁用内存共享页") {
                config.memory.nosharepages = Some(nosharepages);
            }

            let mut locked = config.memory.locked.unwrap_or(false);
            if checkbox(ui, &mut locked, "锁定内存 (mlock)") {
                config.memory.locked = Some(locked);
            }
        });
    }
}
