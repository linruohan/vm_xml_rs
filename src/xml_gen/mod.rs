use std::io::Cursor;

use quick_xml::{
    events::{BytesDecl, BytesEnd, BytesStart, Event},
    Writer,
};

use crate::model::VMConfig;

pub mod advanced;
pub mod cpu;
pub mod devices;
pub mod general;
pub mod memory;
pub mod misc;
pub mod os;
pub mod tuning;

pub struct XMLGenerator;

impl XMLGenerator {
    pub fn generate(config: &VMConfig) -> Result<String, String> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
            .map_err(|e| format!("写入 XML 声明失败：{}", e))?;

        let mut domain = BytesStart::new("domain");
        domain.push_attribute(("type", config.general.vm_type.as_str()));
        writer
            .write_event(Event::Start(domain))
            .map_err(|e| format!("写入 domain 标签失败：{}", e))?;

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
            .map_err(|e| format!("关闭 domain 标签失败：{}", e))?;

        let result = writer.into_inner().into_inner();
        String::from_utf8(result).map_err(|e| format!("转换 UTF-8 失败：{}", e))
    }

    /// 格式化 XML，添加缩进
    pub fn format_xml(xml: &str) -> String {
        // 简单的 XML 格式化实现
        let mut result: String = String::new();
        let mut indent_level: i32 = 0;
        let indent = "  ";
        let mut i = 0;
        let chars: Vec<char> = xml.chars().collect();

        while i < chars.len() {
            // 跳过空白字符
            while i < chars.len() && chars[i].is_whitespace() {
                i += 1;
            }

            if i >= chars.len() {
                break;
            }

            // 找到标签的开始
            if chars[i] == '<' {
                let start = i;
                // 找到标签的结束
                while i < chars.len() && chars[i] != '>' {
                    i += 1;
                }
                if i < chars.len() {
                    i += 1; // 包含 '>'
                }

                let tag: String = chars[start..i].iter().collect();
                let trimmed_tag = tag.trim();

                if trimmed_tag.is_empty() {
                    continue;
                }

                // 处理结束标签
                if trimmed_tag.starts_with("</") {
                    indent_level = indent_level.saturating_sub(1);
                }

                // 添加缩进
                for _ in 0..indent_level {
                    result.push_str(indent);
                }
                result.push_str(trimmed_tag);
                result.push('\n');

                // 处理开始标签（非自闭合）
                if trimmed_tag.starts_with('<')
                    && !trimmed_tag.starts_with("<?")
                    && !trimmed_tag.starts_with("<!--")
                    && !trimmed_tag.ends_with("/>")
                    && !trimmed_tag.starts_with("</")
                {
                    indent_level += 1;
                }
            } else {
                // 文本内容，跳过
                while i < chars.len() && chars[i] != '<' {
                    i += 1;
                }
            }
        }

        result.trim_end().to_string()
    }
}
