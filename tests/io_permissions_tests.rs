//! IO 权限体系集成测试
//!
//! IOPermissions 是进程内权限闸门：默认禁用全部 IO，
//! 显式开启后文件系统函数才会注册。

use aether::{Aether, IOPermissions};
use std::fs;

#[test]
fn test_io_disabled_by_default_blocks_read_file() {
    let mut engine = Aether::new();

    let result = engine.eval(r#"READ_FILE("/etc/hosts")"#);
    assert!(
        result.is_err(),
        "READ_FILE should not be registered by default"
    );
    assert!(
        result.unwrap_err().contains("READ_FILE"),
        "Error should mention READ_FILE"
    );
}

#[test]
fn test_filesystem_permission_allows_read_file() {
    let test_file = std::env::temp_dir().join("aether_io_read_test.txt");
    fs::write(&test_file, "Hello IO").unwrap();
    let test_path = test_file.to_str().unwrap().replace('\\', "\\\\");

    let perms = IOPermissions {
        filesystem_enabled: true,
        ..Default::default()
    };
    let mut engine = Aether::with_permissions(perms);

    let result = engine.eval(&format!(r#"READ_FILE("{}")"#, test_path));
    assert!(result.is_ok(), "Should read file: {:?}", result);
    assert_eq!(result.unwrap().to_string(), "Hello IO");

    let _ = fs::remove_file(&test_file);
}

#[test]
fn test_filesystem_permission_write_read_roundtrip() {
    let test_file = std::env::temp_dir().join("aether_io_write_test.txt");
    let test_path = test_file.to_str().unwrap().replace('\\', "\\\\");

    let perms = IOPermissions {
        filesystem_enabled: true,
        ..Default::default()
    };
    let mut engine = Aether::with_permissions(perms);

    let result = engine.eval(&format!(r#"WRITE_FILE("{}", "Safe content")"#, test_path));
    assert!(result.is_ok(), "Should write file: {:?}", result);

    let result = engine.eval(&format!(r#"READ_FILE("{}")"#, test_path));
    assert!(result.is_ok(), "Should read back: {:?}", result);
    assert_eq!(result.unwrap().to_string(), "Safe content");

    let _ = fs::remove_file(&test_file);
}
