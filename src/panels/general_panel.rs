use crate::{model::VMConfig, panels::utils::*};

pub struct GeneralPanel;

impl GeneralPanel {
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        panel_header(ui, "🔧", "基础配置");

        card_group(ui, "虚拟机基本信息", None, |ui| {
            grid(ui, "general_info_grid", 2, |ui| {
                // 虚拟机类型
                ui.label("虚拟机类型:");
                egui::ComboBox::from_id_source("vm_type")
                    .selected_text(&config.general.vm_type)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut config.general.vm_type, "kvm".to_string(), "kvm");
                        ui.selectable_value(
                            &mut config.general.vm_type,
                            "qemu".to_string(),
                            "qemu",
                        );
                        ui.selectable_value(&mut config.general.vm_type, "xen".to_string(), "xen");
                        ui.selectable_value(&mut config.general.vm_type, "lxc".to_string(), "lxc");
                    });
                ui.end_row();

                // 虚拟机名称
                ui.label("虚拟机名称:");
                ui.text_edit_singleline(&mut config.general.name);
                ui.end_row();

                // UUID
                ui.label("UUID:");
                ui.horizontal(|ui| {
                    let mut uuid_str = config.general.uuid.clone().unwrap_or_default();
                    if ui.text_edit_singleline(&mut uuid_str).changed() {
                        config.general.uuid = Some(uuid_str);
                    }
                    if ui.button("🔄 生成").clicked() {
                        config.general.uuid = Some(uuid::Uuid::new_v4().to_string());
                    }
                });
                ui.end_row();

                // 描述
                ui.label("描述:");
                let desc = config.general.description.get_or_insert_with(String::new);
                let mut desc = desc.clone();
                if ui.text_edit_multiline(&mut desc).changed() {
                    config.general.description = Some(desc);
                }
                ui.end_row();

                // 标题
                ui.label("标题:");
                let title = config.general.title.get_or_insert_with(String::new);
                let mut title = title.clone();
                if ui.text_edit_singleline(&mut title).changed() {
                    config.general.title = Some(title);
                }
                ui.end_row();
            });
        });

        ui.add_space(8.0);

        card_group(ui, "CPU 和内存配置", None, |ui| {
            grid(ui, "general_resources_grid", 2, |ui| {
                // vCPU 数量
                ui.label("vCPU 数量:");
                ui.add(
                    egui::Slider::new(&mut config.general.vcpu.count, 1..=64)
                        .text("核心")
                        .logarithmic(true),
                );
                ui.end_row();

                // vCPU 放置策略
                ui.label("vCPU 放置:");
                let placement =
                    config.general.vcpu.placement.get_or_insert_with(|| "static".to_string());
                egui::ComboBox::from_id_source("vcpu_placement")
                    .selected_text(placement.as_str())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(placement, "static".to_string(), "静态分配");
                        ui.selectable_value(placement, "auto".to_string(), "自动分配");
                    });
                ui.end_row();

                // 内存大小
                ui.label("内存大小:");
                ui.add(
                    egui::Slider::new(&mut config.general.memory.value, 1..=128)
                        .text("单位")
                        .logarithmic(true),
                );
                ui.end_row();

                // 内存单位
                ui.label("内存单位:");
                let unit = config.general.memory.unit.get_or_insert_with(|| "MiB".to_string());
                egui::ComboBox::from_id_source("memory_unit").selected_text(unit.as_str()).show_ui(
                    ui,
                    |ui| {
                        ui.selectable_value(unit, "KiB".to_string(), "KiB");
                        ui.selectable_value(unit, "MiB".to_string(), "MiB");
                        ui.selectable_value(unit, "GiB".to_string(), "GiB");
                        ui.selectable_value(unit, "TiB".to_string(), "TiB");
                    },
                );
                ui.end_row();
            });
        });
    }
}
