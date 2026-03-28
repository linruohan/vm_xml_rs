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
                    writer.write_event(Event::Empty(timer_elem)).map_err(|e| e.to_string())?;
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
