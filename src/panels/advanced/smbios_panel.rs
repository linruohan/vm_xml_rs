use crate::{
    model::{SMBIOSBaseBoard, SMBIOSBios, SMBIOSConfig, SMBIOSSystem, VMConfig},
    panels::utils::*,
};

/// SMBIOS 配置面板
pub struct SMBIOSPanel;

impl SMBIOSPanel {
    /// 显示 SMBIOS 配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        panel_header(ui, "🔬", "SMBIOS 系统信息");

        card_group(ui, "SMBIOS 配置", None, |ui| {
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
                            ui.label("制造商:");
                            let mut mfr = system.manufacturer.clone().unwrap_or_default();
                            ui.text_edit_singleline(&mut mfr);
                            system.manufacturer = if mfr.is_empty() { None } else { Some(mfr) };
                            ui.end_row();

                            ui.label("产品:");
                            let mut product = system.product.clone().unwrap_or_default();
                            ui.text_edit_singleline(&mut product);
                            system.product = if product.is_empty() { None } else { Some(product) };
                            ui.end_row();

                            ui.label("版本:");
                            let mut version = system.version.clone().unwrap_or_default();
                            ui.text_edit_singleline(&mut version);
                            system.version = if version.is_empty() { None } else { Some(version) };
                            ui.end_row();

                            ui.label("序列号:");
                            let mut serial = system.serial.clone().unwrap_or_default();
                            ui.text_edit_singleline(&mut serial);
                            system.serial = if serial.is_empty() { None } else { Some(serial) };
                            ui.end_row();

                            ui.label("SKU:");
                            let mut sku = system.sku.clone().unwrap_or_default();
                            ui.text_edit_singleline(&mut sku);
                            system.sku = if sku.is_empty() { None } else { Some(sku) };
                            ui.end_row();

                            ui.label("系列:");
                            let mut family = system.family.clone().unwrap_or_default();
                            ui.text_edit_singleline(&mut family);
                            system.family = if family.is_empty() { None } else { Some(family) };
                            ui.end_row();
                        });
                    }
                });

                ui.collapsing("BIOS 信息", |ui| {
                    if smbios.bios.is_none() {
                        smbios.bios = Some(SMBIOSBios::default());
                    }
                    if let Some(ref mut bios) = smbios.bios {
                        grid(ui, "smbios_bios_grid", 2, |ui| {
                            ui.label("供应商:");
                            let mut vendor = bios.vendor.clone().unwrap_or_default();
                            ui.text_edit_singleline(&mut vendor);
                            bios.vendor = if vendor.is_empty() { None } else { Some(vendor) };
                            ui.end_row();

                            ui.label("版本:");
                            let mut version = bios.version.clone().unwrap_or_default();
                            ui.text_edit_singleline(&mut version);
                            bios.version = if version.is_empty() { None } else { Some(version) };
                            ui.end_row();

                            ui.label("日期:");
                            let mut date = bios.date.clone().unwrap_or_default();
                            ui.text_edit_singleline(&mut date);
                            bios.date = if date.is_empty() { None } else { Some(date) };
                            ui.end_row();
                        });
                    }
                });

                ui.collapsing("主板信息", |ui| {
                    if smbios.base_board.is_none() {
                        smbios.base_board = Some(SMBIOSBaseBoard::default());
                    }
                    if let Some(ref mut board) = smbios.base_board {
                        grid(ui, "smbios_board_grid", 2, |ui| {
                            ui.label("制造商:");
                            let mut mfr = board.manufacturer.clone().unwrap_or_default();
                            ui.text_edit_singleline(&mut mfr);
                            board.manufacturer = if mfr.is_empty() { None } else { Some(mfr) };
                            ui.end_row();

                            ui.label("产品:");
                            let mut product = board.product.clone().unwrap_or_default();
                            ui.text_edit_singleline(&mut product);
                            board.product = if product.is_empty() { None } else { Some(product) };
                            ui.end_row();

                            ui.label("版本:");
                            let mut version = board.version.clone().unwrap_or_default();
                            ui.text_edit_singleline(&mut version);
                            board.version = if version.is_empty() { None } else { Some(version) };
                            ui.end_row();

                            ui.label("序列号:");
                            let mut serial = board.serial.clone().unwrap_or_default();
                            ui.text_edit_singleline(&mut serial);
                            board.serial = if serial.is_empty() { None } else { Some(serial) };
                            ui.end_row();

                            ui.label("资产标签:");
                            let mut asset = board.asset.clone().unwrap_or_default();
                            ui.text_edit_singleline(&mut asset);
                            board.asset = if asset.is_empty() { None } else { Some(asset) };
                            ui.end_row();

                            ui.label("位置:");
                            let mut location = board.location.clone().unwrap_or_default();
                            ui.text_edit_singleline(&mut location);
                            board.location =
                                if location.is_empty() { None } else { Some(location) };
                            ui.end_row();
                        });
                    }
                });
            }
        });
    }
}
