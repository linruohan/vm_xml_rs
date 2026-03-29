use egui::{Align, Layout};

use crate::{
    model::{DefaultIOThread, IOThread, IOThreadsConfig, VMConfig},
    panels::utils::*,
};

/// IOThreads 配置面板
pub struct IOThreadsPanel;

impl IOThreadsPanel {
    /// 显示 IOThreads 配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "🔄", "IOThreads 配置");

        // 卡片宽度和间距配置
        let card_width = 380.0;
        let spacing = 8.0;

        // 使用流式布局实现卡片自动换行
        ui.with_layout(Layout::left_to_right(Align::TOP).with_main_wrap(true), |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);

            // IO 线程基础设置卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "IO 线程基础设置", Some("⚙"), colors, |ui| {
                        let mut has_iothreads = config.iothreads.is_some();
                        if checkbox(ui, &mut has_iothreads, "启用 IOThreads") {
                            if has_iothreads {
                                config.iothreads = Some(IOThreadsConfig {
                                    value: Some(4),
                                    iothreadids: Some(vec![IOThread {
                                        id: 1,
                                        thread_pool_min: None,
                                        thread_pool_max: None,
                                        poll: None,
                                    }]),
                                    defaultiothread: None,
                                });
                            } else {
                                config.iothreads = None;
                            }
                        }

                        if let Some(ref mut iothreads_config) = config.iothreads {
                            ui.add_space(5.0);

                            ui.label("IOThreads 数量:");
                            let mut value = iothreads_config.value.unwrap_or(4);
                            ui.add(egui::Slider::new(&mut value, 0..=16).text("个"));
                            iothreads_config.value = Some(value);

                            ui.add_space(5.0);

                            // 默认 IO 线程配置
                            ui.collapsing("默认 IO 线程配置", |ui| {
                                let mut has_default = iothreads_config.defaultiothread.is_some();
                                if checkbox(ui, &mut has_default, "启用默认 IO 线程") {
                                    if has_default {
                                        iothreads_config.defaultiothread = Some(DefaultIOThread {
                                            thread_pool_min: Some(0),
                                            thread_pool_max: Some(16),
                                        });
                                    } else {
                                        iothreads_config.defaultiothread = None;
                                    }
                                }

                                if let Some(ref mut default_io) = iothreads_config.defaultiothread {
                                    grid(ui, "default_io_grid", 2, |ui| {
                                        ui.label("线程池最小:");
                                        let mut min = default_io.thread_pool_min.unwrap_or(0);
                                        ui.add(egui::Slider::new(&mut min, 0..=16).text(""));
                                        default_io.thread_pool_min =
                                            if min > 0 { Some(min) } else { None };
                                        ui.end_row();

                                        ui.label("线程池最大:");
                                        let mut max = default_io.thread_pool_max.unwrap_or(0);
                                        ui.add(egui::Slider::new(&mut max, 0..=16).text(""));
                                        default_io.thread_pool_max =
                                            if max > 0 { Some(max) } else { None };
                                        ui.end_row();
                                    });
                                }
                            });
                        }
                    });
                },
            );

            // IOThread 列表卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "IOThread 列表", Some("📋"), colors, |ui| {
                        if let Some(ref mut iothreads_config) = config.iothreads {
                            if let Some(ref mut iothread_list) = iothreads_config.iothreadids {
                                ui.horizontal(|ui| {
                                    if add_button(ui, "➕ 添加 IOThread", colors) {
                                        let new_id = iothread_list.len() as u32 + 1;
                                        iothread_list.push(IOThread {
                                            id: new_id,
                                            thread_pool_min: None,
                                            thread_pool_max: None,
                                            poll: None,
                                        });
                                    }
                                });

                                let mut to_remove = None;
                                for (i, iothread) in iothread_list.iter_mut().enumerate() {
                                    ui.push_id(i, |ui| {
                                        inner_group(ui, colors, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(format!("IOThread {}", iothread.id));
                                                if delete_button(ui, None) {
                                                    to_remove = Some(i);
                                                }
                                            });

                                            grid(ui, format!("iothread_grid_{}", i), 2, |ui| {
                                                ui.label("ID:");
                                                ui.add(
                                                    egui::Slider::new(&mut iothread.id, 1..=16)
                                                        .text(""),
                                                );
                                                ui.end_row();

                                                ui.label("线程池最小:");
                                                let mut min = iothread.thread_pool_min.unwrap_or(0);
                                                ui.add(
                                                    egui::Slider::new(&mut min, 0..=16).text(""),
                                                );
                                                iothread.thread_pool_min =
                                                    if min > 0 { Some(min) } else { None };
                                                ui.end_row();

                                                ui.label("线程池最大:");
                                                let mut max = iothread.thread_pool_max.unwrap_or(0);
                                                ui.add(
                                                    egui::Slider::new(&mut max, 0..=16).text(""),
                                                );
                                                iothread.thread_pool_max =
                                                    if max > 0 { Some(max) } else { None };
                                                ui.end_row();
                                            });
                                        });
                                        ui.add_space(5.0);
                                    });
                                }

                                if let Some(idx) = to_remove {
                                    iothread_list.remove(idx);
                                }
                            }
                        }
                    });
                },
            );
        });
    }
}
