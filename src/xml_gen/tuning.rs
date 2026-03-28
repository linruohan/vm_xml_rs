#![allow(dead_code)]

use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Writer,
};

use super::general::write_element;
use crate::model::VMConfig;

/// 写入 CPU 调优配置（cputune 部分）
pub fn write_cputune<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    if let Some(ref cputune) = config.cpu_tuning {
        let cputune_elem = BytesStart::new("cputune");
        writer.write_event(Event::Start(cputune_elem)).map_err(|e| e.to_string())?;

        if let Some(ref vcpupin_list) = cputune.vcpupin {
            for vcpupin in vcpupin_list {
                let mut vcpupin_elem = BytesStart::new("vcpupin");
                vcpupin_elem.push_attribute(("vcpu", vcpupin.vcpu.to_string().as_str()));
                vcpupin_elem.push_attribute(("cpuset", vcpupin.cpuset.as_str()));
                writer.write_event(Event::Empty(vcpupin_elem)).map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref emulatorpin) = cputune.emulatorpin {
            let mut emulatorpin_elem = BytesStart::new("emulatorpin");
            emulatorpin_elem.push_attribute(("cpuset", emulatorpin.cpuset.as_str()));
            writer.write_event(Event::Empty(emulatorpin_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref iothreadpin_list) = cputune.iothreadpin {
            for iothreadpin in iothreadpin_list {
                let mut iothreadpin_elem = BytesStart::new("iothreadpin");
                iothreadpin_elem
                    .push_attribute(("iothread", iothreadpin.iothread.to_string().as_str()));
                iothreadpin_elem.push_attribute(("cpuset", iothreadpin.cpuset.as_str()));
                writer.write_event(Event::Empty(iothreadpin_elem)).map_err(|e| e.to_string())?;
            }
        }

        if let Some(shares) = cputune.shares {
            write_element(writer, "shares", &shares.to_string())?;
        }

        if let Some(period) = cputune.period {
            write_element(writer, "period", &period.to_string())?;
        }

        if let Some(quota) = cputune.quota {
            write_element(writer, "quota", &quota.to_string())?;
        }

        if let Some(global_period) = cputune.global_period {
            write_element(writer, "global_period", &global_period.to_string())?;
        }

        if let Some(global_quota) = cputune.global_quota {
            write_element(writer, "global_quota", &global_quota.to_string())?;
        }

        if let Some(emulator_period) = cputune.emulator_period {
            write_element(writer, "emulator_period", &emulator_period.to_string())?;
        }

        if let Some(emulator_quota) = cputune.emulator_quota {
            write_element(writer, "emulator_quota", &emulator_quota.to_string())?;
        }

        if let Some(iothread_period) = cputune.iothread_period {
            write_element(writer, "iothread_period", &iothread_period.to_string())?;
        }

        if let Some(iothread_quota) = cputune.iothread_quota {
            write_element(writer, "iothread_quota", &iothread_quota.to_string())?;
        }

        if let Some(ref vcpusched_list) = cputune.vcpusched {
            for vcpusched in vcpusched_list {
                let mut vcpusched_elem = BytesStart::new("vcpusched");
                vcpusched_elem.push_attribute(("vcpus", vcpusched.vcpus.as_str()));
                vcpusched_elem.push_attribute(("scheduler", vcpusched.scheduler.as_str()));
                if let Some(ref priority) = vcpusched.priority {
                    vcpusched_elem.push_attribute(("priority", priority.to_string().as_str()));
                }
                writer.write_event(Event::Empty(vcpusched_elem)).map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref iothreadsched_list) = cputune.iothreadsched {
            for iothreadsched in iothreadsched_list {
                let mut iothreadsched_elem = BytesStart::new("iothreadsched");
                iothreadsched_elem.push_attribute(("iothreads", iothreadsched.iothreads.as_str()));
                iothreadsched_elem.push_attribute(("scheduler", iothreadsched.scheduler.as_str()));
                if let Some(ref priority) = iothreadsched.priority {
                    iothreadsched_elem.push_attribute(("priority", priority.to_string().as_str()));
                }
                writer.write_event(Event::Empty(iothreadsched_elem)).map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref emulatorsched) = cputune.emulatorsched {
            let mut emulatorsched_elem = BytesStart::new("emulatorsched");
            emulatorsched_elem.push_attribute(("scheduler", emulatorsched.scheduler.as_str()));
            if let Some(ref priority) = emulatorsched.priority {
                emulatorsched_elem.push_attribute(("priority", priority.to_string().as_str()));
            }
            writer.write_event(Event::Empty(emulatorsched_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref cachetune_list) = cputune.cachetune {
            for cachetune in cachetune_list {
                let mut cachetune_elem = BytesStart::new("cachetune");
                cachetune_elem.push_attribute(("vcpus", cachetune.vcpus.as_str()));
                writer.write_event(Event::Start(cachetune_elem)).map_err(|e| e.to_string())?;

                if let Some(ref cache_list) = cachetune.cache {
                    for cache in cache_list {
                        let mut cache_elem = BytesStart::new("cache");
                        if let Some(level) = cache.level {
                            cache_elem.push_attribute(("level", level.to_string().as_str()));
                        }
                        if let Some(ref mode) = cache.mode {
                            cache_elem.push_attribute(("mode", mode.as_str()));
                        }
                        writer.write_event(Event::Empty(cache_elem)).map_err(|e| e.to_string())?;
                    }
                }

                if let Some(ref monitor_list) = cachetune.monitor {
                    for monitor in monitor_list {
                        let mut monitor_elem = BytesStart::new("monitor");
                        monitor_elem.push_attribute(("level", monitor.level.to_string().as_str()));
                        monitor_elem.push_attribute(("vcpus", monitor.vcpus.as_str()));
                        writer
                            .write_event(Event::Empty(monitor_elem))
                            .map_err(|e| e.to_string())?;
                    }
                }

                writer
                    .write_event(Event::End(BytesEnd::new("cachetune")))
                    .map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref memorytune_list) = cputune.memorytune {
            for memorytune in memorytune_list {
                let mut memorytune_elem = BytesStart::new("memorytune");
                memorytune_elem.push_attribute(("vcpus", memorytune.vcpus.as_str()));
                writer.write_event(Event::Start(memorytune_elem)).map_err(|e| e.to_string())?;

                if let Some(ref node_list) = memorytune.node {
                    for node in node_list {
                        let mut node_elem = BytesStart::new("node");
                        node_elem.push_attribute(("id", node.id.to_string().as_str()));
                        node_elem
                            .push_attribute(("bandwidth", node.bandwidth.to_string().as_str()));
                        writer.write_event(Event::Empty(node_elem)).map_err(|e| e.to_string())?;
                    }
                }

                writer
                    .write_event(Event::End(BytesEnd::new("memorytune")))
                    .map_err(|e| e.to_string())?;
            }
        }

        writer.write_event(Event::End(BytesEnd::new("cputune"))).map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// 写入内存调优配置（memtune 部分）
pub fn write_memtune<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    if let Some(ref memtune) = config.memory_tuning {
        let memtune_elem = BytesStart::new("memtune");
        writer.write_event(Event::Start(memtune_elem)).map_err(|e| e.to_string())?;

        if let Some(hard_limit) = memtune.hard_limit {
            write_element(writer, "hard_limit", &hard_limit.to_string())?;
        }

        if let Some(soft_limit) = memtune.soft_limit {
            write_element(writer, "soft_limit", &soft_limit.to_string())?;
        }

        if let Some(swap_hard_limit) = memtune.swap_hard_limit {
            write_element(writer, "swap_hard_limit", &swap_hard_limit.to_string())?;
        }

        if let Some(guarantee) = memtune.guarantee {
            write_element(writer, "min_guarantee", &guarantee.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("memtune"))).map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// 写入块 IO 调优配置（blkiotune 部分）
pub fn write_blkiotune<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    if let Some(ref blkiotune) = config.blockio_tuning {
        let blkiotune_elem = BytesStart::new("blkiotune");
        writer.write_event(Event::Start(blkiotune_elem)).map_err(|e| e.to_string())?;

        if let Some(weight) = blkiotune.weight {
            write_element(writer, "weight", &weight.to_string())?;
        }

        if let Some(ref device_weight_list) = blkiotune.device_weight {
            for device_weight in device_weight_list {
                let device_elem = BytesStart::new("device");
                writer.write_event(Event::Start(device_elem)).map_err(|e| e.to_string())?;

                write_element(writer, "path", &device_weight.dev)?;
                write_element(writer, "weight", &device_weight.weight.to_string())?;

                writer
                    .write_event(Event::End(BytesEnd::new("device")))
                    .map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref throttle) = blkiotune.throttle {
            if let Some(read_bytes_sec) = throttle.read_bytes_sec {
                write_element(writer, "read_bytes_sec", &read_bytes_sec.to_string())?;
            }
            if let Some(write_bytes_sec) = throttle.write_bytes_sec {
                write_element(writer, "write_bytes_sec", &write_bytes_sec.to_string())?;
            }
            if let Some(read_iops_sec) = throttle.read_iops_sec {
                write_element(writer, "read_iops_sec", &read_iops_sec.to_string())?;
            }
            if let Some(write_iops_sec) = throttle.write_iops_sec {
                write_element(writer, "write_iops_sec", &write_iops_sec.to_string())?;
            }
        }

        writer.write_event(Event::End(BytesEnd::new("blkiotune"))).map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// 写入资源分区配置
pub fn write_resource<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    if let Some(ref resource) = config.resource_partitioning {
        let resource_elem = BytesStart::new("resource");
        writer.write_event(Event::Start(resource_elem)).map_err(|e| e.to_string())?;

        if let Some(ref cpuset) = resource.cpuset {
            write_element(writer, "partition", cpuset)?;
        }

        writer.write_event(Event::End(BytesEnd::new("resource"))).map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// 写入电源管理配置
pub fn write_power_management<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    if let Some(ref pm) = config.power_management {
        let pm_elem = BytesStart::new("pm");
        writer.write_event(Event::Start(pm_elem)).map_err(|e| e.to_string())?;

        let mut std_elem = BytesStart::new("suspend-to-disk");
        std_elem.push_attribute(("enabled", if pm.suspend_to_disk { "yes" } else { "no" }));
        writer.write_event(Event::Empty(std_elem)).map_err(|e| e.to_string())?;

        let mut stm_elem = BytesStart::new("suspend-to-mem");
        stm_elem.push_attribute(("enabled", if pm.suspend_to_ram { "yes" } else { "no" }));
        writer.write_event(Event::Empty(stm_elem)).map_err(|e| e.to_string())?;

        writer.write_event(Event::End(BytesEnd::new("pm"))).map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// 写入磁盘限流组配置
pub fn write_disk_throttle_group<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    if let Some(ref throttlegroup) = config.disk_throttle_group {
        let tg_elem = BytesStart::new("throttlegroups");
        writer.write_event(Event::Start(tg_elem)).map_err(|e| e.to_string())?;

        let tgroup_elem = BytesStart::new("throttlegroup");
        writer.write_event(Event::Start(tgroup_elem)).map_err(|e| e.to_string())?;

        write_element(writer, "group_name", &throttlegroup.name)?;

        if let Some(ref throttle) = throttlegroup.throttle {
            if let Some(read_bytes_sec) = throttle.read_bytes_sec {
                write_element(writer, "read_bytes_sec", &read_bytes_sec.to_string())?;
            }
            if let Some(write_bytes_sec) = throttle.write_bytes_sec {
                write_element(writer, "write_bytes_sec", &write_bytes_sec.to_string())?;
            }
            if let Some(read_iops_sec) = throttle.read_iops_sec {
                write_element(writer, "read_iops_sec", &read_iops_sec.to_string())?;
            }
            if let Some(write_iops_sec) = throttle.write_iops_sec {
                write_element(writer, "write_iops_sec", &write_iops_sec.to_string())?;
            }
        }

        writer
            .write_event(Event::End(BytesEnd::new("throttlegroup")))
            .map_err(|e| e.to_string())?;

        writer
            .write_event(Event::End(BytesEnd::new("throttlegroups")))
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
