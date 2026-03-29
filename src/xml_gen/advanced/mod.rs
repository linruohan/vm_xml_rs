use quick_xml::Writer;

use crate::{error::AppError, model::VMConfig};

pub mod power_mgmt;
pub mod sysinfo;

/// 写入高级配置（sysinfo 等）
pub fn write_advanced<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), AppError> {
    // Sysinfo 配置
    sysinfo::write_sysinfo(writer, config)?;

    // 电源管理配置
    power_mgmt::write_power_management(writer, config)?;

    // 磁盘限流组配置
    power_mgmt::write_disk_throttle_group(writer, config)?;

    // NUMA 配置
    if let Some(ref numa) = config.numa {
        crate::xml_gen::tuning::write_numa(writer, numa)?;
    }

    // 资源分区配置
    if let Some(ref resource) = config.resource_partitioning {
        crate::xml_gen::tuning::write_resource(writer, resource)?;
    }

    // Fibre Channel VMID 配置
    if let Some(ref fc_vmid) = config.fibre_channel_vmid {
        write_fibre_channel_vmid(writer, fc_vmid)?;
    }

    // 安全标签配置
    if let Some(ref security_label) = config.security_label {
        write_security_label(writer, security_label)?;
    }

    // 密钥封装配置
    if let Some(ref key_wrap) = config.key_wrap {
        write_key_wrap(writer, key_wrap)?;
    }

    // 启动安全配置
    if let Some(ref _launch_security) = config.launch_security {
        write_launch_security(writer, config)?;
    }

    Ok(())
}

/// 写入 Fibre Channel VMID 配置
fn write_fibre_channel_vmid<W: std::io::Write>(
    writer: &mut Writer<W>,
    fc_vmid: &crate::model::FibreChannelVMIDConfig,
) -> Result<(), String> {
    let mut vmid_elem = quick_xml::events::BytesStart::new("vmid");
    vmid_elem.push_attribute(("id", fc_vmid.id.as_str()));
    writer.write_event(quick_xml::events::Event::Empty(vmid_elem)).map_err(|e| e.to_string())?;
    Ok(())
}

/// 写入安全标签配置
fn write_security_label<W: std::io::Write>(
    writer: &mut Writer<W>,
    security_label: &crate::model::SecurityLabelConfig,
) -> Result<(), AppError> {
    let mut label_elem = quick_xml::events::BytesStart::new("seclabel");
    label_elem.push_attribute(("type", security_label.label_type.as_str()));
    label_elem.push_attribute(("model", security_label.model.as_str()));
    if let Some(ref relabel) = security_label.relabel {
        label_elem.push_attribute(("relabel", relabel.as_str()));
    }
    writer.write_event(quick_xml::events::Event::Start(label_elem))?;

    if let Some(ref label) = security_label.label {
        crate::xml_gen::general::write_element(writer, "label", label)?;
    }

    writer
        .write_event(quick_xml::events::Event::End(quick_xml::events::BytesEnd::new("seclabel")))?;
    Ok(())
}

/// 写入密钥封装配置
fn write_key_wrap<W: std::io::Write>(
    writer: &mut Writer<W>,
    key_wrap: &crate::model::KeyWrapConfig,
) -> Result<(), AppError> {
    let kw_elem = quick_xml::events::BytesStart::new("keywrap");
    writer.write_event(quick_xml::events::Event::Start(kw_elem))?;

    if let Some(ref master_key) = key_wrap.master_key {
        let mut mk_elem = quick_xml::events::BytesStart::new("masterKey");
        mk_elem.push_attribute(("type", master_key.key_type.as_str()));
        mk_elem.push_attribute(("uri", master_key.uri.as_str()));
        writer.write_event(quick_xml::events::Event::Empty(mk_elem))?;
    }

    writer
        .write_event(quick_xml::events::Event::End(quick_xml::events::BytesEnd::new("keywrap")))?;
    Ok(())
}

/// 写入启动安全配置
fn write_launch_security<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), AppError> {
    if let Some(ref launch_security) = config.launch_security {
        let ls_elem = quick_xml::events::BytesStart::new("launchSecurity");
        writer.write_event(quick_xml::events::Event::Start(ls_elem)).map_err(|e| e.to_string())?;

        if let Some(ref seclabel) = launch_security.seclabel {
            let mut seclabel_elem = quick_xml::events::BytesStart::new("seclabel");
            seclabel_elem.push_attribute(("type", seclabel.label_type.as_str()));
            if !seclabel.model.is_empty() {
                seclabel_elem.push_attribute(("model", seclabel.model.as_str()));
            }
            if let Some(ref relabel) = seclabel.relabel {
                seclabel_elem.push_attribute(("relabel", relabel.as_str()));
            }
            writer
                .write_event(quick_xml::events::Event::Start(seclabel_elem))
                .map_err(|e| e.to_string())?;

            if let Some(ref label) = seclabel.label {
                crate::xml_gen::general::write_element(writer, "label", label)?;
            }

            writer
                .write_event(quick_xml::events::Event::End(quick_xml::events::BytesEnd::new(
                    "seclabel",
                )))
                .map_err(|e| e.to_string())?;
        }

        if let Some(ref tpm) = launch_security.tpm {
            crate::xml_gen::devices::input_sound_tpm::write_tpm(writer, Some(tpm))?;
        }

        writer.write_event(quick_xml::events::Event::End(quick_xml::events::BytesEnd::new(
            "launchSecurity",
        )))?;
    }
    Ok(())
}
