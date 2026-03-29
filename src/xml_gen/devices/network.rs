use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Writer,
};

use crate::{error::AppError, model::network::InterfaceConfig};

/// 写入 Network 接口设备
pub fn write_interfaces<W: std::io::Write>(
    writer: &mut Writer<W>,
    iface_list: &[InterfaceConfig],
) -> Result<(), AppError> {
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
            if let Some(ref dev) = source.dev {
                source_elem.push_attribute(("dev", dev.as_str()));
            }
            if let Some(ref mode) = source.mode {
                source_elem.push_attribute(("mode", mode.as_str()));
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

        if let Some(ref address) = iface.address {
            let mut address_elem = BytesStart::new("address");
            address_elem.push_attribute(("type", address.address_type.as_str()));
            if let Some(ref domain) = address.domain {
                address_elem.push_attribute(("domain", domain.as_str()));
            }
            if let Some(bus) = address.bus {
                address_elem.push_attribute(("bus", bus.to_string().as_str()));
            }
            if let Some(slot) = address.slot {
                address_elem.push_attribute(("slot", slot.to_string().as_str()));
            }
            if let Some(function) = address.function {
                address_elem.push_attribute(("function", function.to_string().as_str()));
            }
            writer.write_event(Event::Empty(address_elem)).map_err(|e| e.to_string())?;
        }

        // 带宽配置
        if let Some(ref bandwidth) = iface.bandwidth {
            let bandwidth_elem = BytesStart::new("bandwidth");
            writer.write_event(Event::Start(bandwidth_elem)).map_err(|e| e.to_string())?;

            if let Some(ref inbound) = bandwidth.inbound {
                let mut inbound_elem = BytesStart::new("inbound");
                if let Some(average) = inbound.average {
                    inbound_elem.push_attribute(("average", average.to_string().as_str()));
                }
                if let Some(peak) = inbound.peak {
                    inbound_elem.push_attribute(("peak", peak.to_string().as_str()));
                }
                if let Some(burst) = inbound.burst {
                    inbound_elem.push_attribute(("burst", burst.to_string().as_str()));
                }
                writer.write_event(Event::Empty(inbound_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref outbound) = bandwidth.outbound {
                let mut outbound_elem = BytesStart::new("outbound");
                if let Some(average) = outbound.average {
                    outbound_elem.push_attribute(("average", average.to_string().as_str()));
                }
                if let Some(peak) = outbound.peak {
                    outbound_elem.push_attribute(("peak", peak.to_string().as_str()));
                }
                if let Some(burst) = outbound.burst {
                    outbound_elem.push_attribute(("burst", burst.to_string().as_str()));
                }
                writer.write_event(Event::Empty(outbound_elem)).map_err(|e| e.to_string())?;
            }

            writer
                .write_event(Event::End(BytesEnd::new("bandwidth")))
                .map_err(|e| e.to_string())?;
        }

        // 虚拟端口配置
        if let Some(ref virtualport) = iface.virtualport {
            let mut virtualport_elem = BytesStart::new("virtualport");
            virtualport_elem.push_attribute(("type", virtualport.port_type.as_str()));
            writer.write_event(Event::Start(virtualport_elem)).map_err(|e| e.to_string())?;

            if let Some(ref params) = virtualport.parameters {
                let mut params_elem = BytesStart::new("parameters");
                if let Some(ref interfaceid) = params.interfaceid {
                    params_elem.push_attribute(("interfaceid", interfaceid.as_str()));
                }
                if let Some(ref profileid) = params.profileid {
                    params_elem.push_attribute(("profileid", profileid.as_str()));
                }
                if let Some(ref instanceid) = params.instanceid {
                    params_elem.push_attribute(("instanceid", instanceid.as_str()));
                }
                writer.write_event(Event::Empty(params_elem)).map_err(|e| e.to_string())?;
            }

            writer
                .write_event(Event::End(BytesEnd::new("virtualport")))
                .map_err(|e| e.to_string())?;
        }

        // 链接状态配置
        if let Some(ref link) = iface.link {
            let mut link_elem = BytesStart::new("link");
            link_elem.push_attribute(("state", link.state.as_str()));
            writer.write_event(Event::Empty(link_elem)).map_err(|e| e.to_string())?;
        }

        // 目标设备配置
        if let Some(ref target) = iface.target {
            let mut target_elem = BytesStart::new("target");
            target_elem.push_attribute(("dev", target.as_str()));
            writer.write_event(Event::Empty(target_elem)).map_err(|e| e.to_string())?;
        }

        // ROM 配置
        if let Some(ref rom) = iface.rom {
            let mut rom_elem = BytesStart::new("rom");
            rom_elem.push_attribute(("bar", rom.bar.as_str()));
            if let Some(ref file) = rom.file {
                rom_elem.push_attribute(("file", file.as_str()));
            }
            writer.write_event(Event::Empty(rom_elem)).map_err(|e| e.to_string())?;
        }

        // ACPI 索引配置
        if let Some(ref acpi) = iface.acpi {
            let mut acpi_elem = BytesStart::new("acpi");
            acpi_elem.push_attribute(("index", acpi.index.to_string().as_str()));
            writer.write_event(Event::Empty(acpi_elem)).map_err(|e| e.to_string())?;
        }

        // 后端配置
        if let Some(ref backend) = iface.backend {
            let mut backend_elem = BytesStart::new("backend");
            if let Some(ref tap) = backend.tap {
                backend_elem.push_attribute(("tap", tap.as_str()));
            }
            if let Some(ref vhost) = backend.vhost {
                backend_elem.push_attribute(("vhost", vhost.as_str()));
            }
            if let Some(ref backend_type) = backend.backend_type {
                backend_elem.push_attribute(("type", backend_type.as_str()));
            }
            writer.write_event(Event::Empty(backend_elem)).map_err(|e| e.to_string())?;
        }

        // 驱动配置
        if let Some(ref driver) = iface.driver {
            let mut driver_elem = BytesStart::new("driver");
            if let Some(ref name) = driver.name {
                driver_elem.push_attribute(("name", name.as_str()));
            }
            if let Some(ref txmode) = driver.txmode {
                driver_elem.push_attribute(("txmode", txmode.as_str()));
            }
            if let Some(ref ioeventfd) = driver.ioeventfd {
                driver_elem.push_attribute(("ioeventfd", ioeventfd.as_str()));
            }
            if let Some(ref event_idx) = driver.event_idx {
                driver_elem.push_attribute(("event_idx", event_idx.as_str()));
            }
            if let Some(queues) = driver.queues {
                driver_elem.push_attribute(("queues", queues.to_string().as_str()));
            }
            if let Some(ref rx_queue_size) = driver.rx_queue_size {
                driver_elem.push_attribute(("rx_queue_size", rx_queue_size.to_string().as_str()));
            }
            if let Some(ref tx_queue_size) = driver.tx_queue_size {
                driver_elem.push_attribute(("tx_queue_size", tx_queue_size.to_string().as_str()));
            }
            writer.write_event(Event::Start(driver_elem)).map_err(|e| e.to_string())?;

            if let Some(ref host) = driver.host {
                let mut host_elem = BytesStart::new("host");
                if let Some(ref csum) = host.csum {
                    host_elem.push_attribute(("csum", csum.as_str()));
                }
                if let Some(ref gso) = host.gso {
                    host_elem.push_attribute(("gso", gso.as_str()));
                }
                if let Some(ref tso4) = host.tso4 {
                    host_elem.push_attribute(("tso4", tso4.as_str()));
                }
                if let Some(ref tso6) = host.tso6 {
                    host_elem.push_attribute(("tso6", tso6.as_str()));
                }
                if let Some(ref ecn) = host.ecn {
                    host_elem.push_attribute(("ecn", ecn.as_str()));
                }
                if let Some(ref ufo) = host.ufo {
                    host_elem.push_attribute(("ufo", ufo.as_str()));
                }
                if let Some(ref mrg_rxbuf) = host.mrg_rxbuf {
                    host_elem.push_attribute(("mrg_rxbuf", mrg_rxbuf.as_str()));
                }
                writer.write_event(Event::Empty(host_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref guest) = driver.guest {
                let mut guest_elem = BytesStart::new("guest");
                if let Some(ref csum) = guest.csum {
                    guest_elem.push_attribute(("csum", csum.as_str()));
                }
                if let Some(ref tso4) = guest.tso4 {
                    guest_elem.push_attribute(("tso4", tso4.as_str()));
                }
                if let Some(ref tso6) = guest.tso6 {
                    guest_elem.push_attribute(("tso6", tso6.as_str()));
                }
                if let Some(ref ecn) = guest.ecn {
                    guest_elem.push_attribute(("ecn", ecn.as_str()));
                }
                if let Some(ref ufo) = guest.ufo {
                    guest_elem.push_attribute(("ufo", ufo.as_str()));
                }
                writer.write_event(Event::Empty(guest_elem)).map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("driver"))).map_err(|e| e.to_string())?;
        }

        // 调优配置
        if let Some(ref tune) = iface.tune {
            let tune_elem = BytesStart::new("tune");
            writer.write_event(Event::Start(tune_elem)).map_err(|e| e.to_string())?;

            if let Some(sndbuf) = tune.sndbuf {
                let sndbuf_elem = BytesStart::new("sndbuf");
                writer.write_event(Event::Start(sndbuf_elem)).map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::Text(quick_xml::events::BytesText::new(
                        sndbuf.to_string().as_str(),
                    )))
                    .map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::End(BytesEnd::new("sndbuf")))
                    .map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("tune"))).map_err(|e| e.to_string())?;
        }

        // 访客设备配置
        if let Some(ref guest) = iface.guest {
            let mut guest_elem = BytesStart::new("guest");
            guest_elem.push_attribute(("dev", guest.dev.as_str()));
            writer.write_event(Event::Empty(guest_elem)).map_err(|e| e.to_string())?;
        }

        // VLAN 配置
        if let Some(ref vlan) = iface.vlan {
            let mut vlan_elem = BytesStart::new("vlan");
            if let Some(ref trunk) = vlan.trunk {
                vlan_elem.push_attribute(("trunk", trunk.as_str()));
            }
            writer.write_event(Event::Start(vlan_elem)).map_err(|e| e.to_string())?;

            for tag in &vlan.tags {
                let mut tag_elem = BytesStart::new("tag");
                tag_elem.push_attribute(("id", tag.id.to_string().as_str()));
                if let Some(ref native_mode) = tag.native_mode {
                    tag_elem.push_attribute(("nativeMode", native_mode.as_str()));
                }
                writer.write_event(Event::Empty(tag_elem)).map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("vlan"))).map_err(|e| e.to_string())?;
        }

        // 端口隔离配置
        if let Some(ref port) = iface.port {
            let mut port_elem = BytesStart::new("port");
            port_elem.push_attribute(("isolated", port.isolated.as_str()));
            writer.write_event(Event::Empty(port_elem)).map_err(|e| e.to_string())?;
        }

        // IP 配置
        if let Some(ref ip) = iface.ip {
            let mut ip_elem = BytesStart::new("ip");
            ip_elem.push_attribute(("family", ip.family.as_str()));
            ip_elem.push_attribute(("address", ip.address.as_str()));
            if let Some(prefix) = ip.prefix {
                ip_elem.push_attribute(("prefix", prefix.to_string().as_str()));
            }
            writer.write_event(Event::Empty(ip_elem)).map_err(|e| e.to_string())?;
        }

        // 端口转发配置
        if let Some(ref port_forwards) = iface.port_forward {
            for pf in port_forwards {
                let mut pf_elem = BytesStart::new("portForward");
                pf_elem.push_attribute(("proto", pf.proto.as_str()));
                if let Some(ref address) = pf.address {
                    pf_elem.push_attribute(("address", address.as_str()));
                }
                if let Some(ref dev) = pf.dev {
                    pf_elem.push_attribute(("dev", dev.as_str()));
                }
                writer.write_event(Event::Start(pf_elem)).map_err(|e| e.to_string())?;

                for range in &pf.ranges {
                    let mut range_elem = BytesStart::new("range");
                    range_elem.push_attribute(("start", range.start.to_string().as_str()));
                    if let Some(end) = range.end {
                        range_elem.push_attribute(("end", end.to_string().as_str()));
                    }
                    if let Some(to) = range.to {
                        range_elem.push_attribute(("to", to.to_string().as_str()));
                    }
                    if let Some(ref exclude) = range.exclude {
                        range_elem.push_attribute(("exclude", exclude.as_str()));
                    }
                    writer.write_event(Event::Empty(range_elem)).map_err(|e| e.to_string())?;
                }

                writer
                    .write_event(Event::End(BytesEnd::new("portForward")))
                    .map_err(|e| e.to_string())?;
            }
        }

        writer.write_event(Event::End(BytesEnd::new("interface"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}
