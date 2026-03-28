use egui::RichText;

use crate::model::vm_config::{CPUModel, CPUTopology, VMConfig};

pub struct CPUPanel;

impl CPUPanel {
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.heading(RichText::new("CPU 配置").size(18.0));
        ui.separator();
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label(RichText::new("CPU 拓扑结构").strong());
            ui.add_space(5.0);

            let mut topology = config.cpu.topology.take().unwrap_or(CPUTopology {
                sockets: 1,
                dies: 1,
                cores: 2,
                threads: 1,
            });

            egui::Grid::new("cpu_topology_grid").num_columns(2).spacing([10.0, 8.0]).show(
                ui,
                |ui| {
                    ui.label("Sockets:");
                    ui.add(egui::Slider::new(&mut topology.sockets, 1..=8));
                    ui.end_row();

                    ui.label("Dies:");
                    ui.add(egui::Slider::new(&mut topology.dies, 1..=4));
                    ui.end_row();

                    ui.label("Cores:");
                    ui.add(egui::Slider::new(&mut topology.cores, 1..=64));
                    ui.end_row();

                    ui.label("Threads:");
                    ui.add(egui::Slider::new(&mut topology.threads, 1..=8));
                    ui.end_row();
                },
            );

            let total_vcpus = topology.sockets * topology.dies * topology.cores * topology.threads;
            ui.label(format!("总 vCPU 数: {}", total_vcpus));

            config.cpu.topology = Some(topology);
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label(RichText::new("CPU 模式").strong());
            ui.add_space(5.0);

            let mut mode =
                config.cpu.mode.clone().unwrap_or_else(|| "host-passthrough".to_string());
            egui::ComboBox::from_id_source("cpu_mode").selected_text(&mode).show_ui(ui, |ui| {
                ui.selectable_value(&mut mode, "custom".to_string(), "custom");
                ui.selectable_value(&mut mode, "host-model".to_string(), "host-model");
                ui.selectable_value(&mut mode, "host-passthrough".to_string(), "host-passthrough");
                ui.selectable_value(&mut mode, "maximum".to_string(), "maximum");
            });
            config.cpu.mode = Some(mode);
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label(RichText::new("CPU 型号").strong());
            ui.add_space(5.0);

            let mut has_model = config.cpu.model.is_some();
            if ui.checkbox(&mut has_model, "指定 CPU 型号").changed() {
                if has_model {
                    config.cpu.model = Some(CPUModel {
                        fallback: Some("allow".to_string()),
                        name: "qemu64".to_string(),
                    });
                } else {
                    config.cpu.model = None;
                }
            }

            if let Some(ref mut model) = config.cpu.model {
                egui::Grid::new("cpu_model_grid").num_columns(2).spacing([10.0, 8.0]).show(
                    ui,
                    |ui| {
                        ui.label("型号名称:");
                        egui::ComboBox::from_id_source("cpu_model_name")
                            .selected_text(&model.name)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut model.name,
                                    "qemu64".to_string(),
                                    "qemu64",
                                );
                                ui.selectable_value(
                                    &mut model.name,
                                    "qemu32".to_string(),
                                    "qemu32",
                                );
                                ui.selectable_value(&mut model.name, "host".to_string(), "host");
                                ui.selectable_value(&mut model.name, "kvm64".to_string(), "kvm64");
                                ui.selectable_value(&mut model.name, "kvm32".to_string(), "kvm32");
                                ui.selectable_value(
                                    &mut model.name,
                                    "Skylake-Client".to_string(),
                                    "Skylake-Client",
                                );
                                ui.selectable_value(
                                    &mut model.name,
                                    "Skylake-Server".to_string(),
                                    "Skylake-Server",
                                );
                                ui.selectable_value(
                                    &mut model.name,
                                    "Cascadelake-Server".to_string(),
                                    "Cascadelake-Server",
                                );
                                ui.selectable_value(
                                    &mut model.name,
                                    "Icelake-Client".to_string(),
                                    "Icelake-Client",
                                );
                                ui.selectable_value(
                                    &mut model.name,
                                    "Icelake-Server".to_string(),
                                    "Icelake-Server",
                                );
                            });
                        ui.end_row();

                        ui.label("回退策略:");
                        let mut fallback =
                            model.fallback.clone().unwrap_or_else(|| "allow".to_string());
                        egui::ComboBox::from_id_source("cpu_fallback")
                            .selected_text(&fallback)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut fallback, "allow".to_string(), "allow");
                                ui.selectable_value(&mut fallback, "forbid".to_string(), "forbid");
                            });
                        model.fallback = Some(fallback);
                        ui.end_row();
                    },
                );
            }
        });
    }
}
