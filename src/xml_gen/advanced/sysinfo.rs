use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use crate::model::VMConfig;

/// 写入 Sysinfo 配置
pub fn write_sysinfo<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    if let Some(ref sysinfo) = config.devices.sysinfo {
        let mut sysinfo_elem = BytesStart::new("sysinfo");
        sysinfo_elem.push_attribute(("type", sysinfo.sysinfo_type.as_str()));
        writer.write_event(Event::Start(sysinfo_elem)).map_err(|e| e.to_string())?;

        if let Some(ref bios) = sysinfo.bios {
            let bios_elem = BytesStart::new("bios");
            writer.write_event(Event::Start(bios_elem)).map_err(|e| e.to_string())?;

            if let Some(ref entry_list) = bios.entry {
                for entry in entry_list {
                    let mut entry_elem = BytesStart::new("entry");
                    entry_elem.push_attribute(("name", entry.name.as_str()));
                    writer.write_event(Event::Start(entry_elem)).map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::Text(BytesText::new(&entry.value)))
                        .map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::End(BytesEnd::new("entry")))
                        .map_err(|e| e.to_string())?;
                }
            }

            writer.write_event(Event::End(BytesEnd::new("bios"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref system) = sysinfo.system {
            let system_elem = BytesStart::new("system");
            writer.write_event(Event::Start(system_elem)).map_err(|e| e.to_string())?;

            if let Some(ref entry_list) = system.entry {
                for entry in entry_list {
                    let mut entry_elem = BytesStart::new("entry");
                    entry_elem.push_attribute(("name", entry.name.as_str()));
                    writer.write_event(Event::Start(entry_elem)).map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::Text(BytesText::new(&entry.value)))
                        .map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::End(BytesEnd::new("entry")))
                        .map_err(|e| e.to_string())?;
                }
            }

            writer.write_event(Event::End(BytesEnd::new("system"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref base_board_list) = sysinfo.base_board {
            for base_board in base_board_list {
                let base_board_elem = BytesStart::new("baseBoard");
                writer.write_event(Event::Start(base_board_elem)).map_err(|e| e.to_string())?;

                if let Some(ref entry_list) = base_board.entry {
                    for entry in entry_list {
                        let mut entry_elem = BytesStart::new("entry");
                        entry_elem.push_attribute(("name", entry.name.as_str()));
                        writer.write_event(Event::Start(entry_elem)).map_err(|e| e.to_string())?;
                        writer
                            .write_event(Event::Text(BytesText::new(&entry.value)))
                            .map_err(|e| e.to_string())?;
                        writer
                            .write_event(Event::End(BytesEnd::new("entry")))
                            .map_err(|e| e.to_string())?;
                    }
                }

                writer
                    .write_event(Event::End(BytesEnd::new("baseBoard")))
                    .map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref chassis) = sysinfo.chassis {
            let chassis_elem = BytesStart::new("chassis");
            writer.write_event(Event::Start(chassis_elem)).map_err(|e| e.to_string())?;

            if let Some(ref entry_list) = chassis.entry {
                for entry in entry_list {
                    let mut entry_elem = BytesStart::new("entry");
                    entry_elem.push_attribute(("name", entry.name.as_str()));
                    writer.write_event(Event::Start(entry_elem)).map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::Text(BytesText::new(&entry.value)))
                        .map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::End(BytesEnd::new("entry")))
                        .map_err(|e| e.to_string())?;
                }
            }

            writer.write_event(Event::End(BytesEnd::new("chassis"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref oem_strings) = sysinfo.oem_strings {
            let oem_elem = BytesStart::new("oemStrings");
            writer.write_event(Event::Start(oem_elem)).map_err(|e| e.to_string())?;

            for oem_string in oem_strings {
                let entry_elem = BytesStart::new("entry");
                writer.write_event(Event::Start(entry_elem)).map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::Text(BytesText::new(&oem_string.value)))
                    .map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::End(BytesEnd::new("entry")))
                    .map_err(|e| e.to_string())?;
            }

            writer
                .write_event(Event::End(BytesEnd::new("oemStrings")))
                .map_err(|e| e.to_string())?;
        }

        if let Some(ref entry_list) = sysinfo.entry {
            for entry in entry_list {
                let mut entry_elem = BytesStart::new("entry");
                entry_elem.push_attribute(("name", entry.name.as_str()));
                if let Some(ref file) = entry.file {
                    entry_elem.push_attribute(("file", file.as_str()));
                }
                writer.write_event(Event::Start(entry_elem)).map_err(|e| e.to_string())?;
                if let Some(ref value) = entry.value {
                    writer
                        .write_event(Event::Text(BytesText::new(value)))
                        .map_err(|e| e.to_string())?;
                }
                writer
                    .write_event(Event::End(BytesEnd::new("entry")))
                    .map_err(|e| e.to_string())?;
            }
        }

        writer.write_event(Event::End(BytesEnd::new("sysinfo"))).map_err(|e| e.to_string())?;
    }

    Ok(())
}
