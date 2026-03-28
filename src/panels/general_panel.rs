use egui::RichText;

use crate::model::VMConfig;

pub struct GeneralPanel;

impl GeneralPanel {
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.heading(RichText::new("🔧 基础配置").size(18.0));
        ui.separator();
        ui.add_space(10.0);

        egui::Grid::new("general_grid").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
            ui.label("虚拟机类型:");
            egui::ComboBox::from_id_source("vm_type")
                .selected_text(&config.general.vm_type)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut config.general.vm_type, "kvm".to_string(), "kvm");
                    ui.selectable_value(&mut config.general.vm_type, "qemu".to_string(), "qemu");
                    ui.selectable_value(&mut config.general.vm_type, "xen".to_string(), "xen");
                    ui.selectable_value(&mut config.general.vm_type, "lxc".to_string(), "lxc");
                });
            ui.end_row();

            ui.label("虚拟机名称:");
            ui.text_edit_singleline(&mut config.general.name);
            ui.end_row();

            ui.label("UUID:");
            let mut uuid_str = config.general.uuid.clone().unwrap_or_default();
            if ui.text_edit_singleline(&mut uuid_str).changed() {
                config.general.uuid = Some(uuid_str);
            }
            if ui.button("🔄 生成").clicked() {
                config.general.uuid = Some(uuid::Uuid::new_v4().to_string());
            }
            ui.end_row();

            ui.label("描述:");
            let mut desc = config.general.description.clone().unwrap_or_default();
            ui.text_edit_multiline(&mut desc);
            if ui.ctx().input(|i| i.key_pressed(egui::Key::Enter)) {
                config.general.description = Some(desc);
            }
            ui.end_row();

            ui.label("标题:");
            let mut title = config.general.title.clone().unwrap_or_default();
            ui.text_edit_singleline(&mut title);
            config.general.title = if title.is_empty() { None } else { Some(title) };
            ui.end_row();

            ui.label("vCPU 数量:");
            ui.add(egui::Slider::new(&mut config.general.vcpu.count, 1..=64));
            ui.end_row();

            ui.label("vCPU 放置策略:");
            let mut placement = config.general.vcpu.placement.clone().unwrap_or_default();
            egui::ComboBox::from_id_source("vcpu_placement").selected_text(&placement).show_ui(
                ui,
                |ui| {
                    ui.selectable_value(&mut placement, "static".to_string(), "static");
                    ui.selectable_value(&mut placement, "auto".to_string(), "auto");
                },
            );
            config.general.vcpu.placement = Some(placement);
            ui.end_row();

            ui.label("内存大小:");
            ui.add(egui::Slider::new(&mut config.general.memory.value, 1..=128));
            ui.end_row();

            ui.label("内存单位:");
            let mut unit = config.general.memory.unit.clone().unwrap_or_default();
            egui::ComboBox::from_id_source("memory_unit").selected_text(&unit).show_ui(ui, |ui| {
                ui.selectable_value(&mut unit, "KiB".to_string(), "KiB");
                ui.selectable_value(&mut unit, "MiB".to_string(), "MiB");
                ui.selectable_value(&mut unit, "GiB".to_string(), "GiB");
                ui.selectable_value(&mut unit, "TiB".to_string(), "TiB");
            });
            config.general.memory.unit = Some(unit);
            ui.end_row();
        });
    }
}
