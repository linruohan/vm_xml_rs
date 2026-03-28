use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Writer,
};

use crate::model::devices::InterfaceConfig;

/// 写入 Network 接口设备
pub fn write_interfaces<W: std::io::Write>(
    writer: &mut Writer<W>,
    iface_list: &[InterfaceConfig],
) -> Result<(), String> {
    for iface in iface_list {
        let mut iface_elem = BytesStart::new("interface");
        iface_elem.push_attribute(("type", iface.interface_type.as_str()));
        if let Some(ref trust_guest_rx_filters) = iface.trust_guest_rx_filters {
            iface_elem.push_attribute(("trustGuestRxFilters", trust_guest_rx_filters.as_str()));
        }
        writer.write_event(Event::Start(iface_elem)).map_err(|e| e.to_string())?;

        if let Some(ref alias) = iface.alias {
            let mut alias_elem = BytesStart::new("alias");
            alias_elem.push_attribute(("name", alias.name.as_str()));
            writer.write_event(Event::Empty(alias_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref mac) = iface.mac {
            let mut mac_elem = BytesStart::new("mac");
            mac_elem.push_attribute(("address", mac.address.as_str()));
            writer.write_event(Event::Empty(mac_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref source) = iface.source {
            let mut source_elem = BytesStart::new("source");
            if let Some(ref bridge) = source.bridge {
                source_elem.push_attribute(("bridge", bridge.as_str()));
            }
            if let Some(ref network) = source.network {
                source_elem.push_attribute(("network", network.as_str()));
            }
            writer.write_event(Event::Empty(source_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref model) = iface.model {
            let mut model_elem = BytesStart::new("model");
            model_elem.push_attribute(("type", model.model_type.as_str()));
            writer.write_event(Event::Empty(model_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref boot) = iface.boot {
            let mut boot_elem = BytesStart::new("boot");
            boot_elem.push_attribute(("order", boot.order.to_string().as_str()));
            writer.write_event(Event::Empty(boot_elem)).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("interface"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}
