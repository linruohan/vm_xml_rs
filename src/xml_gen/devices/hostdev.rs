use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Writer,
};

use crate::model::devices::HostdevConfig;

/// 写入主机设备直通配置
pub fn write_hostdevs<W: std::io::Write>(
    writer: &mut Writer<W>,
    hostdev_list: &[HostdevConfig],
) -> Result<(), String> {
    for hostdev in hostdev_list {
        let mut hostdev_elem = BytesStart::new("hostdev");
        hostdev_elem.push_attribute(("mode", hostdev.mode.as_str()));
        hostdev_elem.push_attribute(("type", hostdev.device_type.as_str()));
        if let Some(ref managed) = hostdev.managed {
            hostdev_elem.push_attribute(("managed", managed.as_str()));
        }
        if let Some(ref model) = hostdev.model {
            hostdev_elem.push_attribute(("model", model.as_str()));
        }
        if let Some(ref rawio) = hostdev.rawio {
            hostdev_elem.push_attribute(("rawio", rawio.as_str()));
        }
        if let Some(ref display) = hostdev.display {
            hostdev_elem.push_attribute(("display", display.as_str()));
        }
        if let Some(ref ramfb) = hostdev.ramfb {
            hostdev_elem.push_attribute(("ramfb", ramfb.as_str()));
        }
        writer.write_event(Event::Start(hostdev_elem)).map_err(|e| e.to_string())?;

        // Source 配置
        if let Some(ref source) = hostdev.source {
            write_source(writer, source, hostdev.device_type.as_str())?;
        }

        // Boot 配置
        if let Some(ref boot) = hostdev.boot {
            let mut boot_elem = BytesStart::new("boot");
            boot_elem.push_attribute(("order", boot.order.to_string().as_str()));
            writer.write_event(Event::Empty(boot_elem)).map_err(|e| e.to_string())?;
        }

        // ROM 配置
        if let Some(ref rom) = hostdev.rom {
            let mut rom_elem = BytesStart::new("rom");
            if let Some(ref bar) = rom.bar {
                rom_elem.push_attribute(("bar", bar.as_str()));
            }
            if let Some(ref file) = rom.file {
                rom_elem.push_attribute(("file", file.as_str()));
            }
            if let Some(ref enabled) = rom.enabled {
                rom_elem.push_attribute(("enabled", enabled.as_str()));
            }
            writer.write_event(Event::Empty(rom_elem)).map_err(|e| e.to_string())?;
        }

        // Address 配置
        if let Some(ref address) = hostdev.address {
            write_address(writer, address)?;
        }

        // Driver 配置
        if let Some(ref driver) = hostdev.driver {
            let mut driver_elem = BytesStart::new("driver");
            if let Some(ref model) = driver.model {
                driver_elem.push_attribute(("model", model.as_str()));
            }
            if let Some(ref iommufd) = driver.iommufd {
                writer.write_event(Event::Start(driver_elem)).map_err(|e| e.to_string())?;
                let mut iommufd_elem = BytesStart::new("iommufd");
                iommufd_elem.push_attribute(("fd", iommufd.fd.to_string().as_str()));
                writer.write_event(Event::Empty(iommufd_elem)).map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::End(BytesEnd::new("driver")))
                    .map_err(|e| e.to_string())?;
            } else {
                writer.write_event(Event::Empty(driver_elem)).map_err(|e| e.to_string())?;
            }
        }

        // Readonly
        if hostdev.readonly.is_some() {
            let readonly_elem = BytesStart::new("readonly");
            writer.write_event(Event::Empty(readonly_elem)).map_err(|e| e.to_string())?;
        }

        // Shareable
        if hostdev.shareable.is_some() {
            let shareable_elem = BytesStart::new("shareable");
            writer.write_event(Event::Empty(shareable_elem)).map_err(|e| e.to_string())?;
        }

        // ACPI
        if let Some(ref acpi) = hostdev.acpi {
            let mut acpi_elem = BytesStart::new("acpi");
            acpi_elem.push_attribute(("nodeset", acpi.nodeset.as_str()));
            writer.write_event(Event::Empty(acpi_elem)).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("hostdev"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn write_source<W: std::io::Write>(
    writer: &mut Writer<W>,
    source: &crate::model::devices::HostdevSource,
    device_type: &str,
) -> Result<(), String> {
    let mut source_elem = BytesStart::new("source");
    if let Some(ref startup_policy) = source.startup_policy {
        source_elem.push_attribute(("startupPolicy", startup_policy.as_str()));
    }
    if let Some(ref guest_reset) = source.guest_reset {
        source_elem.push_attribute(("guestReset", guest_reset.as_str()));
    }
    if let Some(ref write_filtering) = source.write_filtering {
        source_elem.push_attribute(("writeFiltering", write_filtering.as_str()));
    }
    if let Some(ref protocol) = source.protocol {
        source_elem.push_attribute(("protocol", protocol.as_str()));
    }
    if let Some(ref name) = source.name {
        source_elem.push_attribute(("name", name.as_str()));
    }
    if let Some(ref wwpn) = source.wwpn {
        source_elem.push_attribute(("wwpn", wwpn.as_str()));
    }
    writer.write_event(Event::Start(source_elem)).map_err(|e| e.to_string())?;

    // USB 设备
    if device_type == "usb" {
        if let Some(ref vendor) = source.vendor {
            let mut vendor_elem = BytesStart::new("vendor");
            vendor_elem.push_attribute(("id", vendor.id.as_str()));
            writer.write_event(Event::Empty(vendor_elem)).map_err(|e| e.to_string())?;
        }
        if let Some(ref product) = source.product {
            let mut product_elem = BytesStart::new("product");
            product_elem.push_attribute(("id", product.id.as_str()));
            writer.write_event(Event::Empty(product_elem)).map_err(|e| e.to_string())?;
        }
        if let Some(ref address) = source.address {
            write_usb_address(writer, address)?;
        }
    }

    // PCI 设备
    if device_type == "pci" {
        if let Some(ref address) = source.address {
            write_pci_address(writer, address)?;
        }
    }

    // SCSI 设备
    if device_type == "scsi" {
        if let Some(ref adapter) = source.adapter {
            let mut adapter_elem = BytesStart::new("adapter");
            adapter_elem.push_attribute(("name", adapter.name.as_str()));
            writer.write_event(Event::Empty(adapter_elem)).map_err(|e| e.to_string())?;
        }
        if let Some(ref scsi_addr) = source.scsi_address {
            let mut addr_elem = BytesStart::new("address");
            addr_elem.push_attribute(("bus", scsi_addr.bus.as_str()));
            addr_elem.push_attribute(("target", scsi_addr.target.as_str()));
            addr_elem.push_attribute(("unit", scsi_addr.unit.as_str()));
            writer.write_event(Event::Empty(addr_elem)).map_err(|e| e.to_string())?;
        }
        // iSCSI 协议
        if source.protocol.as_deref() == Some("iscsi") {
            if let Some(ref host) = source.host {
                let mut host_elem = BytesStart::new("host");
                host_elem.push_attribute(("name", host.name.as_str()));
                if let Some(ref port) = host.port {
                    host_elem.push_attribute(("port", port.as_str()));
                }
                writer.write_event(Event::Empty(host_elem)).map_err(|e| e.to_string())?;
            }
            if let Some(ref auth) = source.auth {
                let mut auth_elem = BytesStart::new("auth");
                auth_elem.push_attribute(("username", auth.username.as_str()));
                writer.write_event(Event::Start(auth_elem)).map_err(|e| e.to_string())?;
                if let Some(ref secret) = auth.secret {
                    let mut secret_elem = BytesStart::new("secret");
                    secret_elem.push_attribute(("type", secret.secret_type.as_str()));
                    if let Some(ref usage) = secret.usage {
                        secret_elem.push_attribute(("usage", usage.as_str()));
                    }
                    if let Some(ref uuid) = secret.uuid {
                        secret_elem.push_attribute(("uuid", uuid.as_str()));
                    }
                    writer.write_event(Event::Empty(secret_elem)).map_err(|e| e.to_string())?;
                }
                writer.write_event(Event::End(BytesEnd::new("auth"))).map_err(|e| e.to_string())?;
            }
            if let Some(ref initiator) = source.initiator {
                let initiator_elem = BytesStart::new("initiator");
                writer.write_event(Event::Start(initiator_elem)).map_err(|e| e.to_string())?;
                if let Some(ref iqn) = initiator.iqn {
                    let mut iqn_elem = BytesStart::new("iqn");
                    iqn_elem.push_attribute(("name", iqn.name.as_str()));
                    writer.write_event(Event::Empty(iqn_elem)).map_err(|e| e.to_string())?;
                }
                writer
                    .write_event(Event::End(BytesEnd::new("initiator")))
                    .map_err(|e| e.to_string())?;
            }
        }
    }

    // MDEV 设备
    if device_type == "mdev" {
        if let Some(ref address) = source.address {
            write_mdev_address(writer, address)?;
        }
    }

    writer.write_event(Event::End(BytesEnd::new("source"))).map_err(|e| e.to_string())?;
    Ok(())
}

fn write_address<W: std::io::Write>(
    writer: &mut Writer<W>,
    address: &crate::model::devices::AddressConfig,
) -> Result<(), String> {
    let mut addr_elem = BytesStart::new("address");
    addr_elem.push_attribute(("type", address.address_type.as_str()));
    if let Some(ref domain) = address.domain {
        addr_elem.push_attribute(("domain", domain.as_str()));
    }
    if let Some(bus) = address.bus {
        addr_elem.push_attribute(("bus", format!("0x{:02x}", bus).as_str()));
    }
    if let Some(slot) = address.slot {
        addr_elem.push_attribute(("slot", format!("0x{:02x}", slot).as_str()));
    }
    if let Some(function) = address.function {
        addr_elem.push_attribute(("function", format!("0x{}", function).as_str()));
    }
    if let Some(ref cssid) = address.cssid {
        addr_elem.push_attribute(("cssid", cssid.as_str()));
    }
    if let Some(ssid) = address.ssid {
        addr_elem.push_attribute(("ssid", format!("0x{}", ssid).as_str()));
    }
    if let Some(ref devno) = address.devno {
        addr_elem.push_attribute(("devno", devno.as_str()));
    }
    if let Some(ref reg) = address.reg {
        addr_elem.push_attribute(("reg", reg.as_str()));
    }
    writer.write_event(Event::Empty(addr_elem)).map_err(|e| e.to_string())?;
    Ok(())
}

fn write_usb_address<W: std::io::Write>(
    writer: &mut Writer<W>,
    address: &crate::model::devices::HostdevAddress,
) -> Result<(), String> {
    let mut addr_elem = BytesStart::new("address");
    if let Some(ref bus) = address.bus {
        addr_elem.push_attribute(("bus", bus.as_str()));
    }
    if let Some(ref device) = address.device {
        addr_elem.push_attribute(("device", device.as_str()));
    }
    if let Some(ref port) = address.port {
        addr_elem.push_attribute(("port", port.as_str()));
    }
    writer.write_event(Event::Empty(addr_elem)).map_err(|e| e.to_string())?;
    Ok(())
}

fn write_pci_address<W: std::io::Write>(
    writer: &mut Writer<W>,
    address: &crate::model::devices::HostdevAddress,
) -> Result<(), String> {
    let mut addr_elem = BytesStart::new("address");
    if let Some(ref domain) = address.domain {
        addr_elem.push_attribute(("domain", domain.as_str()));
    }
    if let Some(ref bus) = address.bus {
        addr_elem.push_attribute(("bus", bus.as_str()));
    }
    if let Some(ref slot) = address.slot {
        addr_elem.push_attribute(("slot", slot.as_str()));
    }
    if let Some(ref function) = address.function {
        addr_elem.push_attribute(("function", function.as_str()));
    }
    writer.write_event(Event::Empty(addr_elem)).map_err(|e| e.to_string())?;
    Ok(())
}

fn write_mdev_address<W: std::io::Write>(
    writer: &mut Writer<W>,
    address: &crate::model::devices::HostdevAddress,
) -> Result<(), String> {
    let mut addr_elem = BytesStart::new("address");
    if let Some(ref uuid) = address.uuid {
        addr_elem.push_attribute(("uuid", uuid.as_str()));
    }
    writer.write_event(Event::Empty(addr_elem)).map_err(|e| e.to_string())?;
    Ok(())
}
