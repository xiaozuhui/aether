// src/builtins/payroll/mod.rs
//! 薪酬计算模块
//!
//! 提供全面的薪酬计算功能，包括：
//! - 基本工资计算
//! - 加班费计算
//! - 个人所得税计算
//! - 社保公积金计算
//! - 考勤扣款计算
//! - 奖金计算
//! - 津贴补贴计算
//! - 薪资折算转换
//! - 日期时间计算
//! - 统计分析

pub mod allowance;
pub mod attendance;
pub mod basic;
pub mod bonus;
pub mod conversion;
pub mod datetime;
pub mod insurance;
pub mod overtime;
pub mod statistics;
pub mod tax;

// 重新导出所有函数
pub use allowance::*;
pub use attendance::*;
pub use basic::*;
pub use bonus::*;
pub use conversion::*;
pub use datetime::*;
pub use insurance::*;
pub use overtime::*;
pub use statistics::*;
pub use tax::*;
