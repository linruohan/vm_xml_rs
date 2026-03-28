use egui::RichText;

use crate::model::vm_config::{ResourcePartitioningConfig, VMConfig};

/// 资源隔离与分区配置面板
pub struct ResourcePartitioningPanel;

impl ResourcePartitioningPanel {
    /// 显示资源隔离与分区配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.group(|ui| {
            ui.label(RichText::new("资源隔离与分区配置").strong());
            ui.add_space(5.0);

            let mut has_partitioning = config.resource_partitioning.is_some();
            if ui.checkbox(&mut has_partitioning, "启用资源分区").changed() {
                if has_partitioning {
                    config.resource_partitioning = Some(ResourcePartitioningConfig::default());
                } else {
                    config.resource_partitioning = None;
                }
            }

            if let Some(ref mut partitioning) = config.resource_partitioning {
                egui::Grid::new("resource_partitioning_grid")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("CPU 集合:");
                        let mut cpuset = partitioning.cpuset.clone().unwrap_or_default();
                        ui.text_edit_singleline(&mut cpuset);
                        partitioning.cpuset = if cpuset.is_empty() { None } else { Some(cpuset) };
                        ui.end_row();

                        ui.label("内存节点:");
                        let mut memnode = partitioning.memnode.clone().unwrap_or_default();
                        ui.text_edit_singleline(&mut memnode);
                        partitioning.memnode =
                            if memnode.is_empty() { None } else { Some(memnode) };
                        ui.end_row();
                    });
            }
        });
    }
}
