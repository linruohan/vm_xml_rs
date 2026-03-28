use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use crate::model::VMConfig;

/// 写入 CPU 配置
pub fn write_cpu<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    if config.cpu.topology.is_some()
        || config.cpu.mode.is_some()
        || config.cpu.model.is_some()
        || config.cpu.vendor.is_some()
        || config.cpu.feature.is_some()
        || config.cpu.cache.is_some()
        || config.cpu.maxphysaddr.is_some()
        || config.cpu.numa.is_some()
    {
        let mut cpu_elem = BytesStart::new("cpu");
        if let Some(ref mode) = config.cpu.mode {
            cpu_elem.push_attribute(("mode", mode.as_str()));
        }
        if let Some(ref match_) = config.cpu.match_ {
            cpu_elem.push_attribute(("match", match_.as_str()));
        }
        if let Some(ref check) = config.cpu.check {
            cpu_elem.push_attribute(("check", check.as_str()));
        }
        if let Some(ref migratable) = config.cpu.migratable {
            cpu_elem.push_attribute(("migratable", migratable.as_str()));
        }
        writer.write_event(Event::Start(cpu_elem)).map_err(|e| e.to_string())?;

        if let Some(ref model) = config.cpu.model {
            let mut model_elem = BytesStart::new("model");
            if let Some(ref fallback) = model.fallback {
                model_elem.push_attribute(("fallback", fallback.as_str()));
            }
            if let Some(ref vendor_id) = model.vendor_id {
                model_elem.push_attribute(("vendor_id", vendor_id.as_str()));
            }
            writer.write_event(Event::Start(model_elem)).map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(&model.name)))
                .map_err(|e| e.to_string())?;
            writer.write_event(Event::End(BytesEnd::new("model"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref vendor) = config.cpu.vendor {
            super::general::write_element(writer, "vendor", vendor)?;
        }

        if let Some(ref topology) = config.cpu.topology {
            let mut topo_elem = BytesStart::new("topology");
            topo_elem.push_attribute(("sockets", topology.sockets.to_string().as_str()));
            if let Some(dies) = topology.dies {
                topo_elem.push_attribute(("dies", dies.to_string().as_str()));
            }
            if let Some(clusters) = topology.clusters {
                topo_elem.push_attribute(("clusters", clusters.to_string().as_str()));
            }
            topo_elem.push_attribute(("cores", topology.cores.to_string().as_str()));
            topo_elem.push_attribute(("threads", topology.threads.to_string().as_str()));
            writer.write_event(Event::Empty(topo_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref feature_list) = config.cpu.feature {
            for feature in feature_list {
                let mut feature_elem = BytesStart::new("feature");
                feature_elem.push_attribute(("name", feature.name.as_str()));
                feature_elem.push_attribute(("policy", feature.policy.as_str()));
                writer.write_event(Event::Empty(feature_elem)).map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref cache_list) = config.cpu.cache {
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

        if let Some(ref maxphysaddr) = config.cpu.maxphysaddr {
            let mut maxphysaddr_elem = BytesStart::new("maxphysaddr");
            maxphysaddr_elem.push_attribute(("mode", maxphysaddr.mode.as_str()));
            if let Some(bits) = maxphysaddr.bits {
                maxphysaddr_elem.push_attribute(("bits", bits.to_string().as_str()));
            }
            if let Some(limit) = maxphysaddr.limit {
                maxphysaddr_elem.push_attribute(("limit", limit.to_string().as_str()));
            }
            writer.write_event(Event::Empty(maxphysaddr_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref numa) = config.cpu.numa {
            let numa_elem = BytesStart::new("numa");
            writer.write_event(Event::Start(numa_elem)).map_err(|e| e.to_string())?;

            if let Some(ref cell_list) = numa.cell {
                for cell in cell_list {
                    let mut cell_elem = BytesStart::new("cell");
                    cell_elem.push_attribute(("id", cell.id.to_string().as_str()));
                    cell_elem.push_attribute(("cpus", cell.cpus.as_str()));
                    cell_elem.push_attribute(("memory", cell.memory.to_string().as_str()));
                    if let Some(ref unit) = cell.unit {
                        cell_elem.push_attribute(("unit", unit.as_str()));
                    }
                    if let Some(ref mem_access) = cell.mem_access {
                        cell_elem.push_attribute(("memAccess", mem_access.as_str()));
                    }
                    if let Some(ref discard) = cell.discard {
                        cell_elem.push_attribute(("discard", discard.as_str()));
                    }

                    if let Some(ref distances) = cell.distances {
                        writer.write_event(Event::Start(cell_elem)).map_err(|e| e.to_string())?;

                        let distances_elem = BytesStart::new("distances");
                        writer
                            .write_event(Event::Start(distances_elem))
                            .map_err(|e| e.to_string())?;

                        if let Some(ref sibling_list) = distances.sibling {
                            for sibling in sibling_list {
                                let mut sibling_elem = BytesStart::new("sibling");
                                sibling_elem
                                    .push_attribute(("id", sibling.id.to_string().as_str()));
                                sibling_elem
                                    .push_attribute(("value", sibling.value.to_string().as_str()));
                                writer
                                    .write_event(Event::Empty(sibling_elem))
                                    .map_err(|e| e.to_string())?;
                            }
                        }

                        writer
                            .write_event(Event::End(BytesEnd::new("distances")))
                            .map_err(|e| e.to_string())?;
                        writer
                            .write_event(Event::End(BytesEnd::new("cell")))
                            .map_err(|e| e.to_string())?;
                    } else {
                        writer.write_event(Event::Empty(cell_elem)).map_err(|e| e.to_string())?;
                    }
                }
            }

            if let Some(ref interconnects) = numa.interconnects {
                let interconnects_elem = BytesStart::new("interconnects");
                writer.write_event(Event::Start(interconnects_elem)).map_err(|e| e.to_string())?;

                if let Some(ref latency_list) = interconnects.latency {
                    for latency in latency_list {
                        let mut latency_elem = BytesStart::new("latency");
                        latency_elem
                            .push_attribute(("initiator", latency.initiator.to_string().as_str()));
                        latency_elem
                            .push_attribute(("target", latency.target.to_string().as_str()));
                        latency_elem.push_attribute(("type", latency.type_.as_str()));
                        latency_elem.push_attribute(("value", latency.value.to_string().as_str()));
                        if let Some(cache) = latency.cache {
                            latency_elem.push_attribute(("cache", cache.to_string().as_str()));
                        }
                        writer
                            .write_event(Event::Empty(latency_elem))
                            .map_err(|e| e.to_string())?;
                    }
                }

                if let Some(ref bandwidth_list) = interconnects.bandwidth {
                    for bandwidth in bandwidth_list {
                        let mut bandwidth_elem = BytesStart::new("bandwidth");
                        bandwidth_elem.push_attribute((
                            "initiator",
                            bandwidth.initiator.to_string().as_str(),
                        ));
                        bandwidth_elem
                            .push_attribute(("target", bandwidth.target.to_string().as_str()));
                        bandwidth_elem.push_attribute(("type", bandwidth.type_.as_str()));
                        bandwidth_elem
                            .push_attribute(("value", bandwidth.value.to_string().as_str()));
                        if let Some(ref unit) = bandwidth.unit {
                            bandwidth_elem.push_attribute(("unit", unit.as_str()));
                        }
                        writer
                            .write_event(Event::Empty(bandwidth_elem))
                            .map_err(|e| e.to_string())?;
                    }
                }

                writer
                    .write_event(Event::End(BytesEnd::new("interconnects")))
                    .map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("numa"))).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("cpu"))).map_err(|e| e.to_string())?;
    }

    Ok(())
}
