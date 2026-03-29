use crate::{
    model::{
        os::{BIOSConfig, NVRAMConfig},
        BootConfig, BootMenuConfig, LoaderConfig, VMConfig,
    },
    panels::utils::{ThemeColors, *},
};

pub struct OSPanel;

impl OSPanel {
    /// 显示操作系统引导配置面板
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

                    // 固件类型 (firmware 属性)
                    ui.label("固件类型:");
                    let firmware = os.firmware.get_or_insert_with(|| "".to_string());
                    egui::ComboBox::from_id_source("os_firmware")
                        .selected_text(firmware.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(firmware, "".to_string(), "(默认)");
                            ui.selectable_value(firmware, "bios".to_string(), "BIOS");
                            ui.selectable_value(firmware, "efi".to_string(), "UEFI");
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

                        ui.label("无状态:");
                        let stateless = loader.stateless.get_or_insert_with(|| "".to_string());
                        egui::ComboBox::from_id_source("loader_stateless")
                            .selected_text(stateless.as_str())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(stateless, "".to_string(), "否");
                                ui.selectable_value(stateless, "yes".to_string(), "是");
                            });
                        ui.end_row();
                    });
                }

                ui.add_space(5.0);

                // NVRAM 配置
                let mut has_nvram = os.nvram.is_some();
                if checkbox(ui, &mut has_nvram, "启用 NVRAM") {
                    if has_nvram {
                        os.nvram = Some(vec![NVRAMConfig {
                            template: Some("/usr/share/OVMF/OVMF_VARS.fd".to_string()),
                            template_format: None,
                            nvram_type: None,
                            source: None,
                            path: Some("/var/lib/libvirt/nvram/guest_VARS.fd".to_string()),
                        }]);
                    } else {
                        os.nvram = None;
                    }
                }

                if let Some(ref mut nvram_list) = os.nvram {
                    if let Some(ref mut nvram) = nvram_list.first_mut() {
                        ui.add_space(5.0);
                        grid(ui, "nvram_grid", 2, |ui| {
                            ui.label("NVRAM 路径:");
                            let path = nvram.path.get_or_insert_with(|| "".to_string());
                            ui.text_edit_singleline(path);
                            ui.end_row();

                            ui.label("模板路径:");
                            let template = nvram.template.get_or_insert_with(|| "".to_string());
                            ui.text_edit_singleline(template);
                            ui.end_row();
                        });
                    }
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
            let os = config.general.os.as_mut();

            if let Some(os) = os {
                // 使用 os.bootmenu 而不是 config.os_booting.boot_menu
                let mut has_boot_menu = os.bootmenu.is_some();
                if checkbox(ui, &mut has_boot_menu, "启用引导菜单") {
                    if has_boot_menu {
                        os.bootmenu =
                            Some(BootMenuConfig { enable: "yes".to_string(), timeout: Some(3000) });
                    } else {
                        os.bootmenu = None;
                    }
                }

                if let Some(ref mut boot_menu) = os.bootmenu {
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
            }
        });

        ui.add_space(8.0);

        card_group(ui, "BIOS 配置", None, colors, |ui| {
            let os = config.general.os.as_mut();

            if let Some(os) = os {
                let mut has_bios = os.bios.is_some();
                if checkbox(ui, &mut has_bios, "启用 BIOS 配置") {
                    if has_bios {
                        os.bios = Some(BIOSConfig {
                            useserial: Some("no".to_string()),
                            reboot_timeout: Some(-1),
                        });
                    } else {
                        os.bios = None;
                    }
                }

                if let Some(ref mut bios) = os.bios {
                    ui.add_space(5.0);
                    grid(ui, "bios_grid", 2, |ui| {
                        ui.label("使用串口:");
                        let useserial = bios.useserial.get_or_insert_with(|| "no".to_string());
                        egui::ComboBox::from_id_source("bios_useserial")
                            .selected_text(useserial.as_str())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(useserial, "yes".to_string(), "是");
                                ui.selectable_value(useserial, "no".to_string(), "否");
                            });
                        ui.end_row();

                        ui.label("重启超时:");
                        let timeout = bios.reboot_timeout.get_or_insert(-1);
                        ui.add(egui::Slider::new(timeout, -1..=65535).text("ms"));
                        ui.end_row();
                    });
                }
            }
        });

        ui.add_space(8.0);

        card_group(ui, "SMBIOS 模式", None, colors, |ui| {
            let os = config.general.os.as_mut();

            if let Some(os) = os {
                let mut has_smbios = os.smbios.is_some();
                if checkbox(ui, &mut has_smbios, "启用 SMBIOS 模式") {
                    if has_smbios {
                        os.smbios =
                            Some(crate::model::SMBIOSModeConfig { mode: "sysinfo".to_string() });
                    } else {
                        os.smbios = None;
                    }
                }

                if let Some(ref mut smbios) = os.smbios {
                    ui.add_space(5.0);
                    grid(ui, "smbios_grid", 2, |ui| {
                        ui.label("模式:");
                        egui::ComboBox::from_id_source("smbios_mode")
                            .selected_text(&smbios.mode)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut smbios.mode,
                                    "emulate".to_string(),
                                    "emulate",
                                );
                                ui.selectable_value(&mut smbios.mode, "host".to_string(), "host");
                                ui.selectable_value(
                                    &mut smbios.mode,
                                    "sysinfo".to_string(),
                                    "sysinfo",
                                );
                            });
                        ui.end_row();
                    });
                }
            }
        });

        ui.add_space(8.0);

        card_group(ui, "直接内核引导", None, colors, |ui| {
            let os = config.general.os.as_mut();

            if let Some(os) = os {
                let mut has_kernel = os.kernel.is_some();
                if checkbox(ui, &mut has_kernel, "启用直接内核引导") {
                    if has_kernel {
                        os.kernel = Some("/path/to/vmlinuz".to_string());
                        os.initrd = Some("/path/to/initrd.img".to_string());
                        os.cmdline = Some("console=ttyS0".to_string());
                    } else {
                        os.kernel = None;
                        os.initrd = None;
                        os.cmdline = None;
                    }
                }

                if os.kernel.is_some() {
                    ui.add_space(5.0);
                    grid(ui, "kernel_grid", 2, |ui| {
                        ui.label("内核路径:");
                        let kernel = os.kernel.as_mut().unwrap();
                        ui.text_edit_singleline(kernel);
                        ui.end_row();

                        ui.label("Initrd 路径:");
                        let initrd = os.initrd.get_or_insert_with(|| "".to_string());
                        ui.text_edit_singleline(initrd);
                        ui.end_row();

                        ui.label("内核命令行:");
                        let cmdline = os.cmdline.get_or_insert_with(|| "".to_string());
                        ui.text_edit_singleline(cmdline);
                        ui.end_row();

                        ui.label("Shim 路径:");
                        let shim = os.shim.get_or_insert_with(|| "".to_string());
                        ui.text_edit_singleline(shim);
                        ui.end_row();

                        ui.label("DTB 路径:");
                        let dtb = os.dtb.get_or_insert_with(|| "".to_string());
                        ui.text_edit_singleline(dtb);
                        ui.end_row();
                    });
                }
            }
        });

        ui.add_space(8.0);

        card_group(ui, "ACPI 表", None, colors, |ui| {
            // 注意：ACPIConfig 在 os.rs 中定义，需要确保模型中有此结构
            // 这里暂时跳过，等待模型更新
            ui.label("ACPI 表配置暂不支持");
        });
    }
}
