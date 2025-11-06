// examples/demo_io.rs
//! IO功能演示
//!
//! 此示例展示如何启用和使用文件系统和网络IO功能

use aether::Aether;

fn main() {
    println!("=== Aether IO 功能演示 ===\n");

    // 创建一个启用所有IO权限的引擎
    let mut engine = Aether::with_all_permissions();

    // 1. 文件系统操作示例
    println!("1. 文件系统操作:");
    let fs_code = r#"
        // 写入文件
        Set FILENAME "demo_output.txt"
        Set CONTENT "Hello from Aether!\n这是演示文件。"
        WRITE_FILE(FILENAME, CONTENT)
        PRINTLN("✓ 文件已创建: " + FILENAME)

        // 读取文件
        Set READ_CONTENT READ_FILE(FILENAME)
        PRINTLN("✓ 文件内容: " + READ_CONTENT)

        // 检查文件存在
        Set EXISTS FILE_EXISTS(FILENAME)
        PRINTLN("✓ 文件存在: " + TO_STRING(EXISTS))

        // 删除文件
        DELETE_FILE(FILENAME)
        PRINTLN("✓ 文件已删除")
    "#;

    match engine.eval(fs_code) {
        Ok(_) => println!(),
        Err(e) => eprintln!("错误: {}\n", e),
    }

    // 2. 网络操作示例
    println!("2. 网络操作:");
    let net_code = r#"
        PRINTLN("✓ 发送HTTP请求到 httpbin.org...")
        Set RESPONSE HTTP_GET("https://httpbin.org/json")
        PRINTLN("✓ 收到响应 (前100字符):")
        PRINTLN(RESPONSE)
    "#;

    match engine.eval(net_code) {
        Ok(_) => println!(),
        Err(e) => eprintln!("错误: {}\n", e),
    }

    // 3. 演示安全性：禁用IO的引擎
    println!("3. 安全性演示（禁用IO）:");
    let mut safe_engine = Aether::new(); // 默认禁用IO

    let unsafe_code = r#"
        WRITE_FILE("hacked.txt", "bad content")
    "#;

    match safe_engine.eval(unsafe_code) {
        Ok(_) => println!("  ❌ 安全问题：不应该允许文件操作！"),
        Err(e) => println!("  ✅ 安全阻止：{}", e),
    }

    println!("\n=== 演示完成 ===");
}
