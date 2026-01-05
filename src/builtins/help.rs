// src/builtins/help.rs
//! 帮助文档系统

use crate::evaluator::RuntimeError;
use crate::value::Value;
use std::collections::HashMap;
use std::sync::OnceLock;

/// 全局函数文档存储
static FUNCTION_DOCS: OnceLock<HashMap<String, FunctionDocData>> = OnceLock::new();

/// 函数文档数据
#[derive(Debug, Clone)]
pub struct FunctionDocData {
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

/// 初始化函数文档
pub fn init_docs() -> HashMap<String, FunctionDocData> {
    let mut docs = HashMap::new();

    // 精确计算函数
    docs.insert(
        "TO_FRACTION".to_string(),
        FunctionDocData {
            name: "TO_FRACTION".to_string(),
            description: "将数字转换为分数，用于精确计算".to_string(),
            params: vec![("value".to_string(), "要转换的数字或分数".to_string())],
            returns: "转换后的分数值".to_string(),
            example: Some("TO_FRACTION(0.5)  => 1/2\nTO_FRACTION(0.333)  => 333/1000".to_string()),
        },
    );

    docs.insert(
        "TO_FLOAT".to_string(),
        FunctionDocData {
            name: "TO_FLOAT".to_string(),
            description: "将分数转换为浮点数".to_string(),
            params: vec![("fraction".to_string(), "要转换的分数值".to_string())],
            returns: "转换后的浮点数".to_string(),
            example: Some("TO_FLOAT(TO_FRACTION(1/3))  => 0.333...".to_string()),
        },
    );

    docs.insert(
        "SIMPLIFY".to_string(),
        FunctionDocData {
            name: "SIMPLIFY".to_string(),
            description: "化简分数（约分）为最简形式".to_string(),
            params: vec![("fraction".to_string(), "要化简的分数".to_string())],
            returns: "化简后的最简分数".to_string(),
            example: Some("SIMPLIFY(TO_FRACTION(6/8))  => 3/4".to_string()),
        },
    );

    docs.insert(
        "FRAC_ADD".to_string(),
        FunctionDocData {
            name: "FRAC_ADD".to_string(),
            description: "分数加法运算，保证精确计算".to_string(),
            params: vec![
                ("a".to_string(), "第一个加数（数字或分数）".to_string()),
                ("b".to_string(), "第二个加数（数字或分数）".to_string()),
            ],
            returns: "两个分数相加的精确结果".to_string(),
            example: Some("FRAC_ADD(0.1, 0.2)  => 3/10 (而非 0.30000000000000004)".to_string()),
        },
    );

    docs.insert(
        "FRAC_SUB".to_string(),
        FunctionDocData {
            name: "FRAC_SUB".to_string(),
            description: "分数减法运算，保证精确计算".to_string(),
            params: vec![
                ("a".to_string(), "被减数（数字或分数）".to_string()),
                ("b".to_string(), "减数（数字或分数）".to_string()),
            ],
            returns: "两个分数相减的精确结果".to_string(),
            example: Some("FRAC_SUB(0.5, 0.25)  => 1/4".to_string()),
        },
    );

    docs.insert(
        "FRAC_MUL".to_string(),
        FunctionDocData {
            name: "FRAC_MUL".to_string(),
            description: "分数乘法运算，保证精确计算".to_string(),
            params: vec![
                ("a".to_string(), "第一个乘数（数字或分数）".to_string()),
                ("b".to_string(), "第二个乘数（数字或分数）".to_string()),
            ],
            returns: "两个分数相乘的精确结果".to_string(),
            example: Some("FRAC_MUL(0.1, 0.3)  => 3/100".to_string()),
        },
    );

    docs.insert(
        "FRAC_DIV".to_string(),
        FunctionDocData {
            name: "FRAC_DIV".to_string(),
            description: "分数除法运算，保证精确计算，除数不能为零".to_string(),
            params: vec![
                ("a".to_string(), "被除数（数字或分数）".to_string()),
                ("b".to_string(), "除数（数字或分数，不能为零）".to_string()),
            ],
            returns: "两个分数相除的精确结果".to_string(),
            example: Some("FRAC_DIV(1, 3)  => 1/3".to_string()),
        },
    );

    docs.insert(
        "NUMERATOR".to_string(),
        FunctionDocData {
            name: "NUMERATOR".to_string(),
            description: "获取分数的分子".to_string(),
            params: vec![("fraction".to_string(), "分数值".to_string())],
            returns: "分数的分子（浮点数）".to_string(),
            example: Some("NUMERATOR(TO_FRACTION(3/4))  => 3".to_string()),
        },
    );

    docs.insert(
        "DENOMINATOR".to_string(),
        FunctionDocData {
            name: "DENOMINATOR".to_string(),
            description: "获取分数的分母".to_string(),
            params: vec![("fraction".to_string(), "分数值".to_string())],
            returns: "分数的分母（浮点数）".to_string(),
            example: Some("DENOMINATOR(TO_FRACTION(3/4))  => 4".to_string()),
        },
    );

    docs.insert(
        "GCD".to_string(),
        FunctionDocData {
            name: "GCD".to_string(),
            description: "计算两个整数的最大公约数（Greatest Common Divisor）".to_string(),
            params: vec![
                ("a".to_string(), "第一个整数".to_string()),
                ("b".to_string(), "第二个整数".to_string()),
            ],
            returns: "两个数的最大公约数".to_string(),
            example: Some("GCD(12, 18)  => 6\nGCD(7, 13)  => 1".to_string()),
        },
    );

    docs.insert(
        "LCM".to_string(),
        FunctionDocData {
            name: "LCM".to_string(),
            description: "计算两个整数的最小公倍数（Least Common Multiple）".to_string(),
            params: vec![
                ("a".to_string(), "第一个整数".to_string()),
                ("b".to_string(), "第二个整数".to_string()),
            ],
            returns: "两个数的最小公倍数".to_string(),
            example: Some("LCM(4, 6)  => 12\nLCM(3, 5)  => 15".to_string()),
        },
    );

    // I/O 函数
    docs.insert(
        "PRINT".to_string(),
        FunctionDocData {
            name: "PRINT".to_string(),
            description: "输出内容到控制台（不换行）".to_string(),
            params: vec![("value".to_string(), "要输出的值".to_string())],
            returns: "null".to_string(),
            example: Some("PRINT(\"Hello\")  => 输出: Hello".to_string()),
        },
    );

    docs.insert(
        "PRINTLN".to_string(),
        FunctionDocData {
            name: "PRINTLN".to_string(),
            description: "输出内容到控制台并换行".to_string(),
            params: vec![("value".to_string(), "要输出的值".to_string())],
            returns: "null".to_string(),
            example: Some("PRINTLN(\"Hello World\")  => 输出: Hello World\\n".to_string()),
        },
    );

    docs.insert(
        "INPUT".to_string(),
        FunctionDocData {
            name: "INPUT".to_string(),
            description: "从控制台读取用户输入".to_string(),
            params: vec![("prompt".to_string(), "提示信息".to_string())],
            returns: "用户输入的字符串".to_string(),
            example: Some("name = INPUT(\"请输入姓名: \")".to_string()),
        },
    );

    // 数组函数
    docs.insert(
        "RANGE".to_string(),
        FunctionDocData {
            name: "RANGE".to_string(),
            description: "生成数字范围数组".to_string(),
            params: vec![(
                "end".to_string(),
                "结束值（可选: start, end, step)".to_string(),
            )],
            returns: "数字数组".to_string(),
            example: Some("RANGE(5)  => [0,1,2,3,4]\nRANGE(2, 8, 2)  => [2,4,6]".to_string()),
        },
    );

    docs.insert(
        "LEN".to_string(),
        FunctionDocData {
            name: "LEN".to_string(),
            description: "获取数组、字符串或字典的长度".to_string(),
            params: vec![("value".to_string(), "数组、字符串或字典".to_string())],
            returns: "长度值".to_string(),
            example: Some("LEN([1,2,3])  => 3\nLEN(\"hello\")  => 5".to_string()),
        },
    );

    docs.insert(
        "PUSH".to_string(),
        FunctionDocData {
            name: "PUSH".to_string(),
            description: "向数组末尾添加元素".to_string(),
            params: vec![
                ("array".to_string(), "目标数组".to_string()),
                ("element".to_string(), "要添加的元素".to_string()),
            ],
            returns: "新数组".to_string(),
            example: Some("PUSH([1,2], 3)  => [1,2,3]".to_string()),
        },
    );

    docs.insert(
        "POP".to_string(),
        FunctionDocData {
            name: "POP".to_string(),
            description: "移除并返回数组最后一个元素".to_string(),
            params: vec![("array".to_string(), "目标数组".to_string())],
            returns: "被移除的元素".to_string(),
            example: Some("POP([1,2,3])  => 3".to_string()),
        },
    );

    docs.insert(
        "REVERSE".to_string(),
        FunctionDocData {
            name: "REVERSE".to_string(),
            description: "反转数组元素顺序".to_string(),
            params: vec![("array".to_string(), "要反转的数组".to_string())],
            returns: "反转后的数组".to_string(),
            example: Some("REVERSE([1,2,3])  => [3,2,1]".to_string()),
        },
    );

    docs.insert(
        "SORT".to_string(),
        FunctionDocData {
            name: "SORT".to_string(),
            description: "对数组进行排序".to_string(),
            params: vec![("array".to_string(), "要排序的数组".to_string())],
            returns: "排序后的数组".to_string(),
            example: Some("SORT([3,1,2])  => [1,2,3]".to_string()),
        },
    );

    docs.insert(
        "SUM".to_string(),
        FunctionDocData {
            name: "SUM".to_string(),
            description: "计算数组元素之和".to_string(),
            params: vec![("array".to_string(), "数字数组".to_string())],
            returns: "总和".to_string(),
            example: Some("SUM([1,2,3,4])  => 10".to_string()),
        },
    );

    docs.insert(
        "MAX".to_string(),
        FunctionDocData {
            name: "MAX".to_string(),
            description: "找出数组中的最大值".to_string(),
            params: vec![("array".to_string(), "数字数组".to_string())],
            returns: "最大值".to_string(),
            example: Some("MAX([1,5,3,2])  => 5".to_string()),
        },
    );

    docs.insert(
        "MIN".to_string(),
        FunctionDocData {
            name: "MIN".to_string(),
            description: "找出数组中的最小值".to_string(),
            params: vec![("array".to_string(), "数字数组".to_string())],
            returns: "最小值".to_string(),
            example: Some("MIN([1,5,3,2])  => 1".to_string()),
        },
    );

    // 字符串函数
    docs.insert(
        "SPLIT".to_string(),
        FunctionDocData {
            name: "SPLIT".to_string(),
            description: "按分隔符分割字符串".to_string(),
            params: vec![
                ("string".to_string(), "要分割的字符串".to_string()),
                ("separator".to_string(), "分隔符".to_string()),
            ],
            returns: "字符串数组".to_string(),
            example: Some("SPLIT(\"a,b,c\", \",\")  => [\"a\",\"b\",\"c\"]".to_string()),
        },
    );

    docs.insert(
        "UPPER".to_string(),
        FunctionDocData {
            name: "UPPER".to_string(),
            description: "将字符串转换为大写".to_string(),
            params: vec![("string".to_string(), "源字符串".to_string())],
            returns: "大写字符串".to_string(),
            example: Some("UPPER(\"hello\")  => \"HELLO\"".to_string()),
        },
    );

    docs.insert(
        "LOWER".to_string(),
        FunctionDocData {
            name: "LOWER".to_string(),
            description: "将字符串转换为小写".to_string(),
            params: vec![("string".to_string(), "源字符串".to_string())],
            returns: "小写字符串".to_string(),
            example: Some("LOWER(\"HELLO\")  => \"hello\"".to_string()),
        },
    );

    docs.insert(
        "TRIM".to_string(),
        FunctionDocData {
            name: "TRIM".to_string(),
            description: "去除字符串首尾空白字符".to_string(),
            params: vec![("string".to_string(), "源字符串".to_string())],
            returns: "去除空白后的字符串".to_string(),
            example: Some("TRIM(\"  hello  \")  => \"hello\"".to_string()),
        },
    );

    // 数学函数 - 基础
    docs.insert(
        "ABS".to_string(),
        FunctionDocData {
            name: "ABS".to_string(),
            description: "计算绝对值".to_string(),
            params: vec![("x".to_string(), "数字".to_string())],
            returns: "绝对值".to_string(),
            example: Some("ABS(-5)  => 5".to_string()),
        },
    );

    docs.insert(
        "SQRT".to_string(),
        FunctionDocData {
            name: "SQRT".to_string(),
            description: "计算平方根".to_string(),
            params: vec![("x".to_string(), "数字".to_string())],
            returns: "平方根".to_string(),
            example: Some("SQRT(16)  => 4".to_string()),
        },
    );

    docs.insert(
        "POW".to_string(),
        FunctionDocData {
            name: "POW".to_string(),
            description: "计算幂次方".to_string(),
            params: vec![
                ("base".to_string(), "底数".to_string()),
                ("exponent".to_string(), "指数".to_string()),
            ],
            returns: "幂运算结果".to_string(),
            example: Some("POW(2, 10)  => 1024".to_string()),
        },
    );

    docs.insert(
        "FLOOR".to_string(),
        FunctionDocData {
            name: "FLOOR".to_string(),
            description: "向下取整".to_string(),
            params: vec![("x".to_string(), "数字".to_string())],
            returns: "不大于x的最大整数".to_string(),
            example: Some("FLOOR(3.7)  => 3".to_string()),
        },
    );

    docs.insert(
        "CEIL".to_string(),
        FunctionDocData {
            name: "CEIL".to_string(),
            description: "向上取整".to_string(),
            params: vec![("x".to_string(), "数字".to_string())],
            returns: "不小于x的最小整数".to_string(),
            example: Some("CEIL(3.2)  => 4".to_string()),
        },
    );

    docs.insert(
        "ROUND".to_string(),
        FunctionDocData {
            name: "ROUND".to_string(),
            description: "四舍五入到最接近的整数".to_string(),
            params: vec![("x".to_string(), "数字".to_string())],
            returns: "四舍五入后的整数".to_string(),
            example: Some("ROUND(3.6)  => 4".to_string()),
        },
    );

    // 数学函数 - 三角函数
    docs.insert(
        "SIN".to_string(),
        FunctionDocData {
            name: "SIN".to_string(),
            description: "计算正弦值（弧度）".to_string(),
            params: vec![("x".to_string(), "角度（弧度）".to_string())],
            returns: "正弦值".to_string(),
            example: Some("SIN(PI()/2)  => 1".to_string()),
        },
    );

    docs.insert(
        "COS".to_string(),
        FunctionDocData {
            name: "COS".to_string(),
            description: "计算余弦值（弧度）".to_string(),
            params: vec![("x".to_string(), "角度（弧度）".to_string())],
            returns: "余弦值".to_string(),
            example: Some("COS(0)  => 1".to_string()),
        },
    );

    docs.insert(
        "TAN".to_string(),
        FunctionDocData {
            name: "TAN".to_string(),
            description: "计算正切值（弧度）".to_string(),
            params: vec![("x".to_string(), "角度（弧度）".to_string())],
            returns: "正切值".to_string(),
            example: Some("TAN(PI()/4)  => 1".to_string()),
        },
    );

    // 数学常数
    docs.insert(
        "PI".to_string(),
        FunctionDocData {
            name: "PI".to_string(),
            description: "圆周率 π ≈ 3.14159...".to_string(),
            params: vec![],
            returns: "π 的值".to_string(),
            example: Some("PI()  => 3.141592653589793".to_string()),
        },
    );

    docs.insert(
        "E".to_string(),
        FunctionDocData {
            name: "E".to_string(),
            description: "自然常数 e ≈ 2.71828...".to_string(),
            params: vec![],
            returns: "e 的值".to_string(),
            example: Some("E()  => 2.718281828459045".to_string()),
        },
    );

    // 统计函数
    docs.insert(
        "MEAN".to_string(),
        FunctionDocData {
            name: "MEAN".to_string(),
            description: "计算数组的平均值".to_string(),
            params: vec![("array".to_string(), "数字数组".to_string())],
            returns: "平均值".to_string(),
            example: Some("MEAN([1,2,3,4,5])  => 3".to_string()),
        },
    );

    docs.insert(
        "MEDIAN".to_string(),
        FunctionDocData {
            name: "MEDIAN".to_string(),
            description: "计算数组的中位数".to_string(),
            params: vec![("array".to_string(), "数字数组".to_string())],
            returns: "中位数".to_string(),
            example: Some("MEDIAN([1,2,3,4,5])  => 3".to_string()),
        },
    );

    docs.insert(
        "VARIANCE".to_string(),
        FunctionDocData {
            name: "VARIANCE".to_string(),
            description: "计算数组的方差".to_string(),
            params: vec![("array".to_string(), "数字数组".to_string())],
            returns: "方差".to_string(),
            example: Some("VARIANCE([1,2,3,4,5])  => 2".to_string()),
        },
    );

    docs.insert(
        "STD".to_string(),
        FunctionDocData {
            name: "STD".to_string(),
            description: "计算数组的标准差".to_string(),
            params: vec![("array".to_string(), "数字数组".to_string())],
            returns: "标准差".to_string(),
            example: Some("STD([1,2,3,4,5])  => 1.414...".to_string()),
        },
    );

    // 类型转换函数
    docs.insert(
        "TYPE".to_string(),
        FunctionDocData {
            name: "TYPE".to_string(),
            description: "获取值的类型".to_string(),
            params: vec![("value".to_string(), "任意值".to_string())],
            returns: "类型名称字符串".to_string(),
            example: Some("TYPE(123)  => \"Number\"\nTYPE(\"hello\")  => \"String\"".to_string()),
        },
    );

    docs.insert(
        "TO_STRING".to_string(),
        FunctionDocData {
            name: "TO_STRING".to_string(),
            description: "将值转换为字符串".to_string(),
            params: vec![("value".to_string(), "要转换的值".to_string())],
            returns: "字符串".to_string(),
            example: Some("TO_STRING(123)  => \"123\"".to_string()),
        },
    );

    docs.insert(
        "TO_NUMBER".to_string(),
        FunctionDocData {
            name: "TO_NUMBER".to_string(),
            description: "将字符串转换为数字".to_string(),
            params: vec![("string".to_string(), "数字字符串".to_string())],
            returns: "数字".to_string(),
            example: Some("TO_NUMBER(\"123\")  => 123".to_string()),
        },
    );

    docs
}

/// HELP 函数实现
///
/// 用法：
/// - HELP() - 列出所有可用函数
/// - HELP("函数名") - 显示特定函数的详细文档
pub fn help(args: &[Value]) -> Result<Value, RuntimeError> {
    // 初始化文档（只执行一次）
    let docs = FUNCTION_DOCS.get_or_init(init_docs);

    if args.is_empty() {
        // 列出所有函数
        let mut output = String::from("=== Aether 内置函数列表 ===\n\n");

        // 按类别组织函数
        let categories = vec![
            (
                "精确计算",
                vec![
                    "TO_FRACTION",
                    "TO_FLOAT",
                    "SIMPLIFY",
                    "FRAC_ADD",
                    "FRAC_SUB",
                    "FRAC_MUL",
                    "FRAC_DIV",
                    "NUMERATOR",
                    "DENOMINATOR",
                    "GCD",
                    "LCM",
                ],
            ),
            ("输入输出", vec!["PRINT", "PRINTLN", "INPUT"]),
            (
                "数组操作",
                vec![
                    "RANGE", "LEN", "PUSH", "POP", "REVERSE", "SORT", "SUM", "MAX", "MIN",
                ],
            ),
            (
                "字符串操作",
                vec![
                    "SPLIT",
                    "UPPER",
                    "LOWER",
                    "TRIM",
                    "CONTAINS",
                    "STARTS_WITH",
                    "ENDS_WITH",
                    "REPLACE",
                    "REPEAT",
                    "JOIN",
                ],
            ),
            (
                "数学函数 - 基础",
                vec!["ABS", "SQRT", "POW", "FLOOR", "CEIL", "ROUND"],
            ),
            (
                "数学函数 - 三角",
                vec!["SIN", "COS", "TAN", "ASIN", "ACOS", "ATAN", "ATAN2"],
            ),
            ("数学函数 - 对数", vec!["LOG", "LN", "LOG2", "EXP", "EXP2"]),
            ("数学常数", vec!["PI", "E", "TAU", "PHI"]),
            (
                "统计分析",
                vec!["MEAN", "MEDIAN", "VARIANCE", "STD", "QUANTILE"],
            ),
            (
                "向量运算",
                vec!["DOT", "NORM", "CROSS", "DISTANCE", "NORMALIZE"],
            ),
            (
                "矩阵运算",
                vec!["MATMUL", "TRANSPOSE", "DETERMINANT", "INVERSE"],
            ),
            ("线性回归", vec!["LINEAR_REGRESSION"]),
            ("概率分布", vec!["NORMAL_PDF", "NORMAL_CDF", "POISSON_PMF"]),
            (
                "精度计算",
                vec![
                    "ROUND_TO",
                    "ADD_WITH_PRECISION",
                    "SUB_WITH_PRECISION",
                    "MUL_WITH_PRECISION",
                    "DIV_WITH_PRECISION",
                ],
            ),
            ("类型转换", vec!["TYPE", "TO_STRING", "TO_NUMBER"]),
            ("字典操作", vec!["KEYS", "VALUES", "HAS", "MERGE"]),
        ];

        for (category, funcs) in categories {
            output.push_str(&format!("【{}】\n", category));
            for func_name in funcs {
                if let Some(doc) = docs.get(func_name) {
                    output.push_str(&format!("  {} - {}\n", doc.name, doc.description));
                }
            }
            output.push('\n');
        }

        output.push_str("使用 HELP(\"函数名\") 查看详细文档\n");
        output.push_str("例如: HELP(\"TO_FRACTION\")\n");

        Ok(Value::String(output))
    } else if args.len() == 1 {
        // 显示特定函数的文档
        match &args[0] {
            Value::String(func_name) => {
                let func_name_upper = func_name.to_uppercase();

                if let Some(doc) = docs.get(&func_name_upper) {
                    let mut output = "=".repeat(50);
                    output.push_str(&format!("\n函数: {}\n", doc.name));
                    output.push_str(&"=".repeat(50));
                    output.push_str(&format!("\n\n描述:\n  {}\n\n", doc.description));

                    if !doc.params.is_empty() {
                        output.push_str("参数:\n");
                        for (param_name, param_desc) in &doc.params {
                            output.push_str(&format!("  - {}: {}\n", param_name, param_desc));
                        }
                        output.push('\n');
                    }

                    output.push_str(&format!("返回值:\n  {}\n\n", doc.returns));

                    if let Some(example) = &doc.example {
                        output.push_str("示例:\n");
                        for line in example.lines() {
                            output.push_str(&format!("  {}\n", line));
                        }
                    }

                    output.push_str(&"=".repeat(50));
                    output.push('\n');

                    Ok(Value::String(output))
                } else {
                    Err(RuntimeError::InvalidOperation(format!(
                        "函数 '{}' 不存在。使用 HELP() 查看所有可用函数。",
                        func_name
                    )))
                }
            }
            _ => Err(RuntimeError::TypeErrorDetailed {
                expected: "String".to_string(),
                got: format!("{:?}", args[0]),
            }),
        }
    } else {
        Err(RuntimeError::WrongArity {
            expected: 0, // or 1
            got: args.len(),
        })
    }
}
