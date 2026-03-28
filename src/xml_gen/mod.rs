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
        let xml = String::from_utf8(result).map_err(|e| format!("转换 UTF-8 失败：{}", e))?;

        // 使用 xmlformat 格式化输出
        let formatter = xmlformat::Formatter::default();
        match formatter.format_xml(&xml) {
            Ok(formatted) => Ok(formatted),
            Err(_) => Ok(xml),
        }
    }

    /// 格式化 XML，添加缩进
    pub fn format_xml(xml: &str) -> String {
        let formatter = xmlformat::Formatter::default();
        match formatter.format_xml(xml) {
            Ok(formatted) => formatted,
            Err(_) => xml.to_string(),
        }
    }

    /// 解析 XML 为带样式的文本，支持语法高亮
    pub fn display_formatted_xml(xml: &str) -> Vec<(egui::Color32, String)> {
        let formatted = Self::format_xml(xml);
        let mut result = Vec::new();
        let chars: Vec<char> = formatted.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == '<' {
                // 收集完整标签
                let start = i;
                while i < chars.len() && chars[i] != '>' {
                    i += 1;
                }
                if i < chars.len() {
                    i += 1;
                }
                let tag: String = chars[start..i].iter().collect();

                // 添加样式
                let styled_parts = Self::style_tag(&tag);
                for (color, text) in styled_parts {
                    result.push((color, text));
                }
            } else if chars[i] == '\n' {
                result.push((egui::Color32::LIGHT_GRAY, "\n".to_string()));
                i += 1;
            } else if !chars[i].is_whitespace() {
                // 标签之间的文本内容
                let mut text = String::new();
                while i < chars.len() && chars[i] != '<' && chars[i] != '\n' {
                    if !chars[i].is_whitespace() {
                        text.push(chars[i]);
                    }
                    i += 1;
                }
                if !text.is_empty() {
                    result.push((egui::Color32::LIGHT_GRAY, text));
                }
            } else {
                i += 1;
            }
        }

        result
    }

    /// 为 XML 标签添加语法高亮
    fn style_tag(tag: &str) -> Vec<(egui::Color32, String)> {
        let mut parts = Vec::new();
        let tag_trimmed = tag.trim();

        if tag_trimmed.starts_with("<?") {
            // XML 声明 - 紫色
            parts.push((egui::Color32::from_rgb(180, 100, 180), format!("{}\n", tag_trimmed)));
        } else if tag_trimmed.starts_with("<!--") {
            // 注释 - 绿色
            parts.push((egui::Color32::from_rgb(100, 180, 100), tag_trimmed.to_string()));
        } else {
            let is_closing = tag_trimmed.starts_with("</");
            let content = if is_closing {
                &tag_trimmed[2..tag_trimmed.len() - 1]
            } else if tag_trimmed.ends_with("/>") {
                &tag_trimmed[1..tag_trimmed.len() - 2]
            } else {
                &tag_trimmed[1..tag_trimmed.len() - 1]
            };

            // 使用空格分割，但保留属性结构
            let mut tokens: Vec<String> = Vec::new();
            let mut current = String::new();
            let mut in_attr_value = false;

            for ch in content.chars() {
                if ch == '"' {
                    current.push(ch);
                    in_attr_value = !in_attr_value;
                } else if ch == ' ' && !in_attr_value {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current = String::new();
                    }
                } else {
                    current.push(ch);
                }
            }
            if !current.is_empty() {
                tokens.push(current);
            }

            if tokens.is_empty() {
                parts.push((egui::Color32::LIGHT_GRAY, tag.to_string()));
                return parts;
            }

            // 标签名 - 蓝色
            let tag_name = if is_closing {
                format!("</{}>", tokens[0])
            } else if tag_trimmed.ends_with("/>") {
                format!("<{}/>", tokens[0])
            } else {
                format!("<{}>", tokens[0])
            };
            parts.push((egui::Color32::from_rgb(65, 105, 225), tag_name));

            // 属性 - 橙色名和绿色值
            for attr_token in &tokens[1..] {
                if let Some(eq_pos) = attr_token.find('=') {
                    let attr_name = &attr_token[..eq_pos];
                    let attr_value = attr_token[eq_pos + 1..].trim_matches('"');

                    parts.push((egui::Color32::from_rgb(255, 140, 0), format!(" {}", attr_name)));
                    parts.push((
                        egui::Color32::from_rgb(100, 180, 100),
                        format!("=\"{}\"", attr_value),
                    ));
                } else {
                    parts.push((egui::Color32::LIGHT_GRAY, format!(" {}", attr_token)));
                }
            }
        }

        parts
    }
}
