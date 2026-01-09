// src/builtins/mod.rs
//! Built-in functions standard library

use crate::evaluator::RuntimeError;
use crate::value::Value;
use std::collections::HashMap;

// Module declarations
pub mod array;
pub mod dict;
pub mod filesystem;
pub mod help;
pub mod io;
pub mod json;
pub mod math;
pub mod network;
pub mod payroll;
pub mod precise;
pub mod report;
pub mod string;
pub mod trace;
pub mod types;

/// Type alias for built-in function implementations
pub type BuiltInFn = fn(&[Value]) -> Result<Value, RuntimeError>;

/// 函数文档信息
#[derive(Debug, Clone)]
pub struct FunctionDoc {
    /// 函数名称
    pub name: String,
    /// 函数描述
    pub description: String,
    /// 参数列表（参数名和描述）
    pub params: Vec<(String, String)>,
    /// 返回值描述
    pub returns: String,
    /// 使用示例
    pub example: Option<String>,
}

/// IO 权限配置
#[derive(Debug, Clone, Default)]
pub struct IOPermissions {
    /// 是否允许文件系统操作
    pub filesystem_enabled: bool,
    /// 是否允许网络操作
    pub network_enabled: bool,
}

impl IOPermissions {
    /// 创建启用所有权限的配置
    pub fn allow_all() -> Self {
        Self {
            filesystem_enabled: true,
            network_enabled: true,
        }
    }

    /// 创建禁用所有权限的配置
    pub fn deny_all() -> Self {
        Self::default()
    }
}

/// Registry of all built-in functions
pub struct BuiltInRegistry {
    functions: HashMap<String, (BuiltInFn, usize)>, // (function, arity)
    docs: HashMap<String, FunctionDoc>,             // 函数文档
    #[allow(dead_code)]
    permissions: IOPermissions,
}

impl BuiltInRegistry {
    /// Create a new registry with all built-in functions (默认禁用IO)
    pub fn new() -> Self {
        Self::with_permissions(IOPermissions::default())
    }

    /// Create a new registry with custom permissions
    pub fn with_permissions(permissions: IOPermissions) -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
            docs: HashMap::new(),
            permissions: permissions.clone(),
        };

        // Help function
        registry.register("HELP", help::help, 0); // Variadic: 0-1 args

        // IO functions
        registry.register("PRINT", io::print, 1);
        registry.register("PRINTLN", io::println, 1);
        registry.register("INPUT", io::input, 1);

        // Trace (DSL-safe debug buffer; handled by evaluator)
        registry.register("TRACE", trace::trace, 1);
        registry.register("TRACE_DEBUG", trace::trace_debug, 2);  // (category, value, ...)
        registry.register("TRACE_INFO", trace::trace_info, 2);    // (category, value, ...)
        registry.register("TRACE_WARN", trace::trace_warn, 2);    // (category, value, ...)
        registry.register("TRACE_ERROR", trace::trace_error, 2);  // (category, value, ...)

        // Array functions
        registry.register("RANGE", array::range, 1); // Variadic: 1-3 args
        registry.register("LEN", types::len, 1);
        registry.register("PUSH", array::push, 2);
        registry.register("POP", array::pop, 1);
        registry.register("MAP", array::map, 2);
        registry.register("FILTER", array::filter, 2);
        registry.register("REDUCE", array::reduce, 3);
        registry.register("JOIN", array::join, 2);
        registry.register("REVERSE", array::reverse, 1);
        registry.register("SORT", array::sort, 1);
        registry.register("SUM", array::sum, 1);
        registry.register("MAX", array::max, 1);
        registry.register("MIN", array::min, 1);

        // Dict functions
        registry.register("KEYS", dict::keys, 1);
        registry.register("VALUES", dict::values, 1);
        registry.register("HAS", dict::has, 2);
        registry.register("MERGE", dict::merge, 2);

        // String functions
        registry.register("SPLIT", string::split, 2);
        registry.register("UPPER", string::upper, 1);
        registry.register("LOWER", string::lower, 1);
        registry.register("TRIM", string::trim, 1);
        registry.register("CONTAINS", string::contains, 2);
        registry.register("STARTS_WITH", string::starts_with, 2);
        registry.register("ENDS_WITH", string::ends_with, 2);
        registry.register("REPLACE", string::replace, 3);
        registry.register("REPEAT", string::repeat, 2);
        registry.register("STRSLICE", string::substr, 3);
        registry.register("STRLEN", string::strlen, 1);
        registry.register("INDEXOF", string::index_of, 2);
        registry.register("CHARAT", string::char_at, 2);

        // Math functions - Basic
        registry.register("ABS", math::abs, 1);
        registry.register("FLOOR", math::floor, 1);
        registry.register("CEIL", math::ceil, 1);
        registry.register("ROUND", math::round, 1);
        registry.register("SQRT", math::sqrt, 1);
        registry.register("POW", math::pow, 2);

        // Math functions - Trigonometry
        registry.register("SIN", math::sin, 1);
        registry.register("COS", math::cos, 1);
        registry.register("TAN", math::tan, 1);
        registry.register("ASIN", math::asin, 1);
        registry.register("ACOS", math::acos, 1);
        registry.register("ATAN", math::atan, 1);
        registry.register("ATAN2", math::atan2, 2);
        registry.register("SINH", math::sinh, 1);
        registry.register("COSH", math::cosh, 1);
        registry.register("TANH", math::tanh, 1);

        // Math functions - Logarithms & Exponentials
        registry.register("LOG", math::log, 1);
        registry.register("LN", math::ln, 1);
        registry.register("LOG2", math::log2, 1);
        registry.register("EXP", math::exp, 1);
        registry.register("EXP2", math::exp2, 1);
        registry.register("EXPM1", math::expm1, 1);
        registry.register("LOG1P", math::log1p, 1);

        // Math functions - Special
        registry.register("FACTORIAL", math::factorial, 1);
        registry.register("GAMMA", math::gamma, 1);
        registry.register("ERF", math::erf, 1);
        registry.register("HYPOT", math::hypot, 2);
        registry.register("SIGN", math::sign, 1);
        registry.register("CLAMP", math::clamp, 3);

        // Math functions - Statistics
        registry.register("MEAN", math::mean, 1);
        registry.register("MEDIAN", math::median, 1);
        registry.register("VARIANCE", math::variance, 1);
        registry.register("STD", math::std, 1);
        registry.register("QUANTILE", math::quantile, 2);

        // Math functions - Vector Operations
        registry.register("DOT", math::dot, 2);
        registry.register("NORM", math::norm, 1);
        registry.register("CROSS", math::cross, 2);
        registry.register("DISTANCE", math::distance, 2);
        registry.register("NORMALIZE", math::normalize, 1);

        // Math functions - Matrix Operations
        registry.register("MATMUL", math::matmul, 2);
        registry.register("TRANSPOSE", math::transpose, 1);
        registry.register("DETERMINANT", math::determinant, 1);
        registry.register("INVERSE", math::matrix_inverse, 1);

        // Math functions - Statistics & Regression
        registry.register("LINEAR_REGRESSION", math::linear_regression, 2);

        // Math functions - Probability Distributions
        registry.register("NORMAL_PDF", math::normal_pdf, 1); // Variadic: 1 or 3
        registry.register("NORMAL_CDF", math::normal_cdf, 1); // Variadic: 1 or 3
        registry.register("POISSON_PMF", math::poisson_pmf, 2);

        // Math constants
        registry.register("PI", math::pi, 0);
        registry.register("E", math::e, 0);
        registry.register("TAU", math::tau, 0);
        registry.register("PHI", math::phi, 0);

        // Precision arithmetic functions
        registry.register("ROUND_TO", math::round_to, 2);
        registry.register("ADD_WITH_PRECISION", math::add_with_precision, 3);
        registry.register("SUB_WITH_PRECISION", math::sub_with_precision, 3);
        registry.register("MUL_WITH_PRECISION", math::mul_with_precision, 3);
        registry.register("DIV_WITH_PRECISION", math::div_with_precision, 3);
        registry.register("SET_PRECISION", math::set_precision, 2);

        // Precise (Fraction) arithmetic functions
        registry.register("TO_FRACTION", precise::to_fraction, 1);
        registry.register("TO_FLOAT", precise::to_float, 1);
        registry.register("SIMPLIFY", precise::simplify, 1);
        registry.register("FRAC_ADD", precise::frac_add, 2);
        registry.register("FRAC_SUB", precise::frac_sub, 2);
        registry.register("FRAC_MUL", precise::frac_mul, 2);
        registry.register("FRAC_DIV", precise::frac_div, 2);
        registry.register("NUMERATOR", precise::numerator, 1);
        registry.register("DENOMINATOR", precise::denominator, 1);
        registry.register("GCD", precise::gcd, 2);
        registry.register("LCM", precise::lcm, 2);

        // Type functions
        registry.register("TYPE", types::type_of, 1);
        registry.register("TO_STRING", types::to_string, 1);
        registry.register("TO_NUMBER", types::to_number, 1);
        registry.register("CLONE", types::clone, 1);

        // JSON functions
        registry.register("JSON_PARSE", json::json_parse, 1);
        registry.register("JSON_STRINGIFY", json::json_stringify, 1); // Variadic: 1-2 args

        // Payroll functions - Basic salary calculations (7个)
        registry.register("CALC_HOURLY_PAY", payroll::basic::calc_hourly_pay, 2);
        registry.register("CALC_DAILY_PAY", payroll::basic::calc_daily_pay, 2);
        registry.register(
            "CALC_MONTHLY_FROM_HOURLY",
            payroll::basic::calc_monthly_from_hourly,
            1,
        );
        registry.register("CALC_ANNUAL_SALARY", payroll::basic::calc_annual_salary, 1);
        registry.register("CALC_BASE_SALARY", payroll::basic::calc_base_salary, 1);
        registry.register("CALC_GROSS_SALARY", payroll::basic::calc_gross_salary, 2);
        registry.register("CALC_NET_SALARY", payroll::basic::calc_net_salary, 2);

        // Payroll functions - Overtime pay (5个)
        registry.register("CALC_OVERTIME_PAY", payroll::overtime::calc_overtime_pay, 2);
        registry.register(
            "CALC_WEEKDAY_OVERTIME",
            payroll::overtime::calc_weekday_overtime,
            2,
        );
        registry.register(
            "CALC_WEEKEND_OVERTIME",
            payroll::overtime::calc_weekend_overtime,
            2,
        );
        registry.register(
            "CALC_HOLIDAY_OVERTIME",
            payroll::overtime::calc_holiday_overtime,
            2,
        );
        registry.register(
            "CALC_TOTAL_OVERTIME",
            payroll::overtime::calc_total_overtime,
            4,
        );

        // Payroll functions - Personal income tax (6个)
        registry.register("CALC_PERSONAL_TAX", payroll::tax::calc_personal_tax, 1);
        registry.register("CALC_TAXABLE_INCOME", payroll::tax::calc_taxable_income, 1);
        registry.register(
            "CALC_ANNUAL_BONUS_TAX",
            payroll::tax::calc_annual_bonus_tax,
            1,
        );
        registry.register(
            "CALC_EFFECTIVE_TAX_RATE",
            payroll::tax::calc_effective_tax_rate,
            2,
        );
        registry.register("CALC_GROSS_FROM_NET", payroll::tax::calc_gross_from_net, 1);
        registry.register("CALC_TAX_REFUND", payroll::tax::calc_tax_refund, 2);

        // Payroll functions - Social insurance (10个)
        registry.register(
            "CALC_PENSION_INSURANCE",
            payroll::insurance::calc_pension_insurance,
            1,
        );
        registry.register(
            "CALC_MEDICAL_INSURANCE",
            payroll::insurance::calc_medical_insurance,
            1,
        );
        registry.register(
            "CALC_UNEMPLOYMENT_INSURANCE",
            payroll::insurance::calc_unemployment_insurance,
            1,
        );
        registry.register(
            "CALC_HOUSING_FUND",
            payroll::insurance::calc_housing_fund,
            1,
        );
        registry.register(
            "CALC_SOCIAL_INSURANCE",
            payroll::insurance::calc_social_insurance,
            1,
        );
        registry.register(
            "ADJUST_SOCIAL_BASE",
            payroll::insurance::adjust_social_base,
            3,
        );
        registry.register(
            "CALC_SOCIAL_BASE_LOWER",
            payroll::insurance::calc_social_base_lower,
            2,
        );
        registry.register(
            "CALC_SOCIAL_BASE_UPPER",
            payroll::insurance::calc_social_base_upper,
            2,
        );
        registry.register(
            "CALC_INJURY_INSURANCE",
            payroll::insurance::calc_injury_insurance,
            1,
        );
        registry.register(
            "CALC_MATERNITY_INSURANCE",
            payroll::insurance::calc_maternity_insurance,
            1,
        );

        // Payroll functions - Attendance (7个)
        registry.register(
            "CALC_ATTENDANCE_RATE",
            payroll::attendance::calc_attendance_rate,
            2,
        );
        registry.register(
            "CALC_LATE_DEDUCTION",
            payroll::attendance::calc_late_deduction,
            1,
        );
        registry.register(
            "CALC_EARLY_LEAVE_DEDUCTION",
            payroll::attendance::calc_early_leave_deduction,
            1,
        );
        registry.register(
            "CALC_ABSENT_DEDUCTION",
            payroll::attendance::calc_absent_deduction,
            2,
        );
        registry.register(
            "CALC_LEAVE_DEDUCTION",
            payroll::attendance::calc_leave_deduction,
            2,
        );
        registry.register(
            "CALC_SICK_LEAVE_PAY",
            payroll::attendance::calc_sick_leave_pay,
            3,
        );
        registry.register(
            "CALC_UNPAID_LEAVE_DEDUCTION",
            payroll::attendance::calc_unpaid_leave_deduction,
            2,
        );

        // Payroll functions - Bonus (6个)
        registry.register(
            "CALC_PERFORMANCE_PAY",
            payroll::bonus::calc_performance_pay,
            2,
        );
        registry.register("CALC_ANNUAL_BONUS", payroll::bonus::calc_annual_bonus, 1);
        registry.register(
            "CALC_ATTENDANCE_BONUS",
            payroll::bonus::calc_attendance_bonus,
            2,
        );
        registry.register(
            "CALC_SALES_COMMISSION",
            payroll::bonus::calc_sales_commission,
            2,
        );
        registry.register("CALC_PROJECT_BONUS", payroll::bonus::calc_project_bonus, 2);
        registry.register("CALC_13TH_SALARY", payroll::bonus::calc_13th_salary, 2);

        // Payroll functions - Allowance (7个)
        registry.register(
            "CALC_MEAL_ALLOWANCE",
            payroll::allowance::calc_meal_allowance,
            2,
        );
        registry.register(
            "CALC_TRANSPORT_ALLOWANCE",
            payroll::allowance::calc_transport_allowance,
            2,
        );
        registry.register(
            "CALC_COMMUNICATION_ALLOWANCE",
            payroll::allowance::calc_communication_allowance,
            2,
        );
        registry.register(
            "CALC_HOUSING_ALLOWANCE",
            payroll::allowance::calc_housing_allowance,
            2,
        );
        registry.register(
            "CALC_HIGH_TEMP_ALLOWANCE",
            payroll::allowance::calc_high_temp_allowance,
            2,
        );
        registry.register(
            "CALC_NIGHT_SHIFT_ALLOWANCE",
            payroll::allowance::calc_night_shift_allowance,
            2,
        );
        registry.register(
            "CALC_POSITION_ALLOWANCE",
            payroll::allowance::calc_position_allowance,
            2,
        );

        // Payroll functions - Conversion (12个)
        registry.register(
            "ANNUAL_TO_MONTHLY",
            payroll::conversion::annual_to_monthly,
            1,
        );
        registry.register(
            "MONTHLY_TO_ANNUAL",
            payroll::conversion::monthly_to_annual,
            1,
        );
        registry.register("DAILY_TO_MONTHLY", payroll::conversion::daily_to_monthly, 1);
        registry.register("MONTHLY_TO_DAILY", payroll::conversion::monthly_to_daily, 1);
        registry.register(
            "HOURLY_TO_MONTHLY",
            payroll::conversion::hourly_to_monthly,
            1,
        );
        registry.register(
            "MONTHLY_TO_HOURLY",
            payroll::conversion::monthly_to_hourly,
            1,
        );
        registry.register(
            "PRORATE_BY_NATURAL_DAYS",
            payroll::conversion::prorate_by_natural_days,
            3,
        );
        registry.register(
            "PRORATE_BY_LEGAL_DAYS",
            payroll::conversion::prorate_by_legal_days,
            2,
        );
        registry.register(
            "PRORATE_BY_WORKDAYS",
            payroll::conversion::prorate_by_workdays,
            3,
        );
        registry.register(
            "CALC_ONBOARDING_SALARY",
            payroll::conversion::calc_onboarding_salary,
            4,
        );
        registry.register(
            "CALC_RESIGNATION_SALARY",
            payroll::conversion::calc_resignation_salary,
            4,
        );
        registry.register("CALC_14TH_SALARY", payroll::conversion::calc_14th_salary, 2);

        // Payroll functions - DateTime (12个)
        registry.register("CALC_NATURAL_DAYS", payroll::datetime::calc_natural_days, 2);
        registry.register(
            "GET_LEGAL_PAY_DAYS",
            payroll::datetime::get_legal_pay_days,
            0,
        );
        registry.register("CALC_WORKDAYS", payroll::datetime::calc_workdays, 2);
        registry.register("CALC_WEEKEND_DAYS", payroll::datetime::calc_weekend_days, 2);
        registry.register("CALC_HOLIDAY_DAYS", payroll::datetime::calc_holiday_days, 1);
        registry.register("IS_WORKDAY", payroll::datetime::is_workday, 2);
        registry.register("IS_WEEKEND", payroll::datetime::is_weekend, 1);
        registry.register("IS_HOLIDAY", payroll::datetime::is_holiday, 2);
        registry.register("CALC_WORK_HOURS", payroll::datetime::calc_work_hours, 1);
        registry.register(
            "CALC_MONTHLY_WORK_HOURS",
            payroll::datetime::calc_monthly_work_hours,
            0,
        );
        registry.register(
            "CALC_ANNUAL_WORKDAYS",
            payroll::datetime::calc_annual_workdays,
            0,
        );
        registry.register(
            "CALC_ANNUAL_PAY_DAYS",
            payroll::datetime::calc_annual_pay_days,
            0,
        );

        // Payroll functions - Statistics (6个)
        registry.register(
            "CALC_SALARY_AVERAGE",
            payroll::statistics::calc_salary_average,
            1,
        );
        registry.register(
            "CALC_SALARY_MEDIAN",
            payroll::statistics::calc_salary_median,
            1,
        );
        registry.register(
            "CALC_SALARY_RANGE",
            payroll::statistics::calc_salary_range,
            1,
        );
        registry.register("CALC_PERCENTILE", payroll::statistics::calc_percentile, 2);
        registry.register(
            "CALC_SALARY_STD_DEV",
            payroll::statistics::calc_salary_std_dev,
            1,
        );
        registry.register(
            "CALC_SALARY_DISTRIBUTION",
            payroll::statistics::calc_salary_distribution,
            2,
        );

        // Filesystem functions (根据权限注册)
        if permissions.filesystem_enabled {
            registry.register("READ_FILE", filesystem::read_file, 1);
            registry.register("WRITE_FILE", filesystem::write_file, 2);
            registry.register("APPEND_FILE", filesystem::append_file, 2);
            registry.register("DELETE_FILE", filesystem::delete_file, 1);
            registry.register("FILE_EXISTS", filesystem::file_exists, 1);
            registry.register("LIST_DIR", filesystem::list_dir, 1);
            registry.register("CREATE_DIR", filesystem::create_dir, 1);
        }

        // Network functions (根据权限注册)
        if permissions.network_enabled {
            registry.register("HTTP_GET", network::http_get, 1);
            registry.register("HTTP_POST", network::http_post, 2); // Variadic: 2-3 args
            registry.register("HTTP_PUT", network::http_put, 2); // Variadic: 2-3 args
            registry.register("HTTP_DELETE", network::http_delete, 1);
        }

        // Report functions - Data formatting (always enabled)
        registry.register("FORMAT_NUMBER", report::format_number, 1); // Variadic: 1-3 args
        registry.register("FORMAT_CURRENCY", report::format_currency, 1); // Variadic: 1-3 args
        registry.register("FORMAT_PERCENT", report::format_percent, 1); // Variadic: 1-2 args
        registry.register("FORMAT_DATE", report::format_date, 1); // Variadic: 1-2 args

        // Report functions - Excel operations (根据权限注册)
        if permissions.filesystem_enabled {
            registry.register("EXCEL_CREATE", report::excel_create, 0);
            registry.register("EXCEL_WRITE_CELL", report::excel_write_cell, 5);
            registry.register("EXCEL_WRITE_ROW", report::excel_write_row, 4);
            registry.register("EXCEL_WRITE_COLUMN", report::excel_write_column, 4);
            registry.register("EXCEL_WRITE_TABLE", report::excel_write_table, 5);
            registry.register("EXCEL_SAVE", report::excel_save, 2);
            registry.register("EXCEL_READ_SHEET", report::excel_read_sheet, 2);
            registry.register("EXCEL_READ_CELL", report::excel_read_cell, 4);
            registry.register("EXCEL_READ_RANGE", report::excel_read_range, 6);
            registry.register("EXCEL_GET_SHEETS", report::excel_get_sheets, 1);
        }

        registry
    }

    /// Register a built-in function
    fn register(&mut self, name: &str, func: BuiltInFn, arity: usize) {
        self.functions.insert(name.to_string(), (func, arity));
    }

    /// 注册带文档的函数
    #[allow(dead_code)]
    fn register_with_doc(&mut self, name: &str, func: BuiltInFn, arity: usize, doc: FunctionDoc) {
        self.functions.insert(name.to_string(), (func, arity));
        self.docs.insert(name.to_string(), doc);
    }

    /// Get a built-in function by name
    pub fn get(&self, name: &str) -> Option<(BuiltInFn, usize)> {
        self.functions.get(name).copied()
    }

    /// Check if a function exists
    pub fn has(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Get all function names
    pub fn names(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }

    /// 获取函数文档
    pub fn get_doc(&self, name: &str) -> Option<&FunctionDoc> {
        self.docs.get(name)
    }

    /// 获取所有文档
    pub fn all_docs(&self) -> &HashMap<String, FunctionDoc> {
        &self.docs
    }
}

impl Default for BuiltInRegistry {
    fn default() -> Self {
        Self::new()
    }
}
