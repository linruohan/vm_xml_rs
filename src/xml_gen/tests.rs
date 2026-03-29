#[cfg(test)]
mod tests {
    use crate::{model::VMConfig, xml_gen::XMLGenerator};

    fn gen(config: &VMConfig) -> String {
        XMLGenerator::generate(config).expect("XML 生成不应失败")
    }

    #[test]
    fn test_generate_basic_vm() {
        let config = VMConfig::new();
        let xml = gen(&config);
        assert!(xml.contains("<domain"), "应包含 domain 开始标签");
        assert!(xml.contains("</domain>"), "应包含 domain 结束标签");
        assert!(xml.contains("<name>"), "应包含 name 元素");
    }

    #[test]
    fn test_domain_type_attribute() {
        let mut config = VMConfig::new();
        config.general.vm_type = "kvm".to_string();
        let xml = gen(&config);
        assert!(xml.contains(r#"type="kvm""#), "domain 应有 type=kvm 属性");
    }

    #[test]
    fn test_uuid_included_when_set() {
        let mut config = VMConfig::new();
        config.general.uuid = Some("550e8400-e29b-41d4-a716-446655440000".to_string());
        let xml = gen(&config);
        assert!(xml.contains("<uuid>550e8400-e29b-41d4-a716-446655440000</uuid>"));
    }

    #[test]
    fn test_uuid_omitted_when_none() {
        let mut config = VMConfig::new();
        config.general.uuid = None;
        let xml = gen(&config);
        assert!(!xml.contains("<uuid>"), "uuid 为 None 时不应输出");
    }

    #[test]
    fn test_format_xml_indentation() {
        let config = VMConfig::new();
        let raw = gen(&config);
        let formatted = XMLGenerator::format_xml(&raw);
        // 格式化后应有换行
        assert!(formatted.contains('\n'), "格式化后应包含换行");
        // 不应有连续多个空行
        assert!(!formatted.contains("\n\n\n"), "不应有连续多个空行");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut config = VMConfig::new();
        config.general.name = "test-vm".to_string();
        config.general.uuid = Some("550e8400-e29b-41d4-a716-446655440000".to_string());

        let json = serde_json::to_string(&config).expect("序列化不应失败");
        let restored: VMConfig = serde_json::from_str(&json).expect("反序列化不应失败");

        assert_eq!(restored.general.name, "test-vm");
        assert_eq!(restored.general.uuid, Some("550e8400-e29b-41d4-a716-446655440000".to_string()));
    }

    #[test]
    fn test_vcpu_count_in_xml() {
        let mut config = VMConfig::new();
        config.general.vcpu.count = 4;
        let xml = gen(&config);
        assert!(xml.contains("<vcpu>4</vcpu>") || xml.contains(">4<"), "应包含 vcpu 数量 4");
    }
}
