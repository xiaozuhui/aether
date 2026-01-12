//! 结构化 TRACE 事件
//!
//! 提供带级别、分类、时间戳的结构化 TRACE 事件，支持过滤和查询。

use crate::value::Value;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// TRACE 事件级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TraceLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}

impl TraceLevel {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
        }
    }
}

impl std::str::FromStr for TraceLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warn" | "warning" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            _ => Err(format!("Invalid trace level: {}", s)),
        }
    }
}

impl std::fmt::Display for TraceLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 结构化 TRACE 事件条目
#[derive(Debug, Clone)]
pub struct TraceEntry {
    /// 时间戳
    pub timestamp: Instant,
    /// 事件级别
    pub level: TraceLevel,
    /// 事件类型/类别
    pub category: String,
    /// 标签
    pub label: Option<String>,
    /// 追踪的值（保留原始值）
    pub values: Vec<Value>,
    /// 源代码位置（文件名:行号）
    pub location: Option<String>,
}

impl TraceEntry {
    /// 创建新的 TRACE 条目
    pub fn new(level: TraceLevel, category: String, values: Vec<Value>) -> Self {
        Self {
            timestamp: Instant::now(),
            level,
            category,
            label: None,
            values,
            location: None,
        }
    }

    /// 设置标签
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// 设置位置
    pub fn with_location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }

    /// 格式化为字符串（用于向后兼容的 take_trace()）
    pub fn format(&self) -> String {
        let values_str: Vec<String> = self.values.iter().map(|v| v.to_string()).collect();
        let payload = values_str.join(" ");

        match &self.label {
            Some(l) => format!("[{}:{}:{}] {}", self.level, self.category, l, payload),
            None => format!("[{}:{}] {}", self.level, self.category, payload),
        }
    }
}

/// TRACE 过滤器
#[derive(Debug, Default, Clone)]
pub struct TraceFilter {
    /// 最低级别（如 Some(TraceLevel::Warn) 表示只显示 Warn 及以上）
    pub min_level: Option<TraceLevel>,
    /// 类别匹配
    pub category: Option<String>,
    /// 标签匹配
    pub label: Option<String>,
    /// 起始时间
    pub since: Option<Instant>,
}

impl TraceFilter {
    /// 创建新的过滤器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置最低级别
    pub fn with_min_level(mut self, level: TraceLevel) -> Self {
        self.min_level = Some(level);
        self
    }

    /// 设置类别
    pub fn with_category(mut self, category: String) -> Self {
        self.category = Some(category);
        self
    }

    /// 设置标签
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// 设置起始时间
    pub fn with_since(mut self, since: Instant) -> Self {
        self.since = Some(since);
        self
    }

    /// 检查条目是否匹配过滤器
    pub fn matches(&self, entry: &TraceEntry) -> bool {
        if let Some(min_level) = self.min_level
            && entry.level < min_level
        {
            return false;
        }

        if let Some(ref category) = self.category
            && &entry.category != category
        {
            return false;
        }

        if let Some(ref label) = self.label
            && entry.label.as_ref() != Some(label)
        {
            return false;
        }

        if let Some(since) = self.since
            && entry.timestamp < since
        {
            return false;
        }

        true
    }
}

/// TRACE 统计信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TraceStats {
    /// 总记录数
    pub total_entries: usize,
    /// 按级别统计
    pub by_level: std::collections::HashMap<TraceLevel, usize>,
    /// 按类别统计
    pub by_category: std::collections::HashMap<String, usize>,
    /// 缓冲区大小
    pub buffer_size: usize,
    /// 是否已满（最旧的记录被丢弃）
    pub buffer_full: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_trace_level_from_str() {
        assert_eq!(TraceLevel::from_str("debug"), Ok(TraceLevel::Debug));
        assert_eq!(TraceLevel::from_str("DEBUG"), Ok(TraceLevel::Debug));
        assert_eq!(TraceLevel::from_str("info"), Ok(TraceLevel::Info));
        assert_eq!(TraceLevel::from_str("warn"), Ok(TraceLevel::Warn));
        assert_eq!(TraceLevel::from_str("warning"), Ok(TraceLevel::Warn));
        assert_eq!(TraceLevel::from_str("error"), Ok(TraceLevel::Error));
        assert!(TraceLevel::from_str("invalid").is_err());
    }

    #[test]
    fn test_trace_level_display() {
        assert_eq!(TraceLevel::Debug.to_string(), "DEBUG");
        assert_eq!(TraceLevel::Info.to_string(), "INFO");
        assert_eq!(TraceLevel::Warn.to_string(), "WARN");
        assert_eq!(TraceLevel::Error.to_string(), "ERROR");
    }

    #[test]
    fn test_trace_level_ordering() {
        assert!(TraceLevel::Debug < TraceLevel::Info);
        assert!(TraceLevel::Info < TraceLevel::Warn);
        assert!(TraceLevel::Warn < TraceLevel::Error);
    }

    #[test]
    fn test_trace_entry_creation() {
        let entry = TraceEntry::new(
            TraceLevel::Info,
            "test_category".to_string(),
            vec![Value::Number(42.0)],
        );

        assert_eq!(entry.level, TraceLevel::Info);
        assert_eq!(entry.category, "test_category");
        assert_eq!(entry.values.len(), 1);
        assert!(entry.label.is_none());
        assert!(entry.location.is_none());
    }

    #[test]
    fn test_trace_entry_with_label() {
        let entry = TraceEntry::new(TraceLevel::Info, "test_category".to_string(), vec![])
            .with_label("test_label".to_string());

        assert_eq!(entry.label, Some("test_label".to_string()));
    }

    #[test]
    fn test_trace_entry_format() {
        // 无标签
        let entry1 = TraceEntry::new(
            TraceLevel::Info,
            "category1".to_string(),
            vec![Value::Number(42.0)],
        );
        let formatted1 = entry1.format();
        assert!(formatted1.contains("[INFO:category1]"));
        assert!(formatted1.contains("42"));

        // 有标签
        let entry2 = TraceEntry::new(
            TraceLevel::Error,
            "category2".to_string(),
            vec![Value::String("error_msg".to_string())],
        )
        .with_label("test_label".to_string());
        let formatted2 = entry2.format();
        assert!(formatted2.contains("[ERROR:category2:test_label]"));
        assert!(formatted2.contains("error_msg"));
    }

    #[test]
    fn test_trace_filter() {
        let filter = TraceFilter::new().with_min_level(TraceLevel::Warn);

        let debug_entry = TraceEntry::new(TraceLevel::Debug, "test".to_string(), vec![]);
        let warn_entry = TraceEntry::new(TraceLevel::Warn, "test".to_string(), vec![]);
        let error_entry = TraceEntry::new(TraceLevel::Error, "test".to_string(), vec![]);

        assert!(!filter.matches(&debug_entry));
        assert!(filter.matches(&warn_entry));
        assert!(filter.matches(&error_entry));
    }

    #[test]
    fn test_trace_filter_with_category() {
        let filter = TraceFilter::new().with_category("api_call".to_string());

        let api_entry = TraceEntry::new(TraceLevel::Info, "api_call".to_string(), vec![]);
        let db_entry = TraceEntry::new(TraceLevel::Info, "database".to_string(), vec![]);

        assert!(filter.matches(&api_entry));
        assert!(!filter.matches(&db_entry));
    }

    #[test]
    fn test_trace_filter_with_label() {
        let filter = TraceFilter::new().with_label("slow_request".to_string());

        let entry1 = TraceEntry::new(TraceLevel::Warn, "api".to_string(), vec![])
            .with_label("slow_request".to_string());

        let entry2 = TraceEntry::new(TraceLevel::Warn, "api".to_string(), vec![])
            .with_label("fast_request".to_string());

        assert!(filter.matches(&entry1));
        assert!(!filter.matches(&entry2));
    }

    #[test]
    fn test_trace_filter_combined() {
        let filter = TraceFilter::new()
            .with_min_level(TraceLevel::Warn)
            .with_category("api".to_string());

        // 匹配：级别和类别都匹配
        let entry1 = TraceEntry::new(TraceLevel::Warn, "api".to_string(), vec![]);

        // 不匹配：级别太低
        let entry2 = TraceEntry::new(TraceLevel::Info, "api".to_string(), vec![]);

        // 不匹配：类别不匹配
        let entry3 = TraceEntry::new(TraceLevel::Warn, "database".to_string(), vec![]);

        assert!(filter.matches(&entry1));
        assert!(!filter.matches(&entry2));
        assert!(!filter.matches(&entry3));
    }
}
