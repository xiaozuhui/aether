# Aether 开发文档

## 📋 文档索引

- [1. 项目概述](#1-项目概述)
- [2. 技术栈与架构](#2-技术栈与架构)
- [3. 开发路线图](#3-开发路线图)
- [4. 核心模块实现](#4-核心模块实现)
- [5. 跨语言绑定](#5-跨语言绑定)
- [6. 测试策略](#6-测试策略)
- [7. 构建与部署](#7-构建与部署)
- [8. 贡献指南](#8-贡献指南)

---

## 1. 项目概述

### 1.1 项目愿景

Aether 是一个**轻量级、可嵌入的领域特定语言（DSL）**，旨在为 Rust、Go、TypeScript 等宿主语言提供统一的脚本能力。

**核心目标**：

- 🎯 **简洁易学**：直观的语法，低学习成本
- 🚀 **高性能**：Rust 实现的核心引擎，零成本抽象
- 🔌 **易于嵌入**：支持多种主流编程语言
- 🌍 **跨平台**：支持 x86_64、ARM64 等主流架构
- ✨ **现代特性**：函数式编程、生成器、惰性求值

### 1.2 应用场景

- **配置管理**：替代 JSON/YAML，支持逻辑和计算
- **业务规则引擎**：动态配置业务逻辑
- **数据处理管道**：ETL 转换脚本
- **游戏脚本**：游戏逻辑和 AI 行为
- **自动化工具**：任务编排和执行

### 1.3 核心特性

| 特性 | 描述 | 优先级 |
|------|------|--------|
| 基础语法 | 变量、运算符、控制流 | P0 |
| 函数 | 定义、调用、闭包 | P0 |
| 数据类型 | Number, String, Boolean, Array, Dict | P0 |
| 生成器 | `Generator` 关键字，惰性序列 | P1 |
| 惰性求值 | `Lazy` 关键字，延迟计算 | P1 |
| 模块系统 | Import/Export | P1 |
| 错误处理 | Throw/Catch（映射到宿主语言）| P2 |
| 标准库 | 内置函数（数学、字符串、数组等）| P0 |

### 1.4 语法示例

```javascript
// 变量和基础运算
Set COUNT 10
Set MESSAGE "Hello, Aether"
Set TOTAL (COUNT * 2 + 5)

// 函数定义
Func FIBONACCI (N) {
    If (N <= 1) {
        Return N
    }
    Return (FIBONACCI(N - 1) + FIBONACCI(N - 2))
}

// 生成器
Generator RANGE (START, END) {
    Set I START
    While (I < END) {
        Yield I
        Set I (I + 1)
    }
}

// 惰性求值
Lazy EXPENSIVE_DATA (
    Print "Loading expensive data..."
    Return LOAD_BIG_FILE("data.json")
)

// 使用
For NUM In RANGE(0, 10) {
    Print "Number:", NUM
}
```

---

## 2. 技术栈与架构

### 2.1 技术选型

| 组件 | 技术 | 理由 |
|------|------|------|
| 核心引擎 | **Rust** | 性能、安全性、跨平台编译 |
| C-FFI | **cbindgen** | 自动生成 C 头文件 |
| WASM | **wasm-bindgen** | TypeScript/JavaScript 绑定 |
| 构建工具 | **Cargo** | Rust 标准构建工具 |
| 测试框架 | **cargo test** + **criterion** | 单元测试 + 性能测试 |
| CI/CD | **GitHub Actions** | 自动化构建和测试 |

### 2.2 整体架构

```
┌─────────────────────────────────────────────────────────────────┐
│                       Aether Core (Rust)                        │
│                                                                 │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   │
│  │  Lexer   │ → │  Parser  │ → │   AST    │ → │Evaluator │   │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘   │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │              Environment & Value System                  │ │
│  │  - Variable Scope                                        │ │
│  │  - Function Registry                                     │ │
│  │  - Built-in Functions                                    │ │
│  └──────────────────────────────────────────────────────────┘ │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │              Advanced Features                           │ │
│  │  - Generator (Lazy Iterator)                             │ │
│  │  - Lazy Evaluation (Thunk)                               │ │
│  │  - Module System                                          │ │
│  └──────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                               │
              ┌────────────────┼────────────────┐
              ▼                ▼                ▼
       ┌────────────┐   ┌────────────┐   ┌────────────┐
       │   Rust     │   │   C-FFI    │   │   WASM     │
       │  Native    │   │  (Go 用)   │   │  (TS 用)   │
       └────────────┘   └────────────┘   └────────────┘
              │                │                │
              ▼                ▼                ▼
       ┌────────────┐   ┌────────────┐   ┌────────────┐
       │  Rust App  │   │   Go App   │   │   TS App   │
       └────────────┘   └────────────┘   └────────────┘
```

### 2.3 目录结构

```
aether/
├── Cargo.toml                 # Rust 项目配置
├── README.md                  # 项目说明
├── DESIGN.md                  # 语言设计文档
├── DEVELOPMENT.md             # 开发文档（本文档）
├── LICENSE                    # 开源协议
│
├── src/                       # Rust 核心实现
│   ├── lib.rs                 # 库入口
│   ├── lexer.rs               # 词法分析器
│   ├── token.rs               # Token 定义
│   ├── parser.rs              # 语法解析器
│   ├── ast.rs                 # 抽象语法树
│   ├── evaluator.rs           # 求值器
│   ├── value.rs               # 值类型系统
│   ├── environment.rs         # 环境和作用域
│   ├── builtins/              # 内置函数
│   │   ├── mod.rs
│   │   ├── math.rs            # 数学函数
│   │   ├── string.rs          # 字符串函数
│   │   ├── array.rs           # 数组函数
│   │   ├── dict.rs            # 字典函数
│   │   └── types.rs           # 类型检查函数
│   ├── generator.rs           # 生成器实现
│   ├── lazy.rs                # 惰性求值
│   ├── module.rs              # 模块系统
│   ├── error.rs               # 错误类型
│   ├── ffi.rs                 # C-FFI 接口
│   └── wasm.rs                # WASM 绑定
│
├── bindings/                  # 语言绑定
│   ├── go/                    # Go 绑定
│   │   ├── go.mod
│   │   ├── aether.go          # Go 包装
│   │   ├── value.go           # 值类型转换
│   │   └── examples/
│   │       └── main.go
│   │
│   └── typescript/            # TypeScript 绑定
│       ├── package.json
│       ├── tsconfig.json
│       ├── src/
│       │   ├── index.ts       # TS 包装
│       │   └── types.ts       # 类型定义
│       └── examples/
│           └── example.ts
│
├── tests/                     # 测试套件
│   ├── lexer_tests.rs         # 词法分析器测试
│   ├── parser_tests.rs        # 解析器测试
│   ├── evaluator_tests.rs     # 求值器测试
│   ├── integration_tests.rs   # 集成测试
│   └── cross_lang_tests/      # 跨语言一致性测试
│       └── test-cases.json
│
├── benches/                   # 性能基准测试
│   └── benchmark.rs
│
├── examples/                  # 示例代码
│   ├── basic.aether           # 基础语法示例
│   ├── fibonacci.aether       # 斐波那契数列
│   ├── generator.aether       # 生成器示例
│   └── modules/               # 模块系统示例
│
├── docs/                      # 文档
│   ├── api/                   # API 文档
│   ├── tutorial/              # 教程
│   └── internals/             # 内部实现文档
│
└── scripts/                   # 构建脚本
    ├── build-all.sh           # 构建所有目标
    ├── test-all.sh            # 运行所有测试
    └── release.sh             # 发布脚本
```

---

## 3. 开发路线图

### 3.1 阶段 0：项目初始化（1 周）

**目标**：搭建项目骨架，确定基础设施

- [ ] 创建 Rust 项目结构
- [ ] 配置 Cargo.toml（依赖、元数据）
- [ ] 设置 GitHub 仓库
- [ ] 配置 CI/CD（GitHub Actions）
- [ ] 编写基础文档（README, CONTRIBUTING）
- [ ] 选择开源协议（建议 MIT 或 Apache 2.0）

**产出物**：

- 可编译的 Rust 项目
- 自动化测试流程
- 基础文档

### 3.2 阶段 1：核心解释器（4-6 周）

#### 1.1 词法分析器（Lexer）- 1 周

**任务**：

- [ ] 定义 Token 类型（关键字、标识符、字面量等）
- [ ] 实现扫描器（Scanner）
- [ ] 处理空白符和注释
- [ ] 错误位置跟踪（行号、列号）
- [ ] 单元测试（覆盖率 > 90%）

**关键代码**：

```rust
// src/token.rs
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // 关键字
    Set, Func, If, Else, While, For, Return,
    Generator, Yield, Lazy,
    
    // 标识符和字面量
    Identifier(String),
    Number(f64),
    String(String),
    
    // 运算符
    Plus, Minus, Multiply, Divide,
    Equal, NotEqual, Greater, Less,
    
    // 分隔符
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Newline, EOF,
}
```

#### 1.2 语法解析器（Parser）- 2 周

**任务**：

- [ ] 定义 AST 节点类型
- [ ] 实现递归下降解析器
- [ ] 优先级处理（Pratt Parsing）
- [ ] 语法错误恢复
- [ ] 单元测试

**关键代码**：

```rust
// src/ast.rs
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Identifier(String),
    Binary { left: Box<Expr>, op: BinOp, right: Box<Expr> },
    Call { func: Box<Expr>, args: Vec<Expr> },
    If { condition: Box<Expr>, then_branch: Vec<Stmt>, else_branch: Option<Vec<Stmt>> },
    // ...
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Set { name: String, value: Expr },
    FuncDef { name: String, params: Vec<String>, body: Vec<Stmt> },
    Return(Expr),
    // ...
}
```

#### 1.3 求值器（Evaluator）- 2 周

**任务**：

- [ ] 实现值类型系统（Value enum）
- [ ] 环境管理（作用域）
- [ ] 表达式求值
- [ ] 语句执行
- [ ] 函数调用
- [ ] 单元测试 + 集成测试

**关键代码**：

```rust
// src/value.rs
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Dict(HashMap<String, Value>),
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
        env: Rc<RefCell<Environment>>,
    },
    NativeFunction(NativeFn),
    Null,
}

type NativeFn = Rc<dyn Fn(Vec<Value>) -> Result<Value, EvalError>>;
```

#### 1.4 内置函数库（Builtins）- 1 周

**任务**：

- [ ] 数学函数（ABS, SQRT, POW, MAX, MIN 等）
- [ ] 字符串函数（LENGTH, TO_UPPER, SUBSTRING 等）
- [ ] 数组函数（APPEND, SLICE, MAP, FILTER 等）
- [ ] 类型转换（TO_STRING, TO_NUMBER, TO_BOOL）
- [ ] 类型检查（IS_NUMBER, IS_STRING 等）
- [ ] 单元测试

**验收标准**：

- 所有设计文档中的内置函数都已实现
- 测试覆盖率 > 95%
- 性能基准测试通过

### 3.3 阶段 2：高级特性（3-4 周）

#### 2.1 生成器（Generator）- 1.5 周

**任务**：

- [ ] 设计生成器状态机
- [ ] 实现 `Generator` 关键字解析
- [ ] 实现 `Yield` 语句
- [ ] 迭代器协议
- [ ] For-In 循环支持生成器
- [ ] 测试

**实现思路**：

```rust
pub struct Generator {
    state: GeneratorState,
    context: GeneratorContext,
}

enum GeneratorState {
    Fresh,           // 未开始
    Suspended(usize), // 暂停在某个位置
    Completed,       // 完成
}

struct GeneratorContext {
    locals: HashMap<String, Value>,
    program_counter: usize,
}
```

#### 2.2 惰性求值（Lazy）- 1 周

**任务**：

- [ ] 设计 Thunk 机制
- [ ] 实现 `Lazy` 关键字
- [ ] 记忆化（Memoization）
- [ ] `FORCE()` 函数
- [ ] 测试

**实现思路**：

```rust
pub struct Lazy {
    thunk: OnceCell<Value>,
    expr: Expr,
    env: Rc<RefCell<Environment>>,
}

impl Lazy {
    pub fn force(&mut self) -> Result<Value, EvalError> {
        self.thunk.get_or_try_init(|| {
            // 在保存的环境中求值表达式
            evaluate(&self.expr, &self.env)
        })
    }
}
```

#### 2.3 模块系统（Module）- 1.5 周

**任务**：

- [ ] 文件路径解析
- [ ] Import/Export 语法
- [ ] 模块缓存
- [ ] 循环依赖检测
- [ ] 测试

**功能**：

```javascript
// math.aether
Set PI 3.14159
Func ADD (A, B) { Return (A + B) }
Export PI
Export ADD

// main.aether
Import {ADD, PI} From "math.aether"
Print ADD(1, 2)
```

### 3.4 阶段 3：C-FFI 接口（2 周）

**任务**：

- [ ] 设计 C-ABI 兼容接口
- [ ] 使用 cbindgen 生成头文件
- [ ] 内存管理策略（Box::into_raw / Box::from_raw）
- [ ] 错误码定义
- [ ] 类型转换函数
- [ ] C 示例程序测试

**产出物**：

- `libaether.a` / `libaether.so` 静态/动态库
- `aether.h` C 头文件
- C 语言调用示例

### 3.5 阶段 4：Go 绑定（2 周）

**任务**：

- [ ] CGO 封装
- [ ] Go 值类型转换
- [ ] Go 风格 API 设计
- [ ] 错误处理映射
- [ ] 函数注册接口
- [ ] 示例和文档
- [ ] 集成测试

**产出物**：

- `github.com/yourusername/aether-go` Go 模块
- 完整文档和示例
- 通过所有跨语言一致性测试

### 3.6 阶段 5：WASM/TypeScript 绑定（2-3 周）

**任务**：

- [ ] wasm-bindgen 集成
- [ ] JavaScript 值转换
- [ ] TypeScript 类型定义
- [ ] 异步支持（如果需要）
- [ ] npm 包配置
- [ ] 浏览器 + Node.js 测试
- [ ] 文档和示例

**产出物**：

- `@yourusername/aether` npm 包
- TypeScript 类型定义
- 在线演示（GitHub Pages）

### 3.7 阶段 6：优化与完善（2-3 周）

**任务**：

- [ ] 性能优化（热点分析、算法改进）
- [ ] 内存优化（减少分配、缓存）
- [ ] 错误消息改进（更友好的提示）
- [ ] 文档完善（API 文档、教程）
- [ ] 示例项目（实际应用场景）
- [ ] 安全审计

### 3.8 阶段 7：发布与生态（持续）

**任务**：

- [ ] 发布 1.0.0 版本
- [ ] 宣传推广（博客、社交媒体）
- [ ] 社区建设（Discord、论坛）
- [ ] 第三方库生态
- [ ] 持续维护

---

## 4. 核心模块实现

### 4.1 词法分析器（Lexer）

#### 职责

将源代码字符串转换为 Token 流。

#### 关键实现

```rust
// src/lexer.rs
pub struct Lexer {
    input: Vec<char>,
    position: usize,      // 当前位置
    read_position: usize, // 下一个字符位置
    ch: char,             // 当前字符
    line: usize,          // 行号
    column: usize,        // 列号
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
            line: 1,
            column: 0,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
        self.column += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        let token = match self.ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '"' => self.read_string(),
            '\0' => Token::EOF,
            _ => {
                if self.ch.is_alphabetic() || self.ch == '_' {
                    return self.read_identifier();
                } else if self.ch.is_numeric() {
                    return self.read_number();
                } else {
                    Token::Illegal(self.ch)
                }
            }
        };
        
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.read_char();
        }
        let ident: String = self.input[start..self.position].iter().collect();
        
        // 检查是否为关键字
        match ident.as_str() {
            "Set" => Token::Set,
            "Func" => Token::Func,
            "If" => Token::If,
            "Else" => Token::Else,
            "Return" => Token::Return,
            "Generator" => Token::Generator,
            "Yield" => Token::Yield,
            "Lazy" => Token::Lazy,
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "nil" => Token::Null,
            _ => Token::Identifier(ident),
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        while self.ch.is_numeric() || self.ch == '.' {
            self.read_char();
        }
        let num_str: String = self.input[start..self.position].iter().collect();
        Token::Number(num_str.parse().unwrap())
    }

    fn read_string(&mut self) -> Token {
        self.read_char(); // 跳过开头的 "
        let start = self.position;
        while self.ch != '"' && self.ch != '\0' {
            self.read_char();
        }
        let string: String = self.input[start..self.position].iter().collect();
        Token::String(string)
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\r' {
            self.read_char();
        }
        if self.ch == '\n' {
            self.line += 1;
            self.column = 0;
            self.read_char();
        }
    }
}
```

#### 测试示例

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let input = "Set X 10";
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token(), Token::Set);
        assert_eq!(lexer.next_token(), Token::Identifier("X".to_string()));
        assert_eq!(lexer.next_token(), Token::Number(10.0));
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_string_literal() {
        let input = r#"Set MSG "Hello World""#;
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token(), Token::Set);
        assert_eq!(lexer.next_token(), Token::Identifier("MSG".to_string()));
        assert_eq!(lexer.next_token(), Token::String("Hello World".to_string()));
    }
}
```

### 4.2 语法解析器（Parser）

#### 职责

将 Token 流转换为抽象语法树（AST）。

#### 关键实现

```rust
// src/parser.rs
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        let peek = lexer.next_token();
        Parser {
            lexer,
            current_token: current,
            peek_token: peek,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        
        while self.current_token != Token::EOF {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token();
        }
        
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        match &self.current_token {
            Token::Set => self.parse_set_statement(),
            Token::Func => self.parse_function_definition(),
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            Token::For => self.parse_for_statement(),
            Token::Return => self.parse_return_statement(),
            Token::Generator => self.parse_generator_definition(),
            Token::Yield => self.parse_yield_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_set_statement(&mut self) -> Result<Stmt, ParseError> {
        self.next_token(); // 跳过 Set
        
        let name = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => return Err(ParseError::ExpectedIdentifier),
        };
        
        self.next_token(); // 移到值
        let value = self.parse_expression(Precedence::Lowest)?;
        
        Ok(Stmt::Set { name, value })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expr, ParseError> {
        // Pratt Parsing 实现
        let mut left = self.parse_prefix()?;
        
        while precedence < self.peek_precedence() {
            self.next_token();
            left = self.parse_infix(left)?;
        }
        
        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Expr, ParseError> {
        match &self.current_token {
            Token::Number(n) => Ok(Expr::Number(*n)),
            Token::String(s) => Ok(Expr::String(s.clone())),
            Token::Identifier(name) => Ok(Expr::Identifier(name.clone())),
            Token::LeftParen => self.parse_grouped_expression(),
            Token::LeftBracket => self.parse_array_literal(),
            _ => Err(ParseError::UnexpectedToken(self.current_token.clone())),
        }
    }

    fn parse_infix(&mut self, left: Expr) -> Result<Expr, ParseError> {
        match &self.current_token {
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                let op = self.token_to_binop(&self.current_token);
                let precedence = self.current_precedence();
                self.next_token();
                let right = self.parse_expression(precedence)?;
                Ok(Expr::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                })
            }
            Token::LeftParen => self.parse_call_expression(left),
            _ => Ok(left),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,      // ==, !=
    LessGreater, // <, >
    Sum,         // +, -
    Product,     // *, /
    Call,        // func()
}
```

### 4.3 求值器（Evaluator）

#### 职责

执行 AST，产生结果。

#### 关键实现

```rust
// src/evaluator.rs
pub struct Evaluator {
    environment: Rc<RefCell<Environment>>,
}

impl Evaluator {
    pub fn new() -> Self {
        let mut env = Environment::new();
        register_builtins(&mut env);
        
        Evaluator {
            environment: Rc::new(RefCell::new(env)),
        }
    }

    pub fn eval_program(&mut self, program: Vec<Stmt>) -> Result<Value, EvalError> {
        let mut result = Value::Null;
        
        for stmt in program {
            result = self.eval_statement(&stmt)?;
            
            // 处理 Return
            if let Value::ReturnValue(val) = result {
                return Ok(*val);
            }
        }
        
        Ok(result)
    }

    fn eval_statement(&mut self, stmt: &Stmt) -> Result<Value, EvalError> {
        match stmt {
            Stmt::Set { name, value } => {
                let val = self.eval_expression(value)?;
                self.environment.borrow_mut().set(name.clone(), val.clone());
                Ok(val)
            }
            Stmt::Return(expr) => {
                let val = self.eval_expression(expr)?;
                Ok(Value::ReturnValue(Box::new(val)))
            }
            Stmt::Expression(expr) => self.eval_expression(expr),
            // ... 其他语句类型
        }
    }

    fn eval_expression(&mut self, expr: &Expr) -> Result<Value, EvalError> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Identifier(name) => {
                self.environment
                    .borrow()
                    .get(name)
                    .ok_or_else(|| EvalError::UndefinedVariable(name.clone()))
            }
            Expr::Binary { left, op, right } => {
                let left_val = self.eval_expression(left)?;
                let right_val = self.eval_expression(right)?;
                self.eval_binary_expression(left_val, op, right_val)
            }
            Expr::Call { func, args } => {
                let func_val = self.eval_expression(func)?;
                let arg_vals: Result<Vec<_>, _> = args
                    .iter()
                    .map(|arg| self.eval_expression(arg))
                    .collect();
                self.apply_function(func_val, arg_vals?)
            }
            // ... 其他表达式类型
        }
    }

    fn eval_binary_expression(
        &self,
        left: Value,
        op: &BinOp,
        right: Value,
    ) -> Result<Value, EvalError> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => match op {
                BinOp::Add => Ok(Value::Number(l + r)),
                BinOp::Subtract => Ok(Value::Number(l - r)),
                BinOp::Multiply => Ok(Value::Number(l * r)),
                BinOp::Divide => {
                    if r == 0.0 {
                        Err(EvalError::DivisionByZero)
                    } else {
                        Ok(Value::Number(l / r))
                    }
                }
                // ... 其他运算符
            },
            (Value::String(l), Value::String(r)) if matches!(op, BinOp::Add) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            _ => Err(EvalError::TypeError),
        }
    }

    fn apply_function(&mut self, func: Value, args: Vec<Value>) -> Result<Value, EvalError> {
        match func {
            Value::Function { params, body, env } => {
                if params.len() != args.len() {
                    return Err(EvalError::ArgumentMismatch);
                }
                
                // 创建新环境
                let func_env = Environment::new_enclosed(env);
                for (param, arg) in params.iter().zip(args.iter()) {
                    func_env.borrow_mut().set(param.clone(), arg.clone());
                }
                
                // 在新环境中执行函数体
                let old_env = self.environment.clone();
                self.environment = func_env;
                
                let result = self.eval_program(body);
                
                self.environment = old_env;
                result
            }
            Value::NativeFunction(f) => f(args),
            _ => Err(EvalError::NotAFunction),
        }
    }
}
```

### 4.4 环境管理（Environment）

```rust
// src/environment.rs
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Environment {
    store: HashMap<String, Value>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: Some(outer),
        }))
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        match self.store.get(name) {
            Some(val) => Some(val.clone()),
            None => self.outer.as_ref()?.borrow().get(name),
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.store.insert(name, value);
    }
}
```

---

## 5. 跨语言绑定

### 5.1 Rust 原生使用

最简单，直接使用核心库：

```rust
// 使用示例
use aether::{Aether, Value};

fn main() {
    let mut engine = Aether::new();
    
    // 注册自定义函数
    engine.register_function("PRINT_RUST", |args| {
        println!("From Rust: {:?}", args);
        Ok(Value::Null)
    });
    
    // 执行代码
    let code = r#"
        Set X 10
        Set Y 20
        Set Z (X + Y)
        PRINT_RUST(Z)
    "#;
    
    match engine.eval(code) {
        Ok(result) => println!("Result: {:?}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### 5.2 Go 绑定实现

#### C-FFI 接口

```rust
// src/ffi.rs
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

#[repr(C)]
pub struct AetherHandle {
    _opaque: [u8; 0],
}

#[no_mangle]
pub extern "C" fn aether_new() -> *mut AetherHandle {
    let engine = Box::new(Aether::new());
    Box::into_raw(engine) as *mut AetherHandle
}

#[no_mangle]
pub extern "C" fn aether_eval(
    handle: *mut AetherHandle,
    code: *const c_char,
    result: *mut *mut c_char,
    error: *mut *mut c_char,
) -> c_int {
    if handle.is_null() || code.is_null() {
        return -1;
    }
    
    unsafe {
        let engine = &mut *(handle as *mut Aether);
        let code_str = CStr::from_ptr(code).to_str().unwrap();
        
        match engine.eval(code_str) {
            Ok(val) => {
                let result_str = format!("{:?}", val);
                *result = CString::new(result_str).unwrap().into_raw();
                0
            }
            Err(e) => {
                let error_str = format!("{}", e);
                *error = CString::new(error_str).unwrap().into_raw();
                1
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn aether_free(handle: *mut AetherHandle) {
    if !handle.is_null() {
        unsafe {
            let _ = Box::from_raw(handle as *mut Aether);
        }
    }
}

#[no_mangle]
pub extern "C" fn aether_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
```

#### Go 封装

```go
// bindings/go/aether.go
package aether

/*
#cgo LDFLAGS: -L${SRCDIR}/../../target/release -laether
#include <stdlib.h>

typedef struct AetherHandle AetherHandle;

AetherHandle* aether_new();
int aether_eval(AetherHandle* handle, const char* code, char** result, char** error);
void aether_free(AetherHandle* handle);
void aether_free_string(char* s);
*/
import "C"
import (
    "errors"
    "runtime"
    "unsafe"
)

type Aether struct {
    handle *C.AetherHandle
}

func New() *Aether {
    a := &Aether{
        handle: C.aether_new(),
    }
    runtime.SetFinalizer(a, (*Aether).Close)
    return a
}

func (a *Aether) Eval(code string) (string, error) {
    if a.handle == nil {
        return "", errors.New("aether: engine closed")
    }
    
    cCode := C.CString(code)
    defer C.free(unsafe.Pointer(cCode))
    
    var result *C.char
    var errorMsg *C.char
    
    status := C.aether_eval(a.handle, cCode, &result, &errorMsg)
    
    if status != 0 {
        if errorMsg != nil {
            defer C.aether_free_string(errorMsg)
            return "", errors.New(C.GoString(errorMsg))
        }
        return "", errors.New("unknown error")
    }
    
    if result != nil {
        defer C.aether_free_string(result)
        return C.GoString(result), nil
    }
    
    return "", nil
}

func (a *Aether) Close() {
    if a.handle != nil {
        C.aether_free(a.handle)
        a.handle = nil
    }
}
```

#### Go 使用示例

```go
package main

import (
    "fmt"
    "github.com/yourusername/aether-go"
)

func main() {
    engine := aether.New()
    defer engine.Close()
    
    code := `
        Set X 10
        Set Y 20
        Set Z (X + Y)
        Print "Result:", Z
    `
    
    result, err := engine.Eval(code)
    if err != nil {
        fmt.Println("Error:", err)
        return
    }
    
    fmt.Println("Result:", result)
}
```

### 5.3 TypeScript/WASM 绑定

#### WASM 接口

```rust
// src/wasm.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Aether {
    engine: crate::Aether,
}

#[wasm_bindgen]
impl Aether {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self {
            engine: crate::Aether::new(),
        }
    }

    #[wasm_bindgen]
    pub fn eval(&mut self, code: &str) -> Result<JsValue, JsValue> {
        match self.engine.eval(code) {
            Ok(value) => Ok(value_to_js(&value)),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }
}

fn value_to_js(value: &Value) -> JsValue {
    match value {
        Value::Number(n) => JsValue::from_f64(*n),
        Value::String(s) => JsValue::from_str(s),
        Value::Boolean(b) => JsValue::from_bool(*b),
        Value::Array(arr) => {
            let js_arr = js_sys::Array::new();
            for v in arr {
                js_arr.push(&value_to_js(v));
            }
            js_arr.into()
        }
        Value::Dict(map) => {
            let obj = js_sys::Object::new();
            for (k, v) in map {
                js_sys::Reflect::set(&obj, &JsValue::from_str(k), &value_to_js(v)).unwrap();
            }
            obj.into()
        }
        Value::Null => JsValue::NULL,
        _ => JsValue::UNDEFINED,
    }
}
```

#### TypeScript 封装

```typescript
// bindings/typescript/src/index.ts
import init, { Aether as WasmAether } from '../pkg/aether_wasm';

export class Aether {
    private engine: WasmAether | null = null;
    private initialized = false;

    async init(): Promise<void> {
        if (!this.initialized) {
            await init();
            this.engine = new WasmAether();
            this.initialized = true;
        }
    }

    eval(code: string): any {
        if (!this.engine) {
            throw new Error('Aether not initialized. Call init() first.');
        }
        return this.engine.eval(code);
    }
}

export type Value = 
    | number 
    | string 
    | boolean 
    | Value[] 
    | { [key: string]: Value } 
    | null;
```

#### TypeScript 使用示例

```typescript
import { Aether } from '@yourusername/aether';

async function main() {
    const engine = new Aether();
    await engine.init();
    
    const code = `
        Set X 10
        Set Y 20
        Set Z (X + Y)
        Return Z
    `;
    
    try {
        const result = engine.eval(code);
        console.log('Result:', result); // 30
    } catch (e) {
        console.error('Error:', e);
    }
}

main();
```

---

## 6. 测试策略

### 6.1 单元测试

每个模块都应有独立的单元测试：

```rust
// tests/lexer_tests.rs
#[test]
fn test_tokenize_numbers() {
    let mut lexer = Lexer::new("123 456.78");
    assert_eq!(lexer.next_token(), Token::Number(123.0));
    assert_eq!(lexer.next_token(), Token::Number(456.78));
}

// tests/parser_tests.rs
#[test]
fn test_parse_set_statement() {
    let input = "Set X 10";
    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program().unwrap();
    assert_eq!(program.len(), 1);
}

// tests/evaluator_tests.rs
#[test]
fn test_eval_arithmetic() {
    let mut eval = Evaluator::new();
    let result = eval.eval("Set X (5 + 3 * 2)").unwrap();
    assert_eq!(result, Value::Number(11.0));
}
```

### 6.2 集成测试

测试完整的执行流程：

```rust
// tests/integration_tests.rs
#[test]
fn test_fibonacci_function() {
    let code = r#"
        Func FIB (N) {
            If (N <= 1) {
                Return N
            }
            Return (FIB(N - 1) + FIB(N - 2))
        }
        FIB(10)
    "#;
    
    let mut engine = Aether::new();
    let result = engine.eval(code).unwrap();
    assert_eq!(result, Value::Number(55.0));
}
```

### 6.3 跨语言一致性测试

定义统一的测试用例：

```json
// tests/cross_lang_tests/test-cases.json
[
    {
        "name": "basic_arithmetic",
        "code": "Set X 10\nSet Y 20\nReturn (X + Y)",
        "expected": 30
    },
    {
        "name": "string_concat",
        "code": "Set A \"Hello\"\nSet B \"World\"\nReturn (A + \" \" + B)",
        "expected": "Hello World"
    },
    {
        "name": "function_call",
        "code": "Func ADD (A, B) { Return (A + B) }\nReturn ADD(5, 3)",
        "expected": 8
    }
]
```

每种语言运行相同的测试用例并验证结果。

### 6.4 性能基准测试

```rust
// benches/benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_eval(c: &mut Criterion) {
    let mut engine = Aether::new();
    
    c.bench_function("eval_arithmetic", |b| {
        b.iter(|| {
            engine.eval(black_box("Set X (1 + 2 * 3)")).unwrap()
        });
    });
    
    c.bench_function("eval_function_call", |b| {
        engine.eval("Func ADD (A, B) { Return (A + B) }").unwrap();
        b.iter(|| {
            engine.eval(black_box("ADD(5, 3)")).unwrap()
        });
    });
}

criterion_group!(benches, benchmark_eval);
criterion_main!(benches);
```

---

## 7. 构建与部署

### 7.1 构建脚本

```bash
#!/bin/bash
# scripts/build-all.sh

set -e

echo "Building Aether for all targets..."

# Rust 核心库
echo "Building Rust library..."
cargo build --release

# C 静态库
echo "Building C static library..."
cargo build --release --lib --crate-type staticlib
cp target/release/libaether.a bindings/go/lib/

# 生成 C 头文件
echo "Generating C header..."
cbindgen --config cbindgen.toml --output bindings/go/lib/aether.h

# WASM
echo "Building WASM..."
wasm-pack build --target web --out-dir bindings/typescript/pkg

# Go 模块
echo "Testing Go bindings..."
cd bindings/go && go test ./... && cd ../..

# TypeScript
echo "Building TypeScript bindings..."
cd bindings/typescript && npm install && npm run build && cd ../..

echo "Build complete!"
```

### 7.2 CI/CD 配置

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest]
        rust: [stable, nightly]

    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: ${{ matrix.rust }}
          override: true
          components: rustfmt, clippy
      
      - name: Check formatting
        run: cargo fmt -- --check
      
      - name: Clippy
        run: cargo clippy -- -D warnings
      
      - name: Run tests
        run: cargo test --all-features
      
      - name: Run benchmarks
        run: cargo bench --no-run

  cross-compile:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        target: 
          - x86_64-unknown-linux-gnu
          - aarch64-unknown-linux-gnu
          - x86_64-apple-darwin
          - aarch64-apple-darwin

    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}
      
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}

  wasm:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
      
      - name: Build WASM
        run: wasm-pack build --target web
      
      - name: Test WASM
        run: wasm-pack test --headless --chrome
```

### 7.3 发布流程

```bash
#!/bin/bash
# scripts/release.sh

VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Usage: ./release.sh <version>"
    exit 1
fi

echo "Releasing version $VERSION"

# 更新版本号
sed -i.bak "s/^version = .*/version = \"$VERSION\"/" Cargo.toml
sed -i.bak "s/\"version\": .*/\"version\": \"$VERSION\",/" bindings/typescript/package.json

# 构建所有目标
./scripts/build-all.sh

# 运行测试
cargo test --all-features
cd bindings/go && go test ./... && cd ../..
cd bindings/typescript && npm test && cd ../..

# Git 操作
git add -A
git commit -m "Release v$VERSION"
git tag "v$VERSION"

# 发布
echo "Publishing Rust crate..."
cargo publish

echo "Publishing npm package..."
cd bindings/typescript && npm publish && cd ../..

echo "Pushing to GitHub..."
git push origin main
git push origin "v$VERSION"

echo "Release complete!"
```

---

## 8. 贡献指南

### 8.1 开发环境设置

```bash
# 克隆仓库
git clone https://github.com/yourusername/aether.git
cd aether

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装开发依赖
rustup component add rustfmt clippy
cargo install cargo-watch
cargo install wasm-pack

# 运行测试
cargo test

# 启动开发模式（自动重新编译）
cargo watch -x check -x test
```

### 8.2 代码风格

```rust
// 使用 rustfmt 格式化代码
cargo fmt

// 使用 clippy 检查代码
cargo clippy -- -D warnings
```

**风格指南**：

- 遵循 Rust 标准命名规范
- 函数名使用 `snake_case`
- 类型名使用 `PascalCase`
- 常量使用 `SCREAMING_SNAKE_CASE`
- 每个公共函数/类型都应有文档注释

### 8.3 提交规范

使用 [Conventional Commits](https://www.conventionalcommits.org/)：

```
feat: 添加生成器支持
fix: 修复除零错误
docs: 更新 API 文档
test: 添加解析器测试
refactor: 重构求值器
perf: 优化数组操作性能
```

### 8.4 Pull Request 流程

1. Fork 仓库
2. 创建功能分支：`git checkout -b feature/your-feature`
3. 提交更改：`git commit -m "feat: your feature"`
4. 推送分支：`git push origin feature/your-feature`
5. 创建 Pull Request
6. 等待代码审查

### 8.5 优先级标签

- **P0**：核心功能，必须实现
- **P1**：重要功能，应该实现
- **P2**：增强功能，可以实现
- **P3**：未来功能，暂时搁置

---

## 9. 附录

### 9.1 依赖列表

```toml
[dependencies]
# 核心依赖（尽量少）
# 无外部依赖或仅使用标准库

[dev-dependencies]
criterion = "0.5"  # 性能测试

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
console_error_panic_hook = "0.1"

[build-dependencies]
cbindgen = "0.26"  # 生成 C 头文件
```

### 9.2 有用的命令

```bash
# 开发
cargo check          # 快速检查编译错误
cargo build          # 构建
cargo test           # 运行测试
cargo bench          # 运行基准测试
cargo doc --open     # 生成并打开文档

# 发布
cargo build --release              # 发布构建
cargo build --target wasm32-unknown-unknown  # WASM 构建
cargo publish                      # 发布到 crates.io

# 工具
cargo fmt            # 格式化代码
cargo clippy         # 静态分析
cargo tree           # 查看依赖树
cargo outdated       # 检查过期依赖
```

### 9.3 参考资源

**Rust 解释器/编译器**：

- [Crafting Interpreters](https://craftinginterpreters.com/)
- [Writing An Interpreter In Go](https://interpreterbook.com/)（可用 Rust 实现）
- [rustpython](https://github.com/RustPython/RustPython)
- [rhai](https://github.com/rhaiscript/rhai)

**跨语言 FFI**：

- [The Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/)
- [wasm-bindgen Book](https://rustwasm.github.io/wasm-bindgen/)

**性能优化**：

- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)

---

## 10. 项目里程碑

### Milestone 1: MVP（最小可行产品）- 8 周

- [x] 项目初始化
- [ ] 词法分析器 + 解析器
- [ ] 基础求值器
- [ ] 核心内置函数
- [ ] Rust 原生使用
- [ ] 单元测试覆盖率 > 80%

**交付物**：可以执行基本 Aether 脚本的 Rust 库

### Milestone 2: 跨语言支持 - 4 周

- [ ] C-FFI 接口
- [ ] Go 绑定
- [ ] WASM/TypeScript 绑定
- [ ] 跨语言一致性测试

**交付物**：三种语言都可以嵌入使用 Aether

### Milestone 3: 高级特性 - 4 周

- [ ] 生成器
- [ ] 惰性求值
- [ ] 模块系统
- [ ] 完整标准库

**交付物**：功能完整的 1.0 版本

### Milestone 4: 生态与优化 - 持续

- [ ] 性能优化
- [ ] 文档和教程
- [ ] 示例项目
- [ ] 社区建设

---

## 结语

这份开发文档为 Aether 语言的实现提供了详细的路线图和技术指导。关键原则：

1. **从简单开始**：先实现 MVP，逐步添加功能
2. **测试驱动**：每个功能都有对应测试
3. **文档先行**：API 设计清晰，文档完善
4. **跨平台优先**：从一开始就考虑多语言、多架构支持

祝开发顺利！🚀
