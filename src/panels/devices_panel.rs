use crate::{
    model::{
        DiskConfig, DiskDriver, DiskSource, DiskTarget, GraphicsConfig, InputConfig,
        InterfaceConfig, InterfaceModel, InterfaceSource, MacAddress, TPMBackend, TPMConfig,
        VMConfig, VideoConfig, VideoModel,
    },
    panels::utils::*,
};

pub struct DevicesPanel;

impl DevicesPanel {
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        panel_header(ui, "🔌", "设备配置");

        Self::show_graphics(ui, config);
        ui.add_space(8.0);
        Self::show_video(ui, config);
        ui.add_space(8.0);
        Self::show_disks(ui, config);
        ui.add_space(8.0);
        Self::show_network(ui, config);
        ui.add_space(8.0);
        Self::show_input(ui, config);
        ui.add_space(8.0);
        Self::show_tpm(ui, config);
    }

    fn show_graphics(ui: &mut egui::Ui, config: &mut VMConfig) {
        card_group(ui, "图形显示", Some("🖥"), |ui| {
            let mut has_graphics = config.devices.graphics.is_some();
            if checkbox(ui, &mut has_graphics, "启用图形显示") {
                if has_graphics {
                    config.devices.graphics = Some(vec![GraphicsConfig {
                        graphics_type: "vnc".to_string(),
                        port: None,
                        autoport: Some("yes".to_string()),
                        listen: Some("127.0.0.1".to_string()),
                        listen_type: None,
                    }]);
                } else {
                    config.devices.graphics = None;
                }
            }

            if let Some(ref mut graphics_list) = config.devices.graphics {
                for (i, g) in graphics_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        ui.group(|ui| {
                            ui.label(format!("图形设备 {}", i + 1));
                            grid(ui, format!("graphics_grid_{}", i), 2, |ui| {
                                ui.label("类型:");
                                egui::ComboBox::from_id_source(format!("graphics_type_{}", i))
                                    .selected_text(&g.graphics_type)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut g.graphics_type,
                                            "vnc".to_string(),
                                            "VNC",
                                        );
                                        ui.selectable_value(
                                            &mut g.graphics_type,
                                            "spice".to_string(),
                                            "SPICE",
                                        );
                                        ui.selectable_value(
                                            &mut g.graphics_type,
                                            "sdl".to_string(),
                                            "SDL",
                                        );
                                        ui.selectable_value(
                                            &mut g.graphics_type,
                                            "headless".to_string(),
                                            "Headless",
                                        );
                                    });
                                ui.end_row();

                                ui.label("自动端口:");
                                let mut autoport = g.autoport.clone().unwrap_or_default();
                                egui::ComboBox::from_id_source(format!("autoport_{}", i))
                                    .selected_text(&autoport)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut autoport, "yes".to_string(), "是");
                                        ui.selectable_value(&mut autoport, "no".to_string(), "否");
                                    });
                                g.autoport = Some(autoport);
                                ui.end_row();

                                ui.label("监听地址:");
                                let listen =
                                    g.listen.get_or_insert_with(|| "127.0.0.1".to_string());
                                ui.text_edit_singleline(listen);
                                ui.end_row();
                            });
                        });
                        ui.add_space(5.0);
                    });
                }
            }
        });
    }

    fn show_video(ui: &mut egui::Ui, config: &mut VMConfig) {
        card_group(ui, "视频设备", Some("📺"), |ui| {
            let mut has_video = config.devices.video.is_some();
            if checkbox(ui, &mut has_video, "启用视频设备") {
                if has_video {
                    config.devices.video = Some(vec![VideoConfig {
                        video_type: None,
                        model: VideoModel {
                            model_type: "qxl".to_string(),
                            vram: Some(65536),
                            heads: Some(1),
                            primary: Some("yes".to_string()),
                        },
                    }]);
                } else {
                    config.devices.video = None;
                }
            }

            if let Some(ref mut video_list) = config.devices.video {
                for (i, v) in video_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        ui.group(|ui| {
                            ui.label(format!("视频设备 {}", i + 1));
                            grid(ui, format!("video_grid_{}", i), 2, |ui| {
                                ui.label("视频模型:");
                                egui::ComboBox::from_id_source(format!("video_model_{}", i))
                                    .selected_text(&v.model.model_type)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut v.model.model_type,
                                            "qxl".to_string(),
                                            "QXL",
                                        );
                                        ui.selectable_value(
                                            &mut v.model.model_type,
                                            "virtio".to_string(),
                                            "Virtio",
                                        );
                                        ui.selectable_value(
                                            &mut v.model.model_type,
                                            "vga".to_string(),
                                            "VGA",
                                        );
                                        ui.selectable_value(
                                            &mut v.model.model_type,
                                            "cirrus".to_string(),
                                            "Cirrus",
                                        );
                                        ui.selectable_value(
                                            &mut v.model.model_type,
                                            "vmvga".to_string(),
                                            "VMVGA",
                                        );
                                    });
                                ui.end_row();

                                ui.label("VRAM (KB):");
                                let mut vram = v.model.vram.unwrap_or(65536);
                                ui.add(egui::Slider::new(&mut vram, 4096..=262144).text("KB"));
                                v.model.vram = Some(vram);
                                ui.end_row();

                                ui.label("显示器数量:");
                                let mut heads = v.model.heads.unwrap_or(1);
                                ui.add(egui::Slider::new(&mut heads, 1..=4));
                                v.model.heads = Some(heads);
                                ui.end_row();
                            });
                        });
                        ui.add_space(5.0);
                    });
                }
            }
        });
    }

    fn show_disks(ui: &mut egui::Ui, config: &mut VMConfig) {
        card_group(ui, "磁盘设备", Some("💽"), |ui| {
            if config.devices.disk.is_none() {
                config.devices.disk = Some(Vec::new());
            }

            if let Some(ref mut disk_list) = config.devices.disk {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加磁盘") {
                        disk_list.push(Self::create_default_disk(disk_list.len()));
                    }
                });

                let mut to_remove = None;
                for (i, disk) in disk_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        egui::Frame::group(ui.style()).inner_margin(egui::Margin::same(8.0)).show(
                            ui,
                            |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(format!("**磁盘 {}**", i + 1));
                                    if delete_button(ui, None) {
                                        to_remove = Some(i);
                                    }
                                });

                                grid(ui, format!("disk_grid_{}", i), 2, |ui| {
                                    ui.label("磁盘类型:");
                                    egui::ComboBox::from_id_source(format!("disk_type_{}", i))
                                        .selected_text(&disk.disk_type)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut disk.disk_type,
                                                "file".to_string(),
                                                "file",
                                            );
                                            ui.selectable_value(
                                                &mut disk.disk_type,
                                                "block".to_string(),
                                                "block",
                                            );
                                            ui.selectable_value(
                                                &mut disk.disk_type,
                                                "dir".to_string(),
                                                "dir",
                                            );
                                            ui.selectable_value(
                                                &mut disk.disk_type,
                                                "network".to_string(),
                                                "network",
                                            );
                                        });
                                    ui.end_row();

                                    ui.label("设备类型:");
                                    egui::ComboBox::from_id_source(format!("device_type_{}", i))
                                        .selected_text(&disk.device)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut disk.device,
                                                "disk".to_string(),
                                                "disk",
                                            );
                                            ui.selectable_value(
                                                &mut disk.device,
                                                "cdrom".to_string(),
                                                "cdrom",
                                            );
                                            ui.selectable_value(
                                                &mut disk.device,
                                                "floppy".to_string(),
                                                "floppy",
                                            );
                                            ui.selectable_value(
                                                &mut disk.device,
                                                "lun".to_string(),
                                                "lun",
                                            );
                                        });
                                    ui.end_row();

                                    if let Some(ref mut driver) = disk.driver {
                                        ui.label("驱动格式:");
                                        egui::ComboBox::from_id_source(format!(
                                            "driver_type_{}",
                                            i
                                        ))
                                        .selected_text(&driver.driver_type)
                                        .show_ui(
                                            ui,
                                            |ui| {
                                                ui.selectable_value(
                                                    &mut driver.driver_type,
                                                    "qcow2".to_string(),
                                                    "qcow2",
                                                );
                                                ui.selectable_value(
                                                    &mut driver.driver_type,
                                                    "raw".to_string(),
                                                    "raw",
                                                );
                                                ui.selectable_value(
                                                    &mut driver.driver_type,
                                                    "vmdk".to_string(),
                                                    "vmdk",
                                                );
                                                ui.selectable_value(
                                                    &mut driver.driver_type,
                                                    "vdi".to_string(),
                                                    "vdi",
                                                );
                                            },
                                        );
                                        ui.end_row();

                                        ui.label("缓存模式:");
                                        let cache =
                                            driver.cache.get_or_insert_with(|| "none".to_string());
                                        egui::ComboBox::from_id_source(format!("cache_{}", i))
                                            .selected_text(cache.as_str())
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(
                                                    cache,
                                                    "none".to_string(),
                                                    "none",
                                                );
                                                ui.selectable_value(
                                                    cache,
                                                    "writethrough".to_string(),
                                                    "writethrough",
                                                );
                                                ui.selectable_value(
                                                    cache,
                                                    "writeback".to_string(),
                                                    "writeback",
                                                );
                                                ui.selectable_value(
                                                    cache,
                                                    "directsync".to_string(),
                                                    "directsync",
                                                );
                                                ui.selectable_value(
                                                    cache,
                                                    "unsafe".to_string(),
                                                    "unsafe",
                                                );
                                            });
                                        ui.end_row();
                                    }

                                    if let Some(ref mut source) = disk.source {
                                        ui.label("文件路径:");
                                        let file = source.file.get_or_insert_with(|| {
                                            "/var/lib/libvirt/images/disk.qcow2".to_string()
                                        });
                                        ui.text_edit_singleline(file);
                                        ui.end_row();
                                    }

                                    if let Some(ref mut target) = disk.target {
                                        ui.label("目标设备:");
                                        ui.text_edit_singleline(&mut target.dev);
                                        ui.end_row();

                                        ui.label("总线类型:");
                                        let bus =
                                            target.bus.get_or_insert_with(|| "virtio".to_string());
                                        egui::ComboBox::from_id_source(format!("bus_{}", i))
                                            .selected_text(bus.as_str())
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(
                                                    bus,
                                                    "virtio".to_string(),
                                                    "virtio",
                                                );
                                                ui.selectable_value(
                                                    bus,
                                                    "sata".to_string(),
                                                    "sata",
                                                );
                                                ui.selectable_value(
                                                    bus,
                                                    "scsi".to_string(),
                                                    "scsi",
                                                );
                                                ui.selectable_value(bus, "ide".to_string(), "ide");
                                                ui.selectable_value(bus, "usb".to_string(), "usb");
                                            });
                                        ui.end_row();
                                    }

                                    let mut readonly = disk.readonly.is_some();
                                    if checkbox(ui, &mut readonly, "只读") {
                                        disk.readonly = if readonly { Some(()) } else { None };
                                    }
                                });
                            },
                        );
                        ui.add_space(5.0);
                    });
                }

                if let Some(idx) = to_remove {
                    disk_list.remove(idx);
                }
            }
        });
    }

    fn show_network(ui: &mut egui::Ui, config: &mut VMConfig) {
        card_group(ui, "网络接口", Some("🌐"), |ui| {
            if config.devices.interface.is_none() {
                config.devices.interface = Some(Vec::new());
            }

            if let Some(ref mut iface_list) = config.devices.interface {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加网络接口") {
                        iface_list.push(Self::create_default_interface());
                    }
                });

                let mut to_remove = None;
                for (i, iface) in iface_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        egui::Frame::group(ui.style()).inner_margin(egui::Margin::same(8.0)).show(
                            ui,
                            |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(format!("**接口 {}**", i + 1));
                                    if delete_button(ui, None) {
                                        to_remove = Some(i);
                                    }
                                });

                                grid(ui, format!("iface_grid_{}", i), 2, |ui| {
                                    ui.label("接口类型:");
                                    egui::ComboBox::from_id_source(format!("iface_type_{}", i))
                                        .selected_text(&iface.interface_type)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut iface.interface_type,
                                                "bridge".to_string(),
                                                "bridge",
                                            );
                                            ui.selectable_value(
                                                &mut iface.interface_type,
                                                "network".to_string(),
                                                "network",
                                            );
                                            ui.selectable_value(
                                                &mut iface.interface_type,
                                                "ethernet".to_string(),
                                                "ethernet",
                                            );
                                            ui.selectable_value(
                                                &mut iface.interface_type,
                                                "direct".to_string(),
                                                "direct",
                                            );
                                        });
                                    ui.end_row();

                                    if let Some(ref mut mac) = iface.mac {
                                        ui.label("MAC 地址:");
                                        ui.text_edit_singleline(&mut mac.address);
                                        ui.end_row();
                                    }

                                    if let Some(ref mut source) = iface.source {
                                        match iface.interface_type.as_str() {
                                            "bridge" => {
                                                if let Some(ref mut bridge) = source.bridge {
                                                    ui.label("网桥:");
                                                    ui.text_edit_singleline(bridge);
                                                    ui.end_row();
                                                }
                                            },
                                            "network" => {
                                                if let Some(ref mut network) = source.network {
                                                    ui.label("网络:");
                                                    ui.text_edit_singleline(network);
                                                    ui.end_row();
                                                }
                                            },
                                            _ => {},
                                        }
                                    }

                                    if let Some(ref mut model) = iface.model {
                                        ui.label("网卡模型:");
                                        egui::ComboBox::from_id_source(format!("nic_model_{}", i))
                                            .selected_text(&model.model_type)
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(
                                                    &mut model.model_type,
                                                    "virtio".to_string(),
                                                    "virtio",
                                                );
                                                ui.selectable_value(
                                                    &mut model.model_type,
                                                    "e1000".to_string(),
                                                    "e1000",
                                                );
                                                ui.selectable_value(
                                                    &mut model.model_type,
                                                    "rtl8139".to_string(),
                                                    "rtl8139",
                                                );
                                                ui.selectable_value(
                                                    &mut model.model_type,
                                                    "ne2k_pci".to_string(),
                                                    "ne2k_pci",
                                                );
                                            });
                                        ui.end_row();
                                    }
                                });
                            },
                        );
                        ui.add_space(5.0);
                    });
                }

                if let Some(idx) = to_remove {
                    iface_list.remove(idx);
                }
            }
        });
    }

    fn show_input(ui: &mut egui::Ui, config: &mut VMConfig) {
        card_group(ui, "输入设备", Some("⌨"), |ui| {
            if config.devices.input.is_none() {
                config.devices.input = Some(Vec::new());
            }

            if let Some(ref mut input_list) = config.devices.input {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加输入设备") {
                        input_list.push(InputConfig {
                            input_type: "tablet".to_string(),
                            bus: Some("usb".to_string()),
                        });
                    }
                });

                let mut to_remove = None;
                for (i, input) in input_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!("输入设备 {}", i + 1));
                            if delete_button(ui, None) {
                                to_remove = Some(i);
                            }
                        });

                        grid(ui, format!("input_grid_{}", i), 2, |ui| {
                            ui.label("设备类型:");
                            egui::ComboBox::from_id_source(format!("input_type_{}", i))
                                .selected_text(&input.input_type)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut input.input_type,
                                        "tablet".to_string(),
                                        "tablet",
                                    );
                                    ui.selectable_value(
                                        &mut input.input_type,
                                        "mouse".to_string(),
                                        "mouse",
                                    );
                                    ui.selectable_value(
                                        &mut input.input_type,
                                        "keyboard".to_string(),
                                        "keyboard",
                                    );
                                    ui.selectable_value(
                                        &mut input.input_type,
                                        "passthrough".to_string(),
                                        "passthrough",
                                    );
                                });
                            ui.end_row();

                            ui.label("总线:");
                            let bus = input.bus.get_or_insert_with(|| "usb".to_string());
                            egui::ComboBox::from_id_source(format!("input_bus_{}", i))
                                .selected_text(bus.as_str())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(bus, "usb".to_string(), "USB");
                                    ui.selectable_value(bus, "ps2".to_string(), "PS/2");
                                    ui.selectable_value(bus, "virtio".to_string(), "Virtio");
                                });
                            ui.end_row();
                        });
                    });
                }

                if let Some(idx) = to_remove {
                    input_list.remove(idx);
                }
            }
        });
    }

    fn show_tpm(ui: &mut egui::Ui, config: &mut VMConfig) {
        card_group(ui, "TPM 设备", Some("🔒"), |ui| {
            let mut has_tpm = config.devices.tpm.is_some();
            if checkbox(ui, &mut has_tpm, "启用 TPM") {
                if has_tpm {
                    config.devices.tpm = Some(TPMConfig {
                        model: "tpm-tis".to_string(),
                        backend: TPMBackend {
                            backend_type: "emulator".to_string(),
                            version: Some("2.0".to_string()),
                        },
                    });
                } else {
                    config.devices.tpm = None;
                }
            }

            if let Some(ref mut tpm) = config.devices.tpm {
                grid(ui, "tpm_grid", 2, |ui| {
                    ui.label("TPM 模型:");
                    egui::ComboBox::from_id_source("tpm_model").selected_text(&tpm.model).show_ui(
                        ui,
                        |ui| {
                            ui.selectable_value(&mut tpm.model, "tpm-tis".to_string(), "tpm-tis");
                            ui.selectable_value(&mut tpm.model, "tpm-crb".to_string(), "tpm-crb");
                        },
                    );
                    ui.end_row();

                    ui.label("后端类型:");
                    egui::ComboBox::from_id_source("tpm_backend_type")
                        .selected_text(&tpm.backend.backend_type)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut tpm.backend.backend_type,
                                "emulator".to_string(),
                                "emulator",
                            );
                            ui.selectable_value(
                                &mut tpm.backend.backend_type,
                                "passthrough".to_string(),
                                "passthrough",
                            );
                        });
                    ui.end_row();

                    ui.label("TPM 版本:");
                    let version = tpm.backend.version.get_or_insert_with(|| "2.0".to_string());
                    egui::ComboBox::from_id_source("tpm_version")
                        .selected_text(version.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(version, "1.2".to_string(), "1.2");
                            ui.selectable_value(version, "2.0".to_string(), "2.0");
                        });
                    ui.end_row();
                });
            }
        });
    }

    fn create_default_disk(index: usize) -> DiskConfig {
        DiskConfig {
            disk_type: "file".to_string(),
            device: "disk".to_string(),
            driver: Some(DiskDriver {
                name: "qemu".to_string(),
                driver_type: "qcow2".to_string(),
                cache: Some("none".to_string()),
                io: None,
                ioeventfd: None,
                event_idx: None,
                queues: None,
                queue_size: None,
            }),
            source: Some(DiskSource {
                file: Some("/var/lib/libvirt/images/disk.qcow2".to_string()),
                dev: None,
                protocol: None,
                name: None,
                startup_policy: None,
                host: None,
                auth: None,
                seclabel: None,
            }),
            target: Some(DiskTarget {
                dev: format!("vd{}", (b'a' + index as u8) as char),
                bus: Some("virtio".to_string()),
                tray: None,
                rotation_rate: None,
            }),
            readonly: None,
            geometry: None,
            blockio: None,
            iotune: None,
            backenddomain: None,
            throttlefilters: None,
            address: None,
            snapshot: None,
            alias: None,
            boot: None,
            shareable: None,
            transient: None,
            encryption: None,
            serial: None,
            wwn: None,
            vendor: None,
        }
    }

    fn create_default_interface() -> InterfaceConfig {
        InterfaceConfig {
            interface_type: "bridge".to_string(),
            trust_guest_rx_filters: None,
            mac: Some(MacAddress { address: Self::generate_mac() }),
            source: Some(InterfaceSource {
                bridge: Some("virbr0".to_string()),
                network: None,
                dev: None,
                mode: None,
            }),
            model: Some(InterfaceModel { model_type: "virtio".to_string() }),
            alias: None,
            boot: None,
            address: None,
        }
    }

    fn generate_mac() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        format!(
            "52:54:00:{:02x}:{:02x}:{:02x}",
            (timestamp >> 16) as u8,
            (timestamp >> 8) as u8,
            timestamp as u8
        )
    }
}
