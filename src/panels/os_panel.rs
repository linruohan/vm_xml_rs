use crate::{
    model::{BootConfig, BootMenuConfig, LoaderConfig, VMConfig},
    panels::utils::*,
};

pub struct OSPanel;

impl OSPanel {
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "💿", "操作系统引导配置");

        card_group(ui, "固件配置", None, colors, |ui| {
            let os = config.general.os.as_mut();

            if let Some(os) = os {
                grid(ui, "firmware_grid", 2, |ui| {
                    // 架构
                    ui.label("架构:");
                    let arch = os.arch.get_or_insert_with(|| "x86_64".to_string());
                    egui::ComboBox::from_id_source("os_arch").selected_text(arch.as_str()).show_ui(
                        ui,
                        |ui| {
                            ui.selectable_value(arch, "x86_64".to_string(), "x86_64");
                            ui.selectable_value(arch, "i686".to_string(), "i686");
                            ui.selectable_value(arch, "aarch64".to_string(), "aarch64");
                            ui.selectable_value(arch, "armv7l".to_string(), "armv7l");
                            ui.selectable_value(arch, "ppc64".to_string(), "ppc64");
                            ui.selectable_value(arch, "ppc64le".to_string(), "ppc64le");
                            ui.selectable_value(arch, "s390x".to_string(), "s390x");
                        },
                    );
                    ui.end_row();

                    // 机器类型
                    ui.label("机器类型:");
                    let machine = os.machine.get_or_insert_with(|| "q35".to_string());
                    egui::ComboBox::from_id_source("os_machine")
                        .selected_text(machine.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(machine, "q35".to_string(), "q35");
                            ui.selectable_value(machine, "pc".to_string(), "pc (i440FX)");
                            ui.selectable_value(machine, "pc-q35-8.0".to_string(), "pc-q35-8.0");
                            ui.selectable_value(
                                machine,
                                "pc-i440fx-8.0".to_string(),
                                "pc-i440fx-8.0",
                            );
                            ui.selectable_value(machine, "virt".to_string(), "virt (ARM)");
                        });
                    ui.end_row();
                });

                ui.add_space(5.0);

                // UEFI 固件配置
                let mut has_loader = os.loader.is_some();
                if checkbox(ui, &mut has_loader, "使用 UEFI 固件") {
                    if has_loader {
                        os.loader = Some(LoaderConfig {
                            readonly: Some("yes".to_string()),
                            loader_type: Some("pflash".to_string()),
                            secure: Some("no".to_string()),
                            stateless: None,
                            format: None,
                            path: "/usr/share/OVMF/OVMF_CODE.fd".to_string(),
                        });
                    } else {
                        os.loader = None;
                    }
                }

                if let Some(ref mut loader) = os.loader {
                    ui.add_space(5.0);
                    grid(ui, "loader_grid", 2, |ui| {
                        ui.label("固件路径:");
                        ui.text_edit_singleline(&mut loader.path);
                        ui.end_row();

                        ui.label("类型:");
                        let loader_type =
                            loader.loader_type.get_or_insert_with(|| "pflash".to_string());
                        egui::ComboBox::from_id_source("loader_type")
                            .selected_text(loader_type.as_str())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(loader_type, "pflash".to_string(), "pflash");
                                ui.selectable_value(loader_type, "rom".to_string(), "rom");
                            });
                        ui.end_row();

                        ui.label("只读:");
                        let readonly = loader.readonly.get_or_insert_with(|| "yes".to_string());
                        egui::ComboBox::from_id_source("loader_readonly")
                            .selected_text(readonly.as_str())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(readonly, "yes".to_string(), "是");
                                ui.selectable_value(readonly, "no".to_string(), "否");
                            });
                        ui.end_row();

                        ui.label("安全启动:");
                        let secure = loader.secure.get_or_insert_with(|| "no".to_string());
                        egui::ComboBox::from_id_source("loader_secure")
                            .selected_text(secure.as_str())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(secure, "yes".to_string(), "是");
                                ui.selectable_value(secure, "no".to_string(), "否");
                            });
                        ui.end_row();
                    });
                }
            }
        });

        ui.add_space(8.0);

        card_group(ui, "引导设备", None, colors, |ui| {
            let os = config.general.os.as_mut();
            if let Some(os) = os {
                if os.boot.is_none() {
                    os.boot = Some(Vec::new());
                }

                if let Some(ref mut boot_list) = os.boot {
                    ui.horizontal(|ui| {
                        if add_button(ui, "➕ 添加引导设备", colors) {
                            boot_list.push(BootConfig { dev: "hd".to_string() });
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if boot_list.len() > 1 && ui.button("⬇️ 下移").clicked() {
                                // TODO: 实现下移功能
                            }
                            if boot_list.len() > 1 && ui.button("⬆️ 上移").clicked() {
                                // TODO: 实现上移功能
                            }
                        });
                    });

                    let mut to_remove = None;
                    for (i, boot) in boot_list.iter_mut().enumerate() {
                        ui.push_id(i, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}. ", i + 1));
                                let dev = &mut boot.dev;
                                egui::ComboBox::from_id_source(format!("boot_dev_{}", i))
                                    .selected_text(dev.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(dev, "hd".to_string(), "硬盘");
                                        ui.selectable_value(dev, "cdrom".to_string(), "光驱");
                                        ui.selectable_value(dev, "fd".to_string(), "软驱");
                                        ui.selectable_value(dev, "network".to_string(), "网络");
                                    });
                                if delete_button(ui, None) {
                                    to_remove = Some(i);
                                }
                            });
                        });
                    }

                    if let Some(idx) = to_remove {
                        boot_list.remove(idx);
                    }
                }
            }
        });

        ui.add_space(8.0);

        card_group(ui, "引导菜单", None, colors, |ui| {
            let mut has_boot_menu = config.os_booting.boot_menu.is_some();
            if checkbox(ui, &mut has_boot_menu, "启用引导菜单") {
                if has_boot_menu {
                    config.os_booting.boot_menu =
                        Some(BootMenuConfig { enable: "yes".to_string(), timeout: Some(3000) });
                } else {
                    config.os_booting.boot_menu = None;
                }
            }

            if let Some(ref mut boot_menu) = config.os_booting.boot_menu {
                ui.add_space(5.0);
                grid(ui, "boot_menu_grid", 2, |ui| {
                    ui.label("启用:");
                    let enable = &mut boot_menu.enable;
                    egui::ComboBox::from_id_source("boot_menu_enable")
                        .selected_text(enable.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(enable, "yes".to_string(), "是");
                            ui.selectable_value(enable, "no".to_string(), "否");
                        });
                    ui.end_row();

                    ui.label("超时时间:");
                    let mut timeout = boot_menu.timeout.unwrap_or(3000);
                    ui.add(egui::Slider::new(&mut timeout, 0..=30000).text("ms"));
                    boot_menu.timeout = Some(timeout);
                    ui.end_row();
                });
            }
        });
    }
}
