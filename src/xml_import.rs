//! XML 导入模块 - 将 libvirt XML 文件解析为 VMConfig
//!
//! 使用 quick-xml 的 Reader API 手动解析 libvirt XML 的扁平结构

use quick_xml::events::Event;
use quick_xml::Reader;
use std::io::BufRead;

use crate::{error::AppError, model::vm_config::VMConfig};

/// VCPU 元素解析结果
struct VcpuResult {
    count: u64,
    placement: Option<String>,
    cpuset: Option<String>,
    current: Option<u64>,
}

/// 从 XML 字符串导入 VMConfig
pub fn import_from_xml(xml: &str) -> Result<VMConfig, AppError> {
    let mut reader = Reader::from_str(xml);
    // quick-xml 0.38 不需要 trim_text

    let mut config = VMConfig::default();
    let mut buf = Vec::new();
    let mut in_domain = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = e.name();
                match name.as_ref() {
                    b"domain" => {
                        in_domain = true;
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"type" {
                                config.general.vm_type =
                                    String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                    }
                    b"name" => {
                        config.general.name = read_text_element(&mut reader)?;
                    }
                    b"uuid" => {
                        config.general.uuid = Some(read_text_element(&mut reader)?);
                    }
                    b"hwuuid" => {
                        config.general.hwuuid = Some(read_text_element(&mut reader)?);
                    }
                    b"genid" => {
                        config.general.genid = Some(read_text_element(&mut reader)?);
                    }
                    b"description" => {
                        config.general.description = Some(read_text_element(&mut reader)?);
                    }
                    b"title" => {
                        config.general.title = Some(read_text_element(&mut reader)?);
                    }
                    b"memory" => {
                        let (value, unit) = read_memory_element(&mut reader)?;
                        config.general.memory.value = value;
                        config.general.memory.unit = unit;
                    }
                    b"currentMemory" => {
                        let (value, unit) = read_memory_element(&mut reader)?;
                        config.general.current_memory = Some(crate::model::MemoryInfo {
                            unit,
                            slots: None,
                            dump_core: None,
                            value,
                        });
                    }
                    b"maxMemory" => {
                        let (value, unit) = read_memory_element(&mut reader)?;
                        config.general.max_memory = Some(crate::model::MemoryInfo {
                            unit,
                            slots: None,
                            dump_core: None,
                            value,
                        });
                    }
                    b"vcpu" => {
                        let vcpu = read_vcpu_element(&mut reader)?;
                        config.general.vcpu.count = vcpu.count as u32;
                        config.general.vcpu.placement = vcpu.placement;
                        config.general.vcpu.cpuset = vcpu.cpuset;
                        config.general.vcpu.current = vcpu.current.map(|v| v as u32);
                    }
                    b"os" => {
                        config.os_booting = read_os_element(&mut reader)?;
                        continue;
                    }
                    b"cpu" => {
                        config.cpu = read_cpu_element(&mut reader)?;
                        continue;
                    }
                    b"devices" => {
                        config.devices = read_devices_element(&mut reader)?;
                        continue;
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e))
                if e.name().as_ref() == b"domain" =>
            {
                break;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(AppError::XmlParseError(format!("XML 解析错误：{}", e))),
            _ => {}
        }
        buf.clear();
    }

    if !in_domain {
        return Err(AppError::XmlParseError("未找到 domain 根元素".to_string()));
    }

    Ok(config)
}

/// 从文件导入 VMConfig
#[allow(dead_code)]
pub fn import_from_file(path: &str) -> Result<VMConfig, AppError> {
    let xml_content = std::fs::read_to_string(path).map_err(AppError::FileOperation)?;
    import_from_xml(&xml_content)
}

/// 读取简单文本元素
fn read_text_element<R: BufRead>(reader: &mut Reader<R>) -> Result<String, AppError> {
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => {
                let text = String::from_utf8_lossy(&e).trim().to_string();
                buf.clear();
                return Ok(text);
            }
            Ok(Event::End(_)) | Ok(Event::Eof) => {
                buf.clear();
                return Ok(String::new());
            }
            Err(e) => return Err(AppError::XmlParseError(format!("XML 解析错误：{}", e))),
            _ => {
                buf.clear();
            }
        }
    }
}

/// 读取 memory 元素（带 unit 属性）
fn read_memory_element<R: BufRead>(reader: &mut Reader<R>) -> Result<(u64, Option<String>), AppError> {
    let mut buf = Vec::new();
    let mut value = 0u64;
    let mut unit: Option<String> = None;

    // 读取开始标签的属性
    if let Ok(Event::Start(ref e)) = reader.read_event_into(&mut buf) {
        for attr in e.attributes().flatten() {
            if attr.key.as_ref() == b"unit" {
                unit = Some(String::from_utf8_lossy(&attr.value).to_string());
            }
        }
    }
    buf.clear();

    // 读取文本内容
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => {
                let text = String::from_utf8_lossy(&e).trim().to_string();
                value = text.parse().unwrap_or(0);
            }
            Ok(Event::End(_)) => {
                buf.clear();
                break;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(AppError::XmlParseError(format!("XML 解析错误：{}", e))),
            _ => {}
        }
        buf.clear();
    }

    Ok((value, unit))
}

/// 读取 vcpu 元素（带 placement、cpuset、current 属性）
fn read_vcpu_element<R: BufRead>(
    reader: &mut Reader<R>,
) -> Result<VcpuResult, AppError> {
    let mut buf = Vec::new();
    let mut count = 0u64;
    let mut placement: Option<String> = None;
    let mut cpuset: Option<String> = None;
    let mut current: Option<u64> = None;

    // 读取开始标签的属性
    if let Ok(Event::Start(ref e)) = reader.read_event_into(&mut buf) {
        for attr in e.attributes().flatten() {
            let key = attr.key.as_ref();
            let val = String::from_utf8_lossy(&attr.value).to_string();
            match key {
                b"placement" => placement = Some(val),
                b"cpuset" => cpuset = Some(val),
                b"current" => current = val.parse().ok(),
                _ => {}
            }
        }
    }
    buf.clear();

    // 读取文本内容
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => {
                let text = String::from_utf8_lossy(&e).trim().to_string();
                count = text.parse().unwrap_or(0);
            }
            Ok(Event::End(_)) => {
                buf.clear();
                break;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(AppError::XmlParseError(format!("XML 解析错误：{}", e))),
            _ => {}
        }
        buf.clear();
    }

    Ok(VcpuResult {
        count,
        placement,
        cpuset,
        current,
    })
}

/// 读取 OS 配置元素
fn read_os_element<R: BufRead>(reader: &mut Reader<R>) -> Result<crate::model::OSBootingConfig, AppError> {
    use crate::model::{BootMenuConfig, OSBootingConfig, OSSystem};

    let mut config = OSBootingConfig::default();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"type" => {
                        // 读取 type 元素内容和属性
                        let mut os_system = OSSystem::default();
                        for attr in e.attributes().flatten() {
                            let key = attr.key.as_ref();
                            let val = String::from_utf8_lossy(&attr.value).to_string();
                            match key {
                                b"arch" => os_system.arch = Some(val),
                                b"machine" => os_system.machine = Some(val),
                                b"firmware" => os_system.firmware = Some(val),
                                _ => {}
                            }
                        }
                        // 读取 type 文本内容
                        let text = read_text_element(reader)?;
                        if !text.is_empty() {
                            // 将 type 内容存储到 firmware 字段（临时方案）
                            os_system.firmware = Some(text);
                        }
                        // 我们需要将 OSSystem 的信息转换到 OSBootingConfig
                        // 由于结构不匹配，这里简化处理
                    }
                    b"boot" => {
                        // 读取 boot 元素
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"dev" {
                                let _dev = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                        // 跳过 boot 元素的剩余部分
                        skip_element(reader, b"boot")?;
                    }
                    b"bootmenu" => {
                        let mut enable = "no".to_string();
                        let mut timeout: Option<u32> = None;
                        for attr in e.attributes().flatten() {
                            let _key = attr.key.as_ref();
                            let val = String::from_utf8_lossy(&attr.value).to_string();
                            match attr.key.as_ref() {
                                b"enable" => enable = val,
                                b"timeout" => timeout = val.parse().ok(),
                                _ => {}
                            }
                        }
                        config.boot_menu = Some(BootMenuConfig { enable, timeout });
                        skip_element(reader, b"bootmenu")?;
                    }
                    b"kernel" => {
                        let _kernel_path = read_text_element(reader)?;
                        // OSBootingConfig 没有 kernel 字段，跳过
                    }
                    b"loader" => {
                        // 读取 loader 元素
                        for attr in e.attributes().flatten() {
                            let _key = attr.key.as_ref();
                            let _val = String::from_utf8_lossy(&attr.value).to_string();
                        }
                        // 读取 loader 文本内容
                        loop {
                            match reader.read_event_into(&mut buf) {
                                Ok(Event::Text(_t)) => {}
                                Ok(Event::End(ref end)) if end.name().as_ref() == b"loader" => {
                                    buf.clear();
                                    break;
                                }
                                Ok(Event::Eof) => break,
                                Err(e) => return Err(AppError::XmlParseError(format!("XML 解析错误：{}", e))),
                                _ => {}
                            }
                            buf.clear();
                        }
                    }
                    _ => {
                        // 跳过未知元素
                        skip_current_element(reader, &buf)?;
                    }
                }
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"os" => {
                buf.clear();
                return Ok(config);
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(AppError::XmlParseError(format!("XML 解析错误：{}", e))),
            _ => {}
        }
        buf.clear();
    }

    Ok(config)
}

/// 跳过当前元素的剩余部分
fn skip_element<R: BufRead>(reader: &mut Reader<R>, name: &[u8]) -> Result<(), AppError> {
    let mut buf = Vec::new();
    let mut depth = 1;
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(_)) => depth += 1,
            Ok(Event::End(ref e)) => {
                depth -= 1;
                if depth == 0 && e.name().as_ref() == name {
                    buf.clear();
                    return Ok(());
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(AppError::XmlParseError(format!("XML 解析错误：{}", e))),
            _ => {}
        }
        buf.clear();
    }
    Ok(())
}

/// 跳过当前元素（使用已读取的 buf）
fn skip_current_element<R: BufRead>(reader: &mut Reader<R>, _buf: &Vec<u8>) -> Result<(), AppError> {
    // 简单处理：读取到对应的 End 事件
    let mut depth = 1;
    let mut local_buf = Vec::new();
    loop {
        match reader.read_event_into(&mut local_buf) {
            Ok(Event::Start(_)) => depth += 1,
            Ok(Event::End(_)) => {
                depth -= 1;
                if depth == 0 {
                    local_buf.clear();
                    return Ok(());
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(AppError::XmlParseError(format!("XML 解析错误：{}", e))),
            _ => {}
        }
        local_buf.clear();
    }
    Ok(())
}

/// 读取 CPU 配置元素
fn read_cpu_element<R: BufRead>(reader: &mut Reader<R>) -> Result<crate::model::CPUConfig, AppError> {
    use crate::model::{CPUConfig, CPUFeatureConfig, CPUModel, CPUTopology};

    let mut config = CPUConfig::default();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"topology" => {
                        let mut sockets = 1u32;
                        let mut cores = 1u32;
                        let mut threads = 1u32;
                        let mut dies: Option<u32> = None;
                        let mut clusters: Option<u32> = None;

                        for attr in e.attributes().flatten() {
                            let key = attr.key.as_ref();
                            let val = String::from_utf8_lossy(&attr.value).to_string();
                            match key {
                                b"sockets" => sockets = val.parse().unwrap_or(1),
                                b"cores" => cores = val.parse().unwrap_or(1),
                                b"threads" => threads = val.parse().unwrap_or(1),
                                b"dies" => dies = val.parse().ok(),
                                b"clusters" => clusters = val.parse().ok(),
                                _ => {}
                            }
                        }

                        config.topology = Some(CPUTopology {
                            sockets,
                            cores,
                            threads,
                            dies,
                            clusters,
                        });

                        // 跳过 topology 元素的剩余部分
                        skip_element(reader, b"topology")?;
                    }
                    b"model" => {
                        let mut fallback: Option<String> = None;
                        let mut vendor_id: Option<String> = None;

                        for attr in e.attributes().flatten() {
                            let key = attr.key.as_ref();
                            let val = String::from_utf8_lossy(&attr.value).to_string();
                            match key {
                                b"fallback" => fallback = Some(val),
                                b"vendor_id" => vendor_id = Some(val),
                                _ => {}
                            }
                        }

                        // 读取 model 文本内容
                        let name = read_text_element(reader)?;

                        config.model = Some(CPUModel {
                            fallback,
                            vendor_id,
                            name,
                        });
                    }
                    b"feature" => {
                        let mut policy = String::new();
                        let mut name = String::new();

                        for attr in e.attributes().flatten() {
                            let key = attr.key.as_ref();
                            let val = String::from_utf8_lossy(&attr.value).to_string();
                            match key {
                                b"policy" => policy = val,
                                b"name" => name = val,
                                _ => {}
                            }
                        }

                        if config.feature.is_none() {
                            config.feature = Some(Vec::new());
                        }
                        if let Some(ref mut features) = config.feature {
                            features.push(CPUFeatureConfig { policy, name });
                        }

                        // 跳过 feature 元素的剩余部分
                        skip_element(reader, b"feature")?;
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"cpu" => {
                buf.clear();
                return Ok(config);
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(AppError::XmlParseError(format!("XML 解析错误：{}", e))),
            _ => {}
        }
        buf.clear();
    }

    Ok(config)
}

/// 读取设备配置元素
fn read_devices_element<R: BufRead>(reader: &mut Reader<R>) -> Result<crate::model::DevicesConfig, AppError> {
    let mut buf = Vec::new();
    let mut devices = crate::model::DevicesConfig::default();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"disk" => {
                        let disk = read_disk_element(reader)?;
                        devices.disk.get_or_insert_with(Vec::new).push(disk);
                        continue;
                    }
                    b"interface" => {
                        let network = read_interface_element(reader)?;
                        devices.interface.get_or_insert_with(Vec::new).push(network);
                        continue;
                    }
                    b"graphics" => {
                        let graphics = read_graphics_element(reader)?;
                        devices.graphics.get_or_insert_with(Vec::new).push(graphics);
                        continue;
                    }
                    b"video" => {
                        let video = read_video_element(reader)?;
                        devices.video.get_or_insert_with(Vec::new).push(video);
                        continue;
                    }
                    b"console" => {
                        let console = read_console_element(reader)?;
                        devices.console = Some(console);
                        continue;
                    }
                    b"serial" => {
                        let serial = read_serial_element(reader)?;
                        devices.serial.get_or_insert_with(Vec::new).push(serial);
                        continue;
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"devices" => {
                buf.clear();
                return Ok(devices);
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(AppError::XmlParseError(format!("XML 解析错误：{}", e))),
            _ => {}
        }
        buf.clear();
    }

    Ok(devices)
}

/// 读取 disk 设备元素
fn read_disk_element<R: BufRead>(reader: &mut Reader<R>) -> Result<crate::model::DiskConfig, AppError> {
    use crate::model::devices::disk::{DiskConfig, DiskDriver, DiskSource, DiskTarget};

    let mut disk_type = "file".to_string();
    let mut device = "disk".to_string();
    let mut driver: Option<DiskDriver> = None;
    let mut source: Option<DiskSource> = None;
    let mut target: Option<DiskTarget> = None;

    let mut buf = Vec::new();

    // 读取 disk 开始标签的属性
    if let Ok(Event::Start(ref e)) = reader.read_event_into(&mut buf) {
        for attr in e.attributes().flatten() {
            let key = attr.key.as_ref();
            let val = String::from_utf8_lossy(&attr.value).to_string();
            match key {
                b"type" => disk_type = val,
                b"device" => device = val,
                _ => {}
            }
        }
    }
    buf.clear();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"driver" => {
                        let mut name = "qemu".to_string();
                        let mut driver_type = "raw".to_string();
                        let mut cache: Option<String> = None;

                        for attr in e.attributes().flatten() {
                            let key = attr.key.as_ref();
                            let val = String::from_utf8_lossy(&attr.value).to_string();
                            match key {
                                b"name" => name = val,
                                b"type" => driver_type = val,
                                b"cache" => cache = Some(val),
                                _ => {}
                            }
                        }

                        driver = Some(DiskDriver {
                            name,
                            driver_type,
                            cache,
                            error_policy: None,
                            rerror_policy: None,
                            io: None,
                            ioeventfd: None,
                            event_idx: None,
                            copy_on_read: None,
                            discard: None,
                            detect_zeroes: None,
                            queues: None,
                            queue_size: None,
                            iothread: None,
                            iothreads: None,
                            statistics: None,
                            latency_histogram: None,
                            discard_no_unref: None,
                            metadata_cache: None,
                        });

                        skip_element(reader, b"driver")?;
                    }
                    b"source" => {
                        let mut file: Option<String> = None;
                        let mut dev: Option<String> = None;

                        for attr in e.attributes().flatten() {
                            let key = attr.key.as_ref();
                            let val = String::from_utf8_lossy(&attr.value).to_string();
                            match key {
                                b"file" => file = Some(val),
                                b"dev" => dev = Some(val),
                                _ => {}
                            }
                        }

                        source = Some(DiskSource {
                            file,
                            dev,
                            protocol: None,
                            name: None,
                            startup_policy: None,
                            host: None,
                            auth: None,
                            seclabel: None,
                        });

                        skip_element(reader, b"source")?;
                    }
                    b"target" => {
                        let mut dev = "vda".to_string();
                        let mut bus: Option<String> = None;

                        for attr in e.attributes().flatten() {
                            let key = attr.key.as_ref();
                            let val = String::from_utf8_lossy(&attr.value).to_string();
                            match key {
                                b"dev" => dev = val,
                                b"bus" => bus = Some(val),
                                _ => {}
                            }
                        }

                        target = Some(DiskTarget {
                            dev,
                            bus,
                            tray: None,
                            rotation_rate: None,
                            removable: None,
                            dpofua: None,
                        });

                        skip_element(reader, b"target")?;
                    }
                    b"boot" => {
                        skip_element(reader, b"boot")?;
                    }
                    _ => {
                        skip_current_element(reader, &buf)?;
                    }
                }
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"disk" => {
                buf.clear();
                return Ok(DiskConfig {
                    disk_type,
                    device,
                    driver,
                    source,
                    target,
                    readonly: None,
                    geometry: None,
                    blockio: None,
                    iotune: None,
                    backenddomain: None,
                    throttlefilters: None,
                    address: None,
                    snapshot: None,
                    alias: None,
                    boot: None,
                    shareable: None,
                    transient: None,
                    encryption: None,
                    serial: None,
                    wwn: None,
                    vendor: None,
                });
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(AppError::XmlParseError(format!("XML 解析错误：{}", e))),
            _ => {}
        }
        buf.clear();
    }

    Ok(DiskConfig {
        disk_type,
        device,
        driver: None,
        source: None,
        target: None,
        readonly: None,
        geometry: None,
        blockio: None,
        iotune: None,
        backenddomain: None,
        throttlefilters: None,
        address: None,
        snapshot: None,
        alias: None,
        boot: None,
        shareable: None,
        transient: None,
        encryption: None,
        serial: None,
        wwn: None,
        vendor: None,
    })
}

/// 读取 interface（网络）设备元素
fn read_interface_element<R: BufRead>(reader: &mut Reader<R>) -> Result<crate::model::InterfaceConfig, AppError> {
    use crate::model::devices::network::{InterfaceConfig, InterfaceModel, InterfaceSource, MacAddress};

    let mut interface_type = "network".to_string();
    let mut mac: Option<MacAddress> = None;
    let mut source: Option<InterfaceSource> = None;
    let mut model: Option<InterfaceModel> = None;

    let mut buf = Vec::new();

    // 读取 interface 开始标签的属性
    if let Ok(Event::Start(ref e)) = reader.read_event_into(&mut buf) {
        for attr in e.attributes().flatten() {
            if attr.key.as_ref() == b"type" {
                interface_type = String::from_utf8_lossy(&attr.value).to_string();
            }
        }
    }
    buf.clear();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"mac" => {
                        let mut address = String::new();
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"address" {
                                address = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                        mac = Some(MacAddress {
                            address,
                            mac_type: None,
                            current_address: None,
                        });
                        skip_element(reader, b"mac")?;
                    }
                    b"source" => {
                        let mut bridge: Option<String> = None;
                        let mut network: Option<String> = None;
                        let mut dev: Option<String> = None;
                        let mut mode: Option<String> = None;

                        for attr in e.attributes().flatten() {
                            let key = attr.key.as_ref();
                            let val = String::from_utf8_lossy(&attr.value).to_string();
                            match key {
                                b"bridge" => bridge = Some(val),
                                b"network" => network = Some(val),
                                b"dev" => dev = Some(val),
                                b"mode" => mode = Some(val),
                                _ => {}
                            }
                        }

                        source = Some(InterfaceSource {
                            bridge,
                            network,
                            dev,
                            mode,
                        });
                        skip_element(reader, b"source")?;
                    }
                    b"model" => {
                        let mut model_type = "virtio".to_string();
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"type" {
                                model_type = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                        model = Some(InterfaceModel { model_type });
                        skip_element(reader, b"model")?;
                    }
                    _ => {
                        skip_current_element(reader, &buf)?;
                    }
                }
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"interface" => {
                buf.clear();
                return Ok(InterfaceConfig {
                    interface_type,
                    trust_guest_rx_filters: None,
                    mac,
                    source,
                    model,
                    alias: None,
                    boot: None,
                    address: None,
                    bandwidth: None,
                    virtualport: None,
                    link: None,
                    target: None,
                    rom: None,
                    acpi: None,
                    backend: None,
                    driver: None,
                    tune: None,
                    guest: None,
                    portgroup: None,
                    vlan: None,
                    port: None,
                    ip: None,
                    port_forward: None,
                });
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(AppError::XmlParseError(format!("XML 解析错误：{}", e))),
            _ => {}
        }
        buf.clear();
    }

    Ok(InterfaceConfig {
        interface_type,
        trust_guest_rx_filters: None,
        mac: None,
        source: None,
        model: None,
        alias: None,
        boot: None,
        address: None,
        bandwidth: None,
        virtualport: None,
        link: None,
        target: None,
        rom: None,
        acpi: None,
        backend: None,
        driver: None,
        tune: None,
        guest: None,
        portgroup: None,
        vlan: None,
        port: None,
        ip: None,
        port_forward: None,
    })
}

/// 读取 graphics 设备元素
fn read_graphics_element<R: BufRead>(reader: &mut Reader<R>) -> Result<crate::model::GraphicsConfig, AppError> {
    let mut graphics_type = "spice".to_string();
    let mut port: Option<String> = None;
    let mut autoport: Option<String> = None;
    let mut listen: Option<String> = None;
    let mut keymap: Option<String> = None;

    let mut buf = Vec::new();

    if let Ok(Event::Start(ref e)) = reader.read_event_into(&mut buf) {
        for attr in e.attributes().flatten() {
            let key = attr.key.as_ref();
            let val = String::from_utf8_lossy(&attr.value).to_string();
            match key {
                b"type" => graphics_type = val,
                b"port" => port = Some(val),
                b"autoport" => autoport = Some(val),
                b"listen" => listen = Some(val),
                b"keymap" => keymap = Some(val),
                _ => {}
            }
        }
    }
    buf.clear();

    // 读取到 graphics 结束
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::End(ref e)) if e.name().as_ref() == b"graphics" => {
                buf.clear();
                break;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(AppError::XmlParseError(format!("XML 解析错误：{}", e))),
            _ => {}
        }
        buf.clear();
    }

    Ok(crate::model::GraphicsConfig {
        graphics_type,
        port,
        autoport,
        listen,
        listen_type: None,
        passwd: None,
        keymap,
        share_policy: None,
        default_mode: None,
        connected: None,
        passwd_valid_to: None,
        power_control: None,
        wait: None,
        gl: None,
        channel: None,
        image: None,
        streaming: None,
        clipboard: None,
        mouse: None,
        filetransfer: None,
        audio: None,
    })
}

/// 读取 video 设备元素
fn read_video_element<R: BufRead>(reader: &mut Reader<R>) -> Result<crate::model::VideoConfig, AppError> {
    use crate::model::devices::graphics_video::VideoModel;

    let video_type: Option<String> = None;
    let primary: Option<String> = None;
    let mut model_type = "qxl".to_string();
    let mut vram: Option<u32> = None;
    let mut heads: Option<u32> = None;

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"model" => {
                        for attr in e.attributes().flatten() {
                            let key = attr.key.as_ref();
                            let val = String::from_utf8_lossy(&attr.value).to_string();
                            match key {
                                b"type" => model_type = val,
                                b"vram" => vram = val.parse().ok(),
                                b"heads" => heads = val.parse().ok(),
                                _ => {}
                            }
                        }
                        skip_element(reader, b"model")?;
                    }
                    _ => {
                        skip_current_element(reader, &buf)?;
                    }
                }
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"video" => {
                buf.clear();
                break;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(AppError::XmlParseError(format!("XML 解析错误：{}", e))),
            _ => {}
        }
        buf.clear();
    }

    Ok(crate::model::VideoConfig {
        video_type,
        primary,
        model: VideoModel {
            model_type,
            vram,
            heads,
            primary: None,
            ram: None,
            vgamem: None,
            vram64: None,
            blob: None,
            edid: None,
        },
        acceleration: None,
        driver: None,
        resolution: None,
        address: None,
    })
}

/// 读取 console 设备元素
fn read_console_element<R: BufRead>(reader: &mut Reader<R>) -> Result<crate::model::ConsoleConfig, AppError> {
    let mut console_type = "pty".to_string();

    let mut buf = Vec::new();

    if let Ok(Event::Start(ref e)) = reader.read_event_into(&mut buf) {
        for attr in e.attributes().flatten() {
            if attr.key.as_ref() == b"type" {
                console_type = String::from_utf8_lossy(&attr.value).to_string();
            }
        }
    }
    buf.clear();

    // 读取到 console 结束
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::End(ref e)) if e.name().as_ref() == b"console" => {
                buf.clear();
                break;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(AppError::XmlParseError(format!("XML 解析错误：{}", e))),
            _ => {}
        }
        buf.clear();
    }

    Ok(crate::model::ConsoleConfig {
        console_type,
        source: None,
        target: None,
        log: None,
    })
}

/// 读取 serial 设备元素
fn read_serial_element<R: BufRead>(reader: &mut Reader<R>) -> Result<crate::model::SerialConfig, AppError> {
    let mut serial_type = "pty".to_string();

    let mut buf = Vec::new();

    if let Ok(Event::Start(ref e)) = reader.read_event_into(&mut buf) {
        for attr in e.attributes().flatten() {
            if attr.key.as_ref() == b"type" {
                serial_type = String::from_utf8_lossy(&attr.value).to_string();
            }
        }
    }
    buf.clear();

    // 读取到 serial 结束
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::End(ref e)) if e.name().as_ref() == b"serial" => {
                buf.clear();
                break;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(AppError::XmlParseError(format!("XML 解析错误：{}", e))),
            _ => {}
        }
        buf.clear();
    }

    Ok(crate::model::SerialConfig {
        serial_type,
        port: None,
        source: None,
        target: None,
        log: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_basic_vm() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<domain type="kvm">
  <name>test-vm</name>
  <uuid>12345678-1234-1234-1234-123456789abc</uuid>
  <memory unit="MiB">1024</memory>
  <vcpu>2</vcpu>
</domain>"#;

        let result = import_from_xml(xml);
        assert!(result.is_ok(), "Failed: {:?}", result);
        let config = result.unwrap();
        assert_eq!(config.general.name, "test-vm");
        assert_eq!(config.general.vm_type, "kvm");
    }

    #[test]
    fn test_import_with_os_booting() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<domain type="kvm">
  <name>test-vm</name>
  <os>
    <type arch="x86_64">hvm</type>
    <boot dev="hd"/>
  </os>
  <memory unit="MiB">1024</memory>
  <vcpu>2</vcpu>
</domain>"#;

        let result = import_from_xml(xml);
        assert!(result.is_ok(), "Failed: {:?}", result);
    }
}
