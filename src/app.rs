use egui::RichText;

use crate::{
    model::vm_config::VMConfig,
    panels::{
        utils::{get_theme_colors, Theme},
        BlockIOTuningPanel, CPUPanel, CPUTuningPanel, DevicesPanel, DiskThrottleGroupPanel,
        EventsPanel, FibreChannelVMIDPanel, GeneralPanel, HypervisorFeaturesPanel, IOThreadsPanel,
        KeyWrapPanel, LaunchSecurityPanel, MemoryPanel, MemoryTuningPanel, NUMAPanel, OSPanel,
        PerformanceMonitoringPanel, PowerManagementPanel, ResourcePartitioningPanel, SMBIOSPanel,
        SecurityLabelPanel, TimeKeepingPanel,
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
}

pub struct VMConfigApp {
    config: VMConfig,
    current_tab: Tab,
    generated_xml: String,
    show_xml_preview: bool,
    status_message: Option<(String, bool)>,
    current_theme: Theme,
}

impl VMConfigApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let app = Self {
            config: VMConfig::new(),
            current_tab: Tab::default(),
            generated_xml: String::new(),
            show_xml_preview: false,
            status_message: None,
            current_theme: Theme::Dark,
        };
        // 设置初始主题
        app.set_theme(&cc.egui_ctx);
        app
    }

    fn set_theme(&self, ctx: &egui::Context) {
        let colors = get_theme_colors(self.current_theme);
        let mut style = (*ctx.style()).clone();

        // 根据主题设置深色/浅色模式
        style.visuals.dark_mode = matches!(self.current_theme, Theme::Dark);

        // 背景色
        style.visuals.window_fill = colors.window_fill;
        style.visuals.panel_fill = colors.panel_fill;
        style.visuals.override_text_color = Some(colors.text_primary);

        // 输入框样式（TextEdit/ComboBox）
        style.visuals.widgets.inactive.bg_fill = colors.input_background;
        style.visuals.widgets.hovered.bg_fill = colors.input_background;
        style.visuals.widgets.active.bg_fill = colors.input_background;
        style.visuals.selection.bg_fill = colors.info;

        // 边框颜色
        style.visuals.window_stroke = egui::Stroke::new(1.0, colors.border_color);
        style.visuals.selection.stroke = egui::Stroke::new(1.0, colors.input_text);

        // 输入框边框颜色
        style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, colors.input_border);
        style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, colors.input_border);
        style.visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, colors.input_border);

        // 下拉菜单样式（ComboBox 按钮部分）
        style.visuals.widgets.noninteractive.bg_fill = colors.input_background;
        style.visuals.widgets.noninteractive.bg_stroke =
            egui::Stroke::new(1.0, colors.input_border);
        style.visuals.widgets.noninteractive.fg_stroke =
            egui::Stroke::new(1.0, colors.text_primary);

        // TextEdit 专用样式
        style.visuals.extreme_bg_color = colors.input_background;

        // 按钮样式
        style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, colors.text_primary);
        style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, colors.text_primary);
        style.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, colors.text_primary);

        // 文本光标颜色
        style.visuals.text_cursor = egui::Stroke::new(2.0, colors.text_primary);

        // 滚动条样式
        style.visuals.widgets.noninteractive.bg_fill = colors.card_background;

        ctx.set_style(style);
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

            ui.menu_button("视图", |ui| {
                ui.label("主题切换:");
                for theme in [Theme::Light, Theme::Dark, Theme::Blue] {
                    let is_selected = self.current_theme == theme;
                    if ui.selectable_label(is_selected, theme.name()).clicked() {
                        self.current_theme = theme;
                        self.set_theme(ui.ctx());
                    }
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
        let colors = get_theme_colors(self.current_theme);

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
                (Tab::AdvancedKeyWrap, "🔐 密钥封装"),
                (Tab::AdvancedLaunchSecurity, "🚀 启动安全"),
            ];

            // 基础标签页
            for (tab, label) in base_tabs {
                let is_selected = self.current_tab == tab;
                let bg_color =
                    if is_selected { colors.tab_active_bg } else { colors.tab_inactive_bg };
                let text_color =
                    if is_selected { colors.tab_active_text } else { colors.tab_inactive_text };

                let text = if is_selected {
                    RichText::new(label).strong().color(text_color)
                } else {
                    RichText::new(label).color(text_color)
                };

                let button = egui::Button::new(text)
                    .fill(bg_color)
                    .rounding(8.0)
                    .min_size(egui::vec2(100.0, 32.0));

                if ui.add(button).clicked() {
                    self.current_tab = tab;
                }
            }

            ui.separator();

            // 高级标签页
            for (tab, label) in advanced_tabs {
                let is_selected = self.current_tab == tab;
                let text_color =
                    if is_selected { colors.advanced_tab_active_text } else { colors.text_primary };
                let bg = if is_selected {
                    colors.advanced_tab_active_bg
                } else {
                    colors.tab_inactive_bg
                };

                let text = if is_selected {
                    RichText::new(label).strong().color(text_color)
                } else {
                    RichText::new(label).color(text_color)
                };

                let button =
                    egui::Button::new(text).fill(bg).rounding(6.0).min_size(egui::vec2(90.0, 30.0));

                if ui.add(button).clicked() {
                    self.current_tab = tab;
                }
            }
        });
        ui.separator();
    }

    fn show_tab_content(&mut self, ui: &mut egui::Ui) {
        let colors = get_theme_colors(self.current_theme);

        match self.current_tab {
            Tab::General => GeneralPanel::show(ui, &mut self.config, &colors),
            Tab::OS => OSPanel::show(ui, &mut self.config, &colors),
            Tab::Cpu => CPUPanel::show(ui, &mut self.config, &colors),
            Tab::Memory => MemoryPanel::show(ui, &mut self.config, &colors),
            Tab::Devices => DevicesPanel::show(ui, &mut self.config, &colors),
            Tab::AdvancedSMBIOS => SMBIOSPanel::show(ui, &mut self.config, &colors),
            Tab::AdvancedIOThreads => IOThreadsPanel::show(ui, &mut self.config, &colors),
            Tab::AdvancedCPUTuning => CPUTuningPanel::show(ui, &mut self.config, &colors),
            Tab::AdvancedMemoryTuning => MemoryTuningPanel::show(ui, &mut self.config, &colors),
            Tab::AdvancedNUMA => NUMAPanel::show(ui, &mut self.config, &colors),
            Tab::AdvancedBlockIO => BlockIOTuningPanel::show(ui, &mut self.config, &colors),
            Tab::AdvancedResource => ResourcePartitioningPanel::show(ui, &mut self.config, &colors),
            Tab::AdvancedFCVMID => FibreChannelVMIDPanel::show(ui, &mut self.config, &colors),
            Tab::AdvancedEvents => EventsPanel::show(ui, &mut self.config, &colors),
            Tab::AdvancedPower => PowerManagementPanel::show(ui, &mut self.config, &colors),
            Tab::AdvancedDiskThrottle => {
                DiskThrottleGroupPanel::show(ui, &mut self.config, &colors)
            },
            Tab::AdvancedHypervisor => HypervisorFeaturesPanel::show(ui, &mut self.config, &colors),
            Tab::AdvancedTime => TimeKeepingPanel::show(ui, &mut self.config, &colors),
            Tab::AdvancedPerformance => {
                PerformanceMonitoringPanel::show(ui, &mut self.config, &colors)
            },
            Tab::AdvancedSecurity => SecurityLabelPanel::show(ui, &mut self.config, &colors),
            Tab::AdvancedKeyWrap => KeyWrapPanel::show(ui, &mut self.config, &colors),
            Tab::AdvancedLaunchSecurity => LaunchSecurityPanel::show(ui, &mut self.config, &colors),
        }
    }

    fn show_status_bar(&mut self, ui: &mut egui::Ui) {
        let colors = get_theme_colors(self.current_theme);

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                if let Some((msg, success)) = &self.status_message {
                    let text_color = if *success { colors.success } else { colors.error };
                    let text =
                        RichText::new(format!("{} {}", if *success { "✅" } else { "❌" }, msg))
                            .color(text_color);
                    ui.label(text);
                } else {
                    ui.label(RichText::new("就绪").color(colors.status_ready));
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(RichText::new("📋 预览 XML").strong()).clicked() {
                        match XMLGenerator::generate(&self.config) {
                            Ok(xml) => {
                                self.generated_xml = XMLGenerator::format_xml(&xml);
                                self.show_xml_preview = true;
                            },
                            Err(e) => {
                                self.status_message = Some((format!("生成失败：{}", e), false));
                            },
                        }
                    }
                    if ui.button(RichText::new("💾 导出 XML").strong()).clicked() {
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
        let colors = get_theme_colors(self.current_theme);

        // 如果 generated_xml 为空，先生成 XML 并格式化
        if self.generated_xml.is_empty() {
            match XMLGenerator::generate(&self.config) {
                Ok(xml) => self.generated_xml = XMLGenerator::format_xml(&xml),
                Err(e) => {
                    self.status_message = Some((format!("生成失败：{}", e), false));
                    return;
                },
            }
        }

        // XML 内容区域
        egui::Frame::none()
            .fill(colors.xml_bg)
            .inner_margin(12.0)
            .rounding(8.0)
            .stroke(egui::Stroke::new(1.0, colors.xml_border))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                egui::ScrollArea::vertical().max_height(350.0).stick_to_right(true).show(
                    ui,
                    |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.generated_xml.as_str())
                                .font(egui::TextStyle::Monospace)
                                .text_color(colors.xml_text)
                                .desired_width(ui.available_width())
                                .desired_rows(15)
                                .interactive(false),
                        );
                    },
                );
            });

        // 按钮行
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            let copy_btn = egui::Button::new("📋 复制").fill(colors.btn_copy).rounding(6.0);
            if ui.add(copy_btn).clicked() {
                ui.output_mut(|o| o.copied_text = self.generated_xml.clone());
                self.status_message = Some(("XML 已复制到剪贴板!".to_string(), true));
            }

            let save_btn = egui::Button::new("💾 保存").fill(colors.btn_save).rounding(6.0);
            if ui.add(save_btn).clicked() {
                if let Err(e) = self.export_xml() {
                    self.status_message = Some((format!("保存失败：{}", e), false));
                } else {
                    self.status_message = Some(("XML 已保存!".to_string(), true));
                }
            }

            let format_btn = egui::Button::new("📄 格式化").fill(colors.btn_format).rounding(6.0);
            if ui.add(format_btn).clicked() {
                let formatted = XMLGenerator::format_xml(&self.generated_xml);
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
            let colors = get_theme_colors(self.current_theme);
            egui::TopBottomPanel::bottom("xml_preview")
                .resizable(true)
                .min_height(250.0)
                .max_height(450.0)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("📄 XML 预览").strong().size(16.0).color(colors.info),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.small_button("✕ 关闭").clicked() {
                                        self.show_xml_preview = false;
                                    }
                                },
                            );
                        });
                        ui.add_space(5.0);
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
