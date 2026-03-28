use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Writer,
};

use crate::model::devices::{ChannelConfig, MemballoonConfig, RngConfig, WatchdogConfig};

/// 写入 Channel 通道设备
pub fn write_channels<W: std::io::Write>(
    writer: &mut Writer<W>,
    channel_list: &[ChannelConfig],
) -> Result<(), String> {
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

        if let Some(ref backend) = rng.backend {
            let mut backend_elem = BytesStart::new("backend");
            backend_elem.push_attribute(("model", backend.model.as_str()));
            backend_elem.push_attribute(("type", backend.rng_type.as_str()));
            writer.write_event(Event::Empty(backend_elem)).map_err(|e| e.to_string())?;
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
        writer.write_event(Event::Start(memballoon_elem)).map_err(|e| e.to_string())?;

        writer.write_event(Event::End(BytesEnd::new("memballoon"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}
