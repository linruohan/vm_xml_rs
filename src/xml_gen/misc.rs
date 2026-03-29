use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use super::general::write_element;
use crate::model::VMConfig;

/// 写入事件处理配置
pub fn write_events<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    if let Some(ref events) = config.events {
        if let Some(ref on_poweroff) = events.on_poweroff {
            write_element(writer, "on_poweroff", on_poweroff)?;
        }

        if let Some(ref on_reboot) = events.on_reboot {
            write_element(writer, "on_reboot", on_reboot)?;
        }

        if let Some(ref on_crash) = events.on_crash {
            write_element(writer, "on_crash", on_crash)?;
        }

        if let Some(ref on_lockfailure) = events.on_lockfailure {
            write_element(writer, "on_lockfailure", on_lockfailure)?;
        }
    }

    Ok(())
}

/// 写入虚拟机监控器特性配置
pub fn write_features<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    if let Some(ref hypervisor_features) = config.hypervisor_features {
        let features_elem = BytesStart::new("features");
        writer.write_event(Event::Start(features_elem)).map_err(|e| e.to_string())?;

        // 基础特性（简单开关）
        write_feature_state(writer, "pae", &hypervisor_features.pae)?;
        write_feature_state(writer, "acpi", &hypervisor_features.acpi)?;
        write_feature_state(writer, "apic", &hypervisor_features.apic)?;
        write_feature_state(writer, "hap", &hypervisor_features.hap)?;
        write_feature_state(writer, "viridian", &hypervisor_features.viridian)?;
        write_feature_state(writer, "privnet", &hypervisor_features.privnet)?;
        write_feature_state(writer, "pvspinlock", &hypervisor_features.pvspinlock)?;
        write_feature_state(writer, "pmu", &hypervisor_features.pmu)?;
        write_feature_state(writer, "vmport", &hypervisor_features.vmport)?;
        write_feature_state(writer, "vmcoreinfo", &hypervisor_features.vmcoreinfo)?;
        write_feature_state(writer, "htm", &hypervisor_features.htm)?;
        write_feature_state(writer, "nested-hv", &hypervisor_features.nested_hv)?;
        write_feature_state(writer, "ccf-assist", &hypervisor_features.ccf_assist)?;
        write_feature_state(writer, "hrt", &hypervisor_features.hrt)?;
        write_feature_state(writer, "async-teardown", &hypervisor_features.async_teardown)?;
        write_feature_state(writer, "ras", &hypervisor_features.ras)?;
        write_feature_state(writer, "ps2", &hypervisor_features.ps2)?;

        // Hyper-V 特性
        if let Some(ref hyperv) = hypervisor_features.hyperv {
            let mut hyperv_elem = BytesStart::new("hyperv");
            if let Some(ref mode) = hyperv.mode {
                hyperv_elem.push_attribute(("mode", mode.as_str()));
            }
            writer.write_event(Event::Start(hyperv_elem)).map_err(|e| e.to_string())?;

            write_feature_state_elem(writer, "relaxed", &hyperv.relaxed)?;
            write_feature_state_elem(writer, "vapic", &hyperv.vapic)?;

            if let Some(ref spinlocks) = hyperv.spinlocks {
                let mut spinlocks_elem = BytesStart::new("spinlocks");
                spinlocks_elem.push_attribute(("state", spinlocks.state.as_str()));
                if let Some(retries) = spinlocks.retries {
                    spinlocks_elem.push_attribute(("retries", retries.to_string().as_str()));
                }
                writer.write_event(Event::Empty(spinlocks_elem)).map_err(|e| e.to_string())?;
            }

            write_feature_state_elem(writer, "vpindex", &hyperv.vpindex)?;
            write_feature_state_elem(writer, "runtime", &hyperv.runtime)?;
            write_feature_state_elem(writer, "synic", &hyperv.synic)?;

            if let Some(ref stimer) = hyperv.stimer {
                let mut stimer_elem = BytesStart::new("stimer");
                stimer_elem.push_attribute(("state", stimer.state.as_str()));
                writer.write_event(Event::Start(stimer_elem)).map_err(|e| e.to_string())?;
                if let Some(ref direct) = stimer.direct {
                    let mut direct_elem = BytesStart::new("direct");
                    direct_elem.push_attribute(("state", direct.state.as_str()));
                    writer.write_event(Event::Empty(direct_elem)).map_err(|e| e.to_string())?;
                }
                writer
                    .write_event(Event::End(BytesEnd::new("stimer")))
                    .map_err(|e| e.to_string())?;
            }

            write_feature_state_elem(writer, "reset", &hyperv.reset)?;

            if let Some(ref vendor_id) = hyperv.vendor_id {
                let mut vendor_id_elem = BytesStart::new("vendor_id");
                vendor_id_elem.push_attribute(("state", vendor_id.state.as_str()));
                if let Some(ref value) = vendor_id.value {
                    vendor_id_elem.push_attribute(("value", value.as_str()));
                }
                writer.write_event(Event::Empty(vendor_id_elem)).map_err(|e| e.to_string())?;
            }

            write_feature_state_elem(writer, "frequencies", &hyperv.frequencies)?;
            write_feature_state_elem(writer, "reenlightenment", &hyperv.reenlightenment)?;

            if let Some(ref tlbflush) = hyperv.tlbflush {
                let mut tlbflush_elem = BytesStart::new("tlbflush");
                tlbflush_elem.push_attribute(("state", tlbflush.state.as_str()));
                writer.write_event(Event::Start(tlbflush_elem)).map_err(|e| e.to_string())?;
                if let Some(ref direct) = tlbflush.direct {
                    let mut direct_elem = BytesStart::new("direct");
                    direct_elem.push_attribute(("state", direct.state.as_str()));
                    writer.write_event(Event::Empty(direct_elem)).map_err(|e| e.to_string())?;
                }
                if let Some(ref extended) = tlbflush.extended {
                    let mut extended_elem = BytesStart::new("extended");
                    extended_elem.push_attribute(("state", extended.state.as_str()));
                    writer.write_event(Event::Empty(extended_elem)).map_err(|e| e.to_string())?;
                }
                writer
                    .write_event(Event::End(BytesEnd::new("tlbflush")))
                    .map_err(|e| e.to_string())?;
            }

            write_feature_state_elem(writer, "ipi", &hyperv.ipi)?;
            write_feature_state_elem(writer, "avic", &hyperv.avic)?;
            write_feature_state_elem(writer, "evmcs", &hyperv.evmcs)?;
            write_feature_state_elem(writer, "emsr_bitmap", &hyperv.emsr_bitmap)?;
            write_feature_state_elem(writer, "xmm_input", &hyperv.xmm_input)?;

            writer.write_event(Event::End(BytesEnd::new("hyperv"))).map_err(|e| e.to_string())?;
        }

        // KVM 特性
        if let Some(ref kvm) = hypervisor_features.kvm {
            let kvm_elem = BytesStart::new("kvm");
            writer.write_event(Event::Start(kvm_elem)).map_err(|e| e.to_string())?;

            write_feature_state_elem(writer, "hidden", &kvm.hidden)?;
            write_feature_state_elem(writer, "hint-dedicated", &kvm.hint_dedicated)?;
            write_feature_state_elem(writer, "poll-control", &kvm.poll_control)?;
            write_feature_state_elem(writer, "pv-ipi", &kvm.pv_ipi)?;

            if let Some(ref dirty_ring) = kvm.dirty_ring {
                let mut dirty_ring_elem = BytesStart::new("dirty-ring");
                dirty_ring_elem.push_attribute(("state", dirty_ring.state.as_str()));
                if let Some(size) = dirty_ring.size {
                    dirty_ring_elem.push_attribute(("size", size.to_string().as_str()));
                }
                writer.write_event(Event::Empty(dirty_ring_elem)).map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("kvm"))).map_err(|e| e.to_string())?;
        }

        // Xen 特性
        if let Some(ref xen) = hypervisor_features.xen {
            let xen_elem = BytesStart::new("xen");
            writer.write_event(Event::Start(xen_elem)).map_err(|e| e.to_string())?;

            write_feature_state_elem(writer, "e820_host", &xen.e820_host)?;

            if let Some(ref passthrough) = xen.passthrough {
                let mut passthrough_elem = BytesStart::new("passthrough");
                passthrough_elem.push_attribute(("state", passthrough.state.as_str()));
                if let Some(ref mode) = passthrough.mode {
                    passthrough_elem.push_attribute(("mode", mode.as_str()));
                }
                writer.write_event(Event::Empty(passthrough_elem)).map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("xen"))).map_err(|e| e.to_string())?;
        }

        // GIC 配置
        if let Some(ref gic) = hypervisor_features.gic {
            let mut gic_elem = BytesStart::new("gic");
            if let Some(ref version) = gic.version {
                gic_elem.push_attribute(("version", version.as_str()));
            }
            writer.write_event(Event::Empty(gic_elem)).map_err(|e| e.to_string())?;
        }

        // SMM 配置
        if let Some(ref smm) = hypervisor_features.smm {
            let mut smm_elem = BytesStart::new("smm");
            smm_elem.push_attribute(("state", smm.state.as_str()));
            writer.write_event(Event::Start(smm_elem)).map_err(|e| e.to_string())?;

            if let Some(ref tseg) = smm.tseg {
                let mut tseg_elem = BytesStart::new("tseg");
                if let Some(ref unit) = tseg.unit {
                    tseg_elem.push_attribute(("unit", unit.as_str()));
                }
                writer
                    .write_event(Event::Text(BytesText::new(&tseg.value.to_string())))
                    .map_err(|e| e.to_string())?;
                writer.write_event(Event::End(BytesEnd::new("tseg"))).map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("smm"))).map_err(|e| e.to_string())?;
        }

        // IOAPIC 配置
        if let Some(ref ioapic) = hypervisor_features.ioapic {
            let mut ioapic_elem = BytesStart::new("ioapic");
            if let Some(ref driver) = ioapic.driver {
                ioapic_elem.push_attribute(("driver", driver.as_str()));
            }
            writer.write_event(Event::Empty(ioapic_elem)).map_err(|e| e.to_string())?;
        }

        // HPT 配置
        if let Some(ref hpt) = hypervisor_features.hpt {
            let mut hpt_elem = BytesStart::new("hpt");
            if let Some(ref resizing) = hpt.resizing {
                hpt_elem.push_attribute(("resizing", resizing.as_str()));
            }
            writer.write_event(Event::Start(hpt_elem)).map_err(|e| e.to_string())?;

            if let Some(ref maxpagesize) = hpt.maxpagesize {
                let mut maxpagesize_elem = BytesStart::new("maxpagesize");
                if let Some(ref unit) = maxpagesize.unit {
                    maxpagesize_elem.push_attribute(("unit", unit.as_str()));
                }
                writer
                    .write_event(Event::Text(BytesText::new(&maxpagesize.value.to_string())))
                    .map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::End(BytesEnd::new("maxpagesize")))
                    .map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("hpt"))).map_err(|e| e.to_string())?;
        }

        // MSRs 配置
        if let Some(ref msrs) = hypervisor_features.msrs {
            let mut msrs_elem = BytesStart::new("msrs");
            if let Some(ref unknown) = msrs.unknown {
                msrs_elem.push_attribute(("unknown", unknown.as_str()));
            }
            writer.write_event(Event::Empty(msrs_elem)).map_err(|e| e.to_string())?;
        }

        // CFPC 配置
        if let Some(ref cfpc) = hypervisor_features.cfpc {
            let mut cfpc_elem = BytesStart::new("cfpc");
            if let Some(ref value) = cfpc.value {
                cfpc_elem.push_attribute(("value", value.as_str()));
            }
            writer.write_event(Event::Empty(cfpc_elem)).map_err(|e| e.to_string())?;
        }

        // SBBC 配置
        if let Some(ref sbbc) = hypervisor_features.sbbc {
            let mut sbbc_elem = BytesStart::new("sbbc");
            if let Some(ref value) = sbbc.value {
                sbbc_elem.push_attribute(("value", value.as_str()));
            }
            writer.write_event(Event::Empty(sbbc_elem)).map_err(|e| e.to_string())?;
        }

        // IBS 配置
        if let Some(ref ibs) = hypervisor_features.ibs {
            let mut ibs_elem = BytesStart::new("ibs");
            if let Some(ref value) = ibs.value {
                ibs_elem.push_attribute(("value", value.as_str()));
            }
            writer.write_event(Event::Empty(ibs_elem)).map_err(|e| e.to_string())?;
        }

        // TCG 配置
        if let Some(ref tcg) = hypervisor_features.tcg {
            let tcg_elem = BytesStart::new("tcg");
            writer.write_event(Event::Start(tcg_elem)).map_err(|e| e.to_string())?;

            if let Some(ref tb_cache) = tcg.tb_cache {
                let mut tb_cache_elem = BytesStart::new("tb-cache");
                if let Some(ref unit) = tb_cache.unit {
                    tb_cache_elem.push_attribute(("unit", unit.as_str()));
                }
                writer
                    .write_event(Event::Text(BytesText::new(&tb_cache.value.to_string())))
                    .map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::End(BytesEnd::new("tb-cache")))
                    .map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("tcg"))).map_err(|e| e.to_string())?;
        }

        // AIA 配置
        if let Some(ref aia) = hypervisor_features.aia {
            let mut aia_elem = BytesStart::new("aia");
            aia_elem.push_attribute(("value", aia.value.as_str()));
            writer.write_event(Event::Empty(aia_elem)).map_err(|e| e.to_string())?;
        }

        // 通用特性列表
        if let Some(ref feature_list) = hypervisor_features.feature {
            for feature in feature_list {
                let mut feature_elem = BytesStart::new("feature");
                feature_elem.push_attribute(("enabled", feature.enabled.as_str()));
                feature_elem.push_attribute(("name", feature.name.as_str()));
                writer.write_event(Event::Empty(feature_elem)).map_err(|e| e.to_string())?;
            }
        }

        writer.write_event(Event::End(BytesEnd::new("features"))).map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// 写入特性状态（简单开关，如 <pae/>）
fn write_feature_state<W: std::io::Write>(
    writer: &mut Writer<W>,
    name: &str,
    state: &Option<String>,
) -> Result<(), String> {
    if let Some(ref value) = state {
        if value == "on" || value.is_empty() {
            let elem = BytesStart::new(name);
            writer.write_event(Event::Empty(elem)).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

/// 写入带 state 属性的特性元素
fn write_feature_state_elem<W: std::io::Write>(
    writer: &mut Writer<W>,
    name: &str,
    feature: &Option<crate::model::hypervisor_features::FeatureState>,
) -> Result<(), String> {
    if let Some(ref f) = feature {
        let mut elem = BytesStart::new(name);
        elem.push_attribute(("state", f.state.as_str()));
        writer.write_event(Event::Empty(elem)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入时钟配置
pub fn write_clock<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    if let Some(ref time_keeping) = config.time_keeping {
        if let Some(ref clock) = time_keeping.clock {
            let mut clock_elem = BytesStart::new("clock");
            clock_elem.push_attribute(("offset", clock.offset.as_str()));
            writer.write_event(Event::Start(clock_elem)).map_err(|e| e.to_string())?;

            if let Some(ref timer_list) = clock.timer {
                for timer in timer_list {
                    let mut timer_elem = BytesStart::new("timer");
                    timer_elem.push_attribute(("name", timer.name.as_str()));
                    if let Some(ref present) = timer.present {
                        timer_elem.push_attribute(("present", present.as_str()));
                    }
                    if let Some(frequency) = timer.frequency {
                        timer_elem.push_attribute(("frequency", frequency.to_string().as_str()));
                    }
                    if let Some(ref tickpolicy) = timer.tickpolicy {
                        timer_elem.push_attribute(("tickpolicy", tickpolicy.as_str()));
                    }
                    if let Some(ref track) = timer.track {
                        timer_elem.push_attribute(("track", track.as_str()));
                    }
                    if let Some(ref mode) = timer.mode {
                        timer_elem.push_attribute(("mode", mode.as_str()));
                    }

                    if let Some(ref catchup) = timer.catchup {
                        writer.write_event(Event::Start(timer_elem)).map_err(|e| e.to_string())?;

                        let mut catchup_elem = BytesStart::new("catchup");
                        if let Some(threshold) = catchup.threshold {
                            catchup_elem
                                .push_attribute(("threshold", threshold.to_string().as_str()));
                        }
                        if let Some(slew) = catchup.slew {
                            catchup_elem.push_attribute(("slew", slew.to_string().as_str()));
                        }
                        if let Some(limit) = catchup.limit {
                            catchup_elem.push_attribute(("limit", limit.to_string().as_str()));
                        }
                        writer
                            .write_event(Event::Empty(catchup_elem))
                            .map_err(|e| e.to_string())?;

                        writer
                            .write_event(Event::End(BytesEnd::new("timer")))
                            .map_err(|e| e.to_string())?;
                    } else {
                        writer.write_event(Event::Empty(timer_elem)).map_err(|e| e.to_string())?;
                    }
                }
            }

            writer.write_event(Event::End(BytesEnd::new("clock"))).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

/// 写入性能监控配置
pub fn write_perf<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    if let Some(ref performance_monitoring) = config.performance_monitoring {
        let perf_elem = BytesStart::new("perf");
        writer.write_event(Event::Start(perf_elem)).map_err(|e| e.to_string())?;

        if let Some(ref event_list) = performance_monitoring.events {
            for event in event_list {
                let mut event_elem = BytesStart::new("event");
                event_elem.push_attribute(("name", event.name.as_str()));
                if let Some(count) = event.count {
                    event_elem.push_attribute(("count", count.to_string().as_str()));
                }
                writer.write_event(Event::Empty(event_elem)).map_err(|e| e.to_string())?;
            }
        }

        writer.write_event(Event::End(BytesEnd::new("perf"))).map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// 写入 IO 线程配置
pub fn write_iothreads<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    if let Some(ref iothreads) = config.iothreads {
        if let Some(ref value) = iothreads.value {
            let iothreads_elem = BytesStart::new("iothreads");
            writer.write_event(Event::Start(iothreads_elem)).map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(&value.to_string())))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::End(BytesEnd::new("iothreads")))
                .map_err(|e| e.to_string())?;
        }

        if let Some(ref iothreadids) = iothreads.iothreadids {
            let iothreadids_elem = BytesStart::new("iothreadids");
            writer.write_event(Event::Start(iothreadids_elem)).map_err(|e| e.to_string())?;

            for iothread in iothreadids {
                let mut iothread_elem = BytesStart::new("iothread");
                iothread_elem.push_attribute(("id", iothread.id.to_string().as_str()));
                if let Some(ref thread_pool_min) = iothread.thread_pool_min {
                    iothread_elem
                        .push_attribute(("thread_pool_min", thread_pool_min.to_string().as_str()));
                }
                if let Some(ref thread_pool_max) = iothread.thread_pool_max {
                    iothread_elem
                        .push_attribute(("thread_pool_max", thread_pool_max.to_string().as_str()));
                }
                writer.write_event(Event::Start(iothread_elem)).map_err(|e| e.to_string())?;

                if let Some(ref poll) = iothread.poll {
                    let mut poll_elem = BytesStart::new("poll");
                    poll_elem.push_attribute(("max", poll.max.to_string().as_str()));
                    if let Some(ref grow) = poll.grow {
                        poll_elem.push_attribute(("grow", grow.to_string().as_str()));
                    }
                    if let Some(ref shrink) = poll.shrink {
                        poll_elem.push_attribute(("shrink", shrink.to_string().as_str()));
                    }
                    writer.write_event(Event::Empty(poll_elem)).map_err(|e| e.to_string())?;
                }

                writer
                    .write_event(Event::End(BytesEnd::new("iothread")))
                    .map_err(|e| e.to_string())?;
            }

            writer
                .write_event(Event::End(BytesEnd::new("iothreadids")))
                .map_err(|e| e.to_string())?;
        }

        if let Some(ref defaultiothread) = iothreads.defaultiothread {
            let mut defaultiothread_elem = BytesStart::new("defaultiothread");
            if let Some(ref thread_pool_min) = defaultiothread.thread_pool_min {
                defaultiothread_elem
                    .push_attribute(("thread_pool_min", thread_pool_min.to_string().as_str()));
            }
            if let Some(ref thread_pool_max) = defaultiothread.thread_pool_max {
                defaultiothread_elem
                    .push_attribute(("thread_pool_max", thread_pool_max.to_string().as_str()));
            }
            writer.write_event(Event::Empty(defaultiothread_elem)).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

/// 写入 NUMA 调优配置
pub fn write_numatune<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    if let Some(ref numatune) = config.numatune {
        let numatune_elem = BytesStart::new("numatune");
        writer.write_event(Event::Start(numatune_elem)).map_err(|e| e.to_string())?;

        if let Some(ref memory) = numatune.memory {
            let mut memory_elem = BytesStart::new("memory");
            if let Some(ref mode) = memory.mode {
                memory_elem.push_attribute(("mode", mode.as_str()));
            }
            if let Some(ref nodeset) = memory.nodeset {
                memory_elem.push_attribute(("nodeset", nodeset.as_str()));
            }
            if let Some(ref placement) = memory.placement {
                memory_elem.push_attribute(("placement", placement.as_str()));
            }
            writer.write_event(Event::Empty(memory_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref memnode_list) = numatune.memnode {
            for memnode in memnode_list {
                let mut memnode_elem = BytesStart::new("memnode");
                memnode_elem.push_attribute(("cellid", memnode.cellid.to_string().as_str()));
                if let Some(ref mode) = memnode.mode {
                    memnode_elem.push_attribute(("mode", mode.as_str()));
                }
                if let Some(ref nodeset) = memnode.nodeset {
                    memnode_elem.push_attribute(("nodeset", nodeset.as_str()));
                }
                writer.write_event(Event::Empty(memnode_elem)).map_err(|e| e.to_string())?;
            }
        }

        writer.write_event(Event::End(BytesEnd::new("numatune"))).map_err(|e| e.to_string())?;
    }

    Ok(())
}
