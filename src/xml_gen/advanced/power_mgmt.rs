use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Writer,
};

use crate::{model::VMConfig, xml_gen::general::write_element};

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
