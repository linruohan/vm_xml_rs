use egui::{Color32, RichText};

use crate::{
    model::vm_config::VMConfig,
    panels::{
        BlockIOTuningPanel, CPUPanel, CPUTuningPanel, DevicesPanel, DiskThrottleGroupPanel,
        EventsPanel, FibreChannelVMIDPanel, GeneralPanel, GraphicsPanel, HypervisorFeaturesPanel,
        IOThreadsPanel, InputPanel, KeyWrapPanel, LaunchSecurityPanel, MemoryPanel, MemoryTuningPanel,
        NetworkPanel, NUMAPanel, OSPanel, ParallelPanel, PCIPanel, PerformanceMonitoringPanel,
        PowerManagementPanel, ResourcePartitioningPanel, SerialPanel, SMBIOSPanel, SecurityLabelPanel,
        SoundPanel, TimeKeepingPanel, USBPanel,
    },
    xml_gen::XMLGenerator,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Tab {
    #[default]
    General,
    OS,
    Cpu,
    Memory,
    Devices,
    AdvancedSMBIOS,
    AdvancedIOThreads,
    AdvancedCPUTuning,
    AdvancedMemoryTuning,
    AdvancedNUMA,
    AdvancedBlockIO,
    AdvancedResource,
    AdvancedFCVMID,
    AdvancedEvents,
    AdvancedPower,
    AdvancedDiskThrottle,
    AdvancedHypervisor,
    AdvancedTime,
    AdvancedPerformance,
    AdvancedSecurity,
    AdvancedKeyWrap,
    AdvancedLaunchSecurity,
    AdvancedNetwork,
    AdvancedSound,
    AdvancedGraphics,
    AdvancedInput,
    AdvancedSerial,
    AdvancedParallel,
    AdvancedUSB,
    AdvancedPCI,
}

pub struct VMConfigApp {
    config: VMConfig,
    current_tab: Tab,
    generated_xml: String,
    show_xml_preview: bool,
    status_message: Option<(String, bool)>,
}

impl VMConfigApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            config: VMConfig::new(),
            current_tab: Tab::default(),
            generated_xml: String::new(),
            show_xml_preview: false,
            status_message: None,
        }
    }

    fn show_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("文件", |ui| {
                if ui.button("新建配置").clicked() {
                    self.config = VMConfig::new();
                    self.status_message = Some(("已创建新配置".to_string(), true));
                    ui.close_menu();
                }
                if ui.button("导出 XML...").clicked() {
                    if let Err(e) = self.export_xml() {
                        self.status_message = Some((format!("导出失败：{}", e), false));
                    } else {
                        self.status_message = Some(("XML 已成功导出!".to_string(), true));
                    }
                    ui.close_menu();
                }
                if ui.button("复制 XML 到剪贴板").clicked() {
                    match XMLGenerator::generate(&self.config) {
                        Ok(xml) => {
                            ui.output_mut(|o| o.copied_text = xml.clone());
                            self.generated_xml = xml;
                            self.status_message = Some(("XML 已复制到剪贴板!".to_string(), true));
                        },
                        Err(e) => {
                            self.status_message = Some((format!("生成失败：{}", e), false));
                        },
                    }
                    ui.close_menu();
                }
            });

            ui.menu_button("编辑", |ui| {
                if ui.button("重置为默认值").clicked() {
                    self.config = VMConfig::new();
                    self.status_message = Some(("已重置为默认值".to_string(), true));
                    ui.close_menu();
                }
            });

            ui.menu_button("帮助", |ui| {
                if ui.button("关于").clicked() {
                    ui.close_menu();
                }
                ui.hyperlink_to("libvirt 文档", "https://www.libvirt.org/formatdomain.html");
            });
        });
    }

    fn show_tabs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            let base_tabs = [
                (Tab::General, "⚙ 基础配置"),
                (Tab::OS, "💿 系统引导"),
                (Tab::Cpu, "🖥 CPU"),
                (Tab::Memory, "💾 内存"),
                (Tab::Devices, "🔌 设备"),
            ];

            let advanced_tabs = [
                (Tab::AdvancedSMBIOS, "🔬 SMBIOS"),
                (Tab::AdvancedIOThreads, "🔄 IO 线程"),
                (Tab::AdvancedCPUTuning, "⚡ CPU 调优"),
                (Tab::AdvancedMemoryTuning, "📊 内存调优"),
                (Tab::AdvancedNUMA, "🔢 NUMA"),
                (Tab::AdvancedBlockIO, "💽 块 IO"),
                (Tab::AdvancedResource, "📦 资源分区"),
                (Tab::AdvancedFCVMID, "🔗 FC VMID"),
                (Tab::AdvancedEvents, "📅 事件"),
                (Tab::AdvancedPower, "🔋 电源"),
                (Tab::AdvancedDiskThrottle, "⏱ 磁盘限流"),
                (Tab::AdvancedHypervisor, "🛡 虚拟机监控"),
                (Tab::AdvancedTime, "⏰ 时间同步"),
                (Tab::AdvancedPerformance, "📈 性能监控"),
                (Tab::AdvancedSecurity, "🔒 安全标签"),
                (Tab::AdvancedKeyWrap, " 密钥封装"),
                (Tab::AdvancedLaunchSecurity, "🚀 启动安全"),
                (Tab::AdvancedNetwork, "🌐 网络"),
                (Tab::AdvancedSound, "🔊 声音"),
                (Tab::AdvancedGraphics, "🎨 图形"),
                (Tab::AdvancedInput, "⌨ 输入设备"),
                (Tab::AdvancedSerial, "📟 串口"),
                (Tab::AdvancedParallel, "🖨 并口"),
                (Tab::AdvancedUSB, "🔌 USB"),
                (Tab::AdvancedPCI, "💻 PCI"),
            ];

            for (tab, label) in base_tabs {
                let is_selected = self.current_tab == tab;
                let text = if is_selected {
                    RichText::new(label).strong().color(Color32::WHITE)
                } else {
                    RichText::new(label).color(Color32::BLACK)
                };

                let fill_color = if is_selected {
                    Color32::from_rgb(255, 140, 0)
                } else {
                    Color32::from_rgb(255, 165, 0)
                };

                let button = egui::Button::new(text).fill(fill_color);
                let btn = ui.add_enabled(true, button);
                if btn.clicked() {
                    self.current_tab = tab;
                }
            }

            ui.separator();

            for (tab, label) in advanced_tabs {
                let is_selected = self.current_tab == tab;
                let text = if is_selected {
                    RichText::new(label).strong().color(Color32::from_rgb(100, 149, 237))
                } else {
                    RichText::new(label)
                };

                let btn = ui.selectable_label(is_selected, text);
                if btn.clicked() {
                    self.current_tab = tab;
                }
            }
        });
        ui.separator();
    }

    fn show_tab_content(&mut self, ui: &mut egui::Ui) {
        match self.current_tab {
            Tab::General => GeneralPanel::show(ui, &mut self.config),
            Tab::OS => OSPanel::show(ui, &mut self.config),
            Tab::Cpu => CPUPanel::show(ui, &mut self.config),
            Tab::Memory => MemoryPanel::show(ui, &mut self.config),
            Tab::Devices => DevicesPanel::show(ui, &mut self.config),
            Tab::AdvancedSMBIOS => SMBIOSPanel::show(ui, &mut self.config),
            Tab::AdvancedIOThreads => IOThreadsPanel::show(ui, &mut self.config),
            Tab::AdvancedCPUTuning => CPUTuningPanel::show(ui, &mut self.config),
            Tab::AdvancedMemoryTuning => MemoryTuningPanel::show(ui, &mut self.config),
            Tab::AdvancedNUMA => NUMAPanel::show(ui, &mut self.config),
            Tab::AdvancedBlockIO => BlockIOTuningPanel::show(ui, &mut self.config),
            Tab::AdvancedResource => ResourcePartitioningPanel::show(ui, &mut self.config),
            Tab::AdvancedFCVMID => FibreChannelVMIDPanel::show(ui, &mut self.config),
            Tab::AdvancedEvents => EventsPanel::show(ui, &mut self.config),
            Tab::AdvancedPower => PowerManagementPanel::show(ui, &mut self.config),
            Tab::AdvancedDiskThrottle => DiskThrottleGroupPanel::show(ui, &mut self.config),
            Tab::AdvancedHypervisor => HypervisorFeaturesPanel::show(ui, &mut self.config),
            Tab::AdvancedTime => TimeKeepingPanel::show(ui, &mut self.config),
            Tab::AdvancedPerformance => PerformanceMonitoringPanel::show(ui, &mut self.config),
            Tab::AdvancedSecurity => SecurityLabelPanel::show(ui, &mut self.config),
            Tab::AdvancedKeyWrap => KeyWrapPanel::show(ui, &mut self.config),
            Tab::AdvancedLaunchSecurity => LaunchSecurityPanel::show(ui, &mut self.config),
            Tab::AdvancedNetwork => NetworkPanel::show(ui, &mut self.config),
            Tab::AdvancedSound => SoundPanel::show(ui, &mut self.config),
            Tab::AdvancedGraphics => GraphicsPanel::show(ui, &mut self.config),
            Tab::AdvancedInput => InputPanel::show(ui, &mut self.config),
            Tab::AdvancedSerial => SerialPanel::show(ui, &mut self.config),
            Tab::AdvancedParallel => ParallelPanel::show(ui, &mut self.config),
            Tab::AdvancedUSB => USBPanel::show(ui, &mut self.config),
            Tab::AdvancedPCI => PCIPanel::show(ui, &mut self.config),
        }
    }

    fn show_status_bar(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                if let Some((msg, success)) = &self.status_message {
                    let text = if *success {
                        RichText::new(format!("✅ {}", msg)).color(Color32::GREEN)
                    } else {
                        RichText::new(format!("❌ {}", msg)).color(Color32::RED)
                    };
                    ui.label(text);
                } else {
                    ui.label(RichText::new("就绪").color(Color32::GRAY));
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("📋 预览 XML").clicked() {
                        match XMLGenerator::generate(&self.config) {
                            Ok(xml) => {
                                self.generated_xml = xml;
                                self.show_xml_preview = true;
                            },
                            Err(e) => {
                                self.status_message = Some((format!("生成失败：{}", e), false));
                            },
                        }
                    }

                    if ui.button("💾 导出 XML").clicked() {
                        if let Err(e) = self.export_xml() {
                            self.status_message = Some((format!("导出失败：{}", e), false));
                        } else {
                            self.status_message = Some(("XML 已成功导出!".to_string(), true));
                        }
                    }
                });
            });
        });
    }

    fn show_xml_preview(&mut self, ui: &mut egui::Ui) {
        // 如果 generated_xml 为空，先生成 XML
        if self.generated_xml.is_empty() {
            match XMLGenerator::generate(&self.config) {
                Ok(xml) => self.generated_xml = xml,
                Err(e) => {
                    self.status_message = Some((format!("生成失败：{}", e), false));
                    return;
                },
            }
        }

        // 使用和导出一样的格式化逻辑
        let formatted_xml = crate::xml_gen::XMLGenerator::format_xml(&self.generated_xml);
        self.generated_xml = formatted_xml.clone();

        // XML 内容区域 - 占满整个宽度，直接显示格式化后的文本
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(30, 30, 30))
            .inner_margin(10.0)
            .rounding(5.0)
            .show(ui, |ui| {
                // 设置占满可用宽度
                ui.set_width(ui.available_width());
                egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut formatted_xml.as_str())
                            .font(egui::TextStyle::Monospace)
                            .text_color(egui::Color32::LIGHT_GRAY)
                            .desired_width(ui.available_width())
                            .desired_rows(20)
                            .interactive(false),
                    );
                });
            });

        // 按钮行 - 占满整个宽度
        ui.add_space(5.0);
        ui.horizontal_wrapped(|ui| {
            if ui.button("📋 复制").clicked() {
                ui.output_mut(|o| o.copied_text = self.generated_xml.clone());
                self.status_message = Some(("XML 已复制到剪贴板!".to_string(), true));
            }
            if ui.button("💾 保存").clicked() {
                if let Err(e) = self.export_xml() {
                    self.status_message = Some((format!("保存失败：{}", e), false));
                } else {
                    self.status_message = Some(("XML 已保存!".to_string(), true));
                }
            }
            if ui.button("📄 格式化").clicked() {
                let formatted = crate::xml_gen::XMLGenerator::format_xml(&self.generated_xml);
                self.generated_xml = formatted;
                self.status_message = Some(("XML 已格式化!".to_string(), true));
            }
        });
    }

    fn export_xml(&mut self) -> Result<(), String> {
        // 如果已经有生成的 XML，直接使用；否则重新生成
        let xml = if self.generated_xml.is_empty() {
            XMLGenerator::generate(&self.config)?
        } else {
            self.generated_xml.clone()
        };

        if let Some(path) = rfd::FileDialog::new()
            .add_filter("XML 文件", &["xml"])
            .set_file_name(format!("{}.xml", self.config.general.name))
            .save_file()
        {
            std::fs::write(&path, &xml).map_err(|e| format!("写入文件失败：{}", e))?;
            self.generated_xml = xml;
            Ok(())
        } else {
            Err("用户取消保存".to_string())
        }
    }
}

impl eframe::App for VMConfigApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.show_menu_bar(ui);
        });

        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            self.show_tabs(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.show_tab_content(ui);
            });
        });

        // XML 预览面板（底部）
        if self.show_xml_preview {
            egui::TopBottomPanel::bottom("xml_preview")
                .resizable(true)
                .min_height(200.0)
                .max_height(400.0)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.strong("XML 预览");
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.small_button("✕ 关闭").clicked() {
                                        self.show_xml_preview = false;
                                    }
                                },
                            );
                        });
                        ui.separator();
                        self.show_xml_preview(ui);
                    });
                });
        } else {
            egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
                self.show_status_bar(ui);
            });
        }
    }
}
