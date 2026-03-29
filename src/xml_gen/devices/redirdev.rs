use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Writer,
};

use crate::model::devices::{RedirdevConfig, RedirfilterConfig};

/// 写入 USB 重定向设备配置
pub fn write_redirdevs<W: std::io::Write>(
    writer: &mut Writer<W>,
    redirdev_list: &[RedirdevConfig],
) -> Result<(), String> {
    for redirdev in redirdev_list {
        let mut redirdev_elem = BytesStart::new("redirdev");
        redirdev_elem.push_attribute(("bus", redirdev.bus.as_str()));
        redirdev_elem.push_attribute(("type", redirdev.redir_type.as_str()));
        writer.write_event(Event::Start(redirdev_elem)).map_err(|e| e.to_string())?;

        // Source 配置
        if let Some(ref source) = redirdev.source {
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

        // Protocol 配置
        if let Some(ref protocol) = redirdev.protocol {
            let mut protocol_elem = BytesStart::new("protocol");
            protocol_elem.push_attribute(("type", protocol.protocol_type.as_str()));
            writer.write_event(Event::Empty(protocol_elem)).map_err(|e| e.to_string())?;
        }

        // Address 配置
        if let Some(ref address) = redirdev.address {
            let mut addr_elem = BytesStart::new("address");
            addr_elem.push_attribute(("type", address.address_type.as_str()));
            if let Some(controller) = address.controller {
                addr_elem.push_attribute(("controller", controller.to_string().as_str()));
            }
            if let Some(slot) = address.slot {
                addr_elem.push_attribute(("slot", slot.to_string().as_str()));
            }
            if let Some(bus) = address.bus {
                addr_elem.push_attribute(("bus", bus.to_string().as_str()));
            }
            if let Some(function) = address.function {
                addr_elem.push_attribute(("function", function.to_string().as_str()));
            }
            writer.write_event(Event::Empty(addr_elem)).map_err(|e| e.to_string())?;
        }

        // Boot 配置
        if let Some(ref boot) = redirdev.boot {
            let mut boot_elem = BytesStart::new("boot");
            boot_elem.push_attribute(("order", boot.order.to_string().as_str()));
            writer.write_event(Event::Empty(boot_elem)).map_err(|e| e.to_string())?;
        }

        // Alias 配置
        if let Some(ref alias) = redirdev.alias {
            let mut alias_elem = BytesStart::new("alias");
            alias_elem.push_attribute(("name", alias.name.as_str()));
            writer.write_event(Event::Empty(alias_elem)).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("redirdev"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入 USB 重定向过滤器配置
pub fn write_redirfilter<W: std::io::Write>(
    writer: &mut Writer<W>,
    redirfilter: &RedirfilterConfig,
) -> Result<(), String> {
    let redirfilter_elem = BytesStart::new("redirfilter");
    writer.write_event(Event::Start(redirfilter_elem)).map_err(|e| e.to_string())?;

    for usb_dev in &redirfilter.usb_devices {
        let mut usbdev_elem = BytesStart::new("usbdev");
        usbdev_elem.push_attribute(("allow", usb_dev.allow.as_str()));

        if let Some(ref class) = usb_dev.class {
            usbdev_elem.push_attribute(("class", class.as_str()));
        }
        if let Some(ref vendor) = usb_dev.vendor {
            usbdev_elem.push_attribute(("vendor", vendor.as_str()));
        }
        if let Some(ref product) = usb_dev.product {
            usbdev_elem.push_attribute(("product", product.as_str()));
        }
        if let Some(ref version) = usb_dev.version {
            usbdev_elem.push_attribute(("version", version.as_str()));
        }

        writer.write_event(Event::Empty(usbdev_elem)).map_err(|e| e.to_string())?;
    }

    writer.write_event(Event::End(BytesEnd::new("redirfilter"))).map_err(|e| e.to_string())?;
    Ok(())
}
