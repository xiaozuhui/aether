//! BDD 规格：安全默认与权限边界。
//!
//! 功能点与边界（已冻结）：
//! 1. **`with_stdlib()` 不再静默授予全部 IO 权限**：标准库模块本身
//!    是纯计算（内嵌源码，无需文件/网络），宿主只要「带标准库的
//!    DSL 引擎」时不应意外获得任意文件读写能力。
//!    旧行为：with_stdlib() = with_all_permissions()，脚本可直接
//!    READ_FILE 任意路径。
//! 2. **HTTP 内置函数整体移除**（用户决策：不需要内置网络功能）：
//!    任何权限配置下 HTTP_GET/HTTP_POST 等都不存在。
//! 3. 默认引擎（Aether::new）无 IO（既有边界，锁定）。
//! 4. 显式 with_all_permissions() 仍提供文件能力（信任模型：
//!    启用 filesystem_enabled 即完全信任脚本，无路径沙箱——
//!    用户明确要求保留任意路径访问）。

mod spec_common;

use aether::Aether;

/// with_stdlib() 之后文件类内置函数不可用（IO 默认禁用）。
#[test]
fn with_stdlib_grants_no_filesystem_io() {
    let mut engine = Aether::with_stdlib().expect("预载标准库失败");
    assert!(
        engine.eval(r#"READ_FILE("/etc/hosts")"#).is_err(),
        "with_stdlib 不应授予 READ_FILE 权限"
    );
    assert!(
        engine
            .eval(r#"WRITE_FILE("/tmp/aether_spec_should_not_exist", "x")"#)
            .is_err(),
        "with_stdlib 不应授予 WRITE_FILE 权限"
    );
}

/// with_stdlib() 的本职仍然成立：标准库模块可用。
#[test]
fn with_stdlib_still_loads_stdlib_modules() {
    let mut engine = Aether::with_stdlib().expect("预载标准库失败");
    let v = engine
        .eval("SET_SIZE(SET_FROM_ARRAY([1, 2, 3]))")
        .expect("标准库 set 模块应可用");
    spec_common::assert_number(&v, 3.0);
}

/// HTTP 内置函数在任何权限下都不存在。
/// 断言要点：报错必须是「函数不存在」（Not callable / Undefined
/// variable），而不是网络连接失败——否则旧实现（函数存在但连不上
/// 目标地址）也会空转通过。
#[test]
fn http_builtins_are_gone() {
    let cases = [
        r#"HTTP_GET("http://127.0.0.1:1/")"#,
        r#"HTTP_POST("http://127.0.0.1:1/", "{}")"#,
        r#"HTTP_PUT("http://127.0.0.1:1/", "{}")"#,
        r#"HTTP_DELETE("http://127.0.0.1:1/")"#,
    ];
    let assert_not_callable = |engine: &mut Aether, code: &str| {
        let err = engine
            .eval(code)
            .expect_err("HTTP 内置函数应已删除（求值应失败）");
        assert!(
            err.contains("Not callable") || err.contains("Undefined"),
            "{code} 应报「函数不存在」，实际错误: {err}"
        );
    };
    // 默认权限
    let mut plain = Aether::new();
    for code in cases {
        assert_not_callable(&mut plain, code);
    }
    // 全部权限
    let mut all = Aether::with_all_permissions();
    for code in cases {
        assert_not_callable(&mut all, code);
    }
}

/// 默认引擎无文件 IO（既有边界，锁定）。
#[test]
fn default_engine_has_no_file_io() {
    let mut engine = Aether::new();
    assert!(engine.eval(r#"READ_FILE("/etc/hosts")"#).is_err());
}

/// 显式全权限引擎保留任意路径文件访问（信任模型锁定，无沙箱）。
#[test]
fn all_permissions_still_allow_unrestricted_file_access() {
    let mut engine = Aether::with_all_permissions();
    let path = std::env::temp_dir().join("aether_spec_fs_roundtrip.txt");
    let path_str = path.to_str().expect("临时路径应可转为字符串");

    let script = format!(
        r#"
        WRITE_FILE("{path_str}", "hello")
        Set CONTENT READ_FILE("{path_str}")
        DELETE_FILE("{path_str}")
        CONTENT
        "#,
    );
    let v = engine.eval(&script).expect("文件往返读写失败");
    spec_common::assert_str(&v, "hello");
}
