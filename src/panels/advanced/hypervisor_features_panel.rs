use crate::{
    model::{
        hypervisor_features::{
            AiaConfig, CfpcConfig, DirtyRingConfig, FeatureState, GicConfig, HptConfig,
            HyperVConfig, IbsConfig, IoapicConfig, KvmFeaturesConfig, MaxPageSizeConfig,
            MsrsConfig, PassthroughConfig, SbbcConfig, SmmConfig, SpinlocksConfig, StimerConfig,
            TbCacheConfig, TcgConfig, TlbflushConfig, TsegConfig, VendorIdConfig,
            XenFeaturesConfig,
        },
        HypervisorFeaturesConfig, VMConfig,
    },
    panels::utils::*,
};

/// Hypervisor 特性配置面板
pub struct HypervisorFeaturesPanel;

impl HypervisorFeaturesPanel {
    /// 显示 Hypervisor 特性配置面板
    pub fn show(ui: &mut egui::Ui, config: &mut VMConfig, colors: &ThemeColors) {
        panel_header(ui, "🛡", "Hypervisor 特性配置");

        card_group(ui, "基础特性", None, colors, |ui| {
            let mut has_features = config.hypervisor_features.is_some();
            if checkbox(ui, &mut has_features, "启用 Hypervisor 特性") {
                if has_features {
                    config.hypervisor_features = Some(HypervisorFeaturesConfig::default());
                } else {
                    config.hypervisor_features = None;
                }
            }

            if let Some(ref mut features) = config.hypervisor_features {
                ui.add_space(5.0);
                grid(ui, "basic_features_grid", 2, |ui| {
                    feature_toggle(ui, "PAE", &mut features.pae);
                    ui.end_row();
                    feature_toggle(ui, "ACPI", &mut features.acpi);
                    ui.end_row();
                    feature_toggle(ui, "APIC", &mut features.apic);
                    ui.end_row();
                    feature_toggle(ui, "HAP (硬件辅助分页)", &mut features.hap);
                    ui.end_row();
                    feature_toggle(ui, "Viridian", &mut features.viridian);
                    ui.end_row();
                    feature_toggle(ui, "PrivNet", &mut features.privnet);
                    ui.end_row();
                    feature_toggle(ui, "PV Spinlock", &mut features.pvspinlock);
                    ui.end_row();
                    feature_toggle(ui, "PMU (性能监控)", &mut features.pmu);
                    ui.end_row();
                    feature_toggle(ui, "VMport", &mut features.vmport);
                    ui.end_row();
                    feature_toggle(ui, "VMcoreinfo", &mut features.vmcoreinfo);
                    ui.end_row();
                    feature_toggle(ui, "HTM (硬件事务内存)", &mut features.htm);
                    ui.end_row();
                    feature_toggle(ui, "Nested-HV (嵌套虚拟化)", &mut features.nested_hv);
                    ui.end_row();
                    feature_toggle(ui, "CCF-Assist", &mut features.ccf_assist);
                    ui.end_row();
                    feature_toggle(ui, "HRT", &mut features.hrt);
                    ui.end_row();
                    feature_toggle(ui, "Async-Teardown", &mut features.async_teardown);
                    ui.end_row();
                    feature_toggle(ui, "RAS", &mut features.ras);
                    ui.end_row();
                    feature_toggle(ui, "PS2", &mut features.ps2);
                    ui.end_row();
                });
            }
        });

        ui.add_space(8.0);

        card_group(ui, "Hyper-V 特性", None, colors, |ui| {
            if let Some(ref mut features) = config.hypervisor_features {
                let mut has_hyperv = features.hyperv.is_some();
                if checkbox(ui, &mut has_hyperv, "启用 Hyper-V 特性") {
                    if has_hyperv {
                        features.hyperv = Some(HyperVConfig::default());
                    } else {
                        features.hyperv = None;
                    }
                }

                if let Some(ref mut hyperv) = features.hyperv {
                    ui.add_space(5.0);
                    grid(ui, "hyperv_grid", 2, |ui| {
                        ui.label("模式:");
                        let mode = hyperv.mode.get_or_insert_with(|| "custom".to_string());
                        egui::ComboBox::from_id_source("hyperv_mode")
                            .selected_text(mode.as_str())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(mode, "custom".to_string(), "custom");
                                ui.selectable_value(mode, "passthrough".to_string(), "passthrough");
                                ui.selectable_value(mode, "host-model".to_string(), "host-model");
                            });
                        ui.end_row();
                    });

                    ui.add_space(5.0);
                    grid(ui, "hyperv_features_grid", 2, |ui| {
                        feature_state_toggle(ui, "Relaxed", &mut hyperv.relaxed);
                        ui.end_row();
                        feature_state_toggle(ui, "VAPIC", &mut hyperv.vapic);
                        ui.end_row();
                        feature_state_toggle(ui, "VPIndex", &mut hyperv.vpindex);
                        ui.end_row();
                        feature_state_toggle(ui, "Runtime", &mut hyperv.runtime);
                        ui.end_row();
                        feature_state_toggle(ui, "SynIC", &mut hyperv.synic);
                        ui.end_row();
                        feature_state_toggle(ui, "Reset", &mut hyperv.reset);
                        ui.end_row();
                        feature_state_toggle(ui, "Frequencies", &mut hyperv.frequencies);
                        ui.end_row();
                        feature_state_toggle(ui, "Reenlightenment", &mut hyperv.reenlightenment);
                        ui.end_row();
                        feature_state_toggle(ui, "IPI", &mut hyperv.ipi);
                        ui.end_row();
                        feature_state_toggle(ui, "AVIC", &mut hyperv.avic);
                        ui.end_row();
                        feature_state_toggle(ui, "EVMCS", &mut hyperv.evmcs);
                        ui.end_row();
                        feature_state_toggle(ui, "EMSR Bitmap", &mut hyperv.emsr_bitmap);
                        ui.end_row();
                        feature_state_toggle(ui, "XMM Input", &mut hyperv.xmm_input);
                        ui.end_row();
                    });

                    ui.add_space(5.0);

                    // Spinlocks 配置
                    ui.collapsing("Spinlocks 配置", |ui| {
                        if hyperv.spinlocks.is_none() {
                            hyperv.spinlocks = Some(SpinlocksConfig {
                                state: "off".to_string(),
                                retries: Some(4096),
                            });
                        }
                        if let Some(ref mut spinlocks) = hyperv.spinlocks {
                            grid(ui, "spinlocks_grid", 2, |ui| {
                                ui.label("状态:");
                                egui::ComboBox::from_id_source("spinlocks_state")
                                    .selected_text(&spinlocks.state)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut spinlocks.state,
                                            "on".to_string(),
                                            "on",
                                        );
                                        ui.selectable_value(
                                            &mut spinlocks.state,
                                            "off".to_string(),
                                            "off",
                                        );
                                    });
                                ui.end_row();
                                ui.label("重试次数:");
                                let retries = spinlocks.retries.get_or_insert(4096);
                                ui.add(egui::Slider::new(retries, 4095..=4294967295).text("次"));
                                ui.end_row();
                            });
                        }
                    });

                    ui.add_space(5.0);

                    // Stimer 配置
                    ui.collapsing("Stimer 配置", |ui| {
                        if hyperv.stimer.is_none() {
                            hyperv.stimer =
                                Some(StimerConfig { state: "off".to_string(), direct: None });
                        }
                        if let Some(ref mut stimer) = hyperv.stimer {
                            grid(ui, "stimer_grid", 2, |ui| {
                                ui.label("状态:");
                                egui::ComboBox::from_id_source("stimer_state")
                                    .selected_text(&stimer.state)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut stimer.state,
                                            "on".to_string(),
                                            "on",
                                        );
                                        ui.selectable_value(
                                            &mut stimer.state,
                                            "off".to_string(),
                                            "off",
                                        );
                                    });
                                ui.end_row();
                                ui.label("Direct 模式:");
                                if stimer.direct.is_none() {
                                    stimer.direct = Some(FeatureState { state: "off".to_string() });
                                }
                                if let Some(ref mut direct) = stimer.direct {
                                    egui::ComboBox::from_id_source("stimer_direct")
                                        .selected_text(&direct.state)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut direct.state,
                                                "on".to_string(),
                                                "on",
                                            );
                                            ui.selectable_value(
                                                &mut direct.state,
                                                "off".to_string(),
                                                "off",
                                            );
                                        });
                                }
                                ui.end_row();
                            });
                        }
                    });

                    ui.add_space(5.0);

                    // Vendor ID 配置
                    ui.collapsing("Vendor ID 配置", |ui| {
                        if hyperv.vendor_id.is_none() {
                            hyperv.vendor_id =
                                Some(VendorIdConfig { state: "off".to_string(), value: None });
                        }
                        if let Some(ref mut vendor_id) = hyperv.vendor_id {
                            grid(ui, "vendor_id_grid", 2, |ui| {
                                ui.label("状态:");
                                egui::ComboBox::from_id_source("vendor_id_state")
                                    .selected_text(&vendor_id.state)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut vendor_id.state,
                                            "on".to_string(),
                                            "on",
                                        );
                                        ui.selectable_value(
                                            &mut vendor_id.state,
                                            "off".to_string(),
                                            "off",
                                        );
                                    });
                                ui.end_row();
                                ui.label("Value:");
                                let value = vendor_id.value.get_or_insert_with(|| "".to_string());
                                ui.text_edit_singleline(value);
                                ui.end_row();
                            });
                        }
                    });

                    ui.add_space(5.0);

                    // TLBFlush 配置
                    ui.collapsing("TLBFlush 配置", |ui| {
                        if hyperv.tlbflush.is_none() {
                            hyperv.tlbflush = Some(TlbflushConfig {
                                state: "off".to_string(),
                                direct: None,
                                extended: None,
                            });
                        }
                        if let Some(ref mut tlbflush) = hyperv.tlbflush {
                            grid(ui, "tlbflush_grid", 2, |ui| {
                                ui.label("状态:");
                                egui::ComboBox::from_id_source("tlbflush_state")
                                    .selected_text(&tlbflush.state)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut tlbflush.state,
                                            "on".to_string(),
                                            "on",
                                        );
                                        ui.selectable_value(
                                            &mut tlbflush.state,
                                            "off".to_string(),
                                            "off",
                                        );
                                    });
                                ui.end_row();
                                ui.label("Direct 模式:");
                                if tlbflush.direct.is_none() {
                                    tlbflush.direct =
                                        Some(FeatureState { state: "off".to_string() });
                                }
                                if let Some(ref mut direct) = tlbflush.direct {
                                    egui::ComboBox::from_id_source("tlbflush_direct")
                                        .selected_text(&direct.state)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut direct.state,
                                                "on".to_string(),
                                                "on",
                                            );
                                            ui.selectable_value(
                                                &mut direct.state,
                                                "off".to_string(),
                                                "off",
                                            );
                                        });
                                }
                                ui.end_row();
                                ui.label("Extended 模式:");
                                if tlbflush.extended.is_none() {
                                    tlbflush.extended =
                                        Some(FeatureState { state: "off".to_string() });
                                }
                                if let Some(ref mut extended) = tlbflush.extended {
                                    egui::ComboBox::from_id_source("tlbflush_extended")
                                        .selected_text(&extended.state)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut extended.state,
                                                "on".to_string(),
                                                "on",
                                            );
                                            ui.selectable_value(
                                                &mut extended.state,
                                                "off".to_string(),
                                                "off",
                                            );
                                        });
                                }
                                ui.end_row();
                            });
                        }
                    });
                }
            }
        });

        ui.add_space(8.0);

        card_group(ui, "KVM 特性", None, colors, |ui| {
            if let Some(ref mut features) = config.hypervisor_features {
                let mut has_kvm = features.kvm.is_some();
                if checkbox(ui, &mut has_kvm, "启用 KVM 特性") {
                    if has_kvm {
                        features.kvm = Some(KvmFeaturesConfig::default());
                    } else {
                        features.kvm = None;
                    }
                }

                if let Some(ref mut kvm) = features.kvm {
                    ui.add_space(5.0);
                    grid(ui, "kvm_features_grid", 2, |ui| {
                        feature_state_toggle(ui, "Hidden", &mut kvm.hidden);
                        ui.end_row();
                        feature_state_toggle(ui, "Hint-Dedicated", &mut kvm.hint_dedicated);
                        ui.end_row();
                        feature_state_toggle(ui, "Poll-Control", &mut kvm.poll_control);
                        ui.end_row();
                        feature_state_toggle(ui, "PV-IPI", &mut kvm.pv_ipi);
                        ui.end_row();
                    });

                    ui.add_space(5.0);

                    // Dirty Ring 配置
                    ui.collapsing("Dirty Ring 配置", |ui| {
                        if kvm.dirty_ring.is_none() {
                            kvm.dirty_ring =
                                Some(DirtyRingConfig { state: "off".to_string(), size: None });
                        }
                        if let Some(ref mut dirty_ring) = kvm.dirty_ring {
                            grid(ui, "dirty_ring_grid", 2, |ui| {
                                ui.label("状态:");
                                egui::ComboBox::from_id_source("dirty_ring_state")
                                    .selected_text(&dirty_ring.state)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut dirty_ring.state,
                                            "on".to_string(),
                                            "on",
                                        );
                                        ui.selectable_value(
                                            &mut dirty_ring.state,
                                            "off".to_string(),
                                            "off",
                                        );
                                    });
                                ui.end_row();
                                ui.label("Size:");
                                let size = dirty_ring.size.get_or_insert(4096);
                                ui.add(egui::Slider::new(size, 1024..=65536).text(""));
                                ui.end_row();
                            });
                        }
                    });
                }
            }
        });

        ui.add_space(8.0);

        card_group(ui, "Xen 特性", None, colors, |ui| {
            if let Some(ref mut features) = config.hypervisor_features {
                let mut has_xen = features.xen.is_some();
                if checkbox(ui, &mut has_xen, "启用 Xen 特性") {
                    if has_xen {
                        features.xen = Some(XenFeaturesConfig::default());
                    } else {
                        features.xen = None;
                    }
                }

                if let Some(ref mut xen) = features.xen {
                    ui.add_space(5.0);
                    grid(ui, "xen_features_grid", 2, |ui| {
                        feature_state_toggle(ui, "E820 Host", &mut xen.e820_host);
                        ui.end_row();
                    });

                    ui.add_space(5.0);

                    // Passthrough 配置
                    ui.collapsing("Passthrough 配置", |ui| {
                        if xen.passthrough.is_none() {
                            xen.passthrough =
                                Some(PassthroughConfig { state: "off".to_string(), mode: None });
                        }
                        if let Some(ref mut passthrough) = xen.passthrough {
                            grid(ui, "passthrough_grid", 2, |ui| {
                                ui.label("状态:");
                                egui::ComboBox::from_id_source("passthrough_state")
                                    .selected_text(&passthrough.state)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut passthrough.state,
                                            "on".to_string(),
                                            "on",
                                        );
                                        ui.selectable_value(
                                            &mut passthrough.state,
                                            "off".to_string(),
                                            "off",
                                        );
                                    });
                                ui.end_row();
                                ui.label("模式:");
                                let mode = passthrough.mode.get_or_insert_with(|| "".to_string());
                                egui::ComboBox::from_id_source("passthrough_mode")
                                    .selected_text(mode.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(mode, "".to_string(), "无");
                                        ui.selectable_value(mode, "sync_pt".to_string(), "sync_pt");
                                        ui.selectable_value(
                                            mode,
                                            "share_pt".to_string(),
                                            "share_pt",
                                        );
                                    });
                                ui.end_row();
                            });
                        }
                    });
                }
            }
        });

        ui.add_space(8.0);

        card_group(ui, "其他配置", None, colors, |ui| {
            if let Some(ref mut features) = config.hypervisor_features {
                // GIC 配置
                ui.collapsing("GIC (通用中断控制器)", |ui| {
                    let mut has_gic = features.gic.is_some();
                    if checkbox(ui, &mut has_gic, "启用 GIC") {
                        if has_gic {
                            features.gic = Some(GicConfig { version: Some("2".to_string()) });
                        } else {
                            features.gic = None;
                        }
                    }
                    if let Some(ref mut gic) = features.gic {
                        ui.add_space(5.0);
                        let version = gic.version.get_or_insert_with(|| "2".to_string());
                        egui::ComboBox::from_id_source("gic_version")
                            .selected_text(version.as_str())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(version, "2".to_string(), "2");
                                ui.selectable_value(version, "3".to_string(), "3");
                                ui.selectable_value(version, "host".to_string(), "host");
                            });
                    }
                });

                ui.add_space(5.0);

                // SMM 配置
                ui.collapsing("SMM (系统管理模式)", |ui| {
                    let mut has_smm = features.smm.is_some();
                    if checkbox(ui, &mut has_smm, "启用 SMM") {
                        if has_smm {
                            features.smm = Some(SmmConfig { state: "off".to_string(), tseg: None });
                        } else {
                            features.smm = None;
                        }
                    }
                    if let Some(ref mut smm) = features.smm {
                        ui.add_space(5.0);
                        grid(ui, "smm_grid", 2, |ui| {
                            ui.label("状态:");
                            egui::ComboBox::from_id_source("smm_state")
                                .selected_text(&smm.state)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut smm.state, "on".to_string(), "on");
                                    ui.selectable_value(&mut smm.state, "off".to_string(), "off");
                                });
                            ui.end_row();
                            ui.label("TSEG:");
                            if smm.tseg.is_none() {
                                smm.tseg =
                                    Some(TsegConfig { unit: Some("MiB".to_string()), value: 16 });
                            }
                            if let Some(ref mut tseg) = smm.tseg {
                                let unit = tseg.unit.get_or_insert_with(|| "MiB".to_string());
                                egui::ComboBox::from_id_source("tseg_unit")
                                    .selected_text(unit.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(unit, "KiB".to_string(), "KiB");
                                        ui.selectable_value(unit, "MiB".to_string(), "MiB");
                                        ui.selectable_value(unit, "GiB".to_string(), "GiB");
                                    });
                                ui.end_row();
                                ui.label("大小:");
                                ui.add(egui::Slider::new(&mut tseg.value, 0..=1024).text(""));
                                ui.end_row();
                            }
                        });
                    }
                });

                ui.add_space(5.0);

                // IOAPIC 配置
                ui.collapsing("IOAPIC", |ui| {
                    let mut has_ioapic = features.ioapic.is_some();
                    if checkbox(ui, &mut has_ioapic, "启用 IOAPIC") {
                        if has_ioapic {
                            features.ioapic =
                                Some(IoapicConfig { driver: Some("kvm".to_string()) });
                        } else {
                            features.ioapic = None;
                        }
                    }
                    if let Some(ref mut ioapic) = features.ioapic {
                        ui.add_space(5.0);
                        let driver = ioapic.driver.get_or_insert_with(|| "kvm".to_string());
                        egui::ComboBox::from_id_source("ioapic_driver")
                            .selected_text(driver.as_str())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(driver, "kvm".to_string(), "kvm");
                                ui.selectable_value(driver, "qemu".to_string(), "qemu");
                            });
                    }
                });

                ui.add_space(5.0);

                // HPT 配置
                ui.collapsing("HPT (哈希页表)", |ui| {
                    let mut has_hpt = features.hpt.is_some();
                    if checkbox(ui, &mut has_hpt, "启用 HPT") {
                        if has_hpt {
                            features.hpt = Some(HptConfig {
                                resizing: Some("enabled".to_string()),
                                maxpagesize: None,
                            });
                        } else {
                            features.hpt = None;
                        }
                    }
                    if let Some(ref mut hpt) = features.hpt {
                        ui.add_space(5.0);
                        grid(ui, "hpt_grid", 2, |ui| {
                            ui.label("Resizing:");
                            let resizing =
                                hpt.resizing.get_or_insert_with(|| "enabled".to_string());
                            egui::ComboBox::from_id_source("hpt_resizing")
                                .selected_text(resizing.as_str())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(resizing, "enabled".to_string(), "enabled");
                                    ui.selectable_value(
                                        resizing,
                                        "disabled".to_string(),
                                        "disabled",
                                    );
                                    ui.selectable_value(
                                        resizing,
                                        "required".to_string(),
                                        "required",
                                    );
                                });
                            ui.end_row();
                            ui.label("最大页大小:");
                            if hpt.maxpagesize.is_none() {
                                hpt.maxpagesize = Some(MaxPageSizeConfig {
                                    unit: Some("MiB".to_string()),
                                    value: 16,
                                });
                            }
                            if let Some(ref mut maxpagesize) = hpt.maxpagesize {
                                let unit =
                                    maxpagesize.unit.get_or_insert_with(|| "MiB".to_string());
                                egui::ComboBox::from_id_source("maxpagesize_unit")
                                    .selected_text(unit.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(unit, "KiB".to_string(), "KiB");
                                        ui.selectable_value(unit, "MiB".to_string(), "MiB");
                                        ui.selectable_value(unit, "GiB".to_string(), "GiB");
                                    });
                                ui.end_row();
                                ui.label("大小:");
                                ui.add(
                                    egui::Slider::new(&mut maxpagesize.value, 1..=1024).text(""),
                                );
                                ui.end_row();
                            }
                        });
                    }
                });

                ui.add_space(5.0);

                // MSRs 配置
                ui.collapsing("MSRs (模型特定寄存器)", |ui| {
                    let mut has_msrs = features.msrs.is_some();
                    if checkbox(ui, &mut has_msrs, "启用 MSRs") {
                        if has_msrs {
                            features.msrs = Some(MsrsConfig { unknown: None });
                        } else {
                            features.msrs = None;
                        }
                    }
                    if let Some(ref mut msrs) = features.msrs {
                        ui.add_space(5.0);
                        let unknown = msrs.unknown.get_or_insert_with(|| "".to_string());
                        egui::ComboBox::from_id_source("msrs_unknown")
                            .selected_text(unknown.as_str())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(unknown, "".to_string(), "默认");
                                ui.selectable_value(unknown, "ignore".to_string(), "ignore");
                                ui.selectable_value(unknown, "fault".to_string(), "fault");
                            });
                    }
                });

                ui.add_space(5.0);

                // CFPC 配置
                ui.collapsing("CFPC", |ui| {
                    let mut has_cfpc = features.cfpc.is_some();
                    if checkbox(ui, &mut has_cfpc, "启用 CFPC") {
                        if has_cfpc {
                            features.cfpc = Some(CfpcConfig { value: None });
                        } else {
                            features.cfpc = None;
                        }
                    }
                    if let Some(ref mut cfpc) = features.cfpc {
                        ui.add_space(5.0);
                        let value = cfpc.value.get_or_insert_with(|| "".to_string());
                        egui::ComboBox::from_id_source("cfpc_value")
                            .selected_text(value.as_str())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(value, "".to_string(), "默认");
                                ui.selectable_value(value, "broken".to_string(), "broken");
                                ui.selectable_value(value, "workaround".to_string(), "workaround");
                                ui.selectable_value(value, "fixed".to_string(), "fixed");
                            });
                    }
                });

                ui.add_space(5.0);

                // SBBC 配置
                ui.collapsing("SBBC", |ui| {
                    let mut has_sbbc = features.sbbc.is_some();
                    if checkbox(ui, &mut has_sbbc, "启用 SBBC") {
                        if has_sbbc {
                            features.sbbc = Some(SbbcConfig { value: None });
                        } else {
                            features.sbbc = None;
                        }
                    }
                    if let Some(ref mut sbbc) = features.sbbc {
                        ui.add_space(5.0);
                        let value = sbbc.value.get_or_insert_with(|| "".to_string());
                        egui::ComboBox::from_id_source("sbbc_value")
                            .selected_text(value.as_str())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(value, "".to_string(), "默认");
                                ui.selectable_value(value, "broken".to_string(), "broken");
                                ui.selectable_value(value, "workaround".to_string(), "workaround");
                                ui.selectable_value(value, "fixed".to_string(), "fixed");
                            });
                    }
                });

                ui.add_space(5.0);

                // IBS 配置
                ui.collapsing("IBS", |ui| {
                    let mut has_ibs = features.ibs.is_some();
                    if checkbox(ui, &mut has_ibs, "启用 IBS") {
                        if has_ibs {
                            features.ibs = Some(IbsConfig { value: None });
                        } else {
                            features.ibs = None;
                        }
                    }
                    if let Some(ref mut ibs) = features.ibs {
                        ui.add_space(5.0);
                        let value = ibs.value.get_or_insert_with(|| "".to_string());
                        egui::ComboBox::from_id_source("ibs_value")
                            .selected_text(value.as_str())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(value, "".to_string(), "默认");
                                ui.selectable_value(value, "fixed-na".to_string(), "fixed-na");
                                ui.selectable_value(value, "workaround".to_string(), "workaround");
                                ui.selectable_value(value, "fixed".to_string(), "fixed");
                            });
                    }
                });

                ui.add_space(5.0);

                // TCG 配置
                ui.collapsing("TCG", |ui| {
                    let mut has_tcg = features.tcg.is_some();
                    if checkbox(ui, &mut has_tcg, "启用 TCG") {
                        if has_tcg {
                            features.tcg = Some(TcgConfig { tb_cache: None });
                        } else {
                            features.tcg = None;
                        }
                    }
                    if let Some(ref mut tcg) = features.tcg {
                        if tcg.tb_cache.is_none() {
                            tcg.tb_cache =
                                Some(TbCacheConfig { unit: Some("MiB".to_string()), value: 128 });
                        }
                        if let Some(ref mut tb_cache) = tcg.tb_cache {
                            ui.add_space(5.0);
                            grid(ui, "tb_cache_grid", 2, |ui| {
                                ui.label("单位:");
                                let unit = tb_cache.unit.get_or_insert_with(|| "MiB".to_string());
                                egui::ComboBox::from_id_source("tb_cache_unit")
                                    .selected_text(unit.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(unit, "KiB".to_string(), "KiB");
                                        ui.selectable_value(unit, "MiB".to_string(), "MiB");
                                        ui.selectable_value(unit, "GiB".to_string(), "GiB");
                                    });
                                ui.end_row();
                                ui.label("大小:");
                                ui.add(egui::Slider::new(&mut tb_cache.value, 1..=1024).text(""));
                                ui.end_row();
                            });
                        }
                    }
                });

                ui.add_space(5.0);

                // AIA 配置
                ui.collapsing("AIA", |ui| {
                    let mut has_aia = features.aia.is_some();
                    if checkbox(ui, &mut has_aia, "启用 AIA") {
                        if has_aia {
                            features.aia = Some(AiaConfig { value: "aplic-imsic".to_string() });
                        } else {
                            features.aia = None;
                        }
                    }
                    if let Some(ref mut aia) = features.aia {
                        ui.add_space(5.0);
                        egui::ComboBox::from_id_source("aia_value")
                            .selected_text(&aia.value)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut aia.value,
                                    "aplic-imsic".to_string(),
                                    "aplic-imsic",
                                );
                            });
                    }
                });
            }
        });
    }
}

/// 简单的特性开关 UI 组件
fn feature_toggle(ui: &mut egui::Ui, label: &str, state: &mut Option<String>) {
    ui.label(format!("{}:", label));
    if state.is_none() {
        *state = Some("off".to_string());
    }
    egui::ComboBox::from_id_source(format!("feature_{}", label))
        .selected_text(state.as_ref().unwrap())
        .show_ui(ui, |ui| {
            ui.selectable_value(state, Some("on".to_string()), "on");
            ui.selectable_value(state, Some("off".to_string()), "off");
        });
}

/// 带 state 属性的特性 UI 组件
fn feature_state_toggle(ui: &mut egui::Ui, label: &str, feature: &mut Option<FeatureState>) {
    ui.label(format!("{}:", label));
    if feature.is_none() {
        *feature = Some(FeatureState { state: "off".to_string() });
    }
    if let Some(ref mut f) = feature {
        egui::ComboBox::from_id_source(format!("feature_state_{}", label))
            .selected_text(&f.state)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut f.state, "on".to_string(), "on");
                ui.selectable_value(&mut f.state, "off".to_string(), "off");
            });
    }
}
