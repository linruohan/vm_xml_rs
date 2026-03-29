use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Writer,
};

use crate::{
    error::AppError,
    model::devices::{
        AccelerationConfig, ChannelPolicyConfig, ClipboardConfig, FileTransferConfig, GlConfig,
        GraphicsConfig, ImageConfig, ListenConfig, MouseConfig, ResolutionConfig, StreamingConfig,
        VideoConfig, VideoDriverConfig,
    },
};

/// 写入 Graphics 设备
pub fn write_graphics<W: std::io::Write>(
    writer: &mut Writer<W>,
    graphics_list: &[GraphicsConfig],
) -> Result<(), AppError> {
    for g in graphics_list {
        let mut g_elem = BytesStart::new("graphics");
        g_elem.push_attribute(("type", g.graphics_type.as_str()));
        if let Some(ref port) = g.port {
            g_elem.push_attribute(("port", port.as_str()));
        }
        if let Some(ref autoport) = g.autoport {
            g_elem.push_attribute(("autoport", autoport.as_str()));
        }
        if let Some(ref listen) = g.listen {
            g_elem.push_attribute(("listen", listen.as_str()));
        }
        if let Some(ref passwd) = g.passwd {
            g_elem.push_attribute(("passwd", passwd.as_str()));
        }
        if let Some(ref keymap) = g.keymap {
            g_elem.push_attribute(("keymap", keymap.as_str()));
        }
        if let Some(ref share_policy) = g.share_policy {
            g_elem.push_attribute(("sharePolicy", share_policy.as_str()));
        }
        if let Some(ref default_mode) = g.default_mode {
            g_elem.push_attribute(("defaultMode", default_mode.as_str()));
        }
        if let Some(ref connected) = g.connected {
            g_elem.push_attribute(("connected", connected.as_str()));
        }
        if let Some(ref passwd_valid_to) = g.passwd_valid_to {
            g_elem.push_attribute(("passwdValidTo", passwd_valid_to.as_str()));
        }
        if let Some(ref power_control) = g.power_control {
            g_elem.push_attribute(("powerControl", power_control.as_str()));
        }
        if let Some(ref wait) = g.wait {
            g_elem.push_attribute(("wait", wait.as_str()));
        }

        // listen_type 子元素
        if let Some(ref listen_type) = g.listen_type {
            write_listen_type(writer, listen_type)?;
        }

        // gl 子元素
        if let Some(ref gl) = g.gl {
            write_gl(writer, gl)?;
        }

        // channel 子元素 (SPICE 通道策略)
        if let Some(ref channels) = g.channel {
            write_channel_policies(writer, channels)?;
        }

        // image 子元素
        if let Some(ref image) = g.image {
            write_image(writer, image)?;
        }

        // streaming 子元素
        if let Some(ref streaming) = g.streaming {
            write_streaming(writer, streaming)?;
        }

        // clipboard 子元素
        if let Some(ref clipboard) = g.clipboard {
            write_clipboard(writer, clipboard)?;
        }

        // mouse 子元素
        if let Some(ref mouse) = g.mouse {
            write_mouse(writer, mouse)?;
        }

        // filetransfer 子元素
        if let Some(ref filetransfer) = g.filetransfer {
            write_filetransfer(writer, filetransfer)?;
        }

        // audio 子元素
        if let Some(ref audio) = g.audio {
            write_graphics_audio(writer, audio)?;
        }

        writer.write_event(Event::Empty(g_elem)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入 ListenConfig
fn write_listen_type<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &ListenConfig,
) -> Result<(), String> {
    let mut elem = BytesStart::new("listen");
    elem.push_attribute(("type", config.listen_type.as_str()));
    if let Some(ref address) = config.address {
        elem.push_attribute(("address", address.as_str()));
    }
    writer.write_event(Event::Empty(elem)).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入 GlConfig
fn write_gl<W: std::io::Write>(writer: &mut Writer<W>, config: &GlConfig) -> Result<(), String> {
    let mut elem = BytesStart::new("gl");
    elem.push_attribute(("enable", config.enable.as_str()));
    if let Some(ref rendernode) = config.rendernode {
        elem.push_attribute(("rendernode", rendernode.as_str()));
    }
    writer.write_event(Event::Empty(elem)).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入通道策略列表
fn write_channel_policies<W: std::io::Write>(
    writer: &mut Writer<W>,
    channels: &[ChannelPolicyConfig],
) -> Result<(), String> {
    for ch in channels {
        let mut elem = BytesStart::new("channel");
        elem.push_attribute(("name", ch.name.as_str()));
        elem.push_attribute(("mode", ch.mode.as_str()));
        writer.write_event(Event::Empty(elem)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入 ImageConfig
fn write_image<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &ImageConfig,
) -> Result<(), String> {
    let mut elem = BytesStart::new("image");
    elem.push_attribute(("compression", config.compression.as_str()));
    writer.write_event(Event::Empty(elem)).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入 StreamingConfig
fn write_streaming<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &StreamingConfig,
) -> Result<(), String> {
    let mut elem = BytesStart::new("streaming");
    elem.push_attribute(("mode", config.mode.as_str()));
    writer.write_event(Event::Empty(elem)).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入 ClipboardConfig
fn write_clipboard<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &ClipboardConfig,
) -> Result<(), String> {
    let mut elem = BytesStart::new("clipboard");
    elem.push_attribute(("copypaste", config.copypaste.as_str()));
    writer.write_event(Event::Empty(elem)).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入 MouseConfig
fn write_mouse<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &MouseConfig,
) -> Result<(), String> {
    let mut elem = BytesStart::new("mouse");
    elem.push_attribute(("mode", config.mode.as_str()));
    writer.write_event(Event::Empty(elem)).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入 FileTransferConfig
fn write_filetransfer<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &FileTransferConfig,
) -> Result<(), String> {
    let mut elem = BytesStart::new("filetransfer");
    elem.push_attribute(("enable", config.enable.as_str()));
    writer.write_event(Event::Empty(elem)).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入 GraphicsAudioConfig
fn write_graphics_audio<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &crate::model::devices::GraphicsAudioConfig,
) -> Result<(), String> {
    let mut elem = BytesStart::new("audio");
    elem.push_attribute(("id", config.id.to_string().as_str()));
    writer.write_event(Event::Empty(elem)).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入 Video 设备
pub fn write_video<W: std::io::Write>(
    writer: &mut Writer<W>,
    video_list: &[VideoConfig],
) -> Result<(), String> {
    for v in video_list {
        writer.write_event(Event::Start(BytesStart::new("video"))).map_err(|e| e.to_string())?;

        let mut model_elem = BytesStart::new("model");
        model_elem.push_attribute(("type", v.model.model_type.as_str()));
        if let Some(vram) = v.model.vram {
            model_elem.push_attribute(("vram", vram.to_string().as_str()));
        }
        if let Some(heads) = v.model.heads {
            model_elem.push_attribute(("heads", heads.to_string().as_str()));
        }
        if let Some(ref primary) = v.model.primary {
            model_elem.push_attribute(("primary", primary.as_str()));
        }
        if let Some(ram) = v.model.ram {
            model_elem.push_attribute(("ram", ram.to_string().as_str()));
        }
        if let Some(vgamem) = v.model.vgamem {
            model_elem.push_attribute(("vgamem", vgamem.to_string().as_str()));
        }
        if let Some(vram64) = v.model.vram64 {
            model_elem.push_attribute(("vram64", vram64.to_string().as_str()));
        }
        if let Some(ref blob) = v.model.blob {
            model_elem.push_attribute(("blob", blob.as_str()));
        }
        if let Some(ref edid) = v.model.edid {
            model_elem.push_attribute(("edid", edid.as_str()));
        }
        writer.write_event(Event::Empty(model_elem)).map_err(|e| e.to_string())?;

        // acceleration 子元素
        if let Some(ref accel) = v.acceleration {
            write_acceleration(writer, accel)?;
        }

        // driver 子元素
        if let Some(ref driver) = v.driver {
            write_video_driver(writer, driver)?;
        }

        // resolution 子元素
        if let Some(ref resolution) = v.resolution {
            write_resolution(writer, resolution)?;
        }

        // address 子元素
        if let Some(ref address) = v.address {
            write_address(writer, address)?;
        }

        writer.write_event(Event::End(BytesEnd::new("video"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入 AccelerationConfig
fn write_acceleration<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &AccelerationConfig,
) -> Result<(), String> {
    let mut elem = BytesStart::new("acceleration");
    if let Some(ref accel3d) = config.accel3d {
        elem.push_attribute(("accel3d", accel3d.as_str()));
    }
    if let Some(ref accel2d) = config.accel2d {
        elem.push_attribute(("accel2d", accel2d.as_str()));
    }
    if let Some(ref rendernode) = config.rendernode {
        elem.push_attribute(("rendernode", rendernode.as_str()));
    }
    writer.write_event(Event::Empty(elem)).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入 VideoDriverConfig
fn write_video_driver<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VideoDriverConfig,
) -> Result<(), String> {
    let mut elem = BytesStart::new("driver");
    if let Some(ref name) = config.name {
        elem.push_attribute(("name", name.as_str()));
    }
    if let Some(ref ioeventfd) = config.ioeventfd {
        elem.push_attribute(("ioeventfd", ioeventfd.as_str()));
    }
    if let Some(ref event_idx) = config.event_idx {
        elem.push_attribute(("event_idx", event_idx.as_str()));
    }
    writer.write_event(Event::Empty(elem)).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入 ResolutionConfig
fn write_resolution<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &ResolutionConfig,
) -> Result<(), String> {
    let mut elem = BytesStart::new("resolution");
    elem.push_attribute(("x", config.x.to_string().as_str()));
    elem.push_attribute(("y", config.y.to_string().as_str()));
    writer.write_event(Event::Empty(elem)).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入 AddressConfig
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
    writer.write_event(Event::Empty(address_elem)).map_err(|e| e.to_string())?;
    Ok(())
}
