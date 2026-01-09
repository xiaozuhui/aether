
阶段 3.2：结构化 TRACE 事件（P0 - 核心，约4-6天）
目标：增强 TRACE 系统，支持事件类型、优先级、分类和过滤，提供生产级可观测性。
背景分析
当前状态（基于代码探索）：
✅ 基础 TRACE API：TRACE(x, y, z) 和 TRACE("label", x, y)
✅ 使用 VecDeque<TraceEntry> 存储，最多1024条（ring buffer）
✅ TraceEntry 包含：values: Vec<Value> 和可选 label: Option<String>
❌ 无事件类型（info/warn/error/debug）
❌ 无优先级（高/中/低）
❌ 无时间戳
❌ 无分类或标签系统
❌ 无过滤能力
需求来源：
调试支持：区分不同严重级别的事件
性能分析：按类型过滤，减少噪音
监控集成：导出到监控系统（Prometheus/DataDog）
审计追踪：记录关键操作路径
实现步骤

1. 定义结构化事件模型（1天）
修改文件：src/evaluator.rs（或新建 src/runtime/trace.rs）
增强 TraceEntry 结构：

use std::time::Instant;

# [derive(Debug, Clone)]

pub struct TraceEntry {
    /// 时间戳
    pub timestamp: Instant,
    /// 事件级别
    pub level: TraceLevel,
    /// 事件类型/类别
    pub category: String,
    /// 标签
    pub label: Option<String>,
    /// 追踪的值
    pub values: Vec<Value>,
    /// 源代码位置（文件名:行号）
    pub location: Option<String>,
}

# [derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]

pub enum TraceLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}

impl TraceLevel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "debug" => Some(Self::Debug),
            "info" => Some(Self::Info),
            "warn" | "warning" => Some(Self::Warn),
            "error" => Some(Self::Error),
            _ => None,
        }
    }
}
2. 扩展 TRACE API（1-2天）
修改文件：src/builtins/debug.rs
新增函数：

// 结构化 TRACE
pub fn trace_with_level(args: &[Value]) -> Result<Value, RuntimeError>;
// TRACE_DEBUG("category", x, y)
pub fn trace_debug(args: &[Value]) -> Result<Value, RuntimeError>;
// TRACE_INFO("category", x, y)
pub fn trace_info(args: &[Value]) -> Result<Value, RuntimeError>;
// TRACE_WARN("category", x, y)
pub fn trace_warn(args: &[Value]) -> Result<Value, RuntimeError>;
// TRACE_ERROR("category", x, y)
pub fn trace_error(args: &[Value]) -> Result<Value, RuntimeError>;
保持向后兼容：

// 旧 API 继续工作，默认为 Info 级别
pub fn trace(args: &[Value]) -> Result<Value, RuntimeError> {
    // 原有逻辑
}

// 新 API 支持级别
pub fn trace_with_level(args: &[Value]) -> Result<Value, RuntimeError> {
    let level_str = get_string(&args[0])?;
    let level = TraceLevel::from_str(&level_str)
        .ok_or_else(|| RuntimeError::CustomError(format!("Invalid trace level: {}", level_str)))?;

    let category = get_string(&args[1])?;
    let values = args[2..].to_vec();

    // 记录带级别的事件
    // ...
}
3. 实现事件过滤和查询（1-2天）
修改文件：src/evaluator.rs
在 Evaluator 添加过滤方法：

impl Evaluator {
    /// 获取所有 TRACE 记录
    pub fn trace_records(&self) -> &VecDeque<TraceEntry>;

    /// 按级别过滤
    pub fn trace_by_level(&self, level: TraceLevel) -> Vec<&TraceEntry> {
        self.trace_records()
            .iter()
            .filter(|e| e.level == level)
            .collect()
    }

    /// 按类别过滤
    pub fn trace_by_category(&self, category: &str) -> Vec<&TraceEntry> {
        self.trace_records()
            .iter()
            .filter(|e| e.category == category)
            .collect()
    }

    /// 按标签过滤
    pub fn trace_by_label(&self, label: &str) -> Vec<&TraceEntry> {
        self.trace_records()
            .iter()
            .filter(|e| e.label.as_deref() == Some(label))
            .collect()
    }

    /// 按时间范围过滤
    pub fn trace_since(&self, since: Instant) -> Vec<&TraceEntry> {
        self.trace_records()
            .iter()
            .filter(|e| e.timestamp >= since)
            .collect()
    }

    /// 组合过滤（链式）
    pub fn trace_filter(&self, filter: &TraceFilter) -> Vec<&TraceEntry> {
        self.trace_records()
            .iter()
            .filter(|e| {
                if let Some(level) = filter.min_level {
                    if e.level < level { return false; }
                }
                if let Some(ref category) = filter.category {
                    if &e.category != category { return false; }
                }
                if let Some(ref label) = filter.label {
                    if e.label.as_deref() != Some(label) { return false; }
                }
                true
            })
            .collect()
    }
}

# [derive(Debug, Default)]

pub struct TraceFilter {
    pub min_level: Option<TraceLevel>,
    pub category: Option<String>,
    pub label: Option<String>,
    pub since: Option<Instant>,
}
4. 集成到 Aether API（<1天）
修改文件：src/lib.rs
添加查询方法：

impl Aether {
    /// 获取所有 TRACE 记录
    pub fn trace_records(&self) -> Vec<TraceEntry>;

    /// 按级别过滤
    pub fn trace_by_level(&self, level: TraceLevel) -> Vec<TraceEntry>;

    /// 按类别过滤
    pub fn trace_by_category(&self, category: &str) -> Vec<TraceEntry>;

    /// 清空 TRACE 记录
    pub fn clear_trace(&mut self);

    /// 设置 TRACE 缓冲区大小
    pub fn set_trace_buffer_size(&mut self, size: usize);
}
5. 添加示例和文档（<1天）
修改文件：examples/trace_demo.aether
演示新 API：

# 旧 API（继续工作）

TRACE("basic", 1, 2, 3)

# 新 API（带级别）

TRACE_INFO("user_action", "login", USER_ID)
TRACE_WARN("api_call", "slow_response", 5000)
TRACE_ERROR("database", "connection_failed")

# 自定义类别

TRACE_DEBUG("calculation", "intermediate", RESULT)
验收标准
✅ 支持事件级别（Debug/Info/Warn/Error）
✅ 支持事件分类和标签
✅ 支持时间戳记录
✅ 支持多种过滤方式（级别/类别/标签/时间）
✅ 向后兼容旧 API
✅ 性能开销 < 3%（主要在记录时）
风险与缓解
风险1：TRACE 过多可能影响性能
缓解：ring buffer 限制（已存在）；提供配置关闭 TRACE；在 release 构建时可完全禁用
风险2：时间戳精度问题
缓解：使用 Instant（高精度），仅用于相对时间；不依赖系统时钟
----

阶段 3.3：规则调试体验（P1 - 重要，约4-6天）
目标：提供可视化调试工具，降低规则调试成本，提升开发体验。
背景分析
当前状态（基于代码探索）：
✅ 完整的调用栈（CallStack 和 CallFrame）
✅ 错误消息包含文件名、行号、调用栈
✅ CLI 支持 --debug 模式，打印 AST
✅ TRACE 提供基础调试能力
❌ 无步进执行（step over/into/out）
❌ 无断点支持
❌ 无变量监视
❌ 无调用栈可视化
需求来源：
复杂规则调试：大型规则集难以定位问题
教学场景：帮助初学者理解执行流程
性能分析：识别热点函数和路径
实现步骤

1. 定义调试器 API（1-2天）
新建文件：src/runtime/debugger.rs
实现 Debugger trait 和基础实现：

pub trait Debugger {
    /// 在每个语句执行前调用
    fn on_stmt_exec(&mut self, stmt: &Stmt, env: &Environment) -> DebugAction;

    /// 在函数调用前调用
    fn on_call(&mut self, func: &str, args: &[Value]) -> DebugAction;

    /// 在函数返回前调用
    fn on_return(&mut self, func: &str, result: &Value);

    /// 在错误发生时调用
    fn on_error(&mut self, error: &RuntimeError);
}

pub enum DebugAction {
    Continue,        // 继续执行
    Step,            // 单步执行
    StepIn,          // 步入函数
    StepOut,         // 步出函数
    Pause,           // 暂停执行
    Break,           // 中断执行
}

pub struct PrintDebugger {
    step_mode: bool,
    break_on_errors: bool,
}

impl Debugger for PrintDebugger {
    fn on_stmt_exec(&mut self, stmt: &Stmt, _env: &Environment) -> DebugAction {
        if self.step_mode {
            println!("Executing: {:?}", stmt);
            DebugAction::Step
        } else {
            DebugAction::Continue
        }
    }

    fn on_error(&mut self, error: &RuntimeError) {
        if self.break_on_errors {
            println!("ERROR: {}", error);
        }
    }
}
2. 集成调试器到 Evaluator（1-2天）
修改文件：src/evaluator.rs
在 Evaluator 添加调试器字段：

pub struct Evaluator {
    // ... 现有字段
    debugger: Option<Box<dyn Debugger>>,
}
在关键点调用调试器：

fn eval_stmt(&mut self, stmt: &Stmt) -> Result<Value, RuntimeError> {
    // 检查调试器
    if let Some(ref debugger) = self.debugger {
        match debugger.on_stmt_exec(stmt, &self.env) {
            DebugAction::Continue => {},
            DebugAction::Step => { /*等待用户输入 */ },
            DebugAction::Pause => { /* 暂停执行*/ },
            DebugAction::Break => return Err(RuntimeError::DebugBreak),
            _ => {},
        }
    }

    // ... 原有求值逻辑
}
3. 实现 REPL 调试模式（1-2天）
修改文件：src/cli.rs
添加调试命令：

pub enum ReplCommand {
    Eval(String),
    DebugStep,      // step
    DebugStepIn,    // stepin
    DebugStepOut,   // stepout
    DebugContinue,  // continue
    DebugBreakpoint(String), // break <line>
    DebugWatch(String),      // watch <var>
    DebugTrace,      // trace (启用 TRACE 输出)
    DebugUntrace,    // untrace
    Help,
    Exit,
}

// 在 REPL 中解析调试命令
fn parse_debug_command(input: &str) -> Option<ReplCommand> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }

    match parts[0] {
        ":step" | ":s" => Some(ReplCommand::DebugStep),
        ":stepin" | ":si" => Some(ReplCommand::DebugStepIn),
        ":continue" | ":c" => Some(ReplCommand::DebugContinue),
        ":break" | ":b" => {
            if parts.len() > 1 {
                Some(ReplCommand::DebugBreakpoint(parts[1].to_string()))
            } else {
                None
            }
        },
        ":watch" | ":w" => {
            if parts.len() > 1 {
                Some(ReplCommand::DebugWatch(parts[1].to_string()))
            } else {
                None
            }
        },
        ":trace" => Some(ReplCommand::DebugTrace),
        ":untrace" => Some(ReplCommand::DebugUntrace),
        _ => None,
    }
}
4. 实现变量监视和调用栈查看（1天）
修改文件：src/evaluator.rs
添加查询方法：

impl Evaluator {
    /// 获取当前调用栈
    pub fn call_stack(&self) -> &CallStack;

    /// 获取当前作用域的所有变量
    pub fn current_variables(&self) -> HashMap<String, Value>;

    /// 获取指定变量的值
    pub fn get_variable(&self, name: &str) -> Option<&Value>;
}
在 REPL 中集成：

// :vars 命令
fn print_variables(evaluator: &Evaluator) {
    let vars = evaluator.current_variables();
    for (name, value) in vars {
        println!("{} = {:?}", name, value);
    }
}

// :stack 命令
fn print_call_stack(evaluator: &Evaluator) {
    let stack = evaluator.call_stack();
    for (i, frame) in stack.frames().iter().enumerate() {
        println!("{}: {} @ {}", i, frame.function_name(), frame.location());
    }
}
5. 编写文档和示例（<1天）
新建文件：docs/DEBUGGING.md
内容：
调试模式使用
REPL 调试命令
调试示例
最佳实践
验收标准
✅ 支持单步执行（step/stepin/stepout）
✅ 支持查看调用栈
✅ 支持查看变量
✅ 支持断点（基础实现）
✅ REPL 调试命令完整
✅ 文档清晰，示例完整
风险与缓解
风险1：调试器实现复杂度高
缓解：分阶段实现，先实现基础功能（步进、变量查看），断点作为后续优化
风险2：性能影响
缓解：调试器仅在启用时生效；生产环境不启用
风险3：REPL 交互复杂
缓解：参考其他调试器（gdb/lldb）的设计，保持命令简洁

-----

阶段 3.4：数据结构增强（P2 - 通用语言，约5-7天）
目标：实现 Record 和 Enum 数据结构，提升语言表达能力。
背景分析
当前状态：
✅ 基础类型：Number, String, Boolean, Null
✅ 复合类型：List (数组), Map (字典)
✅ 函数：支持一等函数和闭包
❌ 无 Record（结构体/命名元组）
❌ 无 Enum（枚举/标签联合）
实现步骤

1. 设计 Record 语法和语义（2天）
修改文件：src/parser.rs
添加 Record 语法：

# 定义 Record 类型

Type PERSON = Record {
    NAME: String,
    AGE: Number,
    EMAIL: String
}

# 创建 Record 实例

Let P PERSON("Alice", 30, "<alice@example.com>")

# 访问字段

P.NAME
P.AGE
实现 Value::Record 变体：

pub enum Value {
    // ... 现有变体
    Record {
        type_name: String,
        fields: HashMap<String, Value>,
    },
}
2. 设计 Enum 语法和语义（2-3天）
修改文件：src/parser.rs
添加 Enum 语法：

# 定义 Enum 类型

Enum OPTION {
    Some(T),
    None
}

Enum RESULT {
    Ok(T),
    Error(E)
}

# 使用 Enum

Let X OPTION.Some(42)
Let Y OPTION.None

# 模式匹配

Match X {
    OPTION.Some(V) => V,
    OPTION.None => 0
}
实现 Value::Enum 变体：

pub enum Value {
    // ... 现有变体
    Enum {
        type_name: String,
        variant: String,
        value: Option<Box<Value>>,
    },
}
3. 实现模式匹配（2天）
修改文件：src/evaluator.rs
实现 Match 表达式：

Stmt::Match { expr, arms } => {
    let value = self.eval_expr(expr)?;
    for arm in arms {
        if self.match_pattern(&arm.pattern, &value)? {
            return self.eval_block(&arm.body);
        }
    }
    Err(RuntimeError::CustomError("No matching pattern".to_string()))
}
验收标准
✅ 支持Record 定义和使用
✅ 支持Enum 定义和使用
✅ 支持模式匹配
✅ 类型安全（运行时检查）
✅ 文档完整

-----

阶段 3.5：Formatter + 最小 LSP（P2 - 工具链，约7-10天）
目标：提供代码格式化和语言服务器支持，提升开发体验。
背景分析
当前状态：
✅ 完整的 Lexer 和 Parser
✅ AST 结构完整
❌ 无 Formatter
❌ 无 LSP 实现
实现步骤

1. 实现 Formatter（3-4天）
新建文件：src/fmt/mod.rs, src/fmt/formatter.rs
实现 AST 遍历和格式化：

pub struct Formatter {
    indent_size: usize,
    max_line_width: usize,
}

impl Formatter {
    pub fn format(&self, ast: &Program) -> String {
        // 遍历 AST，生成格式化代码
    }
}
CLI 支持：

aether fmt script.aether          # 格式化文件
aether fmt --check script.aether  # 检查格式
aether fmt --stdin                # 从 stdin 读取
2. 实现最小 LSP（4-6天）
新建文件：src/lsp/server.rs
实现 LSP 核心功能：
initialize: 服务器初始化
textDocument/didOpen: 文档打开
textDocument/didChange: 文档变更
textDocument/completion: 自动补全
textDocument/hover: 悬停提示
textDocument/definition: 跳转到定义
textDocument/diagnostic: 语法和错误检查
支持功能：
语法高亮（基于 token 类型）
自动补全（关键字、函数名、变量名）
诊断信息（语法错误、运行时错误）
格式化（调用 Formatter）
验收标准
✅ Formatter 支持所有语言特性
✅ Formatter 可配置（缩进、行宽）
✅ LSP 支持自动补全
✅ LSP 支持诊断信息
✅ VSCode 插件可用
实施优先级和时间估算
阶段 任务 预计时间 优先级 路线
3.1 执行限制 5-7天 P0 DSL 产品
3.2 结构化 TRACE 4-6天 P0 DSL 产品
3.3 规则调试体验 4-6天 P1 DSL 产品
3.4 数据结构增强 5-7天 P2 通用语言
3.5 Formatter + LSP 7-10天 P2 工具链
总计  25-36天  
推荐实施顺序
第一优先级（DSL 产品核心）：
3.1 执行限制（5-7天）
3.2 结构化 TRACE（4-6天）
小计：9-13天
第二优先级（开发体验）：
3.3 规则调试体验（4-6天）
小计：4-6天
第三优先级（语言完善）：
3.4 数据结构增强（5-7天）
3.5 Formatter + LSP（7-10天）
小计：12-17天
关键文件清单
需要新建的文件（阶段3）
执行限制：
src/runtime/limits.rs - 执行限制配置和错误类型
tests/execution_limits_tests.rs - 执行限制测试
结构化 TRACE： 3. src/runtime/trace.rs - TRACE 事件模型和过滤 4. examples/trace_demo.aether - TRACE 功能演示 规则调试： 5. src/runtime/debugger.rs - 调试器 trait 和实现 6. docs/DEBUGGING.md - 调试指南 数据结构增强： 7. src/types/record.rs - Record 类型实现 8. src/types/enum.rs - Enum 类型实现 Formatter + LSP： 9. src/fmt/mod.rs - Formatter 模块 10. src/fmt/formatter.rs - Formatter 实现 11. src/lsp/server.rs - LSP 服务器 12. editors/vscode/aether-lsp/ - VSCode 插件
需要修改的核心文件
src/evaluator.rs - 集成执行限制、调试器
src/builtins/debug.rs - 扩展 TRACE API
src/parser.rs - 支持 Record/Enum 语法
src/cli.rs - 支持调试模式、格式化命令
src/lib.rs - 导出新 API
src/value.rs - 添加 Record/Enum 变体
API 设计摘要
执行限制 API

// 配置
let limits = ExecutionLimits {
    max_steps: Some(1_000_000),
    max_recursion_depth: Some(1000),
    max_duration_ms: Some(30_000),
    max_memory_bytes: None,
};

let engine = Aether::with_limits(limits);

// 运行时检查
match engine.eval(code) {
    Err(RuntimeError::ExecutionLimit(ExecutionLimitError::StepLimitExceeded { steps, limit })) => {
        eprintln!("Exceeded step limit: {} > {}", steps, limit);
    },
    // ... 其他错误
}
结构化 TRACE API

# DSL 代码中使用

TRACE_INFO("user", "login", USER_ID)
TRACE_WARN("api", "slow_response", LATENCY)
TRACE_ERROR("db", "connection_failed", ERR_MSG)

# Rust 中查询

let traces = engine.trace_by_level(TraceLevel::Error);
for trace in traces {
    println!("{:?} - {} : {:?}", trace.timestamp, trace.category, trace.values);
}
调试器 API

// 启用调试器
let debugger = PrintDebugger::new()
    .step_mode(true)
    .break_on_errors(true);

let engine = Aether::new();
engine.set_debugger(Some(Box::new(debugger)));

// REPL 中
:aether eval :step     # 单步执行
:aether eval :vars     # 查看变量
:aether eval :stack    # 查看调用栈
Record 和 Enum API

# Record

type Person = Record { name: String, age: Number }
Let P Person("Alice", 30)

# Enum

enum Option { Some(T), None }
Let X Option.Some(42)

# 模式匹配

Match X {
    Option.Some(V) => V,
    Option.None => 0
}
验收标准（阶段3完成）
功能完整性
执行限制：
✅ 防止无限循环
✅ 防止栈溢出
✅ 防止长时间执行
✅ 配置灵活
结构化 TRACE：
✅ 支持事件级别
✅ 支持事件分类
✅ 支持过滤
✅ 向后兼容
规则调试：
✅ 支持单步执行
✅ 支持查看变量和调用栈
✅ REPL 调试命令完整
数据结构：
✅ 支持 Record 和 Enum
✅ 支持模式匹配
工具链：
✅ Formatter 可用
✅ LSP 基础功能可用
质量标准
✅ 测试覆盖率 > 80%
✅ 文档完整
✅ 性能开销 < 5%
✅ 向后兼容
用户体验
✅ API 简单易用
✅ 错误消息清晰
✅ 调试体验流畅
✅ 工具链完善
