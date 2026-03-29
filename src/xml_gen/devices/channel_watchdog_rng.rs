use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use crate::{
    error::AppError,
    model::devices::{
        AddressConfig, ChannelConfig, MemballoonConfig, RngConfig, RngRate, RngSize, WatchdogConfig,
    },
};

/// 写入 Channel 通道设备
pub fn write_channels<W: std::io::Write>(
    writer: &mut Writer<W>,
    channel_list: &[ChannelConfig],
) -> Result<(), AppError> {
    for channel in channel_list {
        let mut channel_elem = BytesStart::new("channel");
        channel_elem.push_attribute(("type", channel.channel_type.as_str()));
        writer.write_event(Event::Start(channel_elem)).map_err(|e| e.to_string())?;

        if let Some(ref target) = channel.target {
            let mut target_elem = BytesStart::new("target");
            target_elem.push_attribute(("type", target.target_type.as_str()));
            target_elem.push_attribute(("name", target.name.as_str()));
            writer.write_event(Event::Empty(target_elem)).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("channel"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入 Watchdog 看门狗设备
pub fn write_watchdog<W: std::io::Write>(
    writer: &mut Writer<W>,
    watchdog: Option<&WatchdogConfig>,
) -> Result<(), String> {
    if let Some(watchdog) = watchdog {
        let mut watchdog_elem = BytesStart::new("watchdog");
        watchdog_elem.push_attribute(("model", watchdog.model.as_str()));
        watchdog_elem.push_attribute(("action", watchdog.action.as_str()));
        writer.write_event(Event::Start(watchdog_elem)).map_err(|e| e.to_string())?;

        // Address 配置
        if let Some(ref address) = watchdog.address {
            write_address(writer, address)?;
        }

        writer.write_event(Event::End(BytesEnd::new("watchdog"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入 Rng 随机数生成器设备
pub fn write_rngs<W: std::io::Write>(
    writer: &mut Writer<W>,
    rng_list: &[RngConfig],
) -> Result<(), String> {
    for rng in rng_list {
        let rng_elem = BytesStart::new("rng");
        writer.write_event(Event::Start(rng_elem)).map_err(|e| e.to_string())?;

        // Backend 配置
        if let Some(ref backend) = rng.backend {
            let mut backend_elem = BytesStart::new("backend");
            backend_elem.push_attribute(("model", backend.model.as_str()));
            backend_elem.push_attribute(("type", backend.rng_type.as_str()));
            if let Some(ref device) = backend.device {
                backend_elem.push_attribute(("device", device.as_str()));
            }
            writer.write_event(Event::Empty(backend_elem)).map_err(|e| e.to_string())?;
        }

        // Size 配置
        if let Some(ref size) = rng.size {
            write_size(writer, size)?;
        }

        // Rate 配置
        if let Some(ref rate) = rng.rate {
            write_rate(writer, rate)?;
        }

        writer.write_event(Event::End(BytesEnd::new("rng"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入 Memballoon 内存气球设备
pub fn write_memballoon<W: std::io::Write>(
    writer: &mut Writer<W>,
    memballoon: Option<&MemballoonConfig>,
) -> Result<(), String> {
    if let Some(memballoon) = memballoon {
        let mut memballoon_elem = BytesStart::new("memballoon");
        memballoon_elem.push_attribute(("model", memballoon.model.as_str()));

        if let Some(ref autodeflate) = memballoon.autodeflate {
            memballoon_elem.push_attribute(("autodeflate", autodeflate.as_str()));
        }
        if let Some(period) = memballoon.period {
            memballoon_elem.push_attribute(("period", period.to_string().as_str()));
        }

        writer.write_event(Event::Start(memballoon_elem)).map_err(|e| e.to_string())?;

        // Stats 配置
        if let Some(ref stats) = memballoon.stats {
            let mut stats_elem = BytesStart::new("stats");
            if let Some(period) = stats.period {
                stats_elem.push_attribute(("period", period.to_string().as_str()));
            }
            writer.write_event(Event::Empty(stats_elem)).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("memballoon"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入 Address 元素
fn write_address<W: std::io::Write>(
    writer: &mut Writer<W>,
    address: &AddressConfig,
) -> Result<(), String> {
    let mut address_elem = BytesStart::new("address");
    address_elem.push_attribute(("type", address.address_type.as_str()));

    if let Some(domain) = &address.domain {
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
    if let Some(controller) = address.controller {
        address_elem.push_attribute(("controller", controller.to_string().as_str()));
    }
    if let Some(multifunction) = &address.multifunction {
        address_elem.push_attribute(("multifunction", multifunction.as_str()));
    }
    if let Some(target) = address.target {
        address_elem.push_attribute(("target", target.to_string().as_str()));
    }
    if let Some(unit) = address.unit {
        address_elem.push_attribute(("unit", unit.to_string().as_str()));
    }
    if let Some(reg) = &address.reg {
        address_elem.push_attribute(("reg", reg.as_str()));
    }
    if let Some(cssid) = &address.cssid {
        address_elem.push_attribute(("cssid", cssid.as_str()));
    }
    if let Some(ssid) = address.ssid {
        address_elem.push_attribute(("ssid", ssid.to_string().as_str()));
    }
    if let Some(devno) = &address.devno {
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

/// 写入 Size 元素
fn write_size<W: std::io::Write>(writer: &mut Writer<W>, size: &RngSize) -> Result<(), String> {
    let mut size_elem = BytesStart::new("size");
    if let Some(ref unit) = size.unit {
        size_elem.push_attribute(("unit", unit.as_str()));
    }
    writer
        .write_event(Event::Text(BytesText::new(&size.value.to_string())))
        .map_err(|e| e.to_string())?;
    writer.write_event(Event::End(BytesEnd::new("size"))).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入 Rate 元素
fn write_rate<W: std::io::Write>(writer: &mut Writer<W>, rate: &RngRate) -> Result<(), String> {
    let mut rate_elem = BytesStart::new("rate");
    if let Some(period) = rate.period {
        rate_elem.push_attribute(("period", period.to_string().as_str()));
    }
    if let Some(bytes) = rate.bytes {
        rate_elem.push_attribute(("bytes", bytes.to_string().as_str()));
    }
    writer
        .write_event(Event::Text(BytesText::new(&rate.value.to_string())))
        .map_err(|e| e.to_string())?;
    writer.write_event(Event::End(BytesEnd::new("rate"))).map_err(|e| e.to_string())?;
    Ok(())
}
