// examples/test_permissions.rs
//! 测试IO权限控制

use aether::{Aether, IOPermissions};

fn main() {
    println!("=== 测试IO权限控制 ===\n");

    // 1. 测试默认配置（IO禁用）
    println!("1. 测试默认配置（IO应该被禁用）:");
    let mut engine_disabled = Aether::new();
    let code_fs = r#"
        WRITE_FILE("test.txt", "content")
    "#;

    match engine_disabled.eval(code_fs) {
        Ok(_) => println!("  ❌ 错误：应该禁止访问文件系统"),
        Err(e) => println!("  ✅ 正确：{}", e),
    }

    // 2. 测试启用文件系统权限
    println!("\n2. 测试启用文件系统权限:");
    let mut perms = IOPermissions::default();
    perms.filesystem_enabled = true;
    let mut engine_fs_enabled = Aether::with_permissions(perms);

    match engine_fs_enabled.eval(code_fs) {
        Ok(_) => println!("  ✅ 成功：文件系统操作已执行"),
        Err(e) => println!("  ❌ 错误：{}", e),
    }

    // 3. 测试网络访问（禁用）
    println!("\n3. 测试网络访问（应该被禁用）:");
    let code_net = r#"
        HTTP_GET("https://example.com")
    "#;

    match engine_disabled.eval(code_net) {
        Ok(_) => println!("  ❌ 错误：应该禁止网络访问"),
        Err(e) => println!("  ✅ 正确：{}", e),
    }

    // 4. 测试启用所有权限
    println!("\n4. 测试启用所有权限:");
    let mut engine_all = Aether::with_all_permissions();

    match engine_all.eval(code_net) {
        Ok(_) => println!("  ✅ 成功：网络操作已执行"),
        Err(e) => println!("  ❌ 错误：{}", e),
    }

    // 清理测试文件
    let _ = std::fs::remove_file("test.txt");

    println!("\n=== 测试完成 ===");
}
