use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Writer,
};

use crate::model::devices::{ConsoleConfig, ParallelConfig, SerialConfig};

/// 写入 Serial 串口设备
pub fn write_serials<W: std::io::Write>(
    writer: &mut Writer<W>,
    serial_list: &[SerialConfig],
) -> Result<(), String> {
    for serial in serial_list {
        let mut serial_elem = BytesStart::new("serial");
        serial_elem.push_attribute(("type", serial.serial_type.as_str()));
        if let Some(ref port) = serial.port {
            serial_elem.push_attribute(("port", port.to_string().as_str()));
        }
        writer.write_event(Event::Start(serial_elem)).map_err(|e| e.to_string())?;

        if let Some(ref target) = serial.target {
            if let Some(port) = target.port {
                let mut target_elem = BytesStart::new("target");
                target_elem.push_attribute(("port", port.to_string().as_str()));
                writer.write_event(Event::Empty(target_elem)).map_err(|e| e.to_string())?;
            }
        }

        writer.write_event(Event::End(BytesEnd::new("serial"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入 Parallel 并口设备
pub fn write_parallels<W: std::io::Write>(
    writer: &mut Writer<W>,
    parallel_list: &[ParallelConfig],
) -> Result<(), String> {
    for parallel in parallel_list {
        let mut parallel_elem = BytesStart::new("parallel");
        parallel_elem.push_attribute(("type", parallel.parallel_type.as_str()));
        writer.write_event(Event::Start(parallel_elem)).map_err(|e| e.to_string())?;

        if let Some(ref target) = parallel.target {
            let mut target_elem = BytesStart::new("target");
            target_elem.push_attribute(("port", target.port.to_string().as_str()));
            writer.write_event(Event::Empty(target_elem)).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("parallel"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入 Console 控制台设备
pub fn write_console<W: std::io::Write>(
    writer: &mut Writer<W>,
    console: Option<&ConsoleConfig>,
) -> Result<(), String> {
    if let Some(console) = console {
        let mut console_elem = BytesStart::new("console");
        console_elem.push_attribute(("type", console.console_type.as_str()));
        writer.write_event(Event::Start(console_elem)).map_err(|e| e.to_string())?;

        if let Some(ref target) = console.target {
            let mut target_elem = BytesStart::new("target");
            target_elem.push_attribute(("type", target.target_type.as_str()));
            if let Some(ref port) = target.port {
                target_elem.push_attribute(("port", port.to_string().as_str()));
            }
            writer.write_event(Event::Empty(target_elem)).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("console"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}
