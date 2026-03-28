use crate::{
    model::{IOThread, IOThreadsConfig, VMConfig},
    panels::utils::*,
};

/// IOThreads 配置面板
pub struct IOThreadsPanel;

impl IOThreadsPanel {
    /// 显示 IOThreads 配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "🔄", "IOThreads 配置");

        card_group(ui, "IO 线程设置", None, colors, |ui| {
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

                if let Some(ref mut iothread_list) = iothreads_config.iothreadids {
                    ui.add_space(5.0);
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
                                    ui.add(egui::Slider::new(&mut iothread.id, 1..=16).text(""));
                                    ui.end_row();

                                    ui.label("线程池最小:");
                                    let mut min = iothread.thread_pool_min.unwrap_or(0);
                                    ui.add(egui::Slider::new(&mut min, 0..=16).text(""));
                                    iothread.thread_pool_min =
                                        if min > 0 { Some(min) } else { None };
                                    ui.end_row();

                                    ui.label("线程池最大:");
                                    let mut max = iothread.thread_pool_max.unwrap_or(0);
                                    ui.add(egui::Slider::new(&mut max, 0..=16).text(""));
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
    }
}
