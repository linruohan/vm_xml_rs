use egui::RichText;

use crate::model::vm_config::{IOThread, IOThreadsConfig, VMConfig};

/// IOThreads 配置面板
pub struct IOThreadsPanel;

impl IOThreadsPanel {
    /// 显示 IOThreads 配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
        ui.group(|ui| {
            ui.label(RichText::new("IOThreads 配置").strong());
            ui.add_space(5.0);

            let mut has_iothreads = config.iothreads.is_some();
            if ui.checkbox(&mut has_iothreads, "启用 IOThreads").changed() {
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
                ui.add(egui::Slider::new(&mut value, 0..=16));
                iothreads_config.value = Some(value);

                if let Some(ref mut iothread_list) = iothreads_config.iothreadids {
                    if ui.button("➕ 添加 IOThread").clicked() {
                        let new_id = iothread_list.len() as u32 + 1;
                        iothread_list.push(IOThread {
                            id: new_id,
                            thread_pool_min: None,
                            thread_pool_max: None,
                            poll: None,
                        });
                    }

                    let mut to_remove = None;
                    for (i, iothread) in iothread_list.iter_mut().enumerate() {
                        ui.push_id(i, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("IOThread {}:", iothread.id));
                                if ui.button("🗑️ 删除").clicked() {
                                    to_remove = Some(i);
                                }
                            });

                            egui::Grid::new(format!("iothread_grid_{}", i))
                                .num_columns(2)
                                .spacing([10.0, 8.0])
                                .show(ui, |ui| {
                                    ui.label("ID:");
                                    ui.add(egui::Slider::new(&mut iothread.id, 1..=16));
                                    ui.end_row();

                                    ui.label("线程池最小:");
                                    let mut min = iothread.thread_pool_min.unwrap_or(0);
                                    ui.add(egui::Slider::new(&mut min, 0..=16));
                                    iothread.thread_pool_min =
                                        if min > 0 { Some(min) } else { None };
                                    ui.end_row();

                                    ui.label("线程池最大:");
                                    let mut max = iothread.thread_pool_max.unwrap_or(0);
                                    ui.add(egui::Slider::new(&mut max, 0..=16));
                                    iothread.thread_pool_max =
                                        if max > 0 { Some(max) } else { None };
                                    ui.end_row();
                                });
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
