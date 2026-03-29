//! 输入验证工具函数

/// 验证 UUID 格式（标准 8-4-4-4-12 格式）
pub fn validate_uuid(s: &str) -> bool {
    if s.is_empty() {
        return true; // 空值允许（可选字段）
    }
    uuid::Uuid::parse_str(s).is_ok()
}

/// 验证 MAC 地址格式（xx:xx:xx:xx:xx:xx，十六进制）
pub fn validate_mac(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }
    let parts: Vec<&str> = s.split(':').collect();
    parts.len() == 6 && parts.iter().all(|p| p.len() == 2 && u8::from_str_radix(p, 16).is_ok())
}

/// 验证内存大小（正整数）
#[allow(dead_code)]
pub fn validate_memory_size(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }
    s.parse::<u64>().map(|v| v > 0).unwrap_or(false)
}

/// 验证 vCPU 数量（1-1024）
#[allow(dead_code)]
pub fn validate_vcpu_count(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }
    s.parse::<u32>().map(|v| (1..=1024).contains(&v)).unwrap_or(false)
}

/// 验证文件路径（非空且不含非法字符）
#[allow(dead_code)]
pub fn validate_path(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }
    !s.contains('\0')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_uuid() {
        assert!(validate_uuid(""));
        assert!(validate_uuid("550e8400-e29b-41d4-a716-446655440000"));
        assert!(!validate_uuid("not-a-uuid"));
        assert!(!validate_uuid("550e8400-e29b-41d4-a716-44665544000Z"));
    }

    #[test]
    fn test_validate_mac() {
        assert!(validate_mac(""));
        assert!(validate_mac("52:54:00:ab:cd:ef"));
        assert!(validate_mac("FF:FF:FF:FF:FF:FF"));
        assert!(!validate_mac("52:54:00:ab:cd"));
        assert!(!validate_mac("52:54:00:ab:cd:zz"));
        assert!(!validate_mac("not-a-mac"));
    }

    #[test]
    fn test_validate_memory_size() {
        assert!(validate_memory_size(""));
        assert!(validate_memory_size("1024"));
        assert!(validate_memory_size("1"));
        assert!(!validate_memory_size("0"));
        assert!(!validate_memory_size("-1"));
        assert!(!validate_memory_size("abc"));
    }

    #[test]
    fn test_validate_vcpu_count() {
        assert!(validate_vcpu_count(""));
        assert!(validate_vcpu_count("1"));
        assert!(validate_vcpu_count("64"));
        assert!(validate_vcpu_count("1024"));
        assert!(!validate_vcpu_count("0"));
        assert!(!validate_vcpu_count("1025"));
        assert!(!validate_vcpu_count("abc"));
    }
}
