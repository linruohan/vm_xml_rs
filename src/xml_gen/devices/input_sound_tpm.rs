use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Writer,
};

use crate::model::{
    devices::{InputConfig, SoundConfig},
    TPMConfig,
};

/// 写入 Input 设备
pub fn write_inputs<W: std::io::Write>(
    writer: &mut Writer<W>,
    input_list: &[InputConfig],
) -> Result<(), String> {
    for input in input_list {
        let mut input_elem = BytesStart::new("input");
        input_elem.push_attribute(("type", input.input_type.as_str()));
        if let Some(ref bus) = input.bus {
            input_elem.push_attribute(("bus", bus.as_str()));
        }
        if let Some(ref name) = input.name {
            input_elem.push_attribute(("name", name.as_str()));
        }
        writer.write_event(Event::Start(input_elem)).map_err(|e| e.to_string())?;

        // Source 配置
        if let Some(ref source) = input.source {
            let mut source_elem = BytesStart::new("source");
            if let Some(ref dev) = source.dev {
                source_elem.push_attribute(("dev", dev.as_str()));
            }
            if let Some(ref grab) = source.grab {
                source_elem.push_attribute(("grab", grab.as_str()));
            }
            if let Some(ref repeat) = source.repeat {
                source_elem.push_attribute(("repeat", repeat.as_str()));
            }
            if let Some(ref grab_toggle) = source.grab_toggle {
                source_elem.push_attribute(("grabToggle", grab_toggle.as_str()));
            }
            writer.write_event(Event::Empty(source_elem)).map_err(|e| e.to_string())?;
        }

        // Driver 配置
        if let Some(ref driver) = input.driver {
            let mut driver_elem = BytesStart::new("driver");
            if let Some(queues) = driver.queues {
                driver_elem.push_attribute(("queues", queues.to_string().as_str()));
            }
            if let Some(ref ioeventfd) = driver.ioeventfd {
                driver_elem.push_attribute(("ioeventfd", ioeventfd.as_str()));
            }
            if let Some(ref event_idx) = driver.event_idx {
                driver_elem.push_attribute(("event_idx", event_idx.as_str()));
            }
            writer.write_event(Event::Empty(driver_elem)).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("input"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入 TPM 设备
pub fn write_tpm<W: std::io::Write>(
    writer: &mut Writer<W>,
    tpm: Option<&TPMConfig>,
) -> Result<(), String> {
    if let Some(tpm) = tpm {
        let mut tpm_elem = BytesStart::new("tpm");
        tpm_elem.push_attribute(("model", tpm.model.as_str()));
        writer.write_event(Event::Start(tpm_elem)).map_err(|e| e.to_string())?;

        if let Some(ref backend) = tpm.backend {
            let mut backend_elem = BytesStart::new("backend");
            backend_elem.push_attribute(("type", backend.backend_type.as_str()));
            if let Some(ref version) = backend.version {
                backend_elem.push_attribute(("version", version.as_str()));
            }
            if let Some(ref device) = backend.device {
                backend_elem.push_attribute(("device", device.as_str()));
            }
            if let Some(ref model) = backend.model {
                backend_elem.push_attribute(("model", model.as_str()));
            }
            writer.write_event(Event::Empty(backend_elem)).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("tpm"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入 Sound 设备
pub fn write_sounds<W: std::io::Write>(
    writer: &mut Writer<W>,
    sound_list: &[SoundConfig],
) -> Result<(), String> {
    for sound in sound_list {
        let mut sound_elem = BytesStart::new("sound");
        sound_elem.push_attribute(("model", sound.model.as_str()));
        writer.write_event(Event::Start(sound_elem)).map_err(|e| e.to_string())?;

        // Codec 配置
        if let Some(ref codec) = sound.codec {
            let mut codec_elem = BytesStart::new("codec");
            codec_elem.push_attribute(("type", codec.codec_type.as_str()));
            if let Some(ref input_type) = codec.input_type {
                codec_elem.push_attribute(("input-type", input_type.as_str()));
            }
            if let Some(ref output_type) = codec.output_type {
                codec_elem.push_attribute(("output-type", output_type.as_str()));
            }
            writer.write_event(Event::Empty(codec_elem)).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("sound"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}
