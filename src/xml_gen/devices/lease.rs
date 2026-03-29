use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use crate::{error::AppError, model::devices::LeaseConfig};

/// 写入租约配置
pub fn write_lease<W: std::io::Write>(
    writer: &mut Writer<W>,
    lease: &LeaseConfig,
) -> Result<(), AppError> {
    let lease_elem = BytesStart::new("lease");
    writer.write_event(Event::Start(lease_elem)).map_err(|e| e.to_string())?;

    // Lockspace
    if let Some(ref lockspace) = lease.lockspace {
        let lockspace_elem = BytesStart::new("lockspace");
        writer.write_event(Event::Start(lockspace_elem)).map_err(|e| e.to_string())?;
        writer.write_event(Event::Text(BytesText::new(lockspace))).map_err(|e| e.to_string())?;
        writer.write_event(Event::End(BytesEnd::new("lockspace"))).map_err(|e| e.to_string())?;
    }

    // Key
    if let Some(ref key) = lease.key {
        let key_elem = BytesStart::new("key");
        writer.write_event(Event::Start(key_elem)).map_err(|e| e.to_string())?;
        writer.write_event(Event::Text(BytesText::new(key))).map_err(|e| e.to_string())?;
        writer.write_event(Event::End(BytesEnd::new("key"))).map_err(|e| e.to_string())?;
    }

    // Target
    if let Some(ref target) = lease.target {
        let mut target_elem = BytesStart::new("target");
        target_elem.push_attribute(("path", target.path.as_str()));
        if let Some(offset) = target.offset {
            target_elem.push_attribute(("offset", offset.to_string().as_str()));
        }
        writer.write_event(Event::Empty(target_elem)).map_err(|e| e.to_string())?;
    }

    writer.write_event(Event::End(BytesEnd::new("lease"))).map_err(|e| e.to_string())?;
    Ok(())
}
