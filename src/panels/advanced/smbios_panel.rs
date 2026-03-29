use crate::{
    field_row_opt,
    model::{SMBIOSBaseBoard, SMBIOSBios, SMBIOSConfig, SMBIOSSystem, VMConfig},
    panels::utils::*,
};

/// SMBIOS 配置面板
pub struct SMBIOSPanel;

impl SMBIOSPanel {
    /// 显示 SMBIOS 配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "🔬", "SMBIOS 系统信息");

        card_group(ui, "SMBIOS 配置", None, colors, |ui| {
            let mut has_smbios = config.smbios.is_some();
            if checkbox(ui, &mut has_smbios, "启用 SMBIOS 配置") {
                if has_smbios {
                    config.smbios = Some(SMBIOSConfig::default());
                } else {
                    config.smbios = None;
                }
            }

            if let Some(ref mut smbios) = config.smbios {
                ui.add_space(5.0);

                ui.collapsing("系统信息", |ui| {
                    if smbios.system.is_none() {
                        smbios.system = Some(SMBIOSSystem::default());
                    }
                    if let Some(ref mut system) = smbios.system {
                        grid(ui, "smbios_system_grid", 2, |ui| {
                            field_row_opt!(ui, "制造商:", &mut system.manufacturer);
                            field_row_opt!(ui, "产品:", &mut system.product);
                            field_row_opt!(ui, "版本:", &mut system.version);
                            field_row_opt!(ui, "序列号:", &mut system.serial);
                            field_row_opt!(ui, "SKU:", &mut system.sku);
                            field_row_opt!(ui, "系列:", &mut system.family);
                        });
                    }
                });

                ui.collapsing("BIOS 信息", |ui| {
                    if smbios.bios.is_none() {
                        smbios.bios = Some(SMBIOSBios::default());
                    }
                    if let Some(ref mut bios) = smbios.bios {
                        grid(ui, "smbios_bios_grid", 2, |ui| {
                            field_row_opt!(ui, "供应商:", &mut bios.vendor);
                            field_row_opt!(ui, "版本:", &mut bios.version);
                            field_row_opt!(ui, "日期:", &mut bios.date);
                        });
                    }
                });

                ui.collapsing("主板信息", |ui| {
                    if smbios.base_board.is_none() {
                        smbios.base_board = Some(SMBIOSBaseBoard::default());
                    }
                    if let Some(ref mut board) = smbios.base_board {
                        grid(ui, "smbios_board_grid", 2, |ui| {
                            field_row_opt!(ui, "制造商:", &mut board.manufacturer);
                            field_row_opt!(ui, "产品:", &mut board.product);
                            field_row_opt!(ui, "版本:", &mut board.version);
                            field_row_opt!(ui, "序列号:", &mut board.serial);
                            field_row_opt!(ui, "资产标签:", &mut board.asset);
                            field_row_opt!(ui, "位置:", &mut board.location);
                        });
                    }
                });
            }
        });
    }
}
