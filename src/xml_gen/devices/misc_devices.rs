use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use crate::{
    error::AppError,
    model::devices::{
        AudioConfig, CryptoConfig, HubConfig, IommuConfig, MemoryDeviceConfig, PanicConfig,
        PstoreConfig, ShmemConfig, VsockConfig,
    },
};

/// 写入 Hub 设备
pub fn write_hubs<W: std::io::Write>(
    writer: &mut Writer<W>,
    hub_list: &[HubConfig],
) -> Result<(), AppError> {
    for hub in hub_list {
        let mut hub_elem = BytesStart::new("hub");
        hub_elem.push_attribute(("type", hub.hub_type.as_str()));
        writer.write_event(Event::Empty(hub_elem)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入 Panic 设备
pub fn write_panic<W: std::io::Write>(
    writer: &mut Writer<W>,
    panic: &PanicConfig,
) -> Result<(), String> {
    let mut panic_elem = BytesStart::new("panic");
    if let Some(ref model) = panic.model {
        panic_elem.push_attribute(("model", model.as_str()));
    }
    writer.write_event(Event::Start(panic_elem)).map_err(|e| e.to_string())?;

    if let Some(ref address) = panic.address {
        write_address(writer, address)?;
    }

    writer.write_event(Event::End(BytesEnd::new("panic"))).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入 Shmem 共享内存设备
pub fn write_shmems<W: std::io::Write>(
    writer: &mut Writer<W>,
    shmem_list: &[ShmemConfig],
) -> Result<(), String> {
    for shmem in shmem_list {
        let mut shmem_elem = BytesStart::new("shmem");
        shmem_elem.push_attribute(("name", shmem.name.as_str()));
        if let Some(ref role) = shmem.role {
            shmem_elem.push_attribute(("role", role.as_str()));
        }
        writer.write_event(Event::Start(shmem_elem)).map_err(|e| e.to_string())?;

        if let Some(ref model) = shmem.model {
            let mut model_elem = BytesStart::new("model");
            model_elem.push_attribute(("type", model.model_type.as_str()));
            writer.write_event(Event::Empty(model_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref size) = shmem.size {
            write_size_element(writer, "size", size)?;
        }

        if let Some(ref server) = shmem.server {
            let mut server_elem = BytesStart::new("server");
            if let Some(ref path) = server.path {
                server_elem.push_attribute(("path", path.as_str()));
            }
            writer.write_event(Event::Empty(server_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref msi) = shmem.msi {
            let mut msi_elem = BytesStart::new("msi");
            if let Some(vectors) = msi.vectors {
                msi_elem.push_attribute(("vectors", vectors.to_string().as_str()));
            }
            if let Some(ref ioeventfd) = msi.ioeventfd {
                msi_elem.push_attribute(("ioeventfd", ioeventfd.as_str()));
            }
            writer.write_event(Event::Empty(msi_elem)).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("shmem"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入 Memory Device 设备
pub fn write_memory_devices<W: std::io::Write>(
    writer: &mut Writer<W>,
    memory_device_list: &[MemoryDeviceConfig],
) -> Result<(), String> {
    for mem_dev in memory_device_list {
        let mut mem_elem = BytesStart::new("memory");
        if let Some(ref model) = mem_dev.model {
            mem_elem.push_attribute(("model", model.as_str()));
        }
        if let Some(ref access) = mem_dev.access {
            mem_elem.push_attribute(("access", access.as_str()));
        }
        if let Some(ref discard) = mem_dev.discard {
            mem_elem.push_attribute(("discard", discard.as_str()));
        }
        writer.write_event(Event::Start(mem_elem)).map_err(|e| e.to_string())?;

        if let Some(ref uuid) = mem_dev.uuid {
            write_element(writer, "uuid", uuid)?;
        }

        if let Some(ref source) = mem_dev.source {
            let source_elem = BytesStart::new("source");
            writer.write_event(Event::Start(source_elem)).map_err(|e| e.to_string())?;

            if let Some(ref path) = source.path {
                write_element(writer, "path", path)?;
            }
            if let Some(ref pagesize) = source.pagesize {
                write_size_element(writer, "pagesize", pagesize)?;
            }
            if let Some(ref nodemask) = source.nodemask {
                write_element(writer, "nodemask", nodemask)?;
            }
            if let Some(ref alignsize) = source.alignsize {
                write_size_element(writer, "alignsize", alignsize)?;
            }
            if source.pmem.is_some() {
                let pmem_elem = BytesStart::new("pmem");
                writer.write_event(Event::Empty(pmem_elem)).map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("source"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref target) = mem_dev.target {
            let target_elem = BytesStart::new("target");
            writer.write_event(Event::Start(target_elem)).map_err(|e| e.to_string())?;

            if let Some(ref size) = target.size {
                write_size_element(writer, "size", size)?;
            }
            if let Some(node) = target.node {
                write_element(writer, "node", &node.to_string())?;
            }
            if let Some(ref label) = target.label {
                let mut label_elem = BytesStart::new("label");
                if let Some(size) = label.size {
                    label_elem.push_attribute(("size", size.to_string().as_str()));
                }
                if let Some(ref unit) = label.unit {
                    label_elem.push_attribute(("unit", unit.as_str()));
                }
                writer.write_event(Event::Empty(label_elem)).map_err(|e| e.to_string())?;
            }
            if target.readonly.is_some() {
                let readonly_elem = BytesStart::new("readonly");
                writer.write_event(Event::Empty(readonly_elem)).map_err(|e| e.to_string())?;
            }
            if let Some(ref requested) = target.requested {
                write_size_element(writer, "requested", requested)?;
            }
            if let Some(ref current) = target.current {
                write_size_element(writer, "current", current)?;
            }
            if let Some(ref address) = target.address {
                let mut addr_elem = BytesStart::new("address");
                if let Some(ref base) = address.base {
                    addr_elem.push_attribute(("base", base.as_str()));
                }
                writer.write_event(Event::Empty(addr_elem)).map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("target"))).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("memory"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入 IOMMU 设备
pub fn write_iommu<W: std::io::Write>(
    writer: &mut Writer<W>,
    iommu: &IommuConfig,
) -> Result<(), String> {
    let mut iommu_elem = BytesStart::new("iommu");
    iommu_elem.push_attribute(("model", iommu.model.as_str()));
    writer.write_event(Event::Start(iommu_elem)).map_err(|e| e.to_string())?;

    if let Some(ref driver) = iommu.driver {
        let mut driver_elem = BytesStart::new("driver");
        if let Some(ref intremap) = driver.intremap {
            driver_elem.push_attribute(("intremap", intremap.as_str()));
        }
        if let Some(ref caching_mode) = driver.caching_mode {
            driver_elem.push_attribute(("caching_mode", caching_mode.as_str()));
        }
        if let Some(ref api_mode) = driver.api_mode {
            driver_elem.push_attribute(("api_mode", api_mode.as_str()));
        }
        if let Some(ref ats) = driver.ats {
            driver_elem.push_attribute(("ats", ats.as_str()));
        }
        if let Some(ref aw_bits) = driver.aw_bits {
            driver_elem.push_attribute(("aw_bits", aw_bits.as_str()));
        }
        if let Some(ref snoop_wb) = driver.snoop_wb {
            driver_elem.push_attribute(("snoop_wb", snoop_wb.as_str()));
        }
        if let Some(ref x2apic_scale) = driver.x2apic_scale {
            driver_elem.push_attribute(("x2apic_scale", x2apic_scale.as_str()));
        }
        writer.write_event(Event::Empty(driver_elem)).map_err(|e| e.to_string())?;
    }

    writer.write_event(Event::End(BytesEnd::new("iommu"))).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入 Vsock 设备
pub fn write_vsock<W: std::io::Write>(
    writer: &mut Writer<W>,
    vsock: &VsockConfig,
) -> Result<(), String> {
    let mut vsock_elem = BytesStart::new("vsock");
    if let Some(id) = vsock.id {
        vsock_elem.push_attribute(("id", id.to_string().as_str()));
    }
    writer.write_event(Event::Start(vsock_elem)).map_err(|e| e.to_string())?;

    if let Some(ref source) = vsock.source {
        let mut source_elem = BytesStart::new("source");
        if let Some(ref mode) = source.mode {
            source_elem.push_attribute(("mode", mode.as_str()));
        }
        if let Some(ref path) = source.path {
            source_elem.push_attribute(("path", path.as_str()));
        }
        if let Some(cid) = source.cid {
            source_elem.push_attribute(("cid", cid.to_string().as_str()));
        }
        if let Some(ref auto) = source.auto {
            source_elem.push_attribute(("auto", auto.as_str()));
        }
        writer.write_event(Event::Empty(source_elem)).map_err(|e| e.to_string())?;
    }

    if let Some(ref address) = vsock.address {
        write_address(writer, address)?;
    }

    writer.write_event(Event::End(BytesEnd::new("vsock"))).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入 Crypto 设备
pub fn write_crypto<W: std::io::Write>(
    writer: &mut Writer<W>,
    crypto: &CryptoConfig,
) -> Result<(), String> {
    let mut crypto_elem = BytesStart::new("crypto");
    crypto_elem.push_attribute(("type", crypto.crypto_type.as_str()));
    writer.write_event(Event::Start(crypto_elem)).map_err(|e| e.to_string())?;

    if let Some(ref backend) = crypto.backend {
        let mut backend_elem = BytesStart::new("backend");
        backend_elem.push_attribute(("type", backend.backend_type.as_str()));
        if let Some(node) = backend.node {
            backend_elem.push_attribute(("node", node.to_string().as_str()));
        }
        writer.write_event(Event::Empty(backend_elem)).map_err(|e| e.to_string())?;
    }

    writer.write_event(Event::End(BytesEnd::new("crypto"))).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入 Pstore 设备
pub fn write_pstore<W: std::io::Write>(
    writer: &mut Writer<W>,
    pstore: &PstoreConfig,
) -> Result<(), String> {
    let mut pstore_elem = BytesStart::new("pstore");
    pstore_elem.push_attribute(("path", pstore.path.as_str()));
    writer.write_event(Event::Start(pstore_elem)).map_err(|e| e.to_string())?;

    if let Some(ref size) = pstore.size {
        write_size_element(writer, "size", size)?;
    }

    writer.write_event(Event::End(BytesEnd::new("pstore"))).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入 Audio 设备
pub fn write_audio<W: std::io::Write>(
    writer: &mut Writer<W>,
    audio: &AudioConfig,
) -> Result<(), String> {
    let mut audio_elem = BytesStart::new("audio");
    if let Some(ref id) = audio.id {
        audio_elem.push_attribute(("id", id.as_str()));
    }
    if let Some(ref model) = audio.model {
        audio_elem.push_attribute(("model", model.as_str()));
    }
    writer.write_event(Event::Start(audio_elem)).map_err(|e| e.to_string())?;

    // Source 配置
    if let Some(ref source) = audio.source {
        let mut source_elem = BytesStart::new("source");
        if let Some(ref mode) = source.mode {
            source_elem.push_attribute(("mode", mode.as_str()));
        }
        writer.write_event(Event::Start(source_elem)).map_err(|e| e.to_string())?;

        if let Some(ref backend) = source.backend {
            let mut backend_elem = BytesStart::new("backend");
            backend_elem.push_attribute(("type", backend.backend_type.as_str()));
            if let Some(ref server) = backend.server {
                backend_elem.push_attribute(("server", server.as_str()));
            }
            if let Some(ref name) = backend.name {
                backend_elem.push_attribute(("name", name.as_str()));
            }
            if let Some(ref device) = backend.device {
                backend_elem.push_attribute(("device", device.as_str()));
            }
            if let Some(ref format) = backend.format {
                backend_elem.push_attribute(("format", format.as_str()));
            }
            if let Some(ref global) = backend.global {
                backend_elem.push_attribute(("global", global.as_str()));
            }
            writer.write_event(Event::Empty(backend_elem)).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("source"))).map_err(|e| e.to_string())?;
    }

    // Input 配置
    if let Some(ref input) = audio.input {
        let mut input_elem = BytesStart::new("input");
        input_elem.push_attribute(("type", input.stream_type.as_str()));
        if let Some(ref server) = input.server {
            input_elem.push_attribute(("server", server.as_str()));
        }
        if let Some(ref name) = input.name {
            input_elem.push_attribute(("name", name.as_str()));
        }
        if let Some(ref device) = input.device {
            input_elem.push_attribute(("device", device.as_str()));
        }
        if let Some(ref format) = input.format {
            input_elem.push_attribute(("format", format.as_str()));
        }
        if let Some(ref global) = input.global {
            input_elem.push_attribute(("global", global.as_str()));
        }
        writer.write_event(Event::Empty(input_elem)).map_err(|e| e.to_string())?;
    }

    // Output 配置
    if let Some(ref output) = audio.output {
        let mut output_elem = BytesStart::new("output");
        output_elem.push_attribute(("type", output.stream_type.as_str()));
        if let Some(ref server) = output.server {
            output_elem.push_attribute(("server", server.as_str()));
        }
        if let Some(ref name) = output.name {
            output_elem.push_attribute(("name", name.as_str()));
        }
        if let Some(ref device) = output.device {
            output_elem.push_attribute(("device", device.as_str()));
        }
        if let Some(ref format) = output.format {
            output_elem.push_attribute(("format", format.as_str()));
        }
        if let Some(ref global) = output.global {
            output_elem.push_attribute(("global", global.as_str()));
        }
        writer.write_event(Event::Empty(output_elem)).map_err(|e| e.to_string())?;
    }

    // Address 配置
    if let Some(ref address) = audio.address {
        write_address(writer, address)?;
    }

    writer.write_event(Event::End(BytesEnd::new("audio"))).map_err(|e| e.to_string())?;
    Ok(())
}

/// 辅助函数：写入 Address 元素
fn write_address<W: std::io::Write>(
    writer: &mut Writer<W>,
    address: &crate::model::devices::AddressConfig,
) -> Result<(), String> {
    let mut address_elem = BytesStart::new("address");
    address_elem.push_attribute(("type", address.address_type.as_str()));
    if let Some(controller) = address.controller {
        address_elem.push_attribute(("controller", controller.to_string().as_str()));
    }
    if let Some(bus) = address.bus {
        address_elem.push_attribute(("bus", bus.to_string().as_str()));
    }
    if let Some(target) = address.target {
        address_elem.push_attribute(("target", target.to_string().as_str()));
    }
    if let Some(unit) = address.unit {
        address_elem.push_attribute(("unit", unit.to_string().as_str()));
    }
    if let Some(slot) = address.slot {
        address_elem.push_attribute(("slot", slot.to_string().as_str()));
    }
    if let Some(function) = address.function {
        address_elem.push_attribute(("function", function.to_string().as_str()));
    }
    if let Some(ref domain) = address.domain {
        address_elem.push_attribute(("domain", domain.as_str()));
    }
    if let Some(ref multifunction) = address.multifunction {
        address_elem.push_attribute(("multifunction", multifunction.as_str()));
    }
    if let Some(ref reg) = address.reg {
        address_elem.push_attribute(("reg", reg.as_str()));
    }
    if let Some(ref cssid) = address.cssid {
        address_elem.push_attribute(("cssid", cssid.as_str()));
    }
    if let Some(ssid) = address.ssid {
        address_elem.push_attribute(("ssid", ssid.to_string().as_str()));
    }
    if let Some(ref devno) = address.devno {
        address_elem.push_attribute(("devno", devno.as_str()));
    }
    if let Some(iobase) = address.iobase {
        address_elem.push_attribute(("iobase", iobase.to_string().as_str()));
    }
    if let Some(irq) = address.irq {
        address_elem.push_attribute(("irq", irq.to_string().as_str()));
    }
    writer.write_event(Event::Empty(address_elem)).map_err(|e| e.to_string())?;
    Ok(())
}

/// 辅助函数：写入简单元素
fn write_element<W: std::io::Write>(
    writer: &mut Writer<W>,
    name: &str,
    content: &str,
) -> Result<(), String> {
    let elem = BytesStart::new(name);
    writer.write_event(Event::Start(elem)).map_err(|e| e.to_string())?;
    writer.write_event(Event::Text(BytesText::new(content))).map_err(|e| e.to_string())?;
    writer.write_event(Event::End(BytesEnd::new(name))).map_err(|e| e.to_string())?;
    Ok(())
}

/// 辅助函数：写入 Size 元素
fn write_size_element<W: std::io::Write>(
    writer: &mut Writer<W>,
    element_name: &str,
    size: &crate::model::devices::SizeConfig,
) -> Result<(), String> {
    let mut size_elem = BytesStart::new(element_name);
    if let Some(ref unit) = size.unit {
        size_elem.push_attribute(("unit", unit.as_str()));
    }
    if let Some(value) = size.value {
        writer.write_event(Event::Start(size_elem)).map_err(|e| e.to_string())?;
        writer
            .write_event(Event::Text(BytesText::new(&value.to_string())))
            .map_err(|e| e.to_string())?;
        writer.write_event(Event::End(BytesEnd::new(element_name))).map_err(|e| e.to_string())?;
    } else {
        writer.write_event(Event::Empty(size_elem)).map_err(|e| e.to_string())?;
    }
    Ok(())
}
