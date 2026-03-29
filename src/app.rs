use egui::{Color32, RichText};

use crate::{
    error::AppError,
    model::vm_config::VMConfig,
    panels::{
        utils::{get_theme_colors, Theme},
        BlockIOTuningPanel, CPUPanel, CPUTuningPanel, DevicesPanel, DiskThrottleGroupPanel,
        EventsPanel, FibreChannelVMIDPanel, GeneralPanel, HypervisorFeaturesPanel, IOThreadsPanel,
        KeyWrapPanel, LaunchSecurityPanel, MemoryBackingPanel, MemoryPanel, MemoryTuningPanel,
        NUMAPanel, OSPanel, PerformanceMonitoringPanel, PowerManagementPanel,
        ResourcePartitioningPanel, SMBIOSPanel, SecurityLabelPanel, TimeKeepingPanel,
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
    AdvancedMemoryBacking,
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
    /// (消息, 是否成功, 显示时间)
    status_message: Option<(String, bool, std::time::Instant)>,
    current_theme: Theme,
    /// 撤销/重做历史记录
    history: Vec<VMConfig>,
    history_index: usize,
}

impl VMConfigApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = VMConfig::new();
        let app = Self {
            config: config.clone(),
            current_tab: Tab::default(),
            generated_xml: String::new(),
            show_xml_preview: false,
            status_message: None,
            current_theme: Theme::Dark,
            history: vec![config],
            history_index: 0,
        };
        app.set_theme(&cc.egui_ctx);
        app
    }

    /// 保存当前配置到历史记录
    fn push_history(&mut self) {
        // 如果当前不是最后一个历史记录，删除后面的记录
        if self.history_index < self.history.len() - 1 {
            self.history.truncate(self.history_index + 1);
        }
        // 添加当前配置到历史记录
        self.history.push(self.config.clone());
        self.history_index = self.history.len() - 1;
        // 限制历史记录数量
        if self.history.len() > 50 {
            self.history.remove(0);
            self.history_index = self.history.len() - 1;
        }
    }

    /// 撤销
    fn undo(&mut self) -> bool {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.config = self.history[self.history_index].clone();
            self.generated_xml = String::new();
            true
        } else {
            false
        }
    }

    /// 重做
    fn redo(&mut self) -> bool {
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            self.config = self.history[self.history_index].clone();
            self.generated_xml = String::new();
            true
        } else {
            false
        }
    }

    /// 设置状态消息，3 秒后自动清除
    fn set_status(&mut self, msg: impl Into<String>, success: bool) {
        self.status_message = Some((msg.into(), success, std::time::Instant::now()));
    }

    fn set_theme(&self, ctx: &egui::Context) {
        let colors = get_theme_colors(self.current_theme);
        let mut style = (*ctx.style()).clone();

        style.visuals.dark_mode = matches!(self.current_theme, Theme::Dark);
        style.visuals.window_fill = colors.window_fill;
        style.visuals.panel_fill = colors.panel_fill;
        style.visuals.override_text_color = Some(colors.text_primary);

        // 按钮样式 - 带渐变效果的基础色
        style.visuals.widgets.inactive.bg_fill = colors.button_bg;
        style.visuals.widgets.hovered.bg_fill = colors.button_bg_hover;
        style.visuals.widgets.active.bg_fill = colors.button_bg;

        // 输入框样式
        style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, colors.input_border);
        style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.5, colors.info);
        style.visuals.widgets.active.bg_stroke = egui::Stroke::new(1.5, colors.info);
        style.visuals.widgets.noninteractive.bg_fill = colors.input_background;
        style.visuals.widgets.noninteractive.bg_stroke =
            egui::Stroke::new(1.0, colors.input_border);
        style.visuals.widgets.noninteractive.fg_stroke =
            egui::Stroke::new(1.0, colors.text_primary);

        // 选择状态
        style.visuals.selection.bg_fill = colors.info;
        style.visuals.selection.stroke = egui::Stroke::new(1.0, colors.input_text);

        // 窗口边框
        style.visuals.window_stroke = egui::Stroke::new(1.0, colors.border_color);

        // 扩展背景色
        style.visuals.extreme_bg_color = colors.input_background;

        // 文本光标
        style.visuals.text_cursor = egui::Stroke::new(2.0, colors.info);

        // 交互反馈
        style.interaction.resize_grab_radius_side = 8.0;
        style.interaction.tooltip_delay = 0.1;

        // 菜单样式
        style.visuals.menu_rounding = egui::Rounding::same(8.0);

        ctx.set_style(style);
    }

    fn show_menu_bar(&mut self, ui: &mut egui::Ui) {
        // 菜单栏背景
        let menu_bg = if self.current_theme == Theme::Dark {
            Color32::from_rgb(35, 38, 45)
        } else {
            Color32::from_rgb(250, 250, 252)
        };

        egui::Frame::none()
            .fill(menu_bg)
            .inner_margin(egui::Margin::symmetric(8.0, 4.0))
            .show(ui, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.spacing_mut().button_padding = egui::vec2(8.0, 4.0);

                    ui.menu_button(RichText::new("📁 文件").strong(), |ui| {
                        ui.spacing_mut().button_padding = egui::vec2(4.0, 6.0);

                        if ui.button("📄 新建配置").clicked() {
                            self.config = VMConfig::new();
                            self.set_status("已创建新配置", true);
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("💾 保存配置...").clicked() {
                            match self.save_config() {
                                Ok(()) => self.set_status("配置已保存!", true),
                                Err(e) => self.set_status(format!("保存失败：{}", e), false),
                            }
                            ui.close_menu();
                        }
                        if ui.button("📂 加载配置...").clicked() {
                            match self.load_config() {
                                Ok(()) => self.set_status("配置已加载!", true),
                                Err(e) => self.set_status(format!("加载失败：{}", e), false),
                            }
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("📥 导入 XML...").clicked() {
                            match self.import_xml() {
                                Ok(()) => self.set_status("XML 已导入!", true),
                                Err(e) => self.set_status(format!("导入失败：{}", e), false),
                            }
                            ui.close_menu();
                        }
                        if ui.button("📤 导出 XML...").clicked() {
                            match self.export_xml() {
                                Ok(()) => self.set_status("XML 已成功导出!", true),
                                Err(e) => self.set_status(format!("导出失败：{}", e), false),
                            }
                            ui.close_menu();
                        }
                        if ui.button("📋 复制 XML 到剪贴板").clicked() {
                            match XMLGenerator::generate(&self.config) {
                                Ok(xml) => {
                                    ui.output_mut(|o| o.copied_text = xml.clone());
                                    self.generated_xml = xml;
                                    self.set_status("XML 已复制到剪贴板!", true);
                                },
                                Err(e) => self.set_status(format!("生成失败：{}", e), false),
                            }
                            ui.close_menu();
                        }
                    });

                    ui.menu_button(RichText::new("✏️ 编辑").strong(), |ui| {
                        ui.spacing_mut().button_padding = egui::vec2(4.0, 6.0);

                        ui.horizontal(|ui| {
                            if ui.button("↩ 撤销 (Ctrl+Z)").clicked() {
                                if self.undo() {
                                    self.set_status("已撤销", true);
                                }
                                ui.close_menu();
                            }
                            if ui.button("↪ 重做 (Ctrl+Y)").clicked() {
                                if self.redo() {
                                    self.set_status("已重做", true);
                                }
                                ui.close_menu();
                            }
                        });
                        ui.separator();
                        if ui.button("🔄 重置为默认值").clicked() {
                            self.push_history();
                            self.config = VMConfig::new();
                            self.set_status("已重置为默认值", true);
                            ui.close_menu();
                        }
                    });

                    ui.menu_button(RichText::new("🎨 视图").strong(), |ui| {
                        ui.spacing_mut().button_padding = egui::vec2(4.0, 6.0);

                        ui.label(RichText::new("主题切换:").strong().size(13.0));
                        ui.separator();
                        for theme in [
                            Theme::Light,
                            Theme::Dark,
                            Theme::Blue,
                            Theme::Midnight,
                            Theme::Forest,
                            Theme::Aurora,
                        ] {
                            let is_selected = self.current_theme == theme;
                            let theme_icon = match theme {
                                Theme::Light => "☀️",
                                Theme::Dark => "🌙",
                                Theme::Blue => "🌊",
                                Theme::Midnight => "🌃",
                                Theme::Forest => "🌲",
                                Theme::Aurora => "🌌",
                            };
                            let btn_text = format!("{} {}", theme_icon, theme.name());
                            if ui.selectable_label(is_selected, btn_text).clicked() {
                                self.current_theme = theme;
                                self.set_theme(ui.ctx());
                            }
                        }
                    });

                    ui.menu_button(RichText::new("❓ 帮助").strong(), |ui| {
                        ui.spacing_mut().button_padding = egui::vec2(4.0, 6.0);

                        if ui.button("📖 关于").clicked() {
                            ui.close_menu();
                        }
                        ui.separator();
                        ui.hyperlink_to(
                            "📚 libvirt 文档",
                            "https://www.libvirt.org/formatdomain.html",
                        );
                    });
                });
            });
    }

    fn show_tabs(&mut self, ui: &mut egui::Ui) {
        let colors = get_theme_colors(self.current_theme);

        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);

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
                (Tab::AdvancedMemoryBacking, "📁 内存后端"),
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

            // 绘制基础 Tab
            for (tab, label) in base_tabs {
                let is_selected = self.current_tab == tab;
                let bg_color =
                    if is_selected { colors.tab_active_bg } else { colors.tab_inactive_bg };
                let text_color =
                    if is_selected { colors.tab_active_text } else { colors.tab_inactive_text };
                let text = if is_selected {
                    RichText::new(label).strong().size(14.0).color(text_color)
                } else {
                    RichText::new(label).size(14.0).color(text_color)
                };
                let button = egui::Button::new(text)
                    .fill(bg_color)
                    .rounding(10.0)
                    .min_size(egui::vec2(110.0, 36.0))
                    .stroke(if is_selected {
                        egui::Stroke::new(2.0, colors.accent_gradient_end)
                    } else {
                        egui::Stroke::new(1.0, colors.border_color)
                    });
                if ui.add(button).clicked() {
                    self.current_tab = tab;
                }
            }

            ui.separator();

            // 绘制高级 Tab - 使用更紧凑的样式
            ui.horizontal(|ui| {
                for (tab, label) in advanced_tabs {
                    let is_selected = self.current_tab == tab;
                    let text_color = if is_selected {
                        colors.advanced_tab_active_text
                    } else {
                        colors.text_secondary
                    };
                    let bg = if is_selected {
                        colors.advanced_tab_active_bg
                    } else {
                        colors.tab_inactive_bg
                    };
                    let text = if is_selected {
                        RichText::new(label).strong().size(13.0).color(text_color)
                    } else {
                        RichText::new(label).size(13.0).color(text_color)
                    };
                    let button = egui::Button::new(text)
                        .fill(bg)
                        .rounding(8.0)
                        .min_size(egui::vec2(85.0, 32.0));
                    if ui.add(button).clicked() {
                        self.current_tab = tab;
                    }
                }
            });
        });
        ui.separator();
    }

    fn show_tab_content(&mut self, ui: &mut egui::Ui) {
        let colors = get_theme_colors(self.current_theme);

        // 添加整体背景框架
        egui::Frame::none()
            .inner_margin(egui::Margin::same(16.0))
            .show(ui, |ui| {
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
                    Tab::AdvancedMemoryBacking => MemoryBackingPanel::show(ui, &mut self.config, &colors),
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
            });
    }

    fn show_status_bar(&mut self, ui: &mut egui::Ui) {
        let colors = get_theme_colors(self.current_theme);

        // 自动清除超过 3 秒的状态消息
        if let Some((_, _, time)) = &self.status_message {
            if time.elapsed().as_secs() >= 3 {
                self.status_message = None;
            }
        }

        // 创建带背景的 статус栏
        let fill_color = if self.current_theme == Theme::Dark {
            Color32::from_rgb(35, 38, 45)
        } else {
            Color32::from_rgb(250, 250, 252)
        };

        egui::Frame::none()
            .fill(fill_color)
            .inner_margin(egui::Margin::symmetric(12.0, 8.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if let Some((msg, success, _)) = &self.status_message {
                        let text_color = if *success { colors.success } else { colors.error };
                        let icon = if *success { "✅" } else { "⚠️" };
                        let text = RichText::new(format!("{} {}", icon, msg))
                            .strong()
                            .color(text_color);
                        ui.label(text);
                    } else {
                        ui.label(
                            RichText::new("✨ 就绪")
                                .color(colors.status_ready)
                                .size(13.0),
                        );
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let xml_btn = egui::Button::new(
                            RichText::new("📋 预览 XML").strong().size(13.0),
                        )
                        .fill(colors.info)
                        .rounding(8.0);
                        if ui.add(xml_btn).clicked() {
                            match XMLGenerator::generate(&self.config) {
                                Ok(xml) => {
                                    self.generated_xml = XMLGenerator::format_xml(&xml);
                                    self.show_xml_preview = true;
                                },
                                Err(e) => self.set_status(format!("生成失败：{}", e), false),
                            }
                        }
                        if ui.add(egui::Button::new(RichText::new("💾 导出 XML").strong().size(13.0))).clicked() {
                            match self.export_xml() {
                                Ok(()) => self.set_status("XML 已成功导出!", true),
                                Err(e) => self.set_status(format!("导出失败：{}", e), false),
                            }
                        }
                    });
                });
            });
    }

    fn show_xml_preview(&mut self, ui: &mut egui::Ui) {
        let colors = get_theme_colors(self.current_theme);

        if self.generated_xml.is_empty() {
            match XMLGenerator::generate(&self.config) {
                Ok(xml) => self.generated_xml = XMLGenerator::format_xml(&xml),
                Err(e) => {
                    self.set_status(format!("生成失败：{}", e), false);
                    return;
                },
            }
        }

        // 增强 XML 预览框样式
        egui::Frame::none()
            .fill(colors.xml_bg)
            .inner_margin(egui::Margin::same(14.0))
            .rounding(egui::Rounding::same(10.0))
            .stroke(egui::Stroke::new(1.5, colors.xml_border))
            .shadow(egui::epaint::Shadow {
                offset: egui::vec2(0.0, 2.0),
                blur: 6.0,
                spread: 0.0,
                color: Color32::from_black_alpha(30),
            })
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .stick_to_right(true)
                    .show(ui, |ui| {
                        egui::Frame::none()
                            .inner_margin(egui::Margin::same(10.0))
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut self.generated_xml.as_str())
                                        .font(egui::TextStyle::Monospace)
                                        .text_color(colors.xml_text)
                                        .desired_width(ui.available_width())
                                        .desired_rows(18)
                                        .interactive(false),
                                );
                            });
                    });
            });

        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);

            let copy_btn = egui::Button::new(
                RichText::new("📋 复制").strong().size(13.0),
            )
            .fill(colors.btn_copy)
            .rounding(8.0);
            if ui.add(copy_btn).clicked() {
                ui.output_mut(|o| o.copied_text = self.generated_xml.clone());
                self.set_status("XML 已复制到剪贴板!", true);
            }

            let save_btn = egui::Button::new(
                RichText::new("💾 保存").strong().size(13.0),
            )
            .fill(colors.btn_save)
            .rounding(8.0);
            if ui.add(save_btn).clicked() {
                match self.export_xml() {
                    Ok(()) => self.set_status("XML 已保存!", true),
                    Err(e) => self.set_status(format!("保存失败：{}", e), false),
                }
            }

            let format_btn = egui::Button::new(
                RichText::new("📄 格式化").strong().size(13.0),
            )
            .fill(colors.btn_format)
            .rounding(8.0);
            if ui.add(format_btn).clicked() {
                self.generated_xml = XMLGenerator::format_xml(&self.generated_xml);
                self.set_status("XML 已格式化!", true);
            }
        });
    }

    fn export_xml(&mut self) -> Result<(), AppError> {
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
            std::fs::write(&path, &xml)?;
            self.generated_xml = xml;
            Ok(())
        } else {
            Err(AppError::Validation("用户取消保存".to_string()))
        }
    }

    fn save_config(&mut self) -> Result<(), AppError> {
        self.push_history();
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("配置文件", &["json"])
            .set_file_name(format!("{}.json", self.config.general.name))
            .save_file()
        {
            let json = serde_json::to_string_pretty(&self.config)?;
            std::fs::write(&path, json)?;
            Ok(())
        } else {
            Err(AppError::Validation("用户取消保存".to_string()))
        }
    }

    fn load_config(&mut self) -> Result<(), AppError> {
        self.push_history();
        if let Some(path) = rfd::FileDialog::new().add_filter("配置文件", &["json"]).pick_file()
        {
            let content = std::fs::read_to_string(&path)?;
            self.config = serde_json::from_str(&content)?;
            self.generated_xml = String::new(); // 清除旧的 XML 缓存
            Ok(())
        } else {
            Err(AppError::Validation("用户取消选择".to_string()))
        }
    }

    fn import_xml(&mut self) -> Result<(), AppError> {
        if let Some(path) = rfd::FileDialog::new().add_filter("XML 文件", &["xml"]).pick_file() {
            let content = std::fs::read_to_string(&path)?;
            self.config = crate::xml_import::import_from_xml(&content)?;
            self.generated_xml = String::new(); // 清除旧的 XML 缓存
            Ok(())
        } else {
            Err(AppError::Validation("用户取消选择".to_string()))
        }
    }
}

impl eframe::App for VMConfigApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 状态消息存在时持续请求重绘，以便自动清除生效
        if self.status_message.is_some() {
            ctx.request_repaint_after(std::time::Duration::from_secs(1));
        }

        // 键盘快捷键支持
        if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::Z))
        }) && self.undo()
        {
            self.set_status("已撤销", true);
        }
        if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::Y))
        }) && self.redo()
        {
            self.set_status("已重做", true);
        }

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

        if self.show_xml_preview {
            let colors = get_theme_colors(self.current_theme);

            // 创建渐变背景
            let header_bg = if self.current_theme == Theme::Dark {
                Color32::from_rgb(40, 45, 55)
            } else {
                Color32::from_rgb(240, 245, 250)
            };

            egui::TopBottomPanel::bottom("xml_preview")
                .resizable(true)
                .min_height(280.0)
                .max_height(480.0)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        // 头部带背景
                        egui::Frame::none()
                            .fill(header_bg)
                            .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new("📄 XML 预览")
                                            .strong()
                                            .size(15.0)
                                            .color(colors.info),
                                    );

                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            let close_btn = egui::Button::new(
                                                RichText::new("✕ 关闭").size(12.0),
                                            )
                                            .fill(colors.error)
                                            .rounding(6.0);
                                            if ui.add(close_btn).clicked() {
                                                self.show_xml_preview = false;
                                            }
                                        },
                                    );
                                });
                            });
                        ui.add_space(8.0);
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
