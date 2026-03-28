use egui::RichText;

use crate::model::{BootConfig, BootMenuConfig, LoaderConfig, VMConfig};

pub struct OSPanel;

impl OSPanel {
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.heading(RichText::new("💿 操作系统引导配置").size(18.0));
        ui.separator();
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label(RichText::new("固件配置").strong());
            ui.add_space(5.0);

            let os = config.general.os.as_mut();

            if let Some(os) = os {
                ui.horizontal(|ui| {
                    ui.label("架构:");
                    let mut arch = os.arch.clone().unwrap_or_default();
                    egui::ComboBox::from_id_source("os_arch").selected_text(&arch).show_ui(
                        ui,
                        |ui| {
                            ui.selectable_value(&mut arch, "x86_64".to_string(), "x86_64");
                            ui.selectable_value(&mut arch, "i686".to_string(), "i686");
                            ui.selectable_value(&mut arch, "aarch64".to_string(), "aarch64");
                            ui.selectable_value(&mut arch, "armv7l".to_string(), "armv7l");
                            ui.selectable_value(&mut arch, "ppc64".to_string(), "ppc64");
                            ui.selectable_value(&mut arch, "ppc64le".to_string(), "ppc64le");
                            ui.selectable_value(&mut arch, "s390x".to_string(), "s390x");
                        },
                    );
                    os.arch = Some(arch);
                });

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("机器类型:");
                    let mut machine = os.machine.clone().unwrap_or_default();
                    egui::ComboBox::from_id_source("os_machine").selected_text(&machine).show_ui(
                        ui,
                        |ui| {
                            ui.selectable_value(&mut machine, "q35".to_string(), "q35");
                            ui.selectable_value(&mut machine, "pc".to_string(), "pc (i440FX)");
                            ui.selectable_value(
                                &mut machine,
                                "pc-q35-8.0".to_string(),
                                "pc-q35-8.0",
                            );
                            ui.selectable_value(
                                &mut machine,
                                "pc-i440fx-8.0".to_string(),
                                "pc-i440fx-8.0",
                            );
                            ui.selectable_value(&mut machine, "virt".to_string(), "virt (ARM)");
                        },
                    );
                    os.machine = Some(machine);
                });

                ui.add_space(10.0);

                let mut has_loader = os.loader.is_some();
                if ui.checkbox(&mut has_loader, "使用 UEFI 固件").changed() {
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
                    egui::Grid::new("loader_grid").num_columns(2).spacing([10.0, 8.0]).show(
                        ui,
                        |ui| {
                            ui.label("固件路径:");
                            ui.text_edit_singleline(&mut loader.path);
                            ui.end_row();

                            ui.label("类型:");
                            let mut loader_type = loader.loader_type.clone().unwrap_or_default();
                            egui::ComboBox::from_id_source("loader_type")
                                .selected_text(&loader_type)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut loader_type,
                                        "pflash".to_string(),
                                        "pflash",
                                    );
                                    ui.selectable_value(&mut loader_type, "rom".to_string(), "rom");
                                });
                            loader.loader_type = Some(loader_type);
                            ui.end_row();

                            ui.label("只读:");
                            let mut readonly = loader.readonly.clone().unwrap_or_default();
                            egui::ComboBox::from_id_source("loader_readonly")
                                .selected_text(&readonly)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut readonly, "yes".to_string(), "是");
                                    ui.selectable_value(&mut readonly, "no".to_string(), "否");
                                });
                            loader.readonly = Some(readonly);
                            ui.end_row();

                            ui.label("安全启动:");
                            let mut secure = loader.secure.clone().unwrap_or_default();
                            egui::ComboBox::from_id_source("loader_secure")
                                .selected_text(&secure)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut secure, "yes".to_string(), "是");
                                    ui.selectable_value(&mut secure, "no".to_string(), "否");
                                });
                            loader.secure = Some(secure);
                            ui.end_row();
                        },
                    );
                }
            }
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label(RichText::new("引导设备").strong());
            ui.add_space(5.0);

            let os = config.general.os.as_mut();
            if let Some(os) = os {
                if os.boot.is_none() {
                    os.boot = Some(Vec::new());
                }

                if let Some(ref mut boot_list) = os.boot {
                    ui.horizontal(|ui| {
                        if ui.button("➕ 添加引导设备").clicked() {
                            boot_list.push(BootConfig { dev: "hd".to_string() });
                        }

                        if ui.button("⬆️ 上移").clicked() && boot_list.len() > 1 {
                            // 实现上移功能
                        }
                        if ui.button("⬇️ 下移").clicked() && boot_list.len() > 1 {
                            // 实现下移功能
                        }
                    });

                    let mut to_remove = None;
                    for (i, boot) in boot_list.iter_mut().enumerate() {
                        ui.push_id(i, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}. ", i + 1));
                                egui::ComboBox::from_id_source(format!("boot_dev_{}", i))
                                    .selected_text(&boot.dev)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut boot.dev,
                                            "hd".to_string(),
                                            "硬盘",
                                        );
                                        ui.selectable_value(
                                            &mut boot.dev,
                                            "cdrom".to_string(),
                                            "光驱",
                                        );
                                        ui.selectable_value(
                                            &mut boot.dev,
                                            "fd".to_string(),
                                            "软驱",
                                        );
                                        ui.selectable_value(
                                            &mut boot.dev,
                                            "network".to_string(),
                                            "网络",
                                        );
                                    });
                                if ui.button("🗑️").clicked() {
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

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label(RichText::new("引导菜单").strong());
            ui.add_space(5.0);

            let mut has_boot_menu = config.os_booting.boot_menu.is_some();
            if ui.checkbox(&mut has_boot_menu, "启用引导菜单").changed() {
                if has_boot_menu {
                    config.os_booting.boot_menu =
                        Some(BootMenuConfig { enable: "yes".to_string(), timeout: Some(3000) });
                } else {
                    config.os_booting.boot_menu = None;
                }
            }

            if let Some(ref mut boot_menu) = config.os_booting.boot_menu {
                ui.add_space(5.0);
                egui::Grid::new("boot_menu_grid").num_columns(2).spacing([10.0, 8.0]).show(
                    ui,
                    |ui| {
                        ui.label("启用:");
                        let mut enable = boot_menu.enable.clone();
                        egui::ComboBox::from_id_source("boot_menu_enable")
                            .selected_text(&enable)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut enable, "yes".to_string(), "是");
                                ui.selectable_value(&mut enable, "no".to_string(), "否");
                            });
                        boot_menu.enable = enable;
                        ui.end_row();

                        ui.label("超时时间:");
                        let mut timeout = boot_menu.timeout.unwrap_or(3000);
                        ui.add(egui::Slider::new(&mut timeout, 0..=30000).text("ms"));
                        boot_menu.timeout = Some(timeout);
                        ui.end_row();
                    },
                );
            }
        });
    }
}
