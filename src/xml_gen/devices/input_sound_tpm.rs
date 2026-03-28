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
        writer.write_event(Event::Empty(input_elem)).map_err(|e| e.to_string())?;
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

        let mut backend_elem = BytesStart::new("backend");
        backend_elem.push_attribute(("type", tpm.backend.backend_type.as_str()));
        if let Some(ref version) = tpm.backend.version {
            backend_elem.push_attribute(("version", version.as_str()));
        }
        writer.write_event(Event::Empty(backend_elem)).map_err(|e| e.to_string())?;

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

        writer.write_event(Event::End(BytesEnd::new("sound"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}
