use crate::{
    field_row_drag,
    model::{MemoryTuningConfig, VMConfig},
    panels::utils::*,
};

/// 内存调优配置面板
pub struct MemoryTuningPanel;

impl MemoryTuningPanel {
    /// 显示内存调优配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "📊", "内存调优");

        card_group(ui, "内存限制设置", None, colors, |ui| {
            let mut has_tuning = config.memory_tuning.is_some();
            if checkbox(ui, &mut has_tuning, "启用内存调优") {
                if has_tuning {
                    config.memory_tuning = Some(MemoryTuningConfig::default());
                } else {
                    config.memory_tuning = None;
                }
            }

            if let Some(ref mut tuning) = config.memory_tuning {
                ui.add_space(5.0);
                grid(ui, "memory_tuning_grid", 2, |ui| {
                    field_row_drag!(ui, "硬限制 (MiB):", &mut tuning.hard_limit);
                    field_row_drag!(ui, "软限制 (MiB):", &mut tuning.soft_limit);
                    field_row_drag!(ui, "交换硬限制 (MiB):", &mut tuning.swap_hard_limit);
                    field_row_drag!(ui, "内存保证 (MiB):", &mut tuning.guarantee);
                });
            }
        });
    }
}
