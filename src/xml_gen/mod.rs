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
    /// 简单标签（如 <name>value</name>）会放在一行，复杂标签会展开多行
    pub fn format_xml(xml: &str) -> String {
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
                let tag_start = i;
                // 找到标签的结束
                while i < chars.len() && chars[i] != '>' {
                    i += 1;
                }
                if i < chars.len() {
                    i += 1; // 包含 '>'
                }

                let tag: String = chars[tag_start..i].iter().collect();
                let trimmed_tag = tag.trim();

                if trimmed_tag.is_empty() {
                    continue;
                }

                // 处理结束标签 - 先减少缩进
                if trimmed_tag.starts_with("</") {
                    indent_level = indent_level.saturating_sub(1);
                }

                // 检查是否是简单标签（<tag>value</tag> 在同一行）
                let is_simple_tag = Self::is_simple_tag(&chars, i, trimmed_tag);

                if is_simple_tag {
                    // 简单标签：收集开始标签 + 内容 + 结束标签
                    let (full_line, new_i) =
                        Self::collect_simple_tag(&chars, i, tag_start, indent_level, indent);
                    result.push_str(&full_line);
                    result.push('\n');
                    i = new_i;
                } else {
                    // 复杂标签或自闭合标签
                    // 添加缩进
                    for _ in 0..indent_level {
                        result.push_str(indent);
                    }
                    result.push_str(trimmed_tag);
                    result.push('\n');

                    // 处理开始标签（非自闭合）- 增加缩进
                    if trimmed_tag.starts_with('<')
                        && !trimmed_tag.starts_with("<?")
                        && !trimmed_tag.starts_with("<!--")
                        && !trimmed_tag.ends_with("/>")
                        && !trimmed_tag.starts_with("</")
                    {
                        indent_level += 1;
                    }
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

    /// 检查当前位置开始是否是简单标签（<tag>value</tag>）
    fn is_simple_tag(chars: &[char], pos: usize, start_tag: &str) -> bool {
        // 必须是开始标签
        if !start_tag.starts_with('<')
            || start_tag.starts_with("</")
            || start_tag.starts_with("<?")
            || start_tag.starts_with("<!--")
            || start_tag.ends_with("/>")
        {
            return false;
        }

        // 提取标签名
        let tag_name = Self::extract_tag_name(start_tag);
        if tag_name.is_empty() {
            return false;
        }

        // 查找对应的结束标签
        let mut i = pos;
        let end_tag = format!("</{}>", tag_name);

        // 跳过空白字符
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }

        // 收集内容直到遇到标签
        let content_start = i;
        while i < chars.len() && chars[i] != '<' {
            i += 1;
        }

        // 检查内容是否包含换行或其他标签
        let content: String = chars[content_start..i].iter().collect();
        if content.contains('\n') || content.contains('<') {
            return false;
        }

        // 检查接下来是否是匹配的结束标签
        let remaining: String = chars[i..].iter().collect();
        remaining.trim_start().starts_with(&end_tag)
    }

    /// 提取标签名（从 <tag 或 <tag> 中提取 tag）
    fn extract_tag_name(tag: &str) -> String {
        let tag = tag.trim();
        if !tag.starts_with('<') {
            return String::new();
        }

        let mut start = 1;
        // 跳过 <?xml 等特殊标签的开头
        if tag.starts_with("<?") {
            start = 2;
        }

        let mut end = start;
        while end < tag.len() && !tag[end..].starts_with('>') && !tag[end..].starts_with(' ') {
            end += 1;
        }

        tag[start..end].to_string()
    }

    /// 收集简单标签的完整内容（包括缩进、开始标签、内容、结束标签）
    fn collect_simple_tag(
        chars: &[char],
        _pos: usize,
        tag_start: usize,
        indent_level: i32,
        indent: &str,
    ) -> (String, usize) {
        let mut result = String::new();

        // 添加缩进
        for _ in 0..indent_level {
            result.push_str(indent);
        }

        // 添加开始标签
        let mut i = tag_start;
        while i < chars.len() && chars[i] != '>' {
            result.push(chars[i]);
            i += 1;
        }
        if i < chars.len() {
            result.push(chars[i]); // '>'
            i += 1;
        }

        // 跳过空白字符
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }

        // 添加内容
        let content_start = i;
        while i < chars.len() && chars[i] != '<' {
            i += 1;
        }
        let content: String = chars[content_start..i].iter().collect();
        result.push_str(&content);

        // 跳过空白字符
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }

        // 添加结束标签
        while i < chars.len() && chars[i] != '>' {
            result.push(chars[i]);
            i += 1;
        }
        if i < chars.len() {
            result.push(chars[i]); // '>'
            i += 1;
        }

        (result, i)
    }
}
