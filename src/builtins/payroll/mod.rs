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

use crate::evaluator::RuntimeError;
use crate::value::Value;
use std::collections::HashMap;

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

/// 注册所有薪酬计算函数
pub fn register_payroll_functions() -> HashMap<String, fn(&[Value]) -> Result<Value, RuntimeError>>
{
    let mut functions = HashMap::new();

    // 基本工资计算 (7个)
    functions.insert(
        "CALC_HOURLY_PAY".to_string(),
        basic::calc_hourly_pay as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_DAILY_PAY".to_string(),
        basic::calc_daily_pay as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_MONTHLY_FROM_HOURLY".to_string(),
        basic::calc_monthly_from_hourly as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_ANNUAL_SALARY".to_string(),
        basic::calc_annual_salary as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_BASE_SALARY".to_string(),
        basic::calc_base_salary as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_GROSS_SALARY".to_string(),
        basic::calc_gross_salary as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_NET_SALARY".to_string(),
        basic::calc_net_salary as fn(&[Value]) -> Result<Value, RuntimeError>,
    );

    // 加班费计算 (5个)
    functions.insert(
        "CALC_OVERTIME_PAY".to_string(),
        overtime::calc_overtime_pay as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_WEEKDAY_OVERTIME".to_string(),
        overtime::calc_weekday_overtime as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_WEEKEND_OVERTIME".to_string(),
        overtime::calc_weekend_overtime as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_HOLIDAY_OVERTIME".to_string(),
        overtime::calc_holiday_overtime as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_TOTAL_OVERTIME".to_string(),
        overtime::calc_total_overtime as fn(&[Value]) -> Result<Value, RuntimeError>,
    );

    // 个人所得税 (6个)
    functions.insert(
        "CALC_PERSONAL_TAX".to_string(),
        tax::calc_personal_tax as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_TAXABLE_INCOME".to_string(),
        tax::calc_taxable_income as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_ANNUAL_BONUS_TAX".to_string(),
        tax::calc_annual_bonus_tax as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_EFFECTIVE_TAX_RATE".to_string(),
        tax::calc_effective_tax_rate as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_GROSS_FROM_NET".to_string(),
        tax::calc_gross_from_net as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_TAX_REFUND".to_string(),
        tax::calc_tax_refund as fn(&[Value]) -> Result<Value, RuntimeError>,
    );

    // 社保公积金 (10个)
    functions.insert(
        "CALC_PENSION_INSURANCE".to_string(),
        insurance::calc_pension_insurance as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_MEDICAL_INSURANCE".to_string(),
        insurance::calc_medical_insurance as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_UNEMPLOYMENT_INSURANCE".to_string(),
        insurance::calc_unemployment_insurance as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_HOUSING_FUND".to_string(),
        insurance::calc_housing_fund as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_SOCIAL_INSURANCE".to_string(),
        insurance::calc_social_insurance as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "ADJUST_SOCIAL_BASE".to_string(),
        insurance::adjust_social_base as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_SOCIAL_BASE_LOWER".to_string(),
        insurance::calc_social_base_lower as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_SOCIAL_BASE_UPPER".to_string(),
        insurance::calc_social_base_upper as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_INJURY_INSURANCE".to_string(),
        insurance::calc_injury_insurance as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_MATERNITY_INSURANCE".to_string(),
        insurance::calc_maternity_insurance as fn(&[Value]) -> Result<Value, RuntimeError>,
    );

    // 考勤扣款 (7个)
    functions.insert(
        "CALC_ATTENDANCE_RATE".to_string(),
        attendance::calc_attendance_rate as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_LATE_DEDUCTION".to_string(),
        attendance::calc_late_deduction as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_EARLY_LEAVE_DEDUCTION".to_string(),
        attendance::calc_early_leave_deduction as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_ABSENT_DEDUCTION".to_string(),
        attendance::calc_absent_deduction as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_LEAVE_DEDUCTION".to_string(),
        attendance::calc_leave_deduction as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_SICK_LEAVE_PAY".to_string(),
        attendance::calc_sick_leave_pay as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_UNPAID_LEAVE_DEDUCTION".to_string(),
        attendance::calc_unpaid_leave_deduction as fn(&[Value]) -> Result<Value, RuntimeError>,
    );

    // 奖金计算 (6个)
    functions.insert(
        "CALC_PERFORMANCE_PAY".to_string(),
        bonus::calc_performance_pay as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_ANNUAL_BONUS".to_string(),
        bonus::calc_annual_bonus as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_ATTENDANCE_BONUS".to_string(),
        bonus::calc_attendance_bonus as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_SALES_COMMISSION".to_string(),
        bonus::calc_sales_commission as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_PROJECT_BONUS".to_string(),
        bonus::calc_project_bonus as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_13TH_SALARY".to_string(),
        bonus::calc_13th_salary as fn(&[Value]) -> Result<Value, RuntimeError>,
    );

    // 津贴补贴 (7个)
    functions.insert(
        "CALC_MEAL_ALLOWANCE".to_string(),
        allowance::calc_meal_allowance as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_TRANSPORT_ALLOWANCE".to_string(),
        allowance::calc_transport_allowance as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_COMMUNICATION_ALLOWANCE".to_string(),
        allowance::calc_communication_allowance as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_HOUSING_ALLOWANCE".to_string(),
        allowance::calc_housing_allowance as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_HIGH_TEMP_ALLOWANCE".to_string(),
        allowance::calc_high_temp_allowance as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_NIGHT_SHIFT_ALLOWANCE".to_string(),
        allowance::calc_night_shift_allowance as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_POSITION_ALLOWANCE".to_string(),
        allowance::calc_position_allowance as fn(&[Value]) -> Result<Value, RuntimeError>,
    );

    // 薪资折算转换 (13个)
    functions.insert(
        "ANNUAL_TO_MONTHLY".to_string(),
        conversion::annual_to_monthly as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "MONTHLY_TO_ANNUAL".to_string(),
        conversion::monthly_to_annual as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "DAILY_TO_MONTHLY".to_string(),
        conversion::daily_to_monthly as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "MONTHLY_TO_DAILY".to_string(),
        conversion::monthly_to_daily as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "HOURLY_TO_MONTHLY".to_string(),
        conversion::hourly_to_monthly as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "MONTHLY_TO_HOURLY".to_string(),
        conversion::monthly_to_hourly as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "PRORATE_BY_NATURAL_DAYS".to_string(),
        conversion::prorate_by_natural_days as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "PRORATE_BY_LEGAL_DAYS".to_string(),
        conversion::prorate_by_legal_days as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "PRORATE_BY_WORKDAYS".to_string(),
        conversion::prorate_by_workdays as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_ONBOARDING_SALARY".to_string(),
        conversion::calc_onboarding_salary as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_RESIGNATION_SALARY".to_string(),
        conversion::calc_resignation_salary as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_14TH_SALARY".to_string(),
        conversion::calc_14th_salary as fn(&[Value]) -> Result<Value, RuntimeError>,
    );

    // 日期时间计算 (13个)
    functions.insert(
        "CALC_NATURAL_DAYS".to_string(),
        datetime::calc_natural_days as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "GET_LEGAL_PAY_DAYS".to_string(),
        datetime::get_legal_pay_days as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_WORKDAYS".to_string(),
        datetime::calc_workdays as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_WEEKEND_DAYS".to_string(),
        datetime::calc_weekend_days as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_HOLIDAY_DAYS".to_string(),
        datetime::calc_holiday_days as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "IS_WORKDAY".to_string(),
        datetime::is_workday as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "IS_WEEKEND".to_string(),
        datetime::is_weekend as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "IS_HOLIDAY".to_string(),
        datetime::is_holiday as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_WORK_HOURS".to_string(),
        datetime::calc_work_hours as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_MONTHLY_WORK_HOURS".to_string(),
        datetime::calc_monthly_work_hours as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_ANNUAL_WORKDAYS".to_string(),
        datetime::calc_annual_workdays as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_ANNUAL_PAY_DAYS".to_string(),
        datetime::calc_annual_pay_days as fn(&[Value]) -> Result<Value, RuntimeError>,
    );

    // 统计分析 (6个)
    functions.insert(
        "CALC_SALARY_AVERAGE".to_string(),
        statistics::calc_salary_average as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_SALARY_MEDIAN".to_string(),
        statistics::calc_salary_median as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_SALARY_RANGE".to_string(),
        statistics::calc_salary_range as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_PERCENTILE".to_string(),
        statistics::calc_percentile as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_SALARY_STD_DEV".to_string(),
        statistics::calc_salary_std_dev as fn(&[Value]) -> Result<Value, RuntimeError>,
    );
    functions.insert(
        "CALC_SALARY_DISTRIBUTION".to_string(),
        statistics::calc_salary_distribution as fn(&[Value]) -> Result<Value, RuntimeError>,
    );

    functions
}
