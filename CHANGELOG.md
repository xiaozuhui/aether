# 变更日志 (Changelog)

记录 Aether 项目的所有重要变更。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，版本号遵循[语义化版本](https://semver.org/lang/zh-CN/)。

## [0.6.0] - 2026-08-29

语义修复大版本：修复多项静默错误的数值/结构语义，把三个只有前端没有后端的"幽灵特性"补齐为真实实现，删除全部伪装成功能的假函数与失效文档。升级前请阅读下方**破坏性变更**。

### 破坏性变更（升级必读）

- **HTTP 内置函数整体删除**：`HTTP_GET/HTTP_POST/HTTP_PUT/HTTP_DELETE` 与 `IOPermissions.network_enabled`、`ureq` 依赖一并移除。网络访问请由宿主程序完成后再注入变量
- **假日期函数删除**：`CALC_HOLIDAY_DAYS`、`IS_HOLIDAY`、`CALC_WORKDAYS`、`CALC_ANNUAL_WORKDAYS`、`CALC_ANNUAL_PAY_DAYS`、`GET_LEGAL_PAY_DAYS` 无法在无法定节假日数据源时真实计算，已删除；节假日判断由宿主给出布尔值后传给 `IS_WORKDAY`
- **恒等/无意义函数删除**：`SIMPLIFY`（Ratio 恒约分）、`CLONE`（值语义克隆恒等）删除
- **`with_stdlib()` 不再授予任何 IO 权限**：此前内部调用 `with_all_permissions()` 会静默开全部文件系统访问；现改为 `Aether::new()` 基线，需要 IO 请显式 `with_permissions`
- **`CHARAT` 越界从返回空串改为报错**（与 `s[i]` 索引对齐）
- **浮点相等从错误阈值改为严格位相等**：原先 `f64::EPSILON` 绝对阈值对大数恒等、对小数恒不等；现在 `==` 是位相等，容差需求显式写 `ABS(A - B) < 0.000001`
- **Number×Fraction 混合运算一律提升为分数**：`0.5 + TO_FRACTION(1/3)` 得精确 `5/6`；纯 Number（整数/小数）保持 f64
- **Generator 克隆共享消费状态**（类似 Python 迭代器语义）；**无限生成器触发步数上限报错**而非挂死
- **尾递归优化不再转换循环体内的尾调用**（嵌套循环内改写会破坏语义），保持解释执行
- **字符串操作统一为 Unicode 字符语义**（详见下"修复"）

### 新增

- **Generator / Yield 真实实现**（首次触发急切收集）：第一次 `NEXT(G)` 完整执行函数体一次，所有 `Yield` 值按序收集，副作用恰好发生一次；之后逐个弹出，耗尽返回 `Null`，`DONE(G)` 可查询；`For X In G` 直接迭代
- **Lazy 强制求值**：`Lazy` 定义时不求值，首次读取时求值并记忆化；自引用（`Lazy Y (Y + 1)`）报「循环定义」错误而非死循环
- **调试器条件断点**：`break <line> if <condition>`——位置命中后在当前环境求值条件表达式，为真才暂停；条件解析带缓存，求值出错视为不暂停
- **`TO_FRACTION` 连分数重建**：`TO_FRACTION(1/3)` 精确还原 `1/3`、`TO_FRACTION(0.1)` 得 `1/10`、科学计数法（`TO_FRACTION(1e-7)` 得 `1/10000000`）全部正确；`TO_FLOAT(TO_FRACTION(x)) == x` 往返恒等
- **Dict 深相等**：`{"x": 1} == {"x": 1}` 为真（键序无关、逐键递归）
- **`EnginePool.acquire(&self)`**：借出状态记录在共享内核（`Rc<RefCell>`，无 unsafe），可同时持有多个句柄，池先 drop 也安全
- **`with_stdlib_module(name)`**：通用单模块加载，未知模块名返回 `Err`（原 16 个 `with_stdlib_X` 构造器删除）
- 新示例：`examples/generator_demo.aether`、`examples/lazy_demo.aether`

### 修复

- **大数乘法饱和 bug**：`Number × Fraction` 原先经 `BigInt::from(a as i64)` 提升，超出 i64 范围静默截断；改为十进制字符串精确提升
- **大整数热循环 O(n²) 退化**：`Ratio` 算术运算符内置 gcd 规约，num-bigint 的二进制 gcd 对大整数逐位移位——阶乘累乘（2000 次乘法）修复前需 30 秒以上；分母为 1 的整数乘法现直接构造结果跳过规约，20000!（77338 位）0.65 秒内完成（新增 spec 性能回归用例守住该路径）
- **字符串字节/字符混乱**：`LEN/STRLEN/CHARAT/STRSLICE/INDEXOF/s[i]` 全部统一 Unicode 字符语义——`LEN("你好")` 为 2、`CHARAT("你好", -1)` 为 `"好"`（原按字节换算会切进多字节中间）
- **尾递归优化使死代码复活**：尾调用 `Return` 改写为参数更新后"自然落空"，同块后续语句每轮重复执行；现改写追加 `Continue` 并配合不可达代码消除，`Return` 之后的语句保证永不执行
- **个税税率表错误**：`CALC_PERSONAL_TAX` 补全 7 档年度累进表；`CALC_GROSS_FROM_NET` 从迭代逼近改为单调二分，全档往返误差 < 0.01 元；年终奖单独计税按"除以 12 定档"
- **AST 缓存哈希碰撞**：命中只比对 u64 哈希，碰撞时返回错误 AST；现命中先比对源码全文；LRU 用真实顺序队列替换随机淘汰
- **trace 双缓冲合并**：字符串缓冲与结构化条目两套并存且不同步；现结构化条目为唯一事实来源，`--trace` 输出由其派生

### 变更

- **尾递归/常量折叠/死代码消除差分验证**：优化开启与关闭的执行结果与 TRACE 序列必须一致（新增 spec_tail_recursion 差分电池）
- **跨类型相等保持严格**（有意设计）：`5 == TO_FRACTION(5)` 为 `False`，需显式转换后比较
- `help()` 覆盖全部 192 个注册函数（含无文档函数列出名字）并新增生成器、结构化跟踪等 15 个分类
- 文档全面对齐实现：README、PAYROLL_GUIDE（78→72 个函数、个税口径、`Let`→`Set` 语法修正）、rustdoc 示例块全部可解析、examples 全部可运行且不再引用未注册函数
- 冗余清理：payroll/mod.rs 死代码（338 行）、`EnvironmentPool`、`FunctionDoc` 双文档系统、ureq/chrono/lazy_static/crossbeam/criterion 依赖、`build.rs`、math.rs 40 行×11 样板宏化、CLI runner JSON 分支合并

### 测试

- 新增 12 个 BDD spec 文件（57 个用例）：数值精度、值相等、字符串语义、Generator、Lazy、尾递归差分、条件断点、薪酬、安全 API、缓存与池、文档示例防漂移、stdlib 编译冒烟——全部先红后绿实现

## [0.5.4 之前未发布内容]（并入 0.6.0 一起发布）

以下条目原计划随 0.5.4 发布，因尚未打 tag 一并入 0.6.0。

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
- CONTRIBUTING、LICENSE 的简体中文版本

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
