use crate::{
    model::{
        input_sound_tpm::RngBackend,
        serial_console::{ParallelTarget, SerialTarget},
        AudioConfig, ConsoleConfig, CryptoConfig, DiskConfig, DiskDriver, DiskSource, DiskTarget,
        GraphicsConfig, HostdevConfig, HubConfig, InputConfig, InterfaceConfig, InterfaceModel,
        InterfaceSource, IommuConfig, MacAddress, MemoryDeviceConfig, PanicConfig, ParallelConfig,
        PstoreConfig, RngConfig, SerialConfig, ShmemConfig, SoundConfig, TPMBackend, TPMConfig,
        VMConfig, VideoConfig, VideoModel, VsockConfig, WatchdogConfig,
    },
    panels::utils::*,
};

pub struct DevicesPanel;

impl DevicesPanel {
    /// 显示设备配置面板
    /// 使用流式布局，卡片自动换行排列，左对齐
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "🔌", "设备配置");

        // 卡片宽度和间距配置
        let card_width = 320.0;
        let spacing = 8.0;

        // 使用 horizontal_wrapped 实现流式布局，左对齐
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP).with_main_wrap(true), |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);

            // 图形显示卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_graphics(ui, config, colors);
                },
            );

            // 视频设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_video(ui, config, colors);
                },
            );

            // 磁盘设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_disks(ui, config, colors);
                },
            );

            // 网络接口卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_network(ui, config, colors);
                },
            );

            // 输入设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_input(ui, config, colors);
                },
            );

            // TPM 设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_tpm(ui, config, colors);
                },
            );

            // 串口设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_serial(ui, config, colors);
                },
            );

            // 并口设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_parallel(ui, config, colors);
                },
            );

            // 控制台卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_console(ui, config, colors);
                },
            );

            // 声音设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_sound(ui, config, colors);
                },
            );

            // RNG 设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_rng(ui, config, colors);
                },
            );

            // Watchdog 卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_watchdog(ui, config, colors);
                },
            );

            // Hub 设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_hub(ui, config, colors);
                },
            );

            // Panic 设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_panic(ui, config, colors);
                },
            );

            // Shmem 共享内存卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_shmem(ui, config, colors);
                },
            );

            // Memory Device 内存设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_memory_device(ui, config, colors);
                },
            );

            // IOMMU 设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_iommu(ui, config, colors);
                },
            );

            // Vsock 设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_vsock(ui, config, colors);
                },
            );

            // Crypto 设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_crypto(ui, config, colors);
                },
            );

            // Pstore 设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_pstore(ui, config, colors);
                },
            );

            // Audio 设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_audio(ui, config, colors);
                },
            );

            // Hostdev 主机设备直通卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_hostdev(ui, config, colors);
                },
            );
        });
    }

    fn show_graphics(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "图形显示", Some("🖥"), colors, |ui| {
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
                        inner_group(ui, colors, |ui| {
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

    fn show_video(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "视频设备", Some("📺"), colors, |ui| {
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
                        inner_group(ui, colors, |ui| {
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

    fn show_disks(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "磁盘设备", Some("💽"), colors, |ui| {
            if config.devices.disk.is_none() {
                config.devices.disk = Some(Vec::new());
            }

            if let Some(ref mut disk_list) = config.devices.disk {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加磁盘", colors) {
                        disk_list.push(Self::create_default_disk(disk_list.len()));
                    }
                });

                let mut to_remove = None;
                for (i, disk) in disk_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        egui::Frame::group(ui.style())
                            .fill(colors.card_background)
                            .inner_margin(egui::Margin::same(8.0))
                            .show(ui, |ui| {
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
                            });
                        ui.add_space(5.0);
                    });
                }

                if let Some(idx) = to_remove {
                    disk_list.remove(idx);
                }
            }
        });
    }

    fn show_network(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "网络接口", Some("🌐"), colors, |ui| {
            if config.devices.interface.is_none() {
                config.devices.interface = Some(Vec::new());
            }

            if let Some(ref mut iface_list) = config.devices.interface {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加网络接口", colors) {
                        iface_list.push(Self::create_default_interface());
                    }
                });

                let mut to_remove = None;
                for (i, iface) in iface_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        egui::Frame::group(ui.style())
                            .fill(colors.card_background)
                            .inner_margin(egui::Margin::same(8.0))
                            .show(ui, |ui| {
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
                            });
                        ui.add_space(5.0);
                    });
                }

                if let Some(idx) = to_remove {
                    iface_list.remove(idx);
                }
            }
        });
    }

    fn show_input(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "输入设备", Some("⌨"), colors, |ui| {
            if config.devices.input.is_none() {
                config.devices.input = Some(Vec::new());
            }

            if let Some(ref mut input_list) = config.devices.input {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加输入设备", colors) {
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

    fn show_tpm(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "TPM 设备", Some("🔒"), colors, |ui| {
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

    fn show_serial(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "串行端口", Some("🔌"), colors, |ui| {
            if config.devices.serial.is_none() {
                config.devices.serial = Some(Vec::new());
            }

            if let Some(ref mut serial_list) = config.devices.serial {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加串口", colors) {
                        serial_list.push(SerialConfig {
                            serial_type: "pty".to_string(),
                            port: None,
                            target: None,
                        });
                    }
                });

                let to_remove = None;
                for (i, serial) in serial_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        inner_group(ui, colors, |ui| {
                            ui.label(format!("串口 {}", i + 1));
                            grid(ui, format!("serial_grid_{}", i), 2, |ui| {
                                ui.label("类型:");
                                egui::ComboBox::from_id_source(format!("serial_type_{}", i))
                                    .selected_text(&serial.serial_type)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut serial.serial_type,
                                            "pty".to_string(),
                                            "pty (伪终端)",
                                        );
                                        ui.selectable_value(
                                            &mut serial.serial_type,
                                            "file".to_string(),
                                            "file",
                                        );
                                        ui.selectable_value(
                                            &mut serial.serial_type,
                                            "dev".to_string(),
                                            "dev",
                                        );
                                        ui.selectable_value(
                                            &mut serial.serial_type,
                                            "tcp".to_string(),
                                            "tcp",
                                        );
                                        ui.selectable_value(
                                            &mut serial.serial_type,
                                            "udp".to_string(),
                                            "udp",
                                        );
                                        ui.selectable_value(
                                            &mut serial.serial_type,
                                            "spicevmc".to_string(),
                                            "spicevmc",
                                        );
                                    });
                                ui.end_row();

                                ui.label("端口号:");
                                let port = serial.port.get_or_insert(0);
                                ui.add(egui::Slider::new(port, 0..=4));
                                ui.end_row();

                                // Target 配置
                                ui.label("Target 类型:");
                                let target = serial.target.get_or_insert_with(|| SerialTarget {
                                    target_type: Some("pty".to_string()),
                                    port: None,
                                });
                                let target_type =
                                    target.target_type.get_or_insert_with(|| "pty".to_string());
                                egui::ComboBox::from_id_source(format!("serial_target_type_{}", i))
                                    .selected_text(target_type.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(target_type, "pty".to_string(), "pty");
                                        ui.selectable_value(
                                            target_type,
                                            "file".to_string(),
                                            "file",
                                        );
                                        ui.selectable_value(target_type, "dev".to_string(), "dev");
                                        ui.selectable_value(
                                            target_type,
                                            "unix".to_string(),
                                            "unix",
                                        );
                                        ui.selectable_value(target_type, "udp".to_string(), "udp");
                                        ui.selectable_value(target_type, "tcp".to_string(), "tcp");
                                        ui.selectable_value(
                                            target_type,
                                            "null".to_string(),
                                            "null",
                                        );
                                    });
                                ui.end_row();
                            });
                        });
                        ui.add_space(5.0);
                    });
                }

                if let Some(idx) = to_remove {
                    serial_list.remove(idx);
                }
            }
        });
    }

    fn show_parallel(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "并行端口", Some("🖨"), colors, |ui| {
            if config.devices.parallel.is_none() {
                config.devices.parallel = Some(Vec::new());
            }

            if let Some(ref mut parallel_list) = config.devices.parallel {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加并口", colors) {
                        parallel_list.push(ParallelConfig {
                            parallel_type: "dev".to_string(),
                            target: None,
                        });
                    }
                });

                let to_remove = None;
                for (i, parallel) in parallel_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        inner_group(ui, colors, |ui| {
                            ui.label(format!("并口 {}", i + 1));
                            grid(ui, format!("parallel_grid_{}", i), 2, |ui| {
                                ui.label("类型:");
                                egui::ComboBox::from_id_source(format!("parallel_type_{}", i))
                                    .selected_text(&parallel.parallel_type)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut parallel.parallel_type,
                                            "dev".to_string(),
                                            "dev",
                                        );
                                        ui.selectable_value(
                                            &mut parallel.parallel_type,
                                            "file".to_string(),
                                            "file",
                                        );
                                    });
                                ui.end_row();

                                // Target 配置
                                ui.label("端口地址:");
                                let target =
                                    parallel.target.get_or_insert(ParallelTarget { port: 0x378 });
                                ui.add(egui::Slider::new(&mut target.port, 0x378..=0x37F).text(""));
                                ui.end_row();
                            });
                        });
                        ui.add_space(5.0);
                    });
                }

                if let Some(idx) = to_remove {
                    parallel_list.remove(idx);
                }
            }
        });
    }

    fn show_console(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "控制台", Some("🖥"), colors, |ui| {
            let mut has_console = config.devices.console.is_some();
            if checkbox(ui, &mut has_console, "启用控制台") {
                if has_console {
                    config.devices.console =
                        Some(ConsoleConfig { console_type: "pty".to_string(), target: None });
                } else {
                    config.devices.console = None;
                }
            }

            if let Some(ref mut console) = config.devices.console {
                ui.add_space(5.0);
                grid(ui, "console_grid", 2, |ui| {
                    ui.label("类型:");
                    egui::ComboBox::from_id_source("console_type")
                        .selected_text(&console.console_type)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut console.console_type,
                                "pty".to_string(),
                                "pty (伪终端)",
                            );
                            ui.selectable_value(
                                &mut console.console_type,
                                "file".to_string(),
                                "file",
                            );
                            ui.selectable_value(
                                &mut console.console_type,
                                "dev".to_string(),
                                "dev",
                            );
                            ui.selectable_value(
                                &mut console.console_type,
                                "null".to_string(),
                                "null",
                            );
                        });
                    ui.end_row();
                });
            }
        });
    }

    fn show_sound(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "声音设备", Some("🔊"), colors, |ui| {
            if config.devices.sound.is_none() {
                config.devices.sound = Some(Vec::new());
            }

            if let Some(ref mut sound_list) = config.devices.sound {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加声音设备", colors) {
                        sound_list.push(SoundConfig { model: "ich6".to_string() });
                    }
                });

                let to_remove = None;
                for (i, sound) in sound_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        inner_group(ui, colors, |ui| {
                            ui.label(format!("声音设备 {}", i + 1));
                            grid(ui, format!("sound_grid_{}", i), 2, |ui| {
                                ui.label("模型:");
                                egui::ComboBox::from_id_source(format!("sound_model_{}", i))
                                    .selected_text(&sound.model)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut sound.model,
                                            "ich6".to_string(),
                                            "ich6",
                                        );
                                        ui.selectable_value(
                                            &mut sound.model,
                                            "ich9".to_string(),
                                            "ich9",
                                        );
                                        ui.selectable_value(
                                            &mut sound.model,
                                            "ac97".to_string(),
                                            "ac97",
                                        );
                                        ui.selectable_value(
                                            &mut sound.model,
                                            "sb16".to_string(),
                                            "sb16",
                                        );
                                        ui.selectable_value(
                                            &mut sound.model,
                                            "es1370".to_string(),
                                            "es1370",
                                        );
                                    });
                                ui.end_row();
                            });
                        });
                        ui.add_space(5.0);
                    });
                }

                if let Some(idx) = to_remove {
                    sound_list.remove(idx);
                }
            }
        });
    }

    fn show_rng(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "RNG 设备", Some("🎲"), colors, |ui| {
            let mut has_rng = config.devices.rng.is_some();
            if checkbox(ui, &mut has_rng, "启用 RNG (随机数生成器)") {
                if has_rng {
                    config.devices.rng = Some(vec![RngConfig {
                        backend: Some(RngBackend {
                            model: "virtio".to_string(),
                            rng_type: "random".to_string(),
                            device: "/dev/random".to_string(),
                        }),
                    }]);
                } else {
                    config.devices.rng = None;
                }
            }

            if let Some(ref mut rng_list) = config.devices.rng {
                for (i, rng) in rng_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        inner_group(ui, colors, |ui| {
                            ui.label(format!("RNG 设备 {}", i + 1));
                            if let Some(ref mut backend) = rng.backend {
                                grid(ui, format!("rng_grid_{}", i), 2, |ui| {
                                    ui.label("模型:");
                                    egui::ComboBox::from_id_source(format!("rng_model_{}", i))
                                        .selected_text(&backend.model)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut backend.model,
                                                "virtio".to_string(),
                                                "virtio",
                                            );
                                        });
                                    ui.end_row();

                                    ui.label("类型:");
                                    egui::ComboBox::from_id_source(format!(
                                        "rng_backend_type_{}",
                                        i
                                    ))
                                    .selected_text(&backend.rng_type)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut backend.rng_type,
                                            "random".to_string(),
                                            "random (/dev/random)",
                                        );
                                        ui.selectable_value(
                                            &mut backend.rng_type,
                                            "egd".to_string(),
                                            "egd",
                                        );
                                        ui.selectable_value(
                                            &mut backend.rng_type,
                                            "builtin".to_string(),
                                            "builtin (QEMU 内置)",
                                        );
                                    });
                                    ui.end_row();

                                    if backend.rng_type != "builtin" {
                                        ui.label("设备路径:");
                                        ui.text_edit_singleline(&mut backend.device);
                                        ui.end_row();
                                    }
                                });
                            }
                        });
                        ui.add_space(5.0);
                    });
                }
            }
        });
    }

    fn show_watchdog(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "Watchdog", Some("⏱"), colors, |ui| {
            let mut has_watchdog = config.devices.watchdog.is_some();
            if checkbox(ui, &mut has_watchdog, "启用 Watchdog (看门狗)") {
                if has_watchdog {
                    config.devices.watchdog = Some(WatchdogConfig {
                        model: "i6300esb".to_string(),
                        action: "reset".to_string(),
                    });
                } else {
                    config.devices.watchdog = None;
                }
            }

            if let Some(ref mut watchdog) = config.devices.watchdog {
                ui.add_space(5.0);
                grid(ui, "watchdog_grid", 2, |ui| {
                    ui.label("模型:");
                    egui::ComboBox::from_id_source("watchdog_model")
                        .selected_text(&watchdog.model)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut watchdog.model,
                                "i6300esb".to_string(),
                                "i6300esb",
                            );
                            ui.selectable_value(&mut watchdog.model, "ib700".to_string(), "ib700");
                            ui.selectable_value(
                                &mut watchdog.model,
                                "diag288".to_string(),
                                "diag288",
                            );
                        });
                    ui.end_row();

                    ui.label("动作:");
                    egui::ComboBox::from_id_source("watchdog_action")
                        .selected_text(&watchdog.action)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut watchdog.action, "reset".to_string(), "reset");
                            ui.selectable_value(
                                &mut watchdog.action,
                                "poweroff".to_string(),
                                "poweroff",
                            );
                            ui.selectable_value(
                                &mut watchdog.action,
                                "shutdown".to_string(),
                                "shutdown",
                            );
                            ui.selectable_value(&mut watchdog.action, "pause".to_string(), "pause");
                            ui.selectable_value(&mut watchdog.action, "none".to_string(), "none");
                        });
                    ui.end_row();
                });
            }
        });
    }

    fn show_hub(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "USB Hub", Some("🔌"), colors, |ui| {
            if config.devices.hub.is_none() {
                config.devices.hub = Some(Vec::new());
            }

            if let Some(ref mut hub_list) = config.devices.hub {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加 Hub", colors) {
                        hub_list.push(HubConfig { hub_type: "usb".to_string() });
                    }
                });

                let to_remove = None;
                for (i, hub) in hub_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        inner_group(ui, colors, |ui| {
                            ui.label(format!("USB Hub {}", i + 1));
                            grid(ui, format!("hub_grid_{}", i), 2, |ui| {
                                ui.label("类型:");
                                egui::ComboBox::from_id_source(format!("hub_type_{}", i))
                                    .selected_text(&hub.hub_type)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut hub.hub_type,
                                            "usb".to_string(),
                                            "USB",
                                        );
                                    });
                                ui.end_row();
                            });
                        });
                        ui.add_space(5.0);
                    });
                }

                if let Some(idx) = to_remove {
                    hub_list.remove(idx);
                }
            }
        });
    }

    fn show_panic(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "Panic 设备", Some("⚠"), colors, |ui| {
            let mut has_panic = config.devices.panic.is_some();
            if checkbox(ui, &mut has_panic, "启用 Panic 设备") {
                if has_panic {
                    config.devices.panic =
                        Some(PanicConfig { model: Some("hyperv".to_string()), address: None });
                } else {
                    config.devices.panic = None;
                }
            }

            if let Some(ref mut panic) = config.devices.panic {
                ui.add_space(5.0);
                grid(ui, "panic_grid", 2, |ui| {
                    ui.label("模型:");
                    let model = panic.model.get_or_insert_with(|| "hyperv".to_string());
                    egui::ComboBox::from_id_source("panic_model")
                        .selected_text(model.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(model, "hyperv".to_string(), "Hyper-V");
                            ui.selectable_value(model, "isa".to_string(), "ISA");
                            ui.selectable_value(model, "pseries".to_string(), "pSeries");
                            ui.selectable_value(model, "s390".to_string(), "S390");
                            ui.selectable_value(model, "pvpanic".to_string(), "PCI pvpanic");
                        });
                    ui.end_row();
                });
            }
        });
    }

    fn show_shmem(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "共享内存", Some("📊"), colors, |ui| {
            if config.devices.shmem.is_none() {
                config.devices.shmem = Some(Vec::new());
            }

            if let Some(ref mut shmem_list) = config.devices.shmem {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加共享内存", colors) {
                        shmem_list.push(ShmemConfig {
                            name: "shmem0".to_string(),
                            role: None,
                            model: Some(crate::model::devices::ShmemModel {
                                model_type: "ivshmem-plain".to_string(),
                            }),
                            size: Some(crate::model::devices::SizeConfig {
                                value: Some(4),
                                unit: Some("M".to_string()),
                            }),
                            server: None,
                            msi: None,
                        });
                    }
                });

                let to_remove = None;
                for (i, shmem) in shmem_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        inner_group(ui, colors, |ui| {
                            ui.label(format!("共享内存 {}", i + 1));
                            grid(ui, format!("shmem_grid_{}", i), 2, |ui| {
                                ui.label("名称:");
                                ui.text_edit_singleline(&mut shmem.name);
                                ui.end_row();

                                ui.label("模型:");
                                if let Some(ref mut model) = shmem.model {
                                    egui::ComboBox::from_id_source(format!("shmem_model_{}", i))
                                        .selected_text(&model.model_type)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut model.model_type,
                                                "ivshmem".to_string(),
                                                "ivshmem",
                                            );
                                            ui.selectable_value(
                                                &mut model.model_type,
                                                "ivshmem-plain".to_string(),
                                                "ivshmem-plain",
                                            );
                                            ui.selectable_value(
                                                &mut model.model_type,
                                                "ivshmem-doorbell".to_string(),
                                                "ivshmem-doorbell",
                                            );
                                        });
                                    ui.end_row();
                                }

                                ui.label("大小:");
                                if let Some(ref mut size) = shmem.size {
                                    let mut val = size.value.unwrap_or(4);
                                    ui.add(egui::Slider::new(&mut val, 1..=1024));
                                    size.value = Some(val);
                                    ui.end_row();
                                }
                            });
                        });
                        ui.add_space(5.0);
                    });
                }

                if let Some(idx) = to_remove {
                    shmem_list.remove(idx);
                }
            }
        });
    }

    fn show_memory_device(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "内存设备", Some("💾"), colors, |ui| {
            if config.devices.memory_device.is_none() {
                config.devices.memory_device = Some(Vec::new());
            }

            if let Some(ref mut mem_dev_list) = config.devices.memory_device {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加内存设备", colors) {
                        mem_dev_list.push(MemoryDeviceConfig {
                            model: Some("dimm".to_string()),
                            access: None,
                            discard: None,
                            uuid: None,
                            source: None,
                            target: Some(crate::model::devices::MemoryDeviceTarget {
                                size: Some(crate::model::devices::SizeConfig {
                                    value: Some(524288),
                                    unit: Some("KiB".to_string()),
                                }),
                                node: Some(0),
                                label: None,
                            }),
                            address: None,
                        });
                    }
                });

                let to_remove = None;
                for (i, mem_dev) in mem_dev_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        inner_group(ui, colors, |ui| {
                            ui.label(format!("内存设备 {}", i + 1));
                            grid(ui, format!("mem_dev_grid_{}", i), 2, |ui| {
                                ui.label("模型:");
                                let model = mem_dev.model.get_or_insert_with(|| "dimm".to_string());
                                egui::ComboBox::from_id_source(format!("mem_dev_model_{}", i))
                                    .selected_text(model.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(model, "dimm".to_string(), "dimm");
                                        ui.selectable_value(model, "nvdimm".to_string(), "nvdimm");
                                    });
                                ui.end_row();

                                ui.label("访问模式:");
                                let access =
                                    mem_dev.access.get_or_insert_with(|| "shared".to_string());
                                egui::ComboBox::from_id_source(format!("mem_dev_access_{}", i))
                                    .selected_text(access.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(access, "shared".to_string(), "shared");
                                        ui.selectable_value(
                                            access,
                                            "private".to_string(),
                                            "private",
                                        );
                                    });
                                ui.end_row();
                            });
                        });
                        ui.add_space(5.0);
                    });
                }

                if let Some(idx) = to_remove {
                    mem_dev_list.remove(idx);
                }
            }
        });
    }

    fn show_iommu(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "IOMMU", Some("🔧"), colors, |ui| {
            let mut has_iommu = config.devices.iommu.is_some();
            if checkbox(ui, &mut has_iommu, "启用 IOMMU") {
                if has_iommu {
                    config.devices.iommu =
                        Some(IommuConfig { model: "intel".to_string(), driver: None });
                } else {
                    config.devices.iommu = None;
                }
            }

            if let Some(ref mut iommu) = config.devices.iommu {
                ui.add_space(5.0);
                grid(ui, "iommu_grid", 2, |ui| {
                    ui.label("模型:");
                    egui::ComboBox::from_id_source("iommu_model")
                        .selected_text(&iommu.model)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut iommu.model, "intel".to_string(), "Intel");
                            ui.selectable_value(&mut iommu.model, "amd".to_string(), "AMD");
                        });
                    ui.end_row();
                });
            }
        });
    }

    fn show_vsock(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "Vsock", Some("🔗"), colors, |ui| {
            let mut has_vsock = config.devices.vsock.is_some();
            if checkbox(ui, &mut has_vsock, "启用 Vsock") {
                if has_vsock {
                    config.devices.vsock = Some(VsockConfig {
                        id: Some(0),
                        source: Some(crate::model::devices::VsockSource {
                            mode: None,
                            path: None,
                            cid: None,
                            auto: None,
                        }),
                        address: None,
                    });
                } else {
                    config.devices.vsock = None;
                }
            }

            if let Some(ref mut vsock) = config.devices.vsock {
                ui.add_space(5.0);
                grid(ui, "vsock_grid", 2, |ui| {
                    ui.label("ID:");
                    let id = vsock.id.get_or_insert(0);
                    ui.add(egui::Slider::new(id, 0..=u32::MAX));
                    ui.end_row();

                    if let Some(ref mut source) = vsock.source {
                        ui.label("CID:");
                        let cid = source.cid.get_or_insert(0);
                        ui.add(egui::Slider::new(cid, 0..=u32::MAX));
                        ui.end_row();
                    }
                });
            }
        });
    }

    fn show_crypto(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "Crypto", Some("🔐"), colors, |ui| {
            let mut has_crypto = config.devices.crypto.is_some();
            if checkbox(ui, &mut has_crypto, "启用 Crypto") {
                if has_crypto {
                    config.devices.crypto = Some(CryptoConfig {
                        crypto_type: "ceph".to_string(),
                        backend: Some(crate::model::devices::CryptoBackend {
                            backend_type: "default".to_string(),
                            node: None,
                        }),
                    });
                } else {
                    config.devices.crypto = None;
                }
            }

            if let Some(ref mut crypto) = config.devices.crypto {
                ui.add_space(5.0);
                grid(ui, "crypto_grid", 2, |ui| {
                    ui.label("类型:");
                    egui::ComboBox::from_id_source("crypto_type")
                        .selected_text(&crypto.crypto_type)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut crypto.crypto_type,
                                "ceph".to_string(),
                                "Ceph",
                            );
                        });
                    ui.end_row();
                });
            }
        });
    }

    fn show_pstore(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "Pstore", Some("💾"), colors, |ui| {
            let mut has_pstore = config.devices.pstore.is_some();
            if checkbox(ui, &mut has_pstore, "启用 Pstore") {
                if has_pstore {
                    config.devices.pstore = Some(PstoreConfig {
                        path: "/sys/fs/pstore".to_string(),
                        size: Some(crate::model::devices::SizeConfig {
                            value: Some(64),
                            unit: Some("K".to_string()),
                        }),
                    });
                } else {
                    config.devices.pstore = None;
                }
            }

            if let Some(ref mut pstore) = config.devices.pstore {
                ui.add_space(5.0);
                grid(ui, "pstore_grid", 2, |ui| {
                    ui.label("路径:");
                    ui.text_edit_singleline(&mut pstore.path);
                    ui.end_row();
                });
            }
        });
    }

    fn show_audio(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "Audio", Some("🔊"), colors, |ui| {
            let mut has_audio = config.devices.audio.is_some();
            if checkbox(ui, &mut has_audio, "启用 Audio") {
                if has_audio {
                    config.devices.audio =
                        Some(AudioConfig { id: None, input: None, output: None });
                } else {
                    config.devices.audio = None;
                }
            }

            if let Some(ref mut audio) = config.devices.audio {
                ui.add_space(5.0);

                // Input 配置
                let mut has_input = audio.input.is_some();
                if checkbox(ui, &mut has_input, "启用输入") {
                    if has_input {
                        audio.input = Some(crate::model::devices::AudioStream {
                            stream_type: "pulseaudio".to_string(),
                            server: None,
                            name: None,
                            device: None,
                            format: None,
                            global: None,
                        });
                    } else {
                        audio.input = None;
                    }
                }

                if let Some(ref mut input) = audio.input {
                    ui.add_space(5.0);
                    grid(ui, "audio_input_grid", 2, |ui| {
                        ui.label("类型:");
                        egui::ComboBox::from_id_source("audio_input_type")
                            .selected_text(&input.stream_type)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut input.stream_type,
                                    "pulseaudio".to_string(),
                                    "PulseAudio",
                                );
                                ui.selectable_value(
                                    &mut input.stream_type,
                                    "alsa".to_string(),
                                    "ALSA",
                                );
                                ui.selectable_value(
                                    &mut input.stream_type,
                                    "coreaudio".to_string(),
                                    "CoreAudio",
                                );
                            });
                        ui.end_row();
                    });
                }

                // Output 配置
                let mut has_output = audio.output.is_some();
                if checkbox(ui, &mut has_output, "启用输出") {
                    if has_output {
                        audio.output = Some(crate::model::devices::AudioStream {
                            stream_type: "pulseaudio".to_string(),
                            server: None,
                            name: None,
                            device: None,
                            format: None,
                            global: None,
                        });
                    } else {
                        audio.output = None;
                    }
                }

                if let Some(ref mut output) = audio.output {
                    ui.add_space(5.0);
                    grid(ui, "audio_output_grid", 2, |ui| {
                        ui.label("类型:");
                        egui::ComboBox::from_id_source("audio_output_type")
                            .selected_text(&output.stream_type)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut output.stream_type,
                                    "pulseaudio".to_string(),
                                    "PulseAudio",
                                );
                                ui.selectable_value(
                                    &mut output.stream_type,
                                    "alsa".to_string(),
                                    "ALSA",
                                );
                                ui.selectable_value(
                                    &mut output.stream_type,
                                    "coreaudio".to_string(),
                                    "CoreAudio",
                                );
                            });
                        ui.end_row();
                    });
                }
            }
        });
    }

    fn show_hostdev(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "主机设备直通", Some("🔌"), colors, |ui| {
            if config.devices.hostdev.is_none() {
                config.devices.hostdev = Some(Vec::new());
            }

            if let Some(ref mut hostdev_list) = config.devices.hostdev {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加主机设备", colors) {
                        hostdev_list.push(HostdevConfig {
                            mode: "subsystem".to_string(),
                            device_type: "pci".to_string(),
                            managed: Some("yes".to_string()),
                            model: None,
                            rawio: None,
                            display: None,
                            ramfb: None,
                            source: Some(crate::model::devices::HostdevSource {
                                startup_policy: None,
                                guest_reset: None,
                                write_filtering: None,
                                protocol: None,
                                name: None,
                                wwpn: None,
                                vendor: None,
                                product: None,
                                address: Some(crate::model::devices::HostdevAddress {
                                    bus: None,
                                    device: None,
                                    port: None,
                                    domain: Some("0x0000".to_string()),
                                    slot: Some("0x06".to_string()),
                                    function: Some("0x0".to_string()),
                                    uuid: None,
                                }),
                                adapter: None,
                                scsi_address: None,
                                host: None,
                                auth: None,
                                initiator: None,
                            }),
                            boot: None,
                            rom: None,
                            address: None,
                            driver: None,
                            readonly: None,
                            shareable: None,
                            acpi: None,
                        });
                    }
                });

                let mut to_remove = None;
                for (i, hostdev) in hostdev_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        inner_group(ui, colors, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("主机设备 {}", i + 1));
                                if delete_button(ui, None) {
                                    to_remove = Some(i);
                                }
                            });

                            grid(ui, format!("hostdev_grid_{}", i), 2, |ui| {
                                ui.label("设备类型:");
                                egui::ComboBox::from_id_source(format!("hostdev_type_{}", i))
                                    .selected_text(&hostdev.device_type)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut hostdev.device_type,
                                            "pci".to_string(),
                                            "PCI",
                                        );
                                        ui.selectable_value(
                                            &mut hostdev.device_type,
                                            "usb".to_string(),
                                            "USB",
                                        );
                                        ui.selectable_value(
                                            &mut hostdev.device_type,
                                            "scsi".to_string(),
                                            "SCSI",
                                        );
                                        ui.selectable_value(
                                            &mut hostdev.device_type,
                                            "mdev".to_string(),
                                            "MDEV (中介设备)",
                                        );
                                    });
                                ui.end_row();

                                if hostdev.device_type == "pci" {
                                    ui.label("自动管理:");
                                    let managed = hostdev.managed.clone().unwrap_or_default();
                                    egui::ComboBox::from_id_source(format!("managed_{}", i))
                                        .selected_text(&managed)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut hostdev.managed,
                                                Some("yes".to_string()),
                                                "是",
                                            );
                                            ui.selectable_value(
                                                &mut hostdev.managed,
                                                Some("no".to_string()),
                                                "否",
                                            );
                                        });
                                    ui.end_row();
                                }

                                if hostdev.device_type == "mdev" {
                                    ui.label("模型:");
                                    let model =
                                        hostdev.model.get_or_insert_with(|| "vfio-pci".to_string());
                                    egui::ComboBox::from_id_source(format!("mdev_model_{}", i))
                                        .selected_text(model.as_str())
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                model,
                                                "vfio-pci".to_string(),
                                                "vfio-pci",
                                            );
                                            ui.selectable_value(
                                                model,
                                                "vfio-ccw".to_string(),
                                                "vfio-ccw",
                                            );
                                            ui.selectable_value(
                                                model,
                                                "vfio-ap".to_string(),
                                                "vfio-ap",
                                            );
                                        });
                                    ui.end_row();

                                    ui.label("显示设备:");
                                    let display = hostdev.display.clone().unwrap_or_default();
                                    egui::ComboBox::from_id_source(format!("display_{}", i))
                                        .selected_text(&display)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut hostdev.display,
                                                Some("on".to_string()),
                                                "开启",
                                            );
                                            ui.selectable_value(
                                                &mut hostdev.display,
                                                Some("off".to_string()),
                                                "关闭",
                                            );
                                        });
                                    ui.end_row();
                                }
                            });

                            // Source 配置
                            ui.add_space(5.0);
                            ui.collapsing("源设备配置", |ui| {
                                if let Some(ref mut source) = hostdev.source {
                                    if hostdev.device_type == "usb" {
                                        grid(ui, format!("usb_source_{}", i), 2, |ui| {
                                            ui.label("Vendor ID:");
                                            let mut vendor_id = source
                                                .vendor
                                                .as_ref()
                                                .map(|v| v.id.clone())
                                                .unwrap_or_default();
                                            ui.text_edit_singleline(&mut vendor_id);
                                            if !vendor_id.is_empty() {
                                                source.vendor =
                                                    Some(crate::model::devices::HostdevVendor {
                                                        id: vendor_id,
                                                    });
                                            }
                                            ui.end_row();

                                            ui.label("Product ID:");
                                            let mut product_id = source
                                                .product
                                                .as_ref()
                                                .map(|p| p.id.clone())
                                                .unwrap_or_default();
                                            ui.text_edit_singleline(&mut product_id);
                                            if !product_id.is_empty() {
                                                source.product =
                                                    Some(crate::model::devices::HostdevProduct {
                                                        id: product_id,
                                                    });
                                            }
                                            ui.end_row();
                                        });
                                    }

                                    if hostdev.device_type == "pci" || hostdev.device_type == "mdev"
                                    {
                                        grid(ui, format!("pci_source_{}", i), 2, |ui| {
                                            ui.label("Domain:");
                                            if let Some(ref mut addr) = source.address {
                                                let mut domain = addr
                                                    .domain
                                                    .clone()
                                                    .unwrap_or_else(|| "0x0000".to_string());
                                                ui.text_edit_singleline(&mut domain);
                                                addr.domain = Some(domain);
                                            }
                                            ui.end_row();

                                            ui.label("Bus:");
                                            if let Some(ref mut addr) = source.address {
                                                let mut bus = addr
                                                    .bus
                                                    .clone()
                                                    .unwrap_or_else(|| "0x00".to_string());
                                                ui.text_edit_singleline(&mut bus);
                                                addr.bus = Some(bus);
                                            }
                                            ui.end_row();

                                            ui.label("Slot:");
                                            if let Some(ref mut addr) = source.address {
                                                let mut slot = addr
                                                    .slot
                                                    .clone()
                                                    .unwrap_or_else(|| "0x00".to_string());
                                                ui.text_edit_singleline(&mut slot);
                                                addr.slot = Some(slot);
                                            }
                                            ui.end_row();

                                            ui.label("Function:");
                                            if let Some(ref mut addr) = source.address {
                                                let mut func = addr
                                                    .function
                                                    .clone()
                                                    .unwrap_or_else(|| "0x0".to_string());
                                                ui.text_edit_singleline(&mut func);
                                                addr.function = Some(func);
                                            }
                                            ui.end_row();

                                            if hostdev.device_type == "mdev" {
                                                ui.label("UUID:");
                                                if let Some(ref mut addr) = source.address {
                                                    let mut uuid =
                                                        addr.uuid.clone().unwrap_or_default();
                                                    ui.text_edit_singleline(&mut uuid);
                                                    addr.uuid = Some(uuid);
                                                }
                                                ui.end_row();
                                            }
                                        });
                                    }
                                }
                            });
                        });
                        ui.add_space(5.0);
                    });
                }

                if let Some(idx) = to_remove {
                    hostdev_list.remove(idx);
                }
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
