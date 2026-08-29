//! BDD 规格：标准库模块的编译与加载（替代已删除的 build.rs 校验）。
//!
//! 功能点与边界（已冻结）：
//! 旧的 build.rs 自带一套与真实词法器不一致的第二语法检查器
//! （把 `#` 当注释、`cargo:error=` 非法指令无法阻断构建）。
//! 真正的校验应该用**真实解析器**完成，并作为常规测试运行：
//! 1. 每个 stdlib 模块源码必须能通过 Parser 解析；
//! 2. 每个模块能通过宿主 API（load_stdlib_module）加载；
//! 3. 全量预载后引擎可正常执行冒烟脚本。

use aether::{Aether, Parser};

/// 每个 stdlib 模块：可解析 + 可单独加载。
#[test]
fn all_stdlib_modules_parse_and_load() {
    for (name, code) in aether::stdlib::ALL_MODULES {
        let parsed = Parser::new(code).parse_program();
        assert!(
            parsed.is_ok(),
            "stdlib 模块 {name} 无法解析: {:?}",
            parsed.err()
        );
        let mut engine = Aether::new();
        let loaded = engine.load_stdlib_module(name);
        assert!(
            loaded.is_ok(),
            "stdlib 模块 {name} 加载失败: {:?}",
            loaded.err()
        );
    }
}

/// 全量预载后冒烟执行。
#[test]
fn full_stdlib_preload_smoke() {
    let mut engine = Aether::with_stdlib().expect("全量预载失败");
    let v = engine.eval("(1 + 1)").expect("冒烟脚本失败");
    if let aether::Value::Number(n) = v {
        assert_eq!(n, 2.0);
    } else {
        panic!("预期 Number，实际 {v:?}");
    }
}
