use egui::RichText;

use crate::{
    model::{CPUModel, CPUTopology, VMConfig},
    panels::utils::{get_theme_colors, Theme, *},
};

pub struct CPUPanel;

impl CPUPanel {
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        let colors = get_theme_colors(Theme::Light);

        panel_header(ui, "🖥", "CPU 配置");

        card_group(ui, "CPU 拓扑结构", None, |ui| {
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

                ui.label("Cores:");
                ui.add(egui::Slider::new(&mut topology.cores, 1..=64).text("核"));
                ui.end_row();

                ui.label("Threads:");
                ui.add(egui::Slider::new(&mut topology.threads, 1..=8).text("线程"));
                ui.end_row();
            });

            let total_vcpus =
                topology.sockets * topology.dies.unwrap_or(1) * topology.cores * topology.threads;
            ui.label(
                RichText::new(format!("总 vCPU 数：{}", total_vcpus))
                    .strong()
                    .color(colors.info),
            );

            config.cpu.topology = Some(topology);
        });

        ui.add_space(8.0);

        card_group(ui, "CPU 模式", None, |ui| {
            let cpu_mode = config.cpu.mode.get_or_insert_with(|| "host-passthrough".to_string());
            grid(ui, "cpu_mode_grid", 2, |ui| {
                ui.label("CPU 模式:");
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
            });
        });

        ui.add_space(8.0);

        card_group(ui, "CPU 型号", None, |ui| {
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
                });
            }
        });
    }
}
