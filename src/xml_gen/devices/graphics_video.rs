use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Writer,
};

use crate::model::devices::{GraphicsConfig, VideoConfig};

/// 写入 Graphics 设备
pub fn write_graphics<W: std::io::Write>(
    writer: &mut Writer<W>,
    graphics_list: &[GraphicsConfig],
) -> Result<(), String> {
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
        writer.write_event(Event::Empty(g_elem)).map_err(|e| e.to_string())?;
    }
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
        writer.write_event(Event::Empty(model_elem)).map_err(|e| e.to_string())?;

        writer.write_event(Event::End(BytesEnd::new("video"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}
