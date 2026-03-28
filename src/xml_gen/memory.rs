use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Writer,
};

use crate::model::VMConfig;

/// 写入内存配置（memoryBacking 部分）
pub fn write_memory<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    if let Some(ref memory_backing) = config.memory_backing {
        let mem_elem = BytesStart::new("memoryBacking");
        writer.write_event(Event::Start(mem_elem)).map_err(|e| e.to_string())?;

        if let Some(ref hugepages) = memory_backing.hugepages {
            let hugepages_elem = BytesStart::new("hugepages");
            writer.write_event(Event::Start(hugepages_elem)).map_err(|e| e.to_string())?;

            if let Some(ref page_list) = hugepages.page {
                for page in page_list {
                    let mut page_elem = BytesStart::new("page");
                    page_elem.push_attribute(("size", page.size.as_str()));
                    if let Some(ref unit) = page.unit {
                        page_elem.push_attribute(("unit", unit.as_str()));
                    }
                    if let Some(ref nodeset) = page.nodeset {
                        page_elem.push_attribute(("nodeset", nodeset.as_str()));
                    }
                    writer.write_event(Event::Empty(page_elem)).map_err(|e| e.to_string())?;
                }
            }

            writer
                .write_event(Event::End(BytesEnd::new("hugepages")))
                .map_err(|e| e.to_string())?;
        }

        if memory_backing.nosharepages.is_some() {
            writer
                .write_event(Event::Empty(BytesStart::new("nosharepages")))
                .map_err(|e| e.to_string())?;
        }

        if memory_backing.locked.is_some() {
            writer
                .write_event(Event::Empty(BytesStart::new("locked")))
                .map_err(|e| e.to_string())?;
        }

        if let Some(ref source) = memory_backing.source {
            let mut source_elem = BytesStart::new("source");
            source_elem.push_attribute(("type", source.source_type.as_str()));
            writer.write_event(Event::Empty(source_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref access) = memory_backing.access {
            let mut access_elem = BytesStart::new("access");
            access_elem.push_attribute(("mode", access.mode.as_str()));
            writer.write_event(Event::Empty(access_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref allocation) = memory_backing.allocation {
            let mut allocation_elem = BytesStart::new("allocation");
            allocation_elem.push_attribute(("mode", allocation.mode.as_str()));
            if let Some(ref threads) = allocation.threads {
                allocation_elem.push_attribute(("threads", threads.to_string().as_str()));
            }
            writer.write_event(Event::Empty(allocation_elem)).map_err(|e| e.to_string())?;
        }

        if memory_backing.discard.is_some() {
            writer
                .write_event(Event::Empty(BytesStart::new("discard")))
                .map_err(|e| e.to_string())?;
        }

        writer
            .write_event(Event::End(BytesEnd::new("memoryBacking")))
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
