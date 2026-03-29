use egui::{Align, Layout, RichText};

use crate::{
    field_row_with_validation,
    model::{validation, VMConfig},
    panels::utils::*,
};

pub struct GeneralPanel;

impl GeneralPanel {
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "📋", "基础配置");

        // 卡片宽度和间距配置
        let card_width = 380.0;
        let spacing = 8.0;

        // 使用流式布局实现卡片自动换行
        ui.with_layout(Layout::left_to_right(Align::TOP).with_main_wrap(true), |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);

            // 虚拟机基本信息卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "虚拟机基本信息", Some("🖥"), colors, |ui| {
                        grid(ui, "general_info_grid", 2, |ui| {
                            // 虚拟机类型
                            ui.label("虚拟机类型:");
                            egui::ComboBox::from_id_source("vm_type")
                                .selected_text(&config.general.vm_type)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut config.general.vm_type,
                                        "kvm".to_string(),
                                        "kvm",
                                    );
                                    ui.selectable_value(
                                        &mut config.general.vm_type,
                                        "qemu".to_string(),
                                        "qemu",
                                    );
                                    ui.selectable_value(
                                        &mut config.general.vm_type,
                                        "xen".to_string(),
                                        "xen",
                                    );
                                    ui.selectable_value(
                                        &mut config.general.vm_type,
                                        "lxc".to_string(),
                                        "lxc",
                                    );
                                });
                            ui.end_row();

                            // 虚拟机名称
                            ui.label("虚拟机名称:");
                            ui.text_edit_singleline(&mut config.general.name);
                            ui.end_row();

                            // UUID
                            field_row_with_validation!(
                                ui,
                                "UUID:",
                                config.general.uuid.get_or_insert_with(String::new),
                                validation::validate_uuid(
                                    config.general.uuid.as_ref().unwrap_or(&String::new())
                                ),
                                RichText::new("⚠ UUID 格式无效（应为 8-4-4-4-12 十六进制格式）")
                                    .color(egui::Color32::from_rgb(255, 100, 100))
                            );
                            ui.end_row();

                            // UUID 生成按钮
                            ui.label("UUID 操作:");
                            ui.horizontal(|ui| {
                                if ui.button("🔄 生成").clicked() {
                                    config.general.uuid = Some(uuid::Uuid::new_v4().to_string());
                                }
                            });
                            ui.end_row();

                            // 描述
                            ui.label("描述:");
                            let desc = config.general.description.get_or_insert_with(String::new);
                            ui.text_edit_multiline(desc);
                            ui.end_row();
                        });
                    });
                },
            );

            // CPU 和内存配置卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "CPU 和内存配置", Some("⚙"), colors, |ui| {
                        grid(ui, "general_resources_grid", 2, |ui| {
                            // vCPU 数量
                            ui.label("vCPU 数量:");
                            ui.add(
                                egui::Slider::new(&mut config.general.vcpu.count, 1..=64)
                                    .text("核心")
                                    .logarithmic(true),
                            );
                            ui.end_row();

                            // vCPU 放置策略
                            ui.label("vCPU 放置:");
                            let placement = config
                                .general
                                .vcpu
                                .placement
                                .get_or_insert_with(|| "static".to_string());
                            egui::ComboBox::from_id_source("vcpu_placement")
                                .selected_text(placement.as_str())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        placement,
                                        "static".to_string(),
                                        "静态分配",
                                    );
                                    ui.selectable_value(placement, "auto".to_string(), "自动分配");
                                });
                            ui.end_row();

                            // 内存大小
                            ui.label("内存大小:");
                            ui.add(
                                egui::Slider::new(&mut config.general.memory.value, 1..=128)
                                    .text("单位")
                                    .logarithmic(true),
                            );
                            ui.end_row();

                            // 内存单位
                            ui.label("内存单位:");
                            let unit =
                                config.general.memory.unit.get_or_insert_with(|| "MiB".to_string());
                            egui::ComboBox::from_id_source("memory_unit")
                                .selected_text(unit.as_str())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(unit, "KiB".to_string(), "KiB");
                                    ui.selectable_value(unit, "MiB".to_string(), "MiB");
                                    ui.selectable_value(unit, "GiB".to_string(), "GiB");
                                    ui.selectable_value(unit, "TiB".to_string(), "TiB");
                                });
                            ui.end_row();
                        });
                    });
                },
            );

            // 高级元数据卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "高级元数据", Some("🏷"), colors, |ui| {
                        // hwuuid
                        ui.horizontal(|ui| {
                            ui.label("hwuuid:");
                            let hwuuid =
                                config.general.hwuuid.get_or_insert_with(|| "".to_string());
                            ui.text_edit_singleline(hwuuid);
                        });

                        ui.add_space(5.0);

                        // genid
                        ui.horizontal(|ui| {
                            ui.label("genid:");
                            let genid = config.general.genid.get_or_insert_with(|| "".to_string());
                            ui.text_edit_singleline(genid);
                            if ui.button("🔄 生成").clicked() {
                                *genid = uuid::Uuid::new_v4().to_string();
                            }
                        });

                        ui.add_space(5.0);

                        // 标题
                        ui.horizontal(|ui| {
                            ui.label("标题:");
                            let title = config.general.title.get_or_insert_with(|| "".to_string());
                            ui.text_edit_singleline(title);
                        });
                    });
                },
            );

            // 引导加载器卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "引导加载器", Some("🚀"), colors, |ui| {
                        // bootloader
                        ui.horizontal(|ui| {
                            ui.label("bootloader:");
                            let bootloader =
                                config.general.bootloader.get_or_insert_with(|| "".to_string());
                            ui.text_edit_singleline(bootloader);
                        });

                        ui.add_space(5.0);

                        // bootloader_args
                        ui.horizontal(|ui| {
                            ui.label("bootloader_args:");
                            let bootloader_args = config
                                .general
                                .bootloader_args
                                .get_or_insert_with(|| "".to_string());
                            ui.text_edit_singleline(bootloader_args);
                        });
                    });
                },
            );

            // vCPU 热插拔配置卡片
            ui.allocate_ui_with_layout(
                egui::vec2(card_width, 0.0),
                Layout::top_down(Align::LEFT),
                |ui| {
                    card_group(ui, "vCPU 热插拔配置", Some("🔌"), colors, |ui| {
                        let mut has_vcpus = config.general.vcpus.is_some();
                        if checkbox(ui, &mut has_vcpus, "启用 vCPU 热插拔配置") {
                            if has_vcpus {
                                config.general.vcpus = Some(vec![crate::model::VCPUConfig {
                                    id: 0,
                                    enabled: "yes".to_string(),
                                    hotpluggable: "no".to_string(),
                                    order: None,
                                }]);
                            } else {
                                config.general.vcpus = None;
                            }
                        }

                        if let Some(ref mut vcpus) = config.general.vcpus {
                            ui.add_space(5.0);
                            ui.horizontal(|ui| {
                                if add_button(ui, "➕ 添加 vCPU", colors) {
                                    vcpus.push(crate::model::VCPUConfig {
                                        id: vcpus.len() as u32,
                                        enabled: "yes".to_string(),
                                        hotpluggable: "yes".to_string(),
                                        order: None,
                                    });
                                }
                            });

                            let mut to_remove = None;
                            for (i, vcpu) in vcpus.iter_mut().enumerate() {
                                ui.push_id(i, |ui| {
                                    inner_group(ui, colors, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(format!("vCPU {}", i));
                                            if delete_button(ui, None) {
                                                to_remove = Some(i);
                                            }
                                        });

                                        grid(ui, format!("vcpu_grid_{}", i), 2, |ui| {
                                            ui.label("ID:");
                                            ui.add(egui::Slider::new(&mut vcpu.id, 0..=255));
                                            ui.end_row();

                                            ui.label("启用:");
                                            egui::ComboBox::from_id_source(format!(
                                                "vcpu_enabled_{}",
                                                i
                                            ))
                                            .selected_text(&vcpu.enabled)
                                            .show_ui(
                                                ui,
                                                |ui| {
                                                    ui.selectable_value(
                                                        &mut vcpu.enabled,
                                                        "yes".to_string(),
                                                        "是",
                                                    );
                                                    ui.selectable_value(
                                                        &mut vcpu.enabled,
                                                        "no".to_string(),
                                                        "否",
                                                    );
                                                },
                                            );
                                            ui.end_row();

                                            ui.label("可热插拔:");
                                            egui::ComboBox::from_id_source(format!(
                                                "vcpu_hotpluggable_{}",
                                                i
                                            ))
                                            .selected_text(&vcpu.hotpluggable)
                                            .show_ui(
                                                ui,
                                                |ui| {
                                                    ui.selectable_value(
                                                        &mut vcpu.hotpluggable,
                                                        "yes".to_string(),
                                                        "是",
                                                    );
                                                    ui.selectable_value(
                                                        &mut vcpu.hotpluggable,
                                                        "no".to_string(),
                                                        "否",
                                                    );
                                                },
                                            );
                                            ui.end_row();

                                            ui.label("顺序:");
                                            let order = vcpu.order.get_or_insert(0);
                                            ui.add(egui::Slider::new(order, 0..=255));
                                            ui.end_row();
                                        });
                                    });
                                    ui.add_space(5.0);
                                });
                            }

                            if let Some(idx) = to_remove {
                                vcpus.remove(idx);
                            }
                        }
                    });
                },
            );
        });
    }
}
