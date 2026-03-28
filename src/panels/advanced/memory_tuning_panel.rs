use egui::RichText;

use crate::model::vm_config::{MemoryTuningConfig, VMConfig};

/// 内存调优配置面板
pub struct MemoryTuningPanel;

impl MemoryTuningPanel {
    /// 显示内存调优配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.group(|ui| {
            ui.label(RichText::new("内存调优").strong());
            ui.add_space(5.0);

            let mut has_tuning = config.memory_tuning.is_some();
            if ui.checkbox(&mut has_tuning, "启用内存调优").changed() {
                if has_tuning {
                    config.memory_tuning = Some(MemoryTuningConfig::default());
                } else {
                    config.memory_tuning = None;
                }
            }

            if let Some(ref mut tuning) = config.memory_tuning {
                egui::Grid::new("memory_tuning_grid").num_columns(2).spacing([10.0, 8.0]).show(
                    ui,
                    |ui| {
                        ui.label("硬限制 (MiB):");
                        let mut hard_limit = tuning.hard_limit.unwrap_or(0);
                        ui.add(egui::DragValue::new(&mut hard_limit));
                        tuning.hard_limit = if hard_limit > 0 { Some(hard_limit) } else { None };
                        ui.end_row();

                        ui.label("软限制 (MiB):");
                        let mut soft_limit = tuning.soft_limit.unwrap_or(0);
                        ui.add(egui::DragValue::new(&mut soft_limit));
                        tuning.soft_limit = if soft_limit > 0 { Some(soft_limit) } else { None };
                        ui.end_row();

                        ui.label("交换硬限制 (MiB):");
                        let mut swap_limit = tuning.swap_hard_limit.unwrap_or(0);
                        ui.add(egui::DragValue::new(&mut swap_limit));
                        tuning.swap_hard_limit =
                            if swap_limit > 0 { Some(swap_limit) } else { None };
                        ui.end_row();

                        ui.label("内存保证 (MiB):");
                        let mut guarantee = tuning.guarantee.unwrap_or(0);
                        ui.add(egui::DragValue::new(&mut guarantee));
                        tuning.guarantee = if guarantee > 0 { Some(guarantee) } else { None };
                        ui.end_row();
                    },
                );
            }
        });
    }
}
