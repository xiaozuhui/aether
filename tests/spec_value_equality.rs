//! BDD 规格：值相等性（Dict 深相等 / 严格类型边界 / 浮点严格相等）。
//!
//! 功能点与边界（已冻结）：
//! 1. **Dict 深相等**：键集合一致（与插入顺序无关），且逐键值递归相等
//!    则相等。旧行为：equals() 没有 Dict 分支，`{"x":1} == {"x":1}`
//!    恒为 false，连带 Switch 匹配 Dict、stdlib SET_REMOVE 移除 Dict
//!    元素全部失效。
//! 2. **跨类型严格**：Number 与 Fraction 永不相等（`5 == TO_FRACTION(5)`
//!    为 false），需要显式转换（用户认可的既有边界，本测试将其锁定）。
//! 3. **浮点严格相等**：Number == Number 是位相等，不再使用
//!    `(a-b).abs() < f64::EPSILON` 的绝对容差（该容差在小数值处
//!    恒真、在大数值处无意义）。需要容差时用户应显式写
//!    `ABS(A - B) < 0.000001`。

mod spec_common;

use aether::{Aether, Value};
use spec_common::{assert_bool, eval_ok};

/// 最小复现：两个内容相同的 Dict 字面量必须相等。
#[test]
fn equal_dicts_are_equal() {
    assert_bool(&eval_ok(r#"{"x": 1} == {"x": 1}"#), true);
}

/// Dict 相等与键的书写顺序无关（HashMap 语义）。
#[test]
fn dict_equality_ignores_key_order() {
    assert_bool(&eval_ok(r#"{"a": 1, "b": 2} == {"b": 2, "a": 1}"#), true);
}

/// 嵌套结构（Dict 内含 Array 内含 Dict）递归深比较。
#[test]
fn nested_dict_array_deep_equality() {
    assert_bool(
        &eval_ok(r#"{"arr": [1, {"k": "v"}], "n": 2} == {"n": 2, "arr": [1, {"k": "v"}]}"#),
        true,
    );
}

/// 内容不同必须不相等：值不同 / 键不同 / 多一个键。
#[test]
fn unequal_dicts_are_not_equal() {
    assert_bool(&eval_ok(r#"{"x": 1} == {"x": 2}"#), false);
    assert_bool(&eval_ok(r#"{"x": 1} == {"y": 1}"#), false);
    assert_bool(&eval_ok(r#"{"x": 1} == {"x": 1, "y": 2}"#), false);
}

/// Array 内含 Dict 的深相等（数组路径走既有递归，Dict 分支修复后生效）。
#[test]
fn array_of_dicts_deep_equality() {
    assert_bool(&eval_ok(r#"[{"a": 1}] == [{"a": 1}]"#), true);
    assert_bool(&eval_ok(r#"[{"a": 1}] == [{"a": 2}]"#), false);
}

/// **锁定严格跨类型边界**：Number 与 Fraction 不相等，
/// 即便数学值相同（用户明确认可此设计，防止过度修复）。
#[test]
fn number_and_fraction_are_never_equal() {
    assert_bool(&eval_ok("5 == TO_FRACTION(5)"), false);
}

/// 浮点严格相等：0.1+0.2 与 0.3 是不同的 f64 位模式，必须不相等。
/// 旧的 EPSILON 绝对容差会错误地判定相等。
#[test]
fn float_equality_is_strict() {
    assert_bool(&eval_ok("0.1 + 0.2 == 0.3"), false);
}

/// 浮点严格相等：极小数值 1e-20 与 0 必须不相等。
/// 旧的绝对容差 (|1e-20| < 2.2e-16) 会错误地判定相等。
#[test]
fn tiny_number_is_not_zero() {
    assert_bool(&eval_ok("1e-20 == 0"), false);
}

/// 浮点严格相等：位模式相同的值必须相等。
#[test]
fn identical_floats_are_equal() {
    assert_bool(&eval_ok("0.1 == 0.1"), true);
    assert_bool(&eval_ok("2.0 == 2"), true);
}

/// Switch 语句用 equals 匹配分支值：Dict 相等修复后应命中 Case 而非 Default。
#[test]
fn switch_matches_dict_case() {
    let v = eval_ok(
        r#"
        Switch ({"X": 1}) {
            Case {"X": 1}:
                Set R "matched"
            Default:
                Set R "no"
        }
        R
        "#,
    );
    spec_common::assert_str(&v, "matched");
}

/// stdlib 集合模块的 SET_REMOVE 依赖 `!=`（即 equals）：
/// Dict 相等修复后应能从集合中移除 Dict 元素。
#[test]
fn set_remove_works_with_dict_elements() {
    let mut engine = Aether::new();
    engine.load_stdlib_module("set").expect("加载 set 模块失败");

    let v = engine
        .eval(
            r#"
            Set S SET_FROM_ARRAY([{"K": 1}, {"K": 2}])
            Set S2 SET_REMOVE(S, {"K": 1})
            [SET_SIZE(S2), SET_CONTAINS(S2, {"K": 2}), SET_CONTAINS(S2, {"K": 1})]
            "#,
        )
        .expect("SET_REMOVE 脚本求值失败");

    match v {
        Value::Array(a) => {
            // 移除 {"K":1} 后：集合大小 1；{"K":2} 仍在；{"K":1} 已移除
            spec_common::assert_number(&a[0], 1.0);
            spec_common::assert_bool(&a[1], true);
            spec_common::assert_bool(&a[2], false);
        }
        other => panic!("预期 Array，实际 {}", other.type_name()),
    }
}
