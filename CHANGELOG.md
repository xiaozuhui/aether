# 变更日志 (Changelog)

记录 Aether 项目的所有重要变更。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，版本号遵循[语义化版本](https://semver.org/lang/zh-CN/)。

## [未发布]

Cargo.toml 版本号已升至 0.5.4，尚未打 tag 发布。

### 新增

- **交互式调试器真正可用**（`--debugger`）：
  - AST 语句新增 `Located { line, stmt }` 行号包装，优化 pass 保留行号；新增 `collect_breakable_lines` 校验断点行
  - 行断点 / 函数断点真实触发（此前框架从未接入求值器）；`step` 步入、`next` 步过、`finish` 步出、`continue` 均可暂停与恢复
  - 暂停点 `print` 按作用域链打印（函数内可见局部变量与参数）、`backtrace` 显示调用栈、`list` 带当前位置高亮
  - `Import` 模块的跨文件断点：模块顶层在加载时触发，模块内函数体在调用时按定义文件触发（`Value::Function` 记录定义文件）
  - 嵌入侧 API：`Aether::attach_debugger / detach_debugger / set_source_file / lookup_variable / evaluator_mut`

### 变更

- **许可证由 GPL-3.0（附授权第三方例外条款）变更为 BSL 1.1（Business Source License）**：个人、学习、内部业务及嵌入自有产品的生产使用均无需商业授权，仅竞争性脚本引擎 / DSL 运行时产品需要；Change Date 2030-08-28 到期自动转为 Apache-2.0（见 LICENSE）。已发布的历史版本仍适用其发布时的 GPL-3.0

### 移除

- **移除全部语言绑定**：删除 C FFI 层（`src/ffi.rs`、`aether.h`、cbindgen）、WASM 导出层与 TypeScript 绑定及全部相关依赖。项目定位收敛为纯 Rust 库 + CLI 两种形态，WASM 目标不再可用，跨平台仅支持 x86_64 与 ARM64
- **移除沙箱模块**：删除 `src/sandbox/`（PathValidator / ScopedValidator / SandboxConfig / 指标收集 / 模块缓存，约 1200 行）与 `docs/SANDBOX_GUIDE.md`。不可信脚本的安全隔离交给容器等外部沙箱；进程内保留 IOPermissions（IO 默认禁用、按权限条件注册）与 ExecutionLimits（步数 / 时限 / 递归深度限额）。默认行为不变——路径验证从未被任何构造器激活；对手动使用 `ScopedValidator` 的宿主属破坏性 API 移除
- 清理报表功能的最后残留（占位模块、过时文档与失效示例）

### 新增

- **大整数能力增强**：
  - 位运算符 `&`、`|`、`^`、`<<`、`>>`，普通整数与大整数通用
  - 科学计数法字面量，正指数构造精确值，超阈值自动升级为 BigInteger
  - 可配置大整数阈值：`Aether::set_bigint_threshold` API 与 `--bigint-threshold` CLI 参数
- CONTRIBUTING、DCO、LICENSE 的简体中文版本

### 修复

- Number 与 Fraction 混合运算原先经 `as i64` 提升整数，超出 i64 范围（约 9.2e18）会静默截断，改为十进制字符串精确转换
- Cargo.toml 许可证声明由模板残留的 Apache-2.0 修正为 GPL-3.0-only

### 性能

- **修复 EnginePool acquire 隐式构造占位引擎的缺陷**：引擎池槽位由 `Vec<Aether>` + `Vec<bool>` 双数组改为 `Vec<Option<Aether>>`，避免每次借出都隐式构建完整引擎再丢弃。单次 acquire+eval 由约 61µs 降至约 22µs，与 GlobalEngine 持平
- BigInt 字面量解析期一次性构造、BigInteger 常量折叠、f64 提升时的 Fraction 精确化

### 构建

- 全量升级依赖：num-bigint 0.4 → 0.5（跨大版本），ureq 3.4、serde 1.0.229、tokio 1.53、criterion 0.8.2 等

### 文档

- USER_GUIDE、BIGINT_GUIDE、ENGINE_MODES_GUIDE、PRECISION_GUIDE 四份独立指南陆续并入 README，DEBUG_GUIDE 亦并入 README「调试与排错」章节后删除；SANDBOX_GUIDE 随沙箱模块一并删除，`docs/` 仅余 PAYROLL 一份指南
- 调试指南重写调试器章节：删除文末粘贴的实现报告，修正编号顺序与 `--trace` 示例格式，补 `--debugger` / `--json-error` / `--bigint-threshold` 用法与已知限制
- 新增「引擎模式」章节：GlobalEngine / EnginePool / ScopedEngine 用法、对比与实测性能
- 修正引擎模块注释中的并发语义误导（三种模式均为线程局部，无需 Mutex），修复 5 处无法编译的文档示例
- 精确计算章节按实测输出修订 5 处与实际行为不符的描述

## [0.5.3] - 2026-01-16

### 新增

- debugger 调试器（见 `docs/DEBUG_GUIDE.md`）
- CLI `--trace` 命令行参数与 `--metrics` 性能分析，基础性能指标
- REDUCE 函数
- 全局缓冲区大小的设置

### 变更

- **开源协议由 Apache-2.0 变更为 GPL-3.0**（附授权第三方例外条款，见 LICENSE），同时引入 CONTRIBUTING 与 DCO 流程
- 拆分 main 与 lib，改善可维护性

### 修复

- 移除沙箱测试中的 `set_current_dir` 全局目录切换，消除并行测试竞态；路径验证器自动把相对路径解析到根目录

## [0.5.2] - 2026-01-13

### 移除

- **删除 Excel 与报表功能**，聚焦 DSL 核心能力
- 去除 Python 转译层

### 变更

- 整理测试与项目文档

## [0.5.1] - 2026-01-12

自 0.4.4 直接跃迁，未发布 0.5.0。

### 移除

- 删除 Golang 绑定，迁移至单独仓库维护

### 修复

- FFI 相关错误修复

## [0.4.4] - 2026-01-09

### 新增

- **执行限制**：运行时限额体系（`src/runtime/limits.rs`），可与沙箱配置联动约束脚本执行

### 变更

- trace 能力优化

## [0.4.3] - 2026-01-06

### 新增

- **统一错误模型 + 调用栈 + import 链**：错误信息携带完整调用栈与模块导入链路

## [0.4.2] - 2026-01-06

### 新增

- **Import / Export 模块系统**：脚本间导入与导出能力
- 内联执行（宿主作用域注入）

## [0.4.1] - 2026-01-06

### 新增

- trace 执行追踪能力

## [0.4.0] - 2026-01-05

### 新增

- Python 转译层：将 Aether 代码转译为 Python（后于 0.5.2 移除）

### 变更

- 文档体系清理，删除早期 CHANGELOG.md 与散落文档

## [0.3.0] - 2025-11-09

### 变更

- **包更名**：因 crates.io 名称被占用，`aether` 更名为 `aether-azathoth`

### 新增

- **三种引擎使用方式**：全局单例 GlobalEngine、引擎池 EnginePool、闭包式 ScopedEngine，三者完全隔离（线程局部实现）
- async feature
- **Lambda 箭头语法**：`Lambda X -> X + 1`，与既有 `Func(X) { Return X }` 语法并存
- 多行字符串（三引号 `"""`）
- 字典 / 数组字面量支持换行与深层嵌套
- 编译期自动校验全部内置标准库文件语法
- 调试工具：`--check` 语法检查、`--ast` 查看器、`--debug` 调试执行；错误信息带行列号与上下文指示

## [0.2.0] - 2025-11-08

首个打 tag 发布的版本。

### 新增

- **精确计算**：分数类型与 `TO_FRACTION`、`FRAC_ADD/SUB/MUL/DIV`、`SIMPLIFY`、`NUMERATOR`、`DENOMINATOR` 等函数
- **精度计算**：`ROUND_TO`、`ADD/SUB/MUL/DIV_WITH_PRECISION`、`SET_PRECISION`
- **大整数**：超过 15 位的整数自动启用 BigInt 精确计算
- 数论函数 `GCD`、`LCM`；尾递归优化
- **语义化逻辑运算**：`And` / `Or` / `Not` 关键字并支持短路求值；字符串下标取字符；`SetIndex` 下标更新数组
- **标准库**：以 Aether 自举的标准库并支持细粒度导入；JSON、CSV、高阶函数内置库；Set / Queue / Stack / Heap / Sorting 五个数据结构与算法库（120+ 函数）
- **IO 与安全**：磁盘 IO 与网络 IO 函数（READ_FILE、HTTP_GET 等 11 个），配套 `IOPermissions` 权限体系，默认禁用全部 IO（安全优先）
- **薪酬计算模块**：基本工资、加班费、个税、社保公积金、考勤、奖金、津贴、折算等 78 个函数（见 `docs/PAYROLL_GUIDE.md`）
- 报表与 Excel 基础函数（后于 0.5.2 移除）
- FFI 及 Golang、TypeScript 绑定（后陆续移除）
- HELP 函数；详细错误标识（行列号定位、强制 UPPER_SNAKE_CASE 命名规范）

## [0.1.0] - 2025-11-06

初始版本。

### 新增

- **语言核心**：词法分析器、递归下降 + Pratt 解析器、AST、求值器，作用域与闭包
- 9 种数据类型：Number、String、Boolean、Null、Array、Dict、Function、Generator、Lazy
- 95 个内置函数：基础数学、三角函数、对数指数、统计（Mean / Median / Variance / Std / Quantile）、向量与矩阵运算（Matmul / Transpose / Inverse / Determinant）、线性回归与概率分布（NormalPDF / NormalCDF / PoissonPMF）
- 114 个测试全部通过

---

## 版本备注

- `0.1.0` 与 `0.3.0` 无 `v` 前缀 tag；`0.3.0` 的记载依据早期 CHANGELOG.md（该文件于 0.4.0 文档清理时删除，本文件由 git 历史重新整理）
- 0.4.4 之后直接发布 0.5.1，不存在 0.5.0
