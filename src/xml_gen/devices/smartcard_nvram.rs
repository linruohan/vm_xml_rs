use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use crate::model::devices::{DeviceNVRAMConfig, SmartcardConfig};

/// 写入智能卡设备配置
pub fn write_smartcards<W: std::io::Write>(
    writer: &mut Writer<W>,
    smartcard_list: &[SmartcardConfig],
) -> Result<(), String> {
    for smartcard in smartcard_list {
        let mut smartcard_elem = BytesStart::new("smartcard");
        smartcard_elem.push_attribute(("mode", smartcard.mode.as_str()));
        if let Some(ref smartcard_type) = smartcard.smartcard_type {
            smartcard_elem.push_attribute(("type", smartcard_type.as_str()));
        }
        writer.write_event(Event::Start(smartcard_elem)).map_err(|e| e.to_string())?;

        // host-certificates 模式
        if smartcard.mode == "host-certificates" {
            if let Some(ref certs) = smartcard.certificate {
                for cert in certs {
                    let cert_elem = BytesStart::new("certificate");
                    writer.write_event(Event::Start(cert_elem)).map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::Text(BytesText::new(cert)))
                        .map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::End(BytesEnd::new("certificate")))
                        .map_err(|e| e.to_string())?;
                }
            }
            if let Some(ref database) = smartcard.database {
                let db_elem = BytesStart::new("database");
                writer.write_event(Event::Start(db_elem)).map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::Text(BytesText::new(database)))
                    .map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::End(BytesEnd::new("database")))
                    .map_err(|e| e.to_string())?;
            }
        }

        // passthrough 模式
        if smartcard.mode == "passthrough" {
            if let Some(ref source) = smartcard.source {
                let mut source_elem = BytesStart::new("source");
                if let Some(ref mode) = source.mode {
                    source_elem.push_attribute(("mode", mode.as_str()));
                }
                if let Some(ref host) = source.host {
                    source_elem.push_attribute(("host", host.as_str()));
                }
                if let Some(ref service) = source.service {
                    source_elem.push_attribute(("service", service.as_str()));
                }
                writer.write_event(Event::Empty(source_elem)).map_err(|e| e.to_string())?;
            }
            if let Some(ref protocol) = smartcard.protocol {
                let mut protocol_elem = BytesStart::new("protocol");
                protocol_elem.push_attribute(("type", protocol.protocol_type.as_str()));
                writer.write_event(Event::Empty(protocol_elem)).map_err(|e| e.to_string())?;
            }
        }

        // Address 配置
        if let Some(ref address) = smartcard.address {
            let mut addr_elem = BytesStart::new("address");
            addr_elem.push_attribute(("type", address.address_type.as_str()));
            if let Some(controller) = address.controller {
                addr_elem.push_attribute(("controller", controller.to_string().as_str()));
            }
            if let Some(slot) = address.slot {
                addr_elem.push_attribute(("slot", slot.to_string().as_str()));
            }
            writer.write_event(Event::Empty(addr_elem)).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("smartcard"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入 NVRAM 设备配置
pub fn write_nvram<W: std::io::Write>(
    writer: &mut Writer<W>,
    nvram: &DeviceNVRAMConfig,
) -> Result<(), String> {
    let mut nvram_elem = BytesStart::new("nvram");
    if let Some(ref template) = nvram.template {
        nvram_elem.push_attribute(("template", template.as_str()));
    }
    if let Some(ref source) = nvram.nvram_source {
        writer.write_event(Event::Start(nvram_elem)).map_err(|e| e.to_string())?;
        writer.write_event(Event::Text(BytesText::new(source))).map_err(|e| e.to_string())?;
        writer.write_event(Event::End(BytesEnd::new("nvram"))).map_err(|e| e.to_string())?;
    } else {
        writer.write_event(Event::Empty(nvram_elem)).map_err(|e| e.to_string())?;
    }
    Ok(())
}
