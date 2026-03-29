use std::io::Cursor;

use quick_xml::{
    events::{BytesDecl, BytesEnd, BytesStart, Event},
    Writer,
};

use crate::{error::AppError, model::VMConfig};

pub mod advanced;
pub mod cpu;
pub mod devices;
pub mod general;
pub mod memory;
pub mod misc;
pub mod os;
pub mod tuning;

#[cfg(test)]
mod tests;

pub struct XMLGenerator;

impl XMLGenerator {
    pub fn generate(config: &VMConfig) -> Result<String, AppError> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
            .map_err(|e| AppError::XmlGeneration(format!("写入 XML 声明失败：{}", e)))?;

        let mut domain = BytesStart::new("domain");
        domain.push_attribute(("type", config.general.vm_type.as_str()));
        writer
            .write_event(Event::Start(domain))
            .map_err(|e| AppError::XmlGeneration(format!("写入 domain 标签失败：{}", e)))?;

        // 基础配置
        general::write_general(&mut writer, config)?;

        // OS 配置
        os::write_os(&mut writer, config)?;

        // CPU 配置
        cpu::write_cpu(&mut writer, config)?;

        // 内存配置
        memory::write_memory(&mut writer, config)?;

        // 事件处理
        misc::write_events(&mut writer, config)?;

        // 虚拟机监控器特性
        misc::write_features(&mut writer, config)?;

        // 时钟配置
        misc::write_clock(&mut writer, config)?;

        // 性能监控
        misc::write_perf(&mut writer, config)?;

        // IO 线程
        misc::write_iothreads(&mut writer, config)?;

        // CPU 调优
        tuning::write_cputune(&mut writer, config)?;

        // 设备配置
        devices::write_devices(&mut writer, config)?;

        // 高级配置
        advanced::write_advanced(&mut writer, config)?;

        // NUMA 调优
        misc::write_numatune(&mut writer, config)?;

        writer
            .write_event(Event::End(BytesEnd::new("domain")))
            .map_err(|e| AppError::XmlGeneration(format!("关闭 domain 标签失败：{}", e)))?;

        let result = writer.into_inner().into_inner();
        String::from_utf8(result)
            .map_err(|e| AppError::XmlGeneration(format!("转换 UTF-8 失败：{}", e)))
    }

    /// 格式化 XML，添加缩进
    /// 使用 quick-xml 内置的格式化功能，正确处理 CDATA、注释和特殊字符转义
    pub fn format_xml(xml: &str) -> String {
        // 使用 quick-xml 的 Reader 和 Writer 进行格式化
        let mut reader = quick_xml::Reader::from_str(xml);

        // 使用 new_with_indent 创建带缩进的 Writer
        let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);

        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Eof) => break,
                Ok(event) => {
                    // 克隆事件以避免借用问题
                    let cloned = event.into_owned();
                    // 如果写入失败，返回原始 XML
                    if writer.write_event(cloned).is_err() {
                        return xml.to_string();
                    }
                }
                Err(_) => {
                    // 如果解析失败，返回原始 XML
                    return xml.to_string();
                }
            }
            buf.clear();
        }

        // 获取格式化后的结果
        let result = writer.into_inner().into_inner();
        String::from_utf8(result).unwrap_or_else(|_| xml.to_string())
    }
}
