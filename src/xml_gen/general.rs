use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use crate::{error::AppError, model::VMConfig};

/// 写入基础配置（general 部分）
pub fn write_general<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), AppError> {
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
        // 使用 write_element_with_attrs 简化代码
        let mut attrs = Vec::new();
        if let Some(ref unit) = config.general.memory.unit {
            attrs.push(("unit", unit.as_str()));
        }
        if let Some(ref dump_core) = config.general.memory.dump_core {
            attrs.push(("dumpCore", dump_core.as_str()));
        }
        write_element_with_attrs(
            writer,
            "memory",
            &config.general.memory.value.to_string(),
            &attrs,
        )?;
    }

    if let Some(ref max_memory) = config.general.max_memory {
        let mut attrs = Vec::new();
        if let Some(ref unit) = max_memory.unit {
            attrs.push(("unit", unit.as_str()));
        }
        if let Some(ref slots) = max_memory.slots {
            let slots_str = slots.to_string();
            attrs.push(("slots", slots_str.as_str()));
            write_element_with_attrs(writer, "maxMemory", &max_memory.value.to_string(), &attrs)?;
        } else {
            write_element_with_attrs(writer, "maxMemory", &max_memory.value.to_string(), &attrs)?;
        }
    }

    if let Some(ref current_memory) = config.general.current_memory {
        let mut attrs = Vec::new();
        if let Some(ref unit) = current_memory.unit {
            attrs.push(("unit", unit.as_str()));
        }
        write_element_with_attrs(
            writer,
            "currentMemory",
            &current_memory.value.to_string(),
            &attrs,
        )?;
    }

    {
        // 使用 write_element_with_attrs 简化代码
        let mut attrs = Vec::new();
        if let Some(ref placement) = config.general.vcpu.placement {
            attrs.push(("placement", placement.as_str()));
        }
        if let Some(ref cpuset) = config.general.vcpu.cpuset {
            attrs.push(("cpuset", cpuset.as_str()));
        }
        if let Some(ref current) = config.general.vcpu.current {
            let current_str = current.to_string();
            attrs.push(("current", current_str.as_str()));
            write_element_with_attrs(
                writer,
                "vcpu",
                &config.general.vcpu.count.to_string(),
                &attrs,
            )?;
        } else {
            write_element_with_attrs(
                writer,
                "vcpu",
                &config.general.vcpu.count.to_string(),
                &attrs,
            )?;
        }
    }

    if let Some(ref vcpus) = config.general.vcpus {
        let vcpus_elem = BytesStart::new("vcpus");
        writer.write_event(Event::Start(vcpus_elem)).map_err(|e| e.to_string())?;

        for vcpu in vcpus {
            // 使用 write_empty_element 简化代码
            let id_str = vcpu.id.to_string();
            let order_str_opt = vcpu.order.map(|o| o.to_string());
            let mut attrs = vec![
                ("id", id_str.as_str()),
                ("enabled", vcpu.enabled.as_str()),
                ("hotpluggable", vcpu.hotpluggable.as_str()),
            ];
            if let Some(ref order_str) = order_str_opt {
                attrs.push(("order", order_str.as_str()));
            }
            write_empty_element(writer, "vcpu", &attrs)?;
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
) -> Result<(), AppError> {
    writer
        .write_event(Event::Start(BytesStart::new(name)))
        .map_err(|e| AppError::XmlGeneration(format!("写入元素 {} 失败：{}", name, e)))?;
    writer
        .write_event(Event::Text(BytesText::new(value)))
        .map_err(|e| AppError::XmlGeneration(format!("写入文本内容失败：{}", e)))?;
    writer
        .write_event(Event::End(BytesEnd::new(name)))
        .map_err(|e| AppError::XmlGeneration(format!("关闭元素 {} 失败：{}", name, e)))?;
    Ok(())
}

/// 写入带属性的 XML 元素
///
/// # 参数
/// * `writer` - XML 写入器
/// * `name` - 元素名称
/// * `value` - 元素文本内容
/// * `attributes` - 属性键值对列表
///
/// # 示例
/// ```ignore
/// write_element_with_attrs(
///     writer,
///     "memory",
///     "1024",
///     &[("unit", "MiB"), ("dumpCore", "on")]
/// )?;
/// // 输出：<memory unit="MiB" dumpCore="on">1024</memory>
/// ```
pub fn write_element_with_attrs<W: std::io::Write>(
    writer: &mut Writer<W>,
    name: &str,
    value: &str,
    attributes: &[(&str, &str)],
) -> Result<(), AppError> {
    let mut elem = BytesStart::new(name);
    for (key, val) in attributes {
        elem.push_attribute((*key, *val));
    }
    writer
        .write_event(Event::Start(elem))
        .map_err(|e| AppError::XmlGeneration(format!("写入带属性元素 {} 失败：{}", name, e)))?;
    writer
        .write_event(Event::Text(BytesText::new(value)))
        .map_err(|e| AppError::XmlGeneration(format!("写入文本内容失败：{}", e)))?;
    writer
        .write_event(Event::End(BytesEnd::new(name)))
        .map_err(|e| AppError::XmlGeneration(format!("关闭元素 {} 失败：{}", name, e)))?;
    Ok(())
}

/// 写入空元素（自闭合标签），可带属性
///
/// # 参数
/// * `writer` - XML 写入器
/// * `name` - 元素名称
/// * `attributes` - 属性键值对列表
///
/// # 示例
/// ```ignore
/// write_empty_element(
///     writer,
///     "vcpu",
///     &[("id", "0"), ("enabled", "yes")]
/// )?;
/// // 输出：<vcpu id="0" enabled="yes"/>
/// ```
pub fn write_empty_element<W: std::io::Write>(
    writer: &mut Writer<W>,
    name: &str,
    attributes: &[(&str, &str)],
) -> Result<(), AppError> {
    let mut elem = BytesStart::new(name);
    for (key, val) in attributes {
        elem.push_attribute((*key, *val));
    }
    writer
        .write_event(Event::Empty(elem))
        .map_err(|e| AppError::XmlGeneration(format!("写入空元素 {} 失败：{}", name, e)))?;
    Ok(())
}
