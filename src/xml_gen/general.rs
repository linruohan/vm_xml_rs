use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use crate::model::VMConfig;

/// 写入基础配置（general 部分）
pub fn write_general<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    write_element(writer, "name", &config.general.name)?;

    if let Some(ref uuid) = config.general.uuid {
        write_element(writer, "uuid", uuid)?;
    }

    if let Some(ref hwuuid) = config.general.hwuuid {
        write_element(writer, "hwuuid", hwuuid)?;
    }

    if let Some(ref genid) = config.general.genid {
        write_element(writer, "genid", genid)?;
    }

    if let Some(ref desc) = config.general.description {
        write_element(writer, "description", desc)?;
    }

    if let Some(ref title) = config.general.title {
        write_element(writer, "title", title)?;
    }

    if let Some(ref metadata) = config.general.metadata {
        let metadata_elem = BytesStart::new("metadata");
        writer.write_event(Event::Start(metadata_elem)).map_err(|e| e.to_string())?;

        for entry in &metadata.entries {
            let mut entry_elem =
                BytesStart::new(entry.xmlns.split(':').next_back().unwrap_or("entry"));
            entry_elem.push_attribute(("xmlns", entry.xmlns.as_str()));
            writer.write_event(Event::Start(entry_elem)).map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(&entry.value)))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::End(BytesEnd::new(
                    entry.xmlns.split(':').next_back().unwrap_or("entry"),
                )))
                .map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("metadata"))).map_err(|e| e.to_string())?;
    }

    {
        let mut mem_elem = BytesStart::new("memory");
        if let Some(ref unit) = config.general.memory.unit {
            mem_elem.push_attribute(("unit", unit.as_str()));
        }
        if let Some(ref dump_core) = config.general.memory.dump_core {
            mem_elem.push_attribute(("dumpCore", dump_core.as_str()));
        }
        writer.write_event(Event::Start(mem_elem)).map_err(|e| e.to_string())?;
        writer
            .write_event(Event::Text(BytesText::new(&config.general.memory.value.to_string())))
            .map_err(|e| e.to_string())?;
        writer.write_event(Event::End(BytesEnd::new("memory"))).map_err(|e| e.to_string())?;
    }

    if let Some(ref max_memory) = config.general.max_memory {
        let mut max_mem_elem = BytesStart::new("maxMemory");
        if let Some(ref unit) = max_memory.unit {
            max_mem_elem.push_attribute(("unit", unit.as_str()));
        }
        if let Some(ref slots) = max_memory.slots {
            max_mem_elem.push_attribute(("slots", slots.to_string().as_str()));
        }
        writer.write_event(Event::Start(max_mem_elem)).map_err(|e| e.to_string())?;
        writer
            .write_event(Event::Text(BytesText::new(&max_memory.value.to_string())))
            .map_err(|e| e.to_string())?;
        writer.write_event(Event::End(BytesEnd::new("maxMemory"))).map_err(|e| e.to_string())?;
    }

    if let Some(ref current_memory) = config.general.current_memory {
        let mut current_mem_elem = BytesStart::new("currentMemory");
        if let Some(ref unit) = current_memory.unit {
            current_mem_elem.push_attribute(("unit", unit.as_str()));
        }
        writer.write_event(Event::Start(current_mem_elem)).map_err(|e| e.to_string())?;
        writer
            .write_event(Event::Text(BytesText::new(&current_memory.value.to_string())))
            .map_err(|e| e.to_string())?;
        writer
            .write_event(Event::End(BytesEnd::new("currentMemory")))
            .map_err(|e| e.to_string())?;
    }

    {
        let mut vcpu_elem = BytesStart::new("vcpu");
        if let Some(ref placement) = config.general.vcpu.placement {
            vcpu_elem.push_attribute(("placement", placement.as_str()));
        }
        if let Some(ref cpuset) = config.general.vcpu.cpuset {
            vcpu_elem.push_attribute(("cpuset", cpuset.as_str()));
        }
        if let Some(ref current) = config.general.vcpu.current {
            vcpu_elem.push_attribute(("current", current.to_string().as_str()));
        }
        writer.write_event(Event::Start(vcpu_elem)).map_err(|e| e.to_string())?;
        writer
            .write_event(Event::Text(BytesText::new(&config.general.vcpu.count.to_string())))
            .map_err(|e| e.to_string())?;
        writer.write_event(Event::End(BytesEnd::new("vcpu"))).map_err(|e| e.to_string())?;
    }

    if let Some(ref vcpus) = config.general.vcpus {
        let vcpus_elem = BytesStart::new("vcpus");
        writer.write_event(Event::Start(vcpus_elem)).map_err(|e| e.to_string())?;

        for vcpu in vcpus {
            let mut vcpu_elem = BytesStart::new("vcpu");
            vcpu_elem.push_attribute(("id", vcpu.id.to_string().as_str()));
            vcpu_elem.push_attribute(("enabled", vcpu.enabled.as_str()));
            vcpu_elem.push_attribute(("hotpluggable", vcpu.hotpluggable.as_str()));
            if let Some(ref order) = vcpu.order {
                vcpu_elem.push_attribute(("order", order.to_string().as_str()));
            }
            writer.write_event(Event::Empty(vcpu_elem)).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("vcpus"))).map_err(|e| e.to_string())?;
    }

    if let Some(ref bootloader) = config.general.bootloader {
        write_element(writer, "bootloader", bootloader)?;
    }

    if let Some(ref bootloader_args) = config.general.bootloader_args {
        write_element(writer, "bootloader_args", bootloader_args)?;
    }

    Ok(())
}

/// 写入通用 XML 元素
pub fn write_element<W: std::io::Write>(
    writer: &mut Writer<W>,
    name: &str,
    value: &str,
) -> Result<(), String> {
    writer.write_event(Event::Start(BytesStart::new(name))).map_err(|e| e.to_string())?;
    writer.write_event(Event::Text(BytesText::new(value))).map_err(|e| e.to_string())?;
    writer.write_event(Event::End(BytesEnd::new(name))).map_err(|e| e.to_string())?;
    Ok(())
}
