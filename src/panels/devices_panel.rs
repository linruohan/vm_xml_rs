use crate::{
    model::{
        controller::{ControllerConfig, ControllerDriver, ControllerHotplug},
        devices::{
            AliasConfig, DeviceNVRAMConfig, FilesystemConfig, FilesystemSource, FilesystemTarget,
            InputDriver, InputSource, LeaseConfig, MemballoonConfig, RedirdevConfig,
            RedirfilterConfig, SMBIOSEntry, SmartcardConfig, SysInfoConfig, ThrottleFilter,
        },
        input_sound_tpm::RngBackend,
        serial_console::{ChannelConfig, ChannelTarget, ParallelTarget, SerialTarget},
        AddressConfig, AudioConfig, ConsoleConfig, CryptoConfig, DiskConfig, DiskDriver,
        DiskSource, DiskTarget, GraphicsConfig, HostdevConfig, HubConfig, InputConfig,
        InterfaceConfig, InterfaceModel, InterfaceSource, IommuConfig, MacAddress,
        MemoryDeviceConfig, PanicConfig, ParallelConfig, PstoreConfig, RngConfig, SerialConfig,
        ShmemConfig, SoundCodec, SoundConfig, TPMBackend, TPMConfig, VMConfig, VideoConfig,
        VideoModel, VsockConfig, WatchdogConfig,
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

            // Channel 通道卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_channel(ui, config, colors);
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

            // Smartcard 智能卡卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_smartcard(ui, config, colors);
                },
            );

            // NVRAM 非易失性存储卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_nvram(ui, config, colors);
                },
            );

            // USB 重定向设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_redirdev(ui, config, colors);
                },
            );

            // USB 重定向过滤器卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_redirfilter(ui, config, colors);
                },
            );

            // 租约设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_lease(ui, config, colors);
                },
            );

            // 文件系统设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_filesystem(ui, config, colors);
                },
            );

            // 控制器设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_controller(ui, config, colors);
                },
            );

            // 内存气球设备卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_memballoon(ui, config, colors);
                },
            );

            // 模拟器路径卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_emulator(ui, config, colors);
                },
            );

            // SysInfo 配置卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    Self::show_sysinfo(ui, config, colors);
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

                                // Alias 配置
                                if disk.alias.is_none() {
                                    disk.alias = Some(AliasConfig { name: "".to_string() });
                                }
                                if let Some(ref mut alias) = disk.alias {
                                    grid(ui, format!("disk_alias_grid_{}", i), 2, |ui| {
                                        ui.label("设备别名 (alias):");
                                        ui.text_edit_singleline(&mut alias.name);
                                        ui.end_row();
                                    });
                                }

                                // Throttlefilters 配置
                                ui.collapsing("限流过滤器 (throttlefilters)", |ui| {
                                    if disk.throttlefilters.is_none() {
                                        disk.throttlefilters = Some(Vec::new());
                                    }
                                    if let Some(ref mut filters) = disk.throttlefilters {
                                        ui.horizontal(|ui| {
                                            if add_button(ui, "添加限流过滤器", colors) {
                                                filters
                                                    .push(ThrottleFilter { group: "".to_string() });
                                            }
                                        });

                                        let mut to_remove = None;
                                        for (j, filter) in filters.iter_mut().enumerate() {
                                            ui.horizontal(|ui| {
                                                ui.label(format!("过滤器 {}:", j + 1));
                                                ui.text_edit_singleline(&mut filter.group);
                                                if delete_button(ui, None) {
                                                    to_remove = Some(j);
                                                }
                                            });
                                        }
                                        if let Some(idx) = to_remove {
                                            filters.remove(idx);
                                        }
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
                        iface_list.push(Self::create_default_interface(iface_list.len()));
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

                                // Alias 配置
                                if iface.alias.is_none() {
                                    iface.alias = Some(AliasConfig { name: "".to_string() });
                                }
                                if let Some(ref mut alias) = iface.alias {
                                    grid(ui, format!("iface_alias_grid_{}", i), 2, |ui| {
                                        ui.label("设备别名 (alias):");
                                        ui.text_edit_singleline(&mut alias.name);
                                        ui.end_row();
                                    });
                                }
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
                            name: None,
                            source: None,
                            driver: None,
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

                            ui.label("名称:");
                            if let Some(ref mut name) = input.name {
                                ui.text_edit_singleline(name);
                            }
                            ui.end_row();
                        });

                        // Source 配置（仅用于 passthrough 类型）
                        if input.input_type == "passthrough" {
                            ui.collapsing("Source 配置", |ui| {
                                if input.source.is_none() {
                                    input.source = Some(InputSource {
                                        dev: None,
                                        grab: None,
                                        repeat: None,
                                        grab_toggle: None,
                                    });
                                }
                                if let Some(ref mut source) = input.source {
                                    grid(ui, format!("input_source_grid_{}", i), 2, |ui| {
                                        ui.label("设备:");
                                        if let Some(ref mut dev) = source.dev {
                                            ui.text_edit_singleline(dev);
                                        }
                                        ui.end_row();

                                        ui.label("Grab:");
                                        egui::ComboBox::from_id_source(format!("input_grab_{}", i))
                                            .selected_text(source.grab.as_deref().unwrap_or(""))
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(&mut source.grab, None, "");
                                                ui.selectable_value(
                                                    &mut source.grab,
                                                    Some("on".to_string()),
                                                    "on",
                                                );
                                                ui.selectable_value(
                                                    &mut source.grab,
                                                    Some("off".to_string()),
                                                    "off",
                                                );
                                            });
                                        ui.end_row();

                                        ui.label("Repeat:");
                                        egui::ComboBox::from_id_source(format!(
                                            "input_repeat_{}",
                                            i
                                        ))
                                        .selected_text(source.repeat.as_deref().unwrap_or(""))
                                        .show_ui(
                                            ui,
                                            |ui| {
                                                ui.selectable_value(&mut source.repeat, None, "");
                                                ui.selectable_value(
                                                    &mut source.repeat,
                                                    Some("on".to_string()),
                                                    "on",
                                                );
                                                ui.selectable_value(
                                                    &mut source.repeat,
                                                    Some("off".to_string()),
                                                    "off",
                                                );
                                            },
                                        );
                                        ui.end_row();

                                        ui.label("GrabToggle:");
                                        egui::ComboBox::from_id_source(format!(
                                            "input_grab_toggle_{}",
                                            i
                                        ))
                                        .selected_text(source.grab_toggle.as_deref().unwrap_or(""))
                                        .show_ui(
                                            ui,
                                            |ui| {
                                                ui.selectable_value(
                                                    &mut source.grab_toggle,
                                                    None,
                                                    "",
                                                );
                                                ui.selectable_value(
                                                    &mut source.grab_toggle,
                                                    Some("ctrl-esc".to_string()),
                                                    "ctrl-esc",
                                                );
                                                ui.selectable_value(
                                                    &mut source.grab_toggle,
                                                    Some("alt-enter".to_string()),
                                                    "alt-enter",
                                                );
                                            },
                                        );
                                        ui.end_row();
                                    });
                                }
                            });

                            // Driver 配置（仅用于 virtio 总线）
                            if input.bus.as_deref() == Some("virtio") {
                                ui.collapsing("Driver 配置", |ui| {
                                    if input.driver.is_none() {
                                        input.driver = Some(InputDriver {
                                            queues: None,
                                            ioeventfd: None,
                                            event_idx: None,
                                        });
                                    }
                                    if let Some(ref mut driver) = input.driver {
                                        grid(ui, format!("input_driver_grid_{}", i), 2, |ui| {
                                            ui.label("队列数:");
                                            if let Some(ref mut queues) = driver.queues {
                                                let mut queues_str = queues.to_string();
                                                if ui
                                                    .text_edit_singleline(&mut queues_str)
                                                    .changed()
                                                {
                                                    if let Ok(val) = queues_str.parse() {
                                                        *queues = val;
                                                    }
                                                }
                                            }
                                            ui.end_row();

                                            ui.label("ioeventfd:");
                                            egui::ComboBox::from_id_source(format!(
                                                "input_ioeventfd_{}",
                                                i
                                            ))
                                            .selected_text(
                                                driver.ioeventfd.as_deref().unwrap_or(""),
                                            )
                                            .show_ui(
                                                ui,
                                                |ui| {
                                                    ui.selectable_value(
                                                        &mut driver.ioeventfd,
                                                        None,
                                                        "",
                                                    );
                                                    ui.selectable_value(
                                                        &mut driver.ioeventfd,
                                                        Some("on".to_string()),
                                                        "on",
                                                    );
                                                    ui.selectable_value(
                                                        &mut driver.ioeventfd,
                                                        Some("off".to_string()),
                                                        "off",
                                                    );
                                                },
                                            );
                                            ui.end_row();

                                            ui.label("event_idx:");
                                            egui::ComboBox::from_id_source(format!(
                                                "input_event_idx_{}",
                                                i
                                            ))
                                            .selected_text(
                                                driver.event_idx.as_deref().unwrap_or(""),
                                            )
                                            .show_ui(
                                                ui,
                                                |ui| {
                                                    ui.selectable_value(
                                                        &mut driver.event_idx,
                                                        None,
                                                        "",
                                                    );
                                                    ui.selectable_value(
                                                        &mut driver.event_idx,
                                                        Some("on".to_string()),
                                                        "on",
                                                    );
                                                    ui.selectable_value(
                                                        &mut driver.event_idx,
                                                        Some("off".to_string()),
                                                        "off",
                                                    );
                                                },
                                            );
                                            ui.end_row();
                                        });
                                    }
                                });
                            }
                        }
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
                        backend: Some(TPMBackend {
                            backend_type: "emulator".to_string(),
                            version: Some("2.0".to_string()),
                            device: None,
                            model: None,
                        }),
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
                    if let Some(ref backend) = tpm.backend {
                        egui::ComboBox::from_id_source("tpm_backend_type")
                            .selected_text(&backend.backend_type)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut tpm.backend.as_mut().unwrap().backend_type,
                                    "emulator".to_string(),
                                    "emulator",
                                );
                                ui.selectable_value(
                                    &mut tpm.backend.as_mut().unwrap().backend_type,
                                    "passthrough".to_string(),
                                    "passthrough",
                                );
                            });
                        ui.end_row();

                        ui.label("TPM 版本:");
                        let version = tpm
                            .backend
                            .as_mut()
                            .unwrap()
                            .version
                            .get_or_insert_with(|| "2.0".to_string());
                        egui::ComboBox::from_id_source("tpm_version")
                            .selected_text(version.as_str())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(version, "1.2".to_string(), "1.2");
                                ui.selectable_value(version, "2.0".to_string(), "2.0");
                            });
                        ui.end_row();

                        ui.label("设备:");
                        if let Some(ref mut device) = tpm.backend.as_mut().unwrap().device {
                            ui.text_edit_singleline(device);
                        } else {
                            let mut empty = String::new();
                            ui.text_edit_singleline(&mut empty);
                        }
                        ui.end_row();

                        ui.label("后端模型:");
                        if let Some(ref mut model) = tpm.backend.as_mut().unwrap().model {
                            ui.text_edit_singleline(model);
                        } else {
                            let mut empty = String::new();
                            ui.text_edit_singleline(&mut empty);
                        }
                        ui.end_row();
                    }
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
                        sound_list.push(SoundConfig { model: "ich6".to_string(), codec: None });
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

                            // Codec 配置
                            ui.collapsing("Codec 配置", |ui| {
                                if sound.codec.is_none() {
                                    sound.codec = Some(SoundCodec {
                                        codec_type: "duplex".to_string(),
                                        input_type: None,
                                        output_type: None,
                                    });
                                }
                                if let Some(ref mut codec) = sound.codec {
                                    grid(ui, format!("sound_codec_grid_{}", i), 2, |ui| {
                                        ui.label("编解码器类型:");
                                        egui::ComboBox::from_id_source(format!(
                                            "sound_codec_type_{}",
                                            i
                                        ))
                                        .selected_text(&codec.codec_type)
                                        .show_ui(
                                            ui,
                                            |ui| {
                                                ui.selectable_value(
                                                    &mut codec.codec_type,
                                                    "duplex".to_string(),
                                                    "duplex",
                                                );
                                                ui.selectable_value(
                                                    &mut codec.codec_type,
                                                    "input".to_string(),
                                                    "input",
                                                );
                                                ui.selectable_value(
                                                    &mut codec.codec_type,
                                                    "output".to_string(),
                                                    "output",
                                                );
                                            },
                                        );
                                        ui.end_row();

                                        ui.label("输入类型:");
                                        if let Some(ref mut input_type) = codec.input_type {
                                            ui.text_edit_singleline(input_type);
                                        } else {
                                            let mut empty = String::new();
                                            ui.text_edit_singleline(&mut empty);
                                        }
                                        ui.end_row();

                                        ui.label("输出类型:");
                                        if let Some(ref mut output_type) = codec.output_type {
                                            ui.text_edit_singleline(output_type);
                                        } else {
                                            let mut empty = String::new();
                                            ui.text_edit_singleline(&mut empty);
                                        }
                                        ui.end_row();
                                    });
                                }
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
                            device: Some("/dev/random".to_string()),
                        }),
                        size: None,
                        rate: None,
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
                                        if let Some(ref mut device) = backend.device {
                                            ui.text_edit_singleline(device);
                                        }
                                        ui.end_row();
                                    }
                                });
                            }

                            // Size 配置
                            ui.collapsing("Size 配置", |ui| {
                                if let Some(ref mut size) = rng.size {
                                    grid(ui, format!("rng_size_grid_{}", i), 2, |ui| {
                                        ui.label("大小值:");
                                        let mut value_str = size.value.to_string();
                                        if ui.text_edit_singleline(&mut value_str).changed() {
                                            if let Ok(val) = value_str.parse() {
                                                size.value = val;
                                            }
                                        }
                                        ui.end_row();

                                        ui.label("单位:");
                                        egui::ComboBox::from_id_source(format!(
                                            "rng_size_unit_{}",
                                            i
                                        ))
                                        .selected_text(size.unit.as_deref().unwrap_or(""))
                                        .show_ui(
                                            ui,
                                            |ui| {
                                                ui.selectable_value(&mut size.unit, None, "");
                                                ui.selectable_value(
                                                    &mut size.unit,
                                                    Some("B".to_string()),
                                                    "B",
                                                );
                                                ui.selectable_value(
                                                    &mut size.unit,
                                                    Some("KiB".to_string()),
                                                    "KiB",
                                                );
                                                ui.selectable_value(
                                                    &mut size.unit,
                                                    Some("MiB".to_string()),
                                                    "MiB",
                                                );
                                            },
                                        );
                                        ui.end_row();
                                    });
                                }
                            });

                            // Rate 配置
                            ui.collapsing("Rate 配置", |ui| {
                                if let Some(ref mut rate) = rng.rate {
                                    grid(ui, format!("rng_rate_grid_{}", i), 2, |ui| {
                                        ui.label("速率值:");
                                        let mut value_str = rate.value.to_string();
                                        if ui.text_edit_singleline(&mut value_str).changed() {
                                            if let Ok(val) = value_str.parse() {
                                                rate.value = val;
                                            }
                                        }
                                        ui.end_row();

                                        ui.label("周期:");
                                        if let Some(ref mut period) = rate.period {
                                            let mut period_str = period.to_string();
                                            if ui.text_edit_singleline(&mut period_str).changed() {
                                                if let Ok(val) = period_str.parse() {
                                                    *period = val;
                                                }
                                            }
                                        }
                                        ui.end_row();
                                    });
                                }
                            });
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
                        address: None,
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

                // Address 配置
                ui.collapsing("Address 配置", |ui| {
                    if watchdog.address.is_none() {
                        watchdog.address = Some(crate::model::devices::AddressConfig {
                            address_type: "pci".to_string(),
                            domain: None,
                            bus: None,
                            slot: None,
                            function: None,
                            controller: None,
                            multifunction: None,
                            target: None,
                            unit: None,
                            reg: None,
                            cssid: None,
                            ssid: None,
                            devno: None,
                            iobase: None,
                            irq: None,
                        });
                    }
                    if let Some(ref mut address) = watchdog.address {
                        grid(ui, "watchdog_address_grid", 2, |ui| {
                            ui.label("地址类型:");
                            egui::ComboBox::from_id_source("watchdog_address_type")
                                .selected_text(&address.address_type)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut address.address_type,
                                        "pci".to_string(),
                                        "pci",
                                    );
                                    ui.selectable_value(
                                        &mut address.address_type,
                                        "ccid".to_string(),
                                        "ccid",
                                    );
                                });
                            ui.end_row();

                            ui.label("Domain:");
                            if let Some(ref mut domain) = address.domain {
                                ui.text_edit_singleline(domain);
                            } else {
                                let mut empty = String::new();
                                ui.text_edit_singleline(&mut empty);
                            }
                            ui.end_row();

                            ui.label("Bus:");
                            if let Some(ref mut bus) = address.bus {
                                let mut bus_str = bus.to_string();
                                if ui.text_edit_singleline(&mut bus_str).changed() {
                                    if let Ok(val) = bus_str.parse() {
                                        *bus = val;
                                    }
                                }
                            } else {
                                let mut empty = String::new();
                                ui.text_edit_singleline(&mut empty);
                            }
                            ui.end_row();

                            ui.label("Slot:");
                            if let Some(ref mut slot) = address.slot {
                                let mut slot_str = slot.to_string();
                                if ui.text_edit_singleline(&mut slot_str).changed() {
                                    if let Ok(val) = slot_str.parse() {
                                        *slot = val;
                                    }
                                }
                            } else {
                                let mut empty = String::new();
                                ui.text_edit_singleline(&mut empty);
                            }
                            ui.end_row();

                            ui.label("Function:");
                            if let Some(ref mut function) = address.function {
                                let mut function_str = function.to_string();
                                if ui.text_edit_singleline(&mut function_str).changed() {
                                    if let Ok(val) = function_str.parse() {
                                        *function = val;
                                    }
                                }
                            } else {
                                let mut empty = String::new();
                                ui.text_edit_singleline(&mut empty);
                            }
                            ui.end_row();
                        });
                    }
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
                                        ui.selectable_value(
                                            &mut hub.hub_type,
                                            "usb-serial".to_string(),
                                            "USB-Serial",
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

    fn show_channel(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "Channel 通道", Some("📡"), colors, |ui| {
            if config.devices.channel.is_none() {
                config.devices.channel = Some(Vec::new());
            }

            if let Some(ref mut channel_list) = config.devices.channel {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加通道", colors) {
                        channel_list.push(ChannelConfig {
                            channel_type: "unix".to_string(),
                            target: Some(ChannelTarget {
                                target_type: "virtio".to_string(),
                                name: "org.qemu.guest_agent.0".to_string(),
                            }),
                        });
                    }
                });

                let to_remove = None;
                for (i, channel) in channel_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        inner_group(ui, colors, |ui| {
                            ui.label(format!("通道 {}", i + 1));
                            grid(ui, format!("channel_grid_{}", i), 2, |ui| {
                                ui.label("类型:");
                                egui::ComboBox::from_id_source(format!("channel_type_{}", i))
                                    .selected_text(&channel.channel_type)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut channel.channel_type,
                                            "unix".to_string(),
                                            "unix",
                                        );
                                        ui.selectable_value(
                                            &mut channel.channel_type,
                                            "pty".to_string(),
                                            "pty",
                                        );
                                        ui.selectable_value(
                                            &mut channel.channel_type,
                                            "spicevmc".to_string(),
                                            "spicevmc",
                                        );
                                        ui.selectable_value(
                                            &mut channel.channel_type,
                                            "spiceport".to_string(),
                                            "spiceport",
                                        );
                                    });
                                ui.end_row();

                                if let Some(ref mut target) = channel.target {
                                    ui.label("目标类型:");
                                    egui::ComboBox::from_id_source(format!(
                                        "channel_target_type_{}",
                                        i
                                    ))
                                    .selected_text(&target.target_type)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut target.target_type,
                                            "virtio".to_string(),
                                            "virtio",
                                        );
                                        ui.selectable_value(
                                            &mut target.target_type,
                                            "isa".to_string(),
                                            "isa",
                                        );
                                    });
                                    ui.end_row();

                                    ui.label("名称:");
                                    ui.text_edit_singleline(&mut target.name);
                                    ui.end_row();
                                }
                            });
                        });
                        ui.add_space(5.0);
                    });
                }

                if let Some(idx) = to_remove {
                    channel_list.remove(idx);
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

                // Address 配置
                ui.collapsing("Address 配置", |ui| {
                    if panic.address.is_none() {
                        panic.address = Some(crate::model::devices::AddressConfig {
                            address_type: "pci".to_string(),
                            domain: None,
                            bus: None,
                            slot: None,
                            function: None,
                            controller: None,
                            multifunction: None,
                            target: None,
                            unit: None,
                            reg: None,
                            cssid: None,
                            ssid: None,
                            devno: None,
                            iobase: None,
                            irq: None,
                        });
                    }
                    if let Some(ref mut address) = panic.address {
                        grid(ui, "panic_address_grid", 2, |ui| {
                            ui.label("地址类型:");
                            egui::ComboBox::from_id_source("panic_address_type")
                                .selected_text(&address.address_type)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut address.address_type,
                                        "pci".to_string(),
                                        "pci",
                                    );
                                });
                            ui.end_row();

                            ui.label("Bus:");
                            if let Some(ref mut bus) = address.bus {
                                let mut bus_str = bus.to_string();
                                if ui.text_edit_singleline(&mut bus_str).changed() {
                                    if let Ok(val) = bus_str.parse() {
                                        *bus = val;
                                    }
                                }
                            }
                            ui.end_row();

                            ui.label("Slot:");
                            if let Some(ref mut slot) = address.slot {
                                let mut slot_str = slot.to_string();
                                if ui.text_edit_singleline(&mut slot_str).changed() {
                                    if let Ok(val) = slot_str.parse() {
                                        *slot = val;
                                    }
                                }
                            }
                            ui.end_row();

                            ui.label("Function:");
                            if let Some(ref mut function) = address.function {
                                let mut function_str = function.to_string();
                                if ui.text_edit_singleline(&mut function_str).changed() {
                                    if let Ok(val) = function_str.parse() {
                                        *function = val;
                                    }
                                }
                            }
                            ui.end_row();
                        });
                    }
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

                                ui.label("角色:");
                                let role = shmem.role.get_or_insert_with(|| "peer".to_string());
                                egui::ComboBox::from_id_source(format!("shmem_role_{}", i))
                                    .selected_text(role.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(role, "peer".to_string(), "peer");
                                        ui.selectable_value(role, "server".to_string(), "server");
                                    });
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

                            // Server 配置
                            ui.collapsing("Server 配置", |ui| {
                                if shmem.server.is_none() {
                                    shmem.server =
                                        Some(crate::model::devices::ShmemServer { path: None });
                                }
                                if let Some(ref mut server) = shmem.server {
                                    grid(ui, format!("shmem_server_grid_{}", i), 2, |ui| {
                                        ui.label("路径:");
                                        if let Some(ref mut path) = server.path {
                                            ui.text_edit_singleline(path);
                                        } else {
                                            let mut empty = String::new();
                                            ui.text_edit_singleline(&mut empty);
                                        }
                                        ui.end_row();
                                    });
                                }
                            });

                            // MSI 配置
                            ui.collapsing("MSI 配置", |ui| {
                                if shmem.msi.is_none() {
                                    shmem.msi = Some(crate::model::devices::MsiConfig {
                                        vectors: None,
                                        ioeventfd: None,
                                    });
                                }
                                if let Some(ref mut msi) = shmem.msi {
                                    grid(ui, format!("shmem_msi_grid_{}", i), 2, |ui| {
                                        ui.label("Vectors:");
                                        if let Some(ref mut vectors) = msi.vectors {
                                            let mut vectors_str = vectors.to_string();
                                            if ui.text_edit_singleline(&mut vectors_str).changed() {
                                                if let Ok(val) = vectors_str.parse() {
                                                    *vectors = val;
                                                }
                                            }
                                        }
                                        ui.end_row();

                                        ui.label("ioeventfd:");
                                        egui::ComboBox::from_id_source(format!(
                                            "shmem_ioeventfd_{}",
                                            i
                                        ))
                                        .selected_text(msi.ioeventfd.as_deref().unwrap_or(""))
                                        .show_ui(
                                            ui,
                                            |ui| {
                                                ui.selectable_value(&mut msi.ioeventfd, None, "");
                                                ui.selectable_value(
                                                    &mut msi.ioeventfd,
                                                    Some("on".to_string()),
                                                    "on",
                                                );
                                                ui.selectable_value(
                                                    &mut msi.ioeventfd,
                                                    Some("off".to_string()),
                                                    "off",
                                                );
                                            },
                                        );
                                        ui.end_row();
                                    });
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
                                readonly: None,
                                requested: None,
                                current: None,
                                address: None,
                            }),
                            address: None,
                        });
                    }
                });

                let mut to_remove = None;
                for (i, mem_dev) in mem_dev_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        inner_group(ui, colors, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("内存设备 {}", i + 1));
                                if delete_button(ui, None) {
                                    to_remove = Some(i);
                                }
                            });
                            grid(ui, format!("mem_dev_grid_{}", i), 2, |ui| {
                                ui.label("模型:");
                                let model = mem_dev.model.get_or_insert_with(|| "dimm".to_string());
                                egui::ComboBox::from_id_source(format!("mem_dev_model_{}", i))
                                    .selected_text(model.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(model, "dimm".to_string(), "dimm");
                                        ui.selectable_value(model, "nvdimm".to_string(), "nvdimm");
                                        ui.selectable_value(
                                            model,
                                            "virtio-pmem".to_string(),
                                            "virtio-pmem",
                                        );
                                        ui.selectable_value(
                                            model,
                                            "virtio-mem".to_string(),
                                            "virtio-mem",
                                        );
                                        ui.selectable_value(
                                            model,
                                            "sgx-epc".to_string(),
                                            "sgx-epc",
                                        );
                                    });
                                ui.end_row();

                                ui.label("访问模式:");
                                let access =
                                    mem_dev.access.get_or_insert_with(|| "private".to_string());
                                egui::ComboBox::from_id_source(format!("mem_dev_access_{}", i))
                                    .selected_text(access.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            access,
                                            "private".to_string(),
                                            "private",
                                        );
                                        ui.selectable_value(access, "shared".to_string(), "shared");
                                    });
                                ui.end_row();

                                if model.as_str() == "dimm" {
                                    ui.label("Discard:");
                                    let discard =
                                        mem_dev.discard.clone().unwrap_or_else(|| "no".to_string());
                                    egui::ComboBox::from_id_source(format!(
                                        "mem_dev_discard_{}",
                                        i
                                    ))
                                    .selected_text(&discard)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut mem_dev.discard,
                                            Some("yes".to_string()),
                                            "yes",
                                        );
                                        ui.selectable_value(
                                            &mut mem_dev.discard,
                                            Some("no".to_string()),
                                            "no",
                                        );
                                    });
                                    ui.end_row();
                                }
                            });

                            // Source 配置
                            ui.add_space(5.0);
                            ui.collapsing("源配置", |ui| {
                                let model_str = mem_dev.model.as_deref().unwrap_or("dimm");
                                let mut has_source = mem_dev.source.is_some();
                                if checkbox(ui, &mut has_source, "启用源配置") {
                                    if has_source {
                                        mem_dev.source =
                                            Some(crate::model::devices::MemoryDeviceSource {
                                                path: None,
                                                pagesize: None,
                                                nodemask: None,
                                                alignsize: None,
                                                pmem: None,
                                            });
                                    } else {
                                        mem_dev.source = None;
                                    }
                                }

                                if let Some(ref mut source) = mem_dev.source {
                                    grid(ui, format!("mem_source_grid_{}", i), 2, |ui| {
                                        if model_str == "nvdimm" || model_str == "virtio-pmem" {
                                            ui.label("路径:");
                                            let mut path = source.path.clone().unwrap_or_default();
                                            ui.text_edit_singleline(&mut path);
                                            source.path = Some(path);
                                            ui.end_row();
                                        }

                                        if model_str == "dimm"
                                            || model_str == "virtio-mem"
                                            || model_str == "sgx-epc"
                                        {
                                            ui.label("节点掩码:");
                                            let mut nodemask =
                                                source.nodemask.clone().unwrap_or_default();
                                            ui.text_edit_singleline(&mut nodemask);
                                            source.nodemask = Some(nodemask);
                                            ui.end_row();

                                            ui.label("页面大小:");
                                            if let Some(ref mut pagesize) = source.pagesize {
                                                let mut val = pagesize.value.unwrap_or(2048);
                                                ui.add(egui::Slider::new(&mut val, 1..=65536));
                                                pagesize.value = Some(val);
                                                ui.end_row();
                                            }
                                        }

                                        if model_str == "nvdimm" {
                                            ui.label("对齐大小:");
                                            if let Some(ref mut alignsize) = source.alignsize {
                                                let mut val = alignsize.value.unwrap_or(2048);
                                                ui.add(egui::Slider::new(&mut val, 1..=65536));
                                                alignsize.value = Some(val);
                                                ui.end_row();
                                            }

                                            let mut has_pmem = source.pmem.is_some();
                                            if checkbox(ui, &mut has_pmem, "PMEM") {
                                                if has_pmem {
                                                    source.pmem = Some(());
                                                } else {
                                                    source.pmem = None;
                                                }
                                            }
                                            ui.end_row();
                                        }
                                    });
                                }
                            });

                            // Target 配置
                            ui.add_space(5.0);
                            ui.collapsing("目标配置", |ui| {
                                let model_str = mem_dev.model.as_deref().unwrap_or("dimm");
                                if let Some(ref mut target) = mem_dev.target {
                                    grid(ui, format!("mem_target_grid_{}", i), 2, |ui| {
                                        ui.label("大小:");
                                        if let Some(ref mut size) = target.size {
                                            let mut val = size.value.unwrap_or(524288);
                                            ui.add(egui::Slider::new(&mut val, 1024..=16777216));
                                            size.value = Some(val);
                                            ui.end_row();
                                        }

                                        ui.label("NUMA 节点:");
                                        let mut node = target.node.unwrap_or(0);
                                        ui.add(egui::Slider::new(&mut node, 0..=7));
                                        target.node = Some(node);
                                        ui.end_row();

                                        if model_str == "nvdimm" {
                                            let mut has_readonly = target.readonly.is_some();
                                            if checkbox(ui, &mut has_readonly, "只读") {
                                                if has_readonly {
                                                    target.readonly = Some(());
                                                } else {
                                                    target.readonly = None;
                                                }
                                            }
                                            ui.end_row();
                                        }

                                        if model_str == "virtio-mem" {
                                            ui.label("请求大小:");
                                            if let Some(ref mut requested) = target.requested {
                                                let mut val = requested.value.unwrap_or(1048576);
                                                ui.add(egui::Slider::new(
                                                    &mut val,
                                                    1024..=16777216,
                                                ));
                                                requested.value = Some(val);
                                                ui.end_row();
                                            }

                                            ui.label("当前大小:");
                                            if let Some(ref mut current) = target.current {
                                                let mut val = current.value.unwrap_or(524288);
                                                ui.add(egui::Slider::new(
                                                    &mut val,
                                                    1024..=16777216,
                                                ));
                                                current.value = Some(val);
                                                ui.end_row();
                                            }
                                        }
                                    });
                                }
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
                            ui.selectable_value(&mut iommu.model, "virtio".to_string(), "Virtio");
                        });
                    ui.end_row();
                });

                // Driver 配置
                ui.collapsing("Driver 配置", |ui| {
                    if iommu.driver.is_none() {
                        iommu.driver = Some(crate::model::devices::IommuDriver {
                            intremap: None,
                            caching_mode: None,
                            api_mode: None,
                            ats: None,
                            aw_bits: None,
                            snoop_wb: None,
                            x2apic_scale: None,
                        });
                    }
                    if let Some(ref mut driver) = iommu.driver {
                        grid(ui, "iommu_driver_grid", 2, |ui| {
                            ui.label("intremap:");
                            egui::ComboBox::from_id_source("iommu_intremap")
                                .selected_text(driver.intremap.as_deref().unwrap_or(""))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut driver.intremap, None, "");
                                    ui.selectable_value(
                                        &mut driver.intremap,
                                        Some("on".to_string()),
                                        "on",
                                    );
                                    ui.selectable_value(
                                        &mut driver.intremap,
                                        Some("off".to_string()),
                                        "off",
                                    );
                                });
                            ui.end_row();

                            ui.label("caching_mode:");
                            egui::ComboBox::from_id_source("iommu_caching_mode")
                                .selected_text(driver.caching_mode.as_deref().unwrap_or(""))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut driver.caching_mode, None, "");
                                    ui.selectable_value(
                                        &mut driver.caching_mode,
                                        Some("on".to_string()),
                                        "on",
                                    );
                                    ui.selectable_value(
                                        &mut driver.caching_mode,
                                        Some("off".to_string()),
                                        "off",
                                    );
                                });
                            ui.end_row();

                            ui.label("api_mode:");
                            egui::ComboBox::from_id_source("iommu_api_mode")
                                .selected_text(driver.api_mode.as_deref().unwrap_or(""))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut driver.api_mode, None, "");
                                    ui.selectable_value(
                                        &mut driver.api_mode,
                                        Some("scalable".to_string()),
                                        "scalable",
                                    );
                                    ui.selectable_value(
                                        &mut driver.api_mode,
                                        Some("legacy".to_string()),
                                        "legacy",
                                    );
                                });
                            ui.end_row();

                            ui.label("ats:");
                            egui::ComboBox::from_id_source("iommu_ats")
                                .selected_text(driver.ats.as_deref().unwrap_or(""))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut driver.ats, None, "");
                                    ui.selectable_value(
                                        &mut driver.ats,
                                        Some("on".to_string()),
                                        "on",
                                    );
                                    ui.selectable_value(
                                        &mut driver.ats,
                                        Some("off".to_string()),
                                        "off",
                                    );
                                });
                            ui.end_row();

                            ui.label("aw_bits:");
                            if let Some(ref mut aw_bits) = driver.aw_bits {
                                ui.text_edit_singleline(aw_bits);
                            } else {
                                let mut empty = String::new();
                                ui.text_edit_singleline(&mut empty);
                            }
                            ui.end_row();

                            ui.label("snoop_wb:");
                            egui::ComboBox::from_id_source("iommu_snoop_wb")
                                .selected_text(driver.snoop_wb.as_deref().unwrap_or(""))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut driver.snoop_wb, None, "");
                                    ui.selectable_value(
                                        &mut driver.snoop_wb,
                                        Some("on".to_string()),
                                        "on",
                                    );
                                    ui.selectable_value(
                                        &mut driver.snoop_wb,
                                        Some("off".to_string()),
                                        "off",
                                    );
                                });
                            ui.end_row();

                            ui.label("x2apic_scale:");
                            egui::ComboBox::from_id_source("iommu_x2apic_scale")
                                .selected_text(driver.x2apic_scale.as_deref().unwrap_or(""))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut driver.x2apic_scale, None, "");
                                    ui.selectable_value(
                                        &mut driver.x2apic_scale,
                                        Some("on".to_string()),
                                        "on",
                                    );
                                    ui.selectable_value(
                                        &mut driver.x2apic_scale,
                                        Some("off".to_string()),
                                        "off",
                                    );
                                });
                            ui.end_row();
                        });
                    }
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

                        ui.label("模式:");
                        egui::ComboBox::from_id_source("vsock_mode")
                            .selected_text(source.mode.as_deref().unwrap_or(""))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut source.mode, None, "");
                                ui.selectable_value(
                                    &mut source.mode,
                                    Some("host".to_string()),
                                    "host",
                                );
                                ui.selectable_value(
                                    &mut source.mode,
                                    Some("guest".to_string()),
                                    "guest",
                                );
                            });
                        ui.end_row();

                        ui.label("路径:");
                        if let Some(ref mut path) = source.path {
                            ui.text_edit_singleline(path);
                        } else {
                            let mut empty = String::new();
                            ui.text_edit_singleline(&mut empty);
                        }
                        ui.end_row();

                        ui.label("自动:");
                        egui::ComboBox::from_id_source("vsock_auto")
                            .selected_text(source.auto.as_deref().unwrap_or(""))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut source.auto, None, "");
                                ui.selectable_value(&mut source.auto, Some("on".to_string()), "on");
                                ui.selectable_value(
                                    &mut source.auto,
                                    Some("off".to_string()),
                                    "off",
                                );
                            });
                        ui.end_row();
                    }
                });

                // Address 配置
                ui.collapsing("Address 配置", |ui| {
                    if vsock.address.is_none() {
                        vsock.address = Some(crate::model::devices::AddressConfig {
                            address_type: "pci".to_string(),
                            domain: None,
                            bus: None,
                            slot: None,
                            function: None,
                            controller: None,
                            multifunction: None,
                            target: None,
                            unit: None,
                            reg: None,
                            cssid: None,
                            ssid: None,
                            devno: None,
                            iobase: None,
                            irq: None,
                        });
                    }
                    if let Some(ref mut address) = vsock.address {
                        grid(ui, "vsock_address_grid", 2, |ui| {
                            ui.label("地址类型:");
                            egui::ComboBox::from_id_source("vsock_address_type")
                                .selected_text(&address.address_type)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut address.address_type,
                                        "pci".to_string(),
                                        "pci",
                                    );
                                });
                            ui.end_row();

                            ui.label("Bus:");
                            if let Some(ref mut bus) = address.bus {
                                let mut bus_str = bus.to_string();
                                if ui.text_edit_singleline(&mut bus_str).changed() {
                                    if let Ok(val) = bus_str.parse() {
                                        *bus = val;
                                    }
                                }
                            }
                            ui.end_row();

                            ui.label("Slot:");
                            if let Some(ref mut slot) = address.slot {
                                let mut slot_str = slot.to_string();
                                if ui.text_edit_singleline(&mut slot_str).changed() {
                                    if let Ok(val) = slot_str.parse() {
                                        *slot = val;
                                    }
                                }
                            }
                            ui.end_row();

                            ui.label("Function:");
                            if let Some(ref mut function) = address.function {
                                let mut function_str = function.to_string();
                                if ui.text_edit_singleline(&mut function_str).changed() {
                                    if let Ok(val) = function_str.parse() {
                                        *function = val;
                                    }
                                }
                            }
                            ui.end_row();
                        });
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

                // Backend 配置
                ui.collapsing("Backend 配置", |ui| {
                    if let Some(ref mut backend) = crypto.backend {
                        grid(ui, "crypto_backend_grid", 2, |ui| {
                            ui.label("后端类型:");
                            egui::ComboBox::from_id_source("crypto_backend_type")
                                .selected_text(&backend.backend_type)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut backend.backend_type,
                                        "default".to_string(),
                                        "default",
                                    );
                                    ui.selectable_value(
                                        &mut backend.backend_type,
                                        "ceph".to_string(),
                                        "ceph",
                                    );
                                });
                            ui.end_row();

                            ui.label("节点:");
                            if let Some(ref mut node) = backend.node {
                                let mut node_str = node.to_string();
                                if ui.text_edit_singleline(&mut node_str).changed() {
                                    if let Ok(val) = node_str.parse() {
                                        *node = val;
                                    }
                                }
                            } else {
                                let mut empty = String::new();
                                ui.text_edit_singleline(&mut empty);
                            }
                            ui.end_row();
                        });
                    }
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

                // Size 配置
                ui.collapsing("Size 配置", |ui| {
                    if let Some(ref mut size) = pstore.size {
                        grid(ui, "pstore_size_grid", 2, |ui| {
                            ui.label("大小:");
                            let mut val = size.value.unwrap_or(64);
                            ui.add(egui::Slider::new(&mut val, 1..=1024));
                            size.value = Some(val);
                            ui.end_row();

                            ui.label("单位:");
                            let unit = size.unit.get_or_insert_with(|| "K".to_string());
                            egui::ComboBox::from_id_source("pstore_size_unit")
                                .selected_text(unit.as_str())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(unit, "K".to_string(), "K");
                                    ui.selectable_value(unit, "M".to_string(), "M");
                                });
                            ui.end_row();
                        });
                    }
                });
            }
        });
    }

    fn show_audio(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "Audio", Some("🔊"), colors, |ui| {
            let mut has_audio = config.devices.audio.is_some();
            if checkbox(ui, &mut has_audio, "启用 Audio") {
                if has_audio {
                    config.devices.audio = Some(AudioConfig {
                        id: None,
                        model: None,
                        source: None,
                        input: None,
                        output: None,
                        address: None,
                    });
                } else {
                    config.devices.audio = None;
                }
            }

            if let Some(ref mut audio) = config.devices.audio {
                ui.add_space(5.0);
                grid(ui, "audio_grid", 2, |ui| {
                    ui.label("模型:");
                    let model = audio.model.get_or_insert_with(|| "ich9".to_string());
                    egui::ComboBox::from_id_source("audio_model")
                        .selected_text(model.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(model, "ich6".to_string(), "ich6");
                            ui.selectable_value(model, "ich7".to_string(), "ich7");
                            ui.selectable_value(model, "ich9".to_string(), "ich9");
                            ui.selectable_value(model, "intel-hda".to_string(), "intel-hda");
                            ui.selectable_value(model, "ac97".to_string(), "ac97");
                        });
                    ui.end_row();
                });

                // Source 配置 (仅用于 intel-hda 模型)
                if audio.model.as_deref() == Some("intel-hda") {
                    ui.add_space(5.0);
                    ui.collapsing("Source 配置", |ui| {
                        let mut has_source = audio.source.is_some();
                        if checkbox(ui, &mut has_source, "启用 Source") {
                            if has_source {
                                audio.source = Some(crate::model::devices::AudioSource {
                                    mode: None,
                                    backend: Some(crate::model::devices::AudioSourceBackend {
                                        backend_type: "pulseaudio".to_string(),
                                        server: None,
                                        name: None,
                                        device: None,
                                        format: None,
                                        global: None,
                                    }),
                                });
                            } else {
                                audio.source = None;
                            }
                        }

                        if let Some(ref mut source) = audio.source {
                            ui.add_space(5.0);
                            grid(ui, "audio_source_grid", 2, |ui| {
                                ui.label("模式:");
                                let mode = source.mode.get_or_insert_with(|| "default".to_string());
                                egui::ComboBox::from_id_source("audio_source_mode")
                                    .selected_text(mode.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(mode, "default".to_string(), "default");
                                        ui.selectable_value(mode, "any".to_string(), "any");
                                    });
                                ui.end_row();

                                if let Some(ref mut backend) = source.backend {
                                    ui.label("后端类型:");
                                    egui::ComboBox::from_id_source("audio_backend_type")
                                        .selected_text(&backend.backend_type)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut backend.backend_type,
                                                "pulseaudio".to_string(),
                                                "PulseAudio",
                                            );
                                            ui.selectable_value(
                                                &mut backend.backend_type,
                                                "alsa".to_string(),
                                                "ALSA",
                                            );
                                            ui.selectable_value(
                                                &mut backend.backend_type,
                                                "coreaudio".to_string(),
                                                "CoreAudio",
                                            );
                                            ui.selectable_value(
                                                &mut backend.backend_type,
                                                "sdl".to_string(),
                                                "SDL",
                                            );
                                        });
                                    ui.end_row();
                                }
                            });
                        }
                    });
                }

                // Input 配置
                let mut has_input = audio.input.is_some();
                if checkbox(ui, &mut has_input, "启用输入") {
                    if has_input {
                        audio.input = Some(crate::model::devices::AudioStream {
                            stream_type: "default".to_string(),
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
                                    "default".to_string(),
                                    "default",
                                );
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
                            stream_type: "default".to_string(),
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
                                    "default".to_string(),
                                    "default",
                                );
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

                                if hostdev.device_type == "scsi" {
                                    ui.label("rawio:");
                                    let rawio = hostdev.rawio.clone().unwrap_or_else(|| "no".to_string());
                                    egui::ComboBox::from_id_source(format!("scsi_rawio_{}", i))
                                        .selected_text(&rawio)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(&mut hostdev.rawio, Some("yes".to_string()), "yes");
                                            ui.selectable_value(&mut hostdev.rawio, Some("no".to_string()), "no");
                                        });
                                    ui.end_row();

                                    ui.label("只读:");
                                    let readonly_checked = hostdev.readonly.is_some();
                                    let mut readonly_state = readonly_checked;
                                    if checkbox(ui, &mut readonly_state, "") {
                                        if readonly_state {
                                            hostdev.readonly = Some(());
                                        } else {
                                            hostdev.readonly = None;
                                        }
                                    }
                                    ui.end_row();

                                    ui.label("可共享:");
                                    let shareable_checked = hostdev.shareable.is_some();
                                    let mut shareable_state = shareable_checked;
                                    if checkbox(ui, &mut shareable_state, "") {
                                        if shareable_state {
                                            hostdev.shareable = Some(());
                                        } else {
                                            hostdev.shareable = None;
                                        }
                                    }
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

                                    // SCSI 设备配置
                                    if hostdev.device_type == "scsi" {
                                        // 检查是否有 protocol 属性（iSCSI 场景）
                                        let is_iscsi = source.protocol.as_ref().map_or(false, |p| p == "iscsi");

                                        ui.add_space(5.0);
                                        ui.horizontal(|ui| {
                                            ui.label("SCSI 类型:");
                                            let mut scsi_type = if is_iscsi { "iscsi".to_string() } else { "local".to_string() };
                                            egui::ComboBox::from_id_source(format!("scsi_type_{}", i))
                                                .selected_text(&scsi_type)
                                                .show_ui(ui, |ui| {
                                                    if ui.selectable_value(&mut scsi_type, "local".to_string(), "本地 SCSI").changed() {
                                                        source.protocol = None;
                                                        source.name = None;
                                                        source.host = None;
                                                        source.auth = None;
                                                        source.initiator = None;
                                                    }
                                                    if ui.selectable_value(&mut scsi_type, "iscsi".to_string(), "iSCSI 网络存储").changed() {
                                                        source.protocol = Some("iscsi".to_string());
                                                    }
                                                });
                                        });
                                        ui.add_space(5.0);

                                        if !is_iscsi {
                                            // 本地 SCSI 设备配置
                                            grid(ui, format!("scsi_local_source_{}", i), 2, |ui| {
                                                ui.label("rawio:");
                                                let rawio = hostdev.rawio.clone().unwrap_or_else(|| "no".to_string());
                                                egui::ComboBox::from_id_source(format!("scsi_rawio_{}", i))
                                                    .selected_text(&rawio)
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut hostdev.rawio, Some("yes".to_string()), "yes");
                                                        ui.selectable_value(&mut hostdev.rawio, Some("no".to_string()), "no");
                                                    });
                                                ui.end_row();

                                                ui.label("只读:");
                                                let readonly = hostdev.readonly.is_some();
                                                if checkbox(ui, &mut readonly.clone(), "") {
                                                    if readonly {
                                                        hostdev.readonly = Some(());
                                                    } else {
                                                        hostdev.readonly = None;
                                                    }
                                                }
                                                ui.end_row();

                                                ui.label("可共享:");
                                                let shareable = hostdev.shareable.is_some();
                                                if checkbox(ui, &mut shareable.clone(), "") {
                                                    if shareable {
                                                        hostdev.shareable = Some(());
                                                    } else {
                                                        hostdev.shareable = None;
                                                    }
                                                }
                                                ui.end_row();
                                            });

                                            ui.add_space(5.0);
                                            ui.collapsing("SCSI 适配器地址", |ui| {
                                                if source.adapter.is_none() {
                                                    source.adapter = Some(crate::model::devices::HostdevAdapter {
                                                        name: "scsi_host0".to_string(),
                                                    });
                                                }
                                                if let Some(ref mut adapter) = source.adapter {
                                                    ui.horizontal(|ui| {
                                                        ui.label("适配器名称:");
                                                        ui.text_edit_singleline(&mut adapter.name);
                                                    });
                                                }
                                            });

                                            ui.add_space(5.0);
                                            ui.collapsing("SCSI 总线地址", |ui| {
                                                if source.scsi_address.is_none() {
                                                    source.scsi_address = Some(crate::model::devices::HostdevSCSIAddress {
                                                        bus: "0".to_string(),
                                                        target: "0".to_string(),
                                                        unit: "0".to_string(),
                                                    });
                                                }
                                                if let Some(ref mut scsi_addr) = source.scsi_address {
                                                    grid(ui, format!("scsi_addr_grid_{}", i), 3, |ui| {
                                                        ui.label("bus:");
                                                        ui.text_edit_singleline(&mut scsi_addr.bus);
                                                        ui.label("target:");
                                                        ui.text_edit_singleline(&mut scsi_addr.target);
                                                        ui.label("unit:");
                                                        ui.text_edit_singleline(&mut scsi_addr.unit);
                                                    });
                                                }
                                            });
                                        } else {
                                            // iSCSI 网络存储配置
                                            grid(ui, format!("scsi_iscsi_source_{}", i), 2, |ui| {
                                                ui.label("IQN 名称:");
                                                let name = source.name.get_or_insert_with(|| "".to_string());
                                                ui.text_edit_singleline(name);
                                                ui.end_row();

                                                ui.label("主机:");
                                                if source.host.is_none() {
                                                    source.host = Some(crate::model::devices::HostdevHost {
                                                        name: "".to_string(),
                                                        port: Some("3260".to_string()),
                                                    });
                                                }
                                                if let Some(ref mut host) = source.host {
                                                    ui.text_edit_singleline(&mut host.name);
                                                }
                                                ui.end_row();

                                                ui.label("端口:");
                                                if let Some(ref mut host) = source.host {
                                                    let port = host.port.get_or_insert_with(|| "3260".to_string());
                                                    ui.text_edit_singleline(port);
                                                }
                                                ui.end_row();
                                            });

                                            ui.add_space(5.0);
                                            ui.collapsing("认证配置", |ui| {
                                                if source.auth.is_none() {
                                                    source.auth = Some(crate::model::devices::HostdevAuth {
                                                        username: "".to_string(),
                                                        secret: None,
                                                    });
                                                }
                                                if let Some(ref mut auth) = source.auth {
                                                    ui.horizontal(|ui| {
                                                        ui.label("用户名:");
                                                        ui.text_edit_singleline(&mut auth.username);
                                                    });
                                                    ui.add_space(5.0);
                                                    ui.collapsing("Secret 配置", |ui| {
                                                        if auth.secret.is_none() {
                                                            auth.secret = Some(crate::model::devices::HostdevSecret {
                                                                secret_type: "iscsi".to_string(),
                                                                usage: None,
                                                                uuid: None,
                                                            });
                                                        }
                                                        if let Some(ref mut secret) = auth.secret {
                                                            ui.horizontal(|ui| {
                                                                ui.label("类型:");
                                                                ui.text_edit_singleline(&mut secret.secret_type);
                                                            });
                                                            ui.horizontal(|ui| {
                                                                ui.label("Usage:");
                                                                let usage = secret.usage.get_or_insert_with(|| "".to_string());
                                                                ui.text_edit_singleline(usage);
                                                            });
                                                            ui.horizontal(|ui| {
                                                                ui.label("UUID:");
                                                                let uuid = secret.uuid.get_or_insert_with(|| "".to_string());
                                                                ui.text_edit_singleline(uuid);
                                                            });
                                                        }
                                                    });
                                                }
                                            });

                                            ui.add_space(5.0);
                                            ui.collapsing("发起者配置", |ui| {
                                                if source.initiator.is_none() {
                                                    source.initiator = Some(crate::model::devices::HostdevInitiator {
                                                        iqn: None,
                                                    });
                                                }
                                                if let Some(ref mut initiator) = source.initiator {
                                                    if initiator.iqn.is_none() {
                                                        initiator.iqn = Some(crate::model::devices::HostdevIQN {
                                                            name: "".to_string(),
                                                        });
                                                    }
                                                    if let Some(ref mut iqn) = initiator.iqn {
                                                        ui.horizontal(|ui| {
                                                            ui.label("发起者 IQN:");
                                                            ui.text_edit_singleline(&mut iqn.name);
                                                        });
                                                    }
                                                }
                                            });
                                        }
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

    fn show_filesystem(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "文件系统", Some("📁"), colors, |ui| {
            if config.devices.filesystem.is_none() {
                config.devices.filesystem = Some(Vec::new());
            }

            if let Some(ref mut fs_list) = config.devices.filesystem {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加文件系统", colors) {
                        fs_list.push(FilesystemConfig {
                            fs_type: "mount".to_string(),
                            accessmode: Some("passthrough".to_string()),
                            multidevs: None,
                            fmode: None,
                            dmode: None,
                            driver: None,
                            binary: None,
                            source: Some(FilesystemSource {
                                name: None,
                                dir: None,
                                file: None,
                                socket: None,
                                usage: None,
                                units: None,
                            }),
                            target: Some(FilesystemTarget { dir: "/mnt/share".to_string() }),
                            idmap: None,
                            readonly: None,
                            space_hard_limit: None,
                            space_soft_limit: None,
                        });
                    }
                });

                let mut to_remove = None;
                for (i, fs) in fs_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        inner_group(ui, colors, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("文件系统 {}", i + 1));
                                if delete_button(ui, None) {
                                    to_remove = Some(i);
                                }
                            });

                            grid(ui, format!("fs_grid_{}", i), 2, |ui| {
                                ui.label("类型:");
                                egui::ComboBox::from_id_source(format!("fs_type_{}", i))
                                    .selected_text(&fs.fs_type)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut fs.fs_type,
                                            "mount".to_string(),
                                            "mount",
                                        );
                                        ui.selectable_value(
                                            &mut fs.fs_type,
                                            "file".to_string(),
                                            "file",
                                        );
                                        ui.selectable_value(
                                            &mut fs.fs_type,
                                            "template".to_string(),
                                            "template",
                                        );
                                        ui.selectable_value(
                                            &mut fs.fs_type,
                                            "volume".to_string(),
                                            "volume",
                                        );
                                    });
                                ui.end_row();

                                ui.label("访问模式:");
                                let accessmode =
                                    fs.accessmode.get_or_insert_with(|| "passthrough".to_string());
                                egui::ComboBox::from_id_source(format!("fs_accessmode_{}", i))
                                    .selected_text(accessmode.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            accessmode,
                                            "passthrough".to_string(),
                                            "passthrough",
                                        );
                                        ui.selectable_value(
                                            accessmode,
                                            "mapped".to_string(),
                                            "mapped",
                                        );
                                        ui.selectable_value(accessmode, "none".to_string(), "none");
                                    });
                                ui.end_row();

                                ui.label("多设备:");
                                let multidevs =
                                    fs.multidevs.get_or_insert_with(|| "remap".to_string());
                                egui::ComboBox::from_id_source(format!("fs_multidevs_{}", i))
                                    .selected_text(multidevs.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            multidevs,
                                            "remap".to_string(),
                                            "remap",
                                        );
                                        ui.selectable_value(
                                            multidevs,
                                            "forbid".to_string(),
                                            "forbid",
                                        );
                                    });
                                ui.end_row();
                            });

                            // Source 配置
                            ui.collapsing("Source 配置", |ui| {
                                if fs.source.is_none() {
                                    fs.source = Some(crate::model::devices::FilesystemSource {
                                        name: None,
                                        dir: None,
                                        file: None,
                                        socket: None,
                                        usage: None,
                                        units: None,
                                    });
                                }
                                if let Some(ref mut source) = fs.source {
                                    grid(ui, format!("fs_source_grid_{}", i), 2, |ui| {
                                        ui.label("目录:");
                                        if let Some(ref mut dir) = source.dir {
                                            ui.text_edit_singleline(dir);
                                        } else {
                                            let mut empty = String::new();
                                            ui.text_edit_singleline(&mut empty);
                                        }
                                        ui.end_row();

                                        ui.label("文件:");
                                        if let Some(ref mut file) = source.file {
                                            ui.text_edit_singleline(file);
                                        } else {
                                            let mut empty = String::new();
                                            ui.text_edit_singleline(&mut empty);
                                        }
                                        ui.end_row();

                                        ui.label("名称:");
                                        if let Some(ref mut name) = source.name {
                                            ui.text_edit_singleline(name);
                                        } else {
                                            let mut empty = String::new();
                                            ui.text_edit_singleline(&mut empty);
                                        }
                                        ui.end_row();
                                    });
                                }
                            });

                            // Target 配置
                            ui.collapsing("Target 配置", |ui| {
                                if fs.target.is_none() {
                                    fs.target =
                                        Some(FilesystemTarget { dir: "/mnt/share".to_string() });
                                }
                                if let Some(ref mut target) = fs.target {
                                    grid(ui, format!("fs_target_grid_{}", i), 2, |ui| {
                                        ui.label("目标目录:");
                                        ui.text_edit_singleline(&mut target.dir);
                                        ui.end_row();
                                    });
                                }
                            });
                        });
                        ui.add_space(5.0);
                    });
                }

                if let Some(idx) = to_remove {
                    fs_list.remove(idx);
                }
            }
        });
    }

    fn show_smartcard(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "智能卡", Some("💳"), colors, |ui| {
            if config.devices.smartcard.is_none() {
                config.devices.smartcard = Some(Vec::new());
            }

            if let Some(ref mut smartcard_list) = config.devices.smartcard {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加智能卡", colors) {
                        smartcard_list.push(SmartcardConfig {
                            mode: "host".to_string(),
                            smartcard_type: None,
                            certificate: None,
                            database: None,
                            source: None,
                            protocol: None,
                            address: None,
                        });
                    }
                });

                let mut to_remove = None;
                for (i, smartcard) in smartcard_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        inner_group(ui, colors, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("智能卡 {}", i + 1));
                                if delete_button(ui, None) {
                                    to_remove = Some(i);
                                }
                            });

                            grid(ui, format!("smartcard_grid_{}", i), 2, |ui| {
                                ui.label("模式:");
                                egui::ComboBox::from_id_source(format!("smartcard_mode_{}", i))
                                    .selected_text(&smartcard.mode)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut smartcard.mode,
                                            "host".to_string(),
                                            "host (主机证书)",
                                        );
                                        ui.selectable_value(
                                            &mut smartcard.mode,
                                            "host-certificates".to_string(),
                                            "host-certificates (主机证书模式)",
                                        );
                                        ui.selectable_value(
                                            &mut smartcard.mode,
                                            "passthrough".to_string(),
                                            "passthrough (直通)",
                                        );
                                    });
                                ui.end_row();

                                if smartcard.mode == "passthrough" {
                                    ui.label("类型:");
                                    let smartcard_type = smartcard
                                        .smartcard_type
                                        .get_or_insert_with(|| "spicevmc".to_string());
                                    egui::ComboBox::from_id_source(format!("smartcard_type_{}", i))
                                        .selected_text(smartcard_type.as_str())
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                smartcard_type,
                                                "spicevmc".to_string(),
                                                "spicevmc",
                                            );
                                            ui.selectable_value(
                                                smartcard_type,
                                                "tcp".to_string(),
                                                "tcp",
                                            );
                                        });
                                    ui.end_row();
                                }

                                if smartcard.mode == "host-certificates" {
                                    ui.label("证书数量:");
                                    let cert_count =
                                        smartcard.certificate.as_ref().map_or(0, |c| c.len());
                                    ui.label(format!("{} 个证书", cert_count));
                                    ui.end_row();

                                    if cert_count == 0 && add_button(ui, "添加证书", colors) {
                                        if smartcard.certificate.is_none() {
                                            smartcard.certificate = Some(Vec::new());
                                        }
                                        smartcard.certificate.as_mut().unwrap().push(String::new());
                                    }

                                    if let Some(ref mut certs) = smartcard.certificate {
                                        for (j, cert) in certs.iter_mut().enumerate() {
                                            ui.label(format!("证书 {} 路径:", j + 1));
                                            ui.text_edit_singleline(cert);
                                        }
                                    }

                                    ui.label("数据库路径:");
                                    let database =
                                        smartcard.database.get_or_insert_with(String::new);
                                    ui.text_edit_singleline(database);
                                    ui.end_row();
                                }
                            });
                        });
                        ui.add_space(5.0);
                    });
                }

                if let Some(idx) = to_remove {
                    smartcard_list.remove(idx);
                }
            } else {
                ui.label("未启用智能卡");
            }
        });
    }

    fn show_nvram(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "NVRAM", Some("💾"), colors, |ui| {
            let mut has_nvram = config.devices.nvram.is_some();
            if checkbox(ui, &mut has_nvram, "启用 NVRAM (非易失性存储)") {
                if has_nvram {
                    config.devices.nvram =
                        Some(DeviceNVRAMConfig { template: None, nvram_source: None });
                } else {
                    config.devices.nvram = None;
                }
            }

            if let Some(ref mut nvram) = config.devices.nvram {
                ui.add_space(5.0);
                grid(ui, "nvram_grid", 2, |ui| {
                    ui.label("模板文件:");
                    let template = nvram
                        .template
                        .get_or_insert_with(|| "/usr/share/OVMF/OVMF_CODE.fd".to_string());
                    ui.text_edit_singleline(template);
                    ui.end_row();

                    ui.label("存储文件:");
                    let source = nvram
                        .nvram_source
                        .get_or_insert_with(|| "/var/lib/libvirt/qemu/nvram_VARS.fd".to_string());
                    ui.text_edit_singleline(source);
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
                iothread: None,
                iothreads: None,
                statistics: None,
                latency_histogram: None,
                discard_no_unref: None,
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
            alias: Some(AliasConfig { name: format!("disk{}", index) }),
            boot: None,
            shareable: None,
            transient: None,
            encryption: None,
            serial: None,
            wwn: None,
            vendor: None,
        }
    }

    fn create_default_interface(index: usize) -> InterfaceConfig {
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
            alias: Some(AliasConfig { name: format!("net{}", index) }),
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

    fn show_redirdev(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "USB 重定向", Some("🔌"), colors, |ui| {
            if config.devices.redirdev.is_none() {
                config.devices.redirdev = Some(Vec::new());
            }

            if let Some(ref mut redirdev_list) = config.devices.redirdev {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加 USB 重定向设备", colors) {
                        redirdev_list.push(RedirdevConfig {
                            bus: "usb".to_string(),
                            redir_type: "spicevmc".to_string(),
                            source: None,
                            protocol: None,
                            address: None,
                            boot: None,
                            alias: None,
                        });
                    }
                });

                let mut to_remove = None;
                for (i, redirdev) in redirdev_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        inner_group(ui, colors, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("USB 重定向设备 {}", i + 1));
                                if delete_button(ui, None) {
                                    to_remove = Some(i);
                                }
                            });

                            grid(ui, format!("redirdev_grid_{}", i), 2, |ui| {
                                ui.label("类型:");
                                egui::ComboBox::from_id_source(format!("redirdev_type_{}", i))
                                    .selected_text(&redirdev.redir_type)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut redirdev.redir_type,
                                            "spicevmc".to_string(),
                                            "spicevmc (SPICE VM 通道)",
                                        );
                                        ui.selectable_value(
                                            &mut redirdev.redir_type,
                                            "tcp".to_string(),
                                            "tcp (TCP 网络)",
                                        );
                                    });
                                ui.end_row();

                                // Alias 配置
                                if redirdev.alias.is_none() {
                                    redirdev.alias = Some(AliasConfig { name: "".to_string() });
                                }
                                if let Some(ref mut alias) = redirdev.alias {
                                    ui.label("设备别名 (alias):");
                                    ui.text_edit_singleline(&mut alias.name);
                                    ui.end_row();
                                }

                                if redirdev.redir_type == "tcp" {
                                    ui.label("连接模式:");
                                    let source = redirdev.source.get_or_insert_with(|| {
                                        crate::model::devices::RedirdevSource {
                                            mode: Some("connect".to_string()),
                                            host: Some("localhost".to_string()),
                                            service: Some("4000".to_string()),
                                        }
                                    });
                                    let mode =
                                        source.mode.get_or_insert_with(|| "connect".to_string());
                                    egui::ComboBox::from_id_source(format!("redirdev_mode_{}", i))
                                        .selected_text(mode.as_str())
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                mode,
                                                "connect".to_string(),
                                                "connect (客户端)",
                                            );
                                            ui.selectable_value(
                                                mode,
                                                "bind".to_string(),
                                                "bind (服务端)",
                                            );
                                        });
                                    ui.end_row();

                                    ui.label("主机:");
                                    let host =
                                        source.host.get_or_insert_with(|| "localhost".to_string());
                                    ui.text_edit_singleline(host);
                                    ui.end_row();

                                    ui.label("端口:");
                                    let service =
                                        source.service.get_or_insert_with(|| "4000".to_string());
                                    ui.text_edit_singleline(service);
                                    ui.end_row();
                                }
                            });
                        });
                        ui.add_space(5.0);
                    });
                }

                if let Some(idx) = to_remove {
                    redirdev_list.remove(idx);
                }
            }
        });
    }

    fn show_redirfilter(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "USB 重定向过滤器", Some("🔒"), colors, |ui| {
            let mut has_redirfilter = config.devices.redirfilter.is_some();
            if checkbox(ui, &mut has_redirfilter, "启用 USB 重定向过滤器") {
                if has_redirfilter {
                    config.devices.redirfilter = Some(RedirfilterConfig {
                        usb_devices: vec![crate::model::devices::UsbDevFilter {
                            allow: "yes".to_string(),
                            class: None,
                            vendor: None,
                            product: None,
                            version: None,
                        }],
                    });
                } else {
                    config.devices.redirfilter = None;
                }
            }

            if let Some(ref mut redirfilter) = config.devices.redirfilter {
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加过滤规则", colors) {
                        redirfilter.usb_devices.push(crate::model::devices::UsbDevFilter {
                            allow: "yes".to_string(),
                            class: None,
                            vendor: None,
                            product: None,
                            version: None,
                        });
                    }
                });

                let mut to_remove = None;
                for (i, usb_dev) in redirfilter.usb_devices.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        inner_group(ui, colors, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("规则 {}", i + 1));
                                if delete_button(ui, None) {
                                    to_remove = Some(i);
                                }
                            });

                            grid(ui, format!("redirfilter_grid_{}", i), 2, |ui| {
                                ui.label("操作:");
                                egui::ComboBox::from_id_source(format!("redirfilter_allow_{}", i))
                                    .selected_text(if usb_dev.allow == "yes" {
                                        "允许"
                                    } else {
                                        "拒绝"
                                    })
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut usb_dev.allow,
                                            "yes".to_string(),
                                            "允许",
                                        );
                                        ui.selectable_value(
                                            &mut usb_dev.allow,
                                            "no".to_string(),
                                            "拒绝",
                                        );
                                    });
                                ui.end_row();

                                ui.label("设备类 (Class):");
                                let class = usb_dev.class.get_or_insert_with(String::new);
                                ui.text_edit_singleline(class);
                                ui.end_row();

                                ui.label("Vendor ID:");
                                let vendor = usb_dev.vendor.get_or_insert_with(String::new);
                                ui.text_edit_singleline(vendor);
                                ui.end_row();

                                ui.label("Product ID:");
                                let product = usb_dev.product.get_or_insert_with(String::new);
                                ui.text_edit_singleline(product);
                                ui.end_row();

                                ui.label("版本:");
                                let version = usb_dev.version.get_or_insert_with(String::new);
                                ui.text_edit_singleline(version);
                                ui.end_row();
                            });

                            ui.label("注意：-1 或留空表示匹配任意值");
                        });
                        ui.add_space(5.0);
                    });
                }

                if let Some(idx) = to_remove {
                    redirfilter.usb_devices.remove(idx);
                }
            }
        });
    }

    fn show_lease(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "租约设备", Some("🔐"), colors, |ui| {
            let mut has_lease = config.devices.lease.is_some();
            if checkbox(ui, &mut has_lease, "启用租约设备") {
                if has_lease {
                    config.devices.lease = Some(LeaseConfig {
                        lockspace: Some("somearea".to_string()),
                        key: Some("somekey".to_string()),
                        target: Some(crate::model::devices::LeaseTarget {
                            path: "/some/lease/path".to_string(),
                            offset: Some(1024),
                        }),
                    });
                } else {
                    config.devices.lease = None;
                }
            }

            if let Some(ref mut lease) = config.devices.lease {
                ui.add_space(5.0);
                grid(ui, "lease_grid", 2, |ui| {
                    ui.label("锁空间:");
                    let lockspace = lease.lockspace.get_or_insert_with(|| "somearea".to_string());
                    ui.text_edit_singleline(lockspace);
                    ui.end_row();

                    ui.label("锁键:");
                    let key = lease.key.get_or_insert_with(|| "somekey".to_string());
                    ui.text_edit_singleline(key);
                    ui.end_row();

                    if let Some(ref mut target) = lease.target {
                        ui.label("文件路径:");
                        ui.text_edit_singleline(&mut target.path);
                        ui.end_row();

                        ui.label("偏移量:");
                        let mut offset = target.offset.unwrap_or(1024) as i64;
                        ui.add(egui::Slider::new(&mut offset, 0..=1024 * 1024).text("bytes"));
                        target.offset = Some(offset as u64);
                        ui.end_row();
                    }
                });
            }
        });
    }

    fn show_controller(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "控制器设备", Some("🎮"), colors, |ui| {
            if config.devices.controller.is_none() {
                config.devices.controller = Some(Vec::new());
            }

            if let Some(ref mut controller_list) = config.devices.controller {
                ui.horizontal(|ui| {
                    if add_button(ui, "➕ 添加控制器", colors) {
                        controller_list.push(ControllerConfig {
                            controller_type: "usb".to_string(),
                            index: Some(0),
                            model: Some("qemu-xhci".to_string()),
                            ports: None,
                            vectors: None,
                            max_grant_frames: None,
                            max_event_channels: None,
                            driver: None,
                            master: None,
                            model_elem: None,
                            target: None,
                            pcihole64: None,
                            serial: None,
                            hotplug: None,
                            address: None,
                        });
                    }
                });

                let mut to_remove = None;
                for (i, controller) in controller_list.iter_mut().enumerate() {
                    ui.push_id(i, |ui| {
                        inner_group(ui, colors, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("控制器 {}", i + 1));
                                if delete_button(ui, None) {
                                    to_remove = Some(i);
                                }
                            });

                            grid(ui, format!("controller_grid_{}", i), 2, |ui| {
                                ui.label("类型:");
                                egui::ComboBox::from_id_source(format!("controller_type_{}", i))
                                    .selected_text(&controller.controller_type)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut controller.controller_type,
                                            "usb".to_string(),
                                            "USB",
                                        );
                                        ui.selectable_value(
                                            &mut controller.controller_type,
                                            "pci".to_string(),
                                            "PCI",
                                        );
                                        ui.selectable_value(
                                            &mut controller.controller_type,
                                            "pci-expander-bridge".to_string(),
                                            "PCI 扩展桥",
                                        );
                                        ui.selectable_value(
                                            &mut controller.controller_type,
                                            "virtio-serial".to_string(),
                                            "VirtIO 串口",
                                        );
                                        ui.selectable_value(
                                            &mut controller.controller_type,
                                            "scsi".to_string(),
                                            "SCSI",
                                        );
                                        ui.selectable_value(
                                            &mut controller.controller_type,
                                            "sata".to_string(),
                                            "SATA",
                                        );
                                        ui.selectable_value(
                                            &mut controller.controller_type,
                                            "ide".to_string(),
                                            "IDE",
                                        );
                                        ui.selectable_value(
                                            &mut controller.controller_type,
                                            "fdc".to_string(),
                                            "软盘控制器",
                                        );
                                    });
                                ui.end_row();

                                ui.label("型号:");
                                let model =
                                    controller.model.get_or_insert_with(|| "qemu-xhci".to_string());
                                ui.text_edit_singleline(model);
                                ui.end_row();

                                ui.label("索引:");
                                let mut index = controller.index.unwrap_or(0) as i32;
                                ui.add(
                                    egui::DragValue::new(&mut index)
                                        .clamp_range(0..=255)
                                        .suffix(" #"),
                                );
                                controller.index = Some(index as u32);
                                ui.end_row();

                                ui.label("端口数:");
                                let mut ports = controller.ports.unwrap_or(0) as i32;
                                ui.add(
                                    egui::DragValue::new(&mut ports)
                                        .clamp_range(0..=64)
                                        .suffix(" 个"),
                                );
                                controller.ports = Some(ports as u32);
                                ui.end_row();
                            });

                            // Address 配置
                            ui.collapsing("Address 配置", |ui| {
                                if controller.address.is_none() {
                                    controller.address = Some(AddressConfig {
                                        address_type: "pci".to_string(),
                                        domain: Some("0x0000".to_string()),
                                        bus: Some(0),
                                        slot: Some(0),
                                        function: Some(0),
                                        controller: None,
                                        multifunction: None,
                                        target: None,
                                        unit: None,
                                        reg: None,
                                        cssid: None,
                                        ssid: None,
                                        devno: None,
                                        iobase: None,
                                        irq: None,
                                    });
                                }
                                if let Some(ref mut addr) = controller.address {
                                    grid(ui, format!("controller_addr_grid_{}", i), 2, |ui| {
                                        ui.label("类型:");
                                        egui::ComboBox::from_id_source(format!(
                                            "controller_addr_type_{}",
                                            i
                                        ))
                                        .selected_text(&addr.address_type)
                                        .show_ui(
                                            ui,
                                            |ui| {
                                                ui.selectable_value(
                                                    &mut addr.address_type,
                                                    "pci".to_string(),
                                                    "PCI",
                                                );
                                                ui.selectable_value(
                                                    &mut addr.address_type,
                                                    "ccid".to_string(),
                                                    "CCID",
                                                );
                                                ui.selectable_value(
                                                    &mut addr.address_type,
                                                    "virtio-ccw".to_string(),
                                                    "VirtIO CCW",
                                                );
                                            },
                                        );
                                        ui.end_row();

                                        ui.label("域:");
                                        if let Some(ref mut domain) = addr.domain {
                                            ui.text_edit_singleline(domain);
                                        } else {
                                            let mut empty = String::new();
                                            ui.text_edit_singleline(&mut empty);
                                        }
                                        ui.end_row();

                                        ui.label("总线:");
                                        let mut bus = addr.bus.unwrap_or(0) as i32;
                                        ui.add(
                                            egui::DragValue::new(&mut bus)
                                                .clamp_range(0..=255)
                                                .suffix(" #"),
                                        );
                                        addr.bus = Some(bus as u32);
                                        ui.end_row();

                                        ui.label("插槽:");
                                        let mut slot = addr.slot.unwrap_or(0) as i32;
                                        ui.add(
                                            egui::DragValue::new(&mut slot)
                                                .clamp_range(0..=255)
                                                .suffix(" #"),
                                        );
                                        addr.slot = Some(slot as u32);
                                        ui.end_row();

                                        ui.label("功能:");
                                        let mut function = addr.function.unwrap_or(0) as i32;
                                        ui.add(
                                            egui::DragValue::new(&mut function)
                                                .clamp_range(0..=7)
                                                .suffix(" #"),
                                        );
                                        addr.function = Some(function as u32);
                                        ui.end_row();
                                    });
                                }
                            });

                            // Driver 配置
                            ui.collapsing("驱动配置", |ui| {
                                if controller.driver.is_none() {
                                    controller.driver = Some(ControllerDriver {
                                        driver_type: None,
                                        queues: None,
                                        cmd_per_lun: None,
                                        max_sectors: None,
                                        ioeventfd: None,
                                        iothread: None,
                                        iothreads: None,
                                    });
                                }
                                if let Some(ref mut driver) = controller.driver {
                                    grid(ui, format!("controller_driver_grid_{}", i), 2, |ui| {
                                        ui.label("队列数:");
                                        let mut queues = driver.queues.unwrap_or(0) as i32;
                                        ui.add(
                                            egui::DragValue::new(&mut queues)
                                                .clamp_range(0..=256)
                                                .suffix(" #"),
                                        );
                                        driver.queues = Some(queues as u32);
                                        ui.end_row();

                                        ui.label("每 LUN 命令数:");
                                        let mut cmd = driver.cmd_per_lun.unwrap_or(0) as i32;
                                        ui.add(
                                            egui::DragValue::new(&mut cmd)
                                                .clamp_range(0..=65535)
                                                .suffix(" #"),
                                        );
                                        driver.cmd_per_lun = Some(cmd as u32);
                                        ui.end_row();

                                        ui.label("最大扇区数:");
                                        let mut sectors = driver.max_sectors.unwrap_or(0) as i32;
                                        ui.add(
                                            egui::DragValue::new(&mut sectors)
                                                .clamp_range(0..=65535)
                                                .suffix(" #"),
                                        );
                                        driver.max_sectors = Some(sectors as u32);
                                        ui.end_row();

                                        ui.label("IO 线程:");
                                        let mut iothread = driver.iothread.unwrap_or(0) as i32;
                                        ui.add(
                                            egui::DragValue::new(&mut iothread)
                                                .clamp_range(0..=255)
                                                .suffix(" #"),
                                        );
                                        driver.iothread = Some(iothread as u32);
                                        ui.end_row();
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label("IO 事件 fd:");
                                        let ioeventfd = driver
                                            .ioeventfd
                                            .get_or_insert_with(|| "auto".to_string());
                                        egui::ComboBox::from_id_source(format!(
                                            "controller_ioeventfd_{}",
                                            i
                                        ))
                                        .selected_text(ioeventfd.as_str())
                                        .show_ui(
                                            ui,
                                            |ui| {
                                                ui.selectable_value(
                                                    ioeventfd,
                                                    "auto".to_string(),
                                                    "auto",
                                                );
                                                ui.selectable_value(
                                                    ioeventfd,
                                                    "on".to_string(),
                                                    "on",
                                                );
                                                ui.selectable_value(
                                                    ioeventfd,
                                                    "off".to_string(),
                                                    "off",
                                                );
                                            },
                                        );
                                    });
                                }
                            });

                            // Hotplug 配置
                            ui.collapsing("热插拔配置", |ui| {
                                let mut has_hotplug = controller.hotplug.is_some();
                                if checkbox(ui, &mut has_hotplug, "启用热插拔") {
                                    if has_hotplug {
                                        controller.hotplug =
                                            Some(ControllerHotplug { enabled: "on".to_string() });
                                    } else {
                                        controller.hotplug = None;
                                    }
                                }
                                if let Some(ref mut hotplug) = controller.hotplug {
                                    ui.horizontal(|ui| {
                                        ui.label("状态:");
                                        egui::ComboBox::from_id_source(format!(
                                            "controller_hotplug_{}",
                                            i
                                        ))
                                        .selected_text(&hotplug.enabled)
                                        .show_ui(
                                            ui,
                                            |ui| {
                                                ui.selectable_value(
                                                    &mut hotplug.enabled,
                                                    "on".to_string(),
                                                    "on",
                                                );
                                                ui.selectable_value(
                                                    &mut hotplug.enabled,
                                                    "off".to_string(),
                                                    "off",
                                                );
                                            },
                                        );
                                    });
                                }
                            });
                        });
                    });
                }

                if let Some(idx) = to_remove {
                    controller_list.remove(idx);
                }
            }
        });
    }

    fn show_memballoon(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "内存气球", Some("🎈"), colors, |ui| {
            let mut has_memballoon = config.devices.memballoon.is_some();
            if checkbox(ui, &mut has_memballoon, "启用内存气球") {
                if has_memballoon {
                    config.devices.memballoon = Some(MemballoonConfig {
                        model: "virtio".to_string(),
                        autodeflate: None,
                        period: None,
                        stats: None,
                    });
                } else {
                    config.devices.memballoon = None;
                }
            }

            if let Some(ref mut memballoon) = config.devices.memballoon {
                ui.add_space(5.0);
                grid(ui, "memballoon_grid", 2, |ui| {
                    ui.label("模型:");
                    egui::ComboBox::from_id_source("memballoon_model")
                        .selected_text(&memballoon.model)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut memballoon.model,
                                "virtio".to_string(),
                                "VirtIO",
                            );
                            ui.selectable_value(&mut memballoon.model, "none".to_string(), "None");
                        });
                    ui.end_row();

                    ui.label("自动放气:");
                    let autodeflate =
                        memballoon.autodeflate.get_or_insert_with(|| "off".to_string());
                    egui::ComboBox::from_id_source("memballoon_autodeflate")
                        .selected_text(autodeflate.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(autodeflate, "on".to_string(), "on");
                            ui.selectable_value(autodeflate, "off".to_string(), "off");
                        });
                    ui.end_row();

                    ui.label("周期:");
                    let mut period = memballoon.period.unwrap_or(0) as i32;
                    ui.add(egui::DragValue::new(&mut period).clamp_range(0..=1000).suffix(" ms"));
                    memballoon.period = Some(period as u32);
                    ui.end_row();
                });

                ui.add_space(5.0);
                ui.collapsing("统计配置", |ui| {
                    let mut has_stats = memballoon.stats.is_some();
                    if checkbox(ui, &mut has_stats, "启用统计") {
                        if has_stats {
                            memballoon.stats =
                                Some(crate::model::devices::MemballoonStats { period: Some(10) });
                        } else {
                            memballoon.stats = None;
                        }
                    }
                    if let Some(ref mut stats) = memballoon.stats {
                        ui.horizontal(|ui| {
                            ui.label("统计周期:");
                            let mut period = stats.period.unwrap_or(10) as i32;
                            ui.add(
                                egui::DragValue::new(&mut period)
                                    .clamp_range(0..=1000)
                                    .suffix(" s"),
                            );
                            stats.period = Some(period as u32);
                        });
                    }
                });
            }
        });
    }

    fn show_emulator(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "模拟器路径", Some("🔧"), colors, |ui| {
            ui.horizontal(|ui| {
                ui.label("模拟器路径:");
                let emulator = config
                    .devices
                    .emulator
                    .get_or_insert_with(|| "/usr/bin/qemu-system-x86_64".to_string());
                ui.text_edit_singleline(emulator);
            });
        });
    }

    fn show_sysinfo(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        card_group(ui, "SysInfo 配置", Some("📋"), colors, |ui| {
            let mut has_sysinfo = config.devices.sysinfo.is_some();
            if checkbox(ui, &mut has_sysinfo, "启用 SysInfo") {
                if has_sysinfo {
                    config.devices.sysinfo = Some(SysInfoConfig {
                        sysinfo_type: "smbios".to_string(),
                        bios: None,
                        system: None,
                        base_board: None,
                        chassis: None,
                        oem_strings: None,
                        entry: None,
                    });
                } else {
                    config.devices.sysinfo = None;
                }
            }

            if let Some(ref mut sysinfo) = config.devices.sysinfo {
                ui.add_space(5.0);
                grid(ui, "sysinfo_grid", 2, |ui| {
                    ui.label("类型:");
                    egui::ComboBox::from_id_source("sysinfo_type")
                        .selected_text(&sysinfo.sysinfo_type)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut sysinfo.sysinfo_type,
                                "smbios".to_string(),
                                "SMBIOS",
                            );
                            ui.selectable_value(
                                &mut sysinfo.sysinfo_type,
                                "hwid".to_string(),
                                "HWID",
                            );
                            ui.selectable_value(
                                &mut sysinfo.sysinfo_type,
                                "vmid".to_string(),
                                "VMID",
                            );
                        });
                    ui.end_row();
                });

                ui.add_space(5.0);

                // BIOS 配置
                ui.collapsing("BIOS 配置", |ui| {
                    let mut has_bios = sysinfo.bios.is_some();
                    if checkbox(ui, &mut has_bios, "启用 BIOS") {
                        if has_bios {
                            sysinfo.bios = Some(crate::model::devices::SMBIOSBlock {
                                entry: Some(vec![SMBIOSEntry {
                                    name: "vendor".to_string(),
                                    value: "".to_string(),
                                }]),
                            });
                        } else {
                            sysinfo.bios = None;
                        }
                    }
                    if let Some(ref mut bios) = sysinfo.bios {
                        if let Some(ref mut entry_list) = bios.entry {
                            let mut to_remove = None;
                            for (i, entry) in entry_list.iter_mut().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{}:", i + 1));
                                    ui.text_edit_singleline(&mut entry.name);
                                    ui.text_edit_singleline(&mut entry.value);
                                    if ui.small_button("❌").clicked() {
                                        to_remove = Some(i);
                                    }
                                });
                            }
                            if let Some(idx) = to_remove {
                                entry_list.remove(idx);
                            }
                            if add_button(ui, "添加条目", colors) {
                                entry_list.push(SMBIOSEntry {
                                    name: String::new(),
                                    value: String::new(),
                                });
                            }
                        }
                    }
                });

                ui.add_space(5.0);

                // System 配置
                ui.collapsing("System 配置", |ui| {
                    let mut has_system = sysinfo.system.is_some();
                    if checkbox(ui, &mut has_system, "启用 System") {
                        if has_system {
                            sysinfo.system = Some(crate::model::devices::SMBIOSBlock {
                                entry: Some(vec![SMBIOSEntry {
                                    name: "manufacturer".to_string(),
                                    value: "".to_string(),
                                }]),
                            });
                        } else {
                            sysinfo.system = None;
                        }
                    }
                    if let Some(ref mut system) = sysinfo.system {
                        if let Some(ref mut entry_list) = system.entry {
                            let mut to_remove = None;
                            for (i, entry) in entry_list.iter_mut().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{}:", i + 1));
                                    ui.text_edit_singleline(&mut entry.name);
                                    ui.text_edit_singleline(&mut entry.value);
                                    if ui.small_button("❌").clicked() {
                                        to_remove = Some(i);
                                    }
                                });
                            }
                            if let Some(idx) = to_remove {
                                entry_list.remove(idx);
                            }
                            if add_button(ui, "添加条目", colors) {
                                entry_list.push(SMBIOSEntry {
                                    name: String::new(),
                                    value: String::new(),
                                });
                            }
                        }
                    }
                });

                ui.add_space(5.0);

                // OEM Strings 配置
                ui.collapsing("OEM Strings 配置", |ui| {
                    let mut has_oem = sysinfo.oem_strings.is_some();
                    if checkbox(ui, &mut has_oem, "启用 OEM Strings") {
                        if has_oem {
                            sysinfo.oem_strings = Some(vec![crate::model::devices::OemString {
                                value: "".to_string(),
                            }]);
                        } else {
                            sysinfo.oem_strings = None;
                        }
                    }
                    if let Some(ref mut oem_strings) = sysinfo.oem_strings {
                        let mut to_remove = None;
                        for (i, oem_string) in oem_strings.iter_mut().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}:", i + 1));
                                ui.text_edit_singleline(&mut oem_string.value);
                                if ui.small_button("❌").clicked() {
                                    to_remove = Some(i);
                                }
                            });
                        }
                        if let Some(idx) = to_remove {
                            oem_strings.remove(idx);
                        }
                        if add_button(ui, "添加字符串", colors) {
                            oem_strings
                                .push(crate::model::devices::OemString { value: String::new() });
                        }
                    }
                });
            }
        });
    }
}
