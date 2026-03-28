use quick_xml::Writer;

use crate::model::VMConfig;

pub mod power_mgmt;
pub mod sysinfo;

/// 写入高级配置（sysinfo 等）
pub fn write_advanced<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    // Sysinfo 配置
    sysinfo::write_sysinfo(writer, config)?;

    // 电源管理配置
    power_mgmt::write_power_management(writer, config)?;

    // 磁盘限流组配置
    power_mgmt::write_disk_throttle_group(writer, config)?;

    Ok(())
}
