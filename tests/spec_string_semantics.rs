//! BDD 规格：字符串索引体系统一为 Unicode 字符语义。
//!
//! 功能点与边界（已冻结）——所有字符串操作以**字符**（char）为单位：
//! 1. `LEN` / `STRLEN`：返回字符数（`LEN("你好") == 2`，旧实现返回字节 6）。
//! 2. `s[i]`：字符索引、越界报错（现状已正确，本规格将其锁定）。
//! 3. `CHARAT(s, i)`：字符语义；负索引从尾部数（用**字符数**换算，
//!    旧实现误用字节长度，`CHARAT("你好", -1)` 得 ""）；越界报错
//!    （旧实现返回空串，与 `s[i]` 不一致）。
//! 4. `STRSLICE(s, start, end)`：字符切片，负索引从尾数，越界钳制，
//!    start >= end 返回空串（旧实现是字节切片，切在多字节中间时
//!    静默返回 ""）。
//! 5. `INDEXOF(s, sub)`：返回字符位置（旧实现返回字节位置）。

mod spec_common;

use spec_common::{assert_number, assert_str, eval_err, eval_ok};

/// LEN/STRLEN 按字符计数：中文每字计 1。
#[test]
fn len_counts_chars_not_bytes() {
    assert_number(&eval_ok(r#"LEN("你好")"#), 2.0);
    assert_number(&eval_ok(r#"STRLEN("你好")"#), 2.0);
    // ASCII 保持不变
    assert_number(&eval_ok(r#"LEN("abc")"#), 3.0);
}

/// 下标索引 `s[i]` 已经是字符语义（锁定现状，防止回归）。
#[test]
fn string_index_is_char_based() {
    assert_str(&eval_ok(r#""你好"[1]"#), "好");
    // 越界必须报错（锁定现状）
    eval_err(r#""你好"[5]"#);
}

/// CHARAT 负索引从尾部数：-1 取最后一个字符。
/// 旧实现用字节长度换算：len=6 字节，6-1=5，chars().nth(5) 越界得 ""。
#[test]
fn charat_negative_index_counts_from_end_by_chars() {
    assert_str(&eval_ok(r#"CHARAT("你好", -1)"#), "好");
    assert_str(&eval_ok(r#"CHARAT("Hello", -1)"#), "o");
    assert_str(&eval_ok(r#"CHARAT("Hello", 0)"#), "H");
}

/// CHARAT 越界报错，与 `s[i]` 的行为对齐（旧实现返回空串）。
#[test]
fn charat_out_of_bounds_errors() {
    eval_err(r#"CHARAT("Hello", 10)"#);
}

/// STRSLICE 字符切片：`STRSLICE("你好世界", 0, 2)` 取前两个字符。
/// 旧实现按字节：0..2 只覆盖 "你" 的前两个字节，静默返回 "你"。
#[test]
fn strslice_uses_char_units() {
    assert_str(&eval_ok(r#"STRSLICE("你好世界", 0, 2)"#), "你好");
    // ASCII 与旧实现一致
    assert_str(&eval_ok(r#"STRSLICE("Hello", 1, 3)"#), "el");
}

/// STRSLICE 负索引从尾部数（字符）：-2 起点到末尾。
/// 旧实现按字节换算得到 start(10) > end(4)，错误地返回空串。
#[test]
fn strslice_negative_index_counts_from_end_by_chars() {
    assert_str(&eval_ok(r#"STRSLICE("你好世界", -2, 4)"#), "世界");
}

/// STRSLICE 越界钳制：越界端点收缩到字符串范围内，返回空串当 start >= end。
#[test]
fn strslice_clamps_out_of_range() {
    assert_str(&eval_ok(r#"STRSLICE("Hi", 5, 9)"#), "");
    assert_str(&eval_ok(r#"STRSLICE("Hello", 3, 1)"#), "");
}

/// INDEXOF 返回字符位置：字符串含多字节字符时，
/// 其后子串的位置按字符数计算（旧实现返回字节偏移）。
#[test]
fn indexof_returns_char_position() {
    assert_number(&eval_ok(r#"INDEXOF("héllo", "l")"#), 2.0);
    assert_number(&eval_ok(r#"INDEXOF("你好世界", "世界")"#), 2.0);
    // 未找到保持 -1
    assert_number(&eval_ok(r#"INDEXOF("abc", "z")"#), -1.0);
    // 纯 ASCII 保持字节=字符
    assert_number(&eval_ok(r#"INDEXOF("Hello", "l")"#), 2.0);
}
