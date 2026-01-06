# Aether 规划路线图（DSL 优先 + 通用语言能力）

> 日期：2026-01-06
>
> 总体目标：Aether 同时满足
>
> - **DSL 嵌入模式**：默认安全（禁 IO、禁 Import）、可注入、可隔离、可观测
> - **通用语言/CLI 模式**：可分文件组织代码（Import/Export）、可复用、可调试

---

## 设计原则

- **默认安全**：库模式（`Aether::new()`）默认禁 IO，默认禁 Import；任何扩权必须显式。
- **可定位优先**：优先补齐错误模型、调用栈、import 链路；避免“字符串拼接 + 找不到来源”。
- **可插拔**：模块加载通过 resolver 抽象；宿主可切换文件/内存/DB 等来源。
- **可验证**：每阶段都必须有 tests + examples + 文档同步。

---

## 已确认决策（2026-01-06）

1. **`as` 关键字**：已支持 `As`（并兼容旧写法 `as`）。
2. **命名空间导入**：已支持 `Import M From "./math"`，并可通过 `M["X"]` 访问导出（`M.X` 作为可选语法后续再评估/投入）。
3. **模块缓存与 `reset_env()`**：模块缓存可以默认保留；但需要提供显式 API 清理缓存（便于热更新/多租户）。

---

## 阶段 0：文档与示例闭环（当前）

目标：让新用户可以按 README/Learning 跑通“多文件 Import/Export + eval_file”，同时不破坏 DSL 的默认安全。

交付：

- README/Learning：清晰区分 DSL/CLI 模式下 Import 行为与启用方式
- examples：最小多文件模块示例 + Rust `eval_file` 示例
- tests：import/export 基础、循环依赖、缺失导出、DSL 默认禁用、`eval_file` 不泄漏 base

验收：

- `aether examples/module_import/main.aether` 可运行
- `cargo test` 中相关模块测试稳定通过

---

## 阶段 1：错误模型 + 调用栈 + import 链（优先级最高）

目标：让“出错能定位”，把 Import/Export 的错误从字符串拼接升级到结构化错误，并带足上下文。

交付：

- 结构化错误类型：ImportDisabled/NotFound/AccessDenied/ParseFailed/NotExported/CircularImport 等
- 调用栈（函数调用 frame）
- import 链路（main -> a -> b …）
- CLI 错误展示统一格式（人类可读）

验收：

- 任意 import/函数调用错误都能看到来源链路与调用栈

---

## 阶段 2：安全与沙箱能力产品化（DSL 护城河）

目标：把“可控扩权”做成稳定能力（root_dir 沙箱、内存/DB resolver、缓存生命周期）。

交付：

- 文件 resolver：root_dir 限制完善 + 越权路径测试（`..`、绝对路径等）
- 模块缓存生命周期：提供显式清理 API；明确与 `reset_env()` 的关系
- 可观测指标：模块加载次数、缓存命中、TRACE 条数、执行耗时（最小可用）

验收：

- DSL 宿主可安全允许“仅 root_dir 内导入”，且无法越权

---

## 阶段 3：语言表达力与工具链（按定位择优投入）

A) DSL 产品路线（务实）：执行限制（步数/递归/内存）、结构化 TRACE 事件、规则调试体验

B) 通用语言路线（语言化）：

- 命名空间导入与更多导入语法（冲突规则、批量导入）
- 更强的数据结构（record/struct 或 enum/variant）
- formatter + 最小 LSP（诊断/跳转）

---

## 下一步（建议执行顺序）

1. 完成文档与 examples 的同步（阶段 0 收尾）
2. 落地结构化错误 + import 链路（阶段 1）
3. 设计并实现 `As` 关键字迁移策略（兼容期是否同时支持 `as`/`As`）
4. 设计命名空间导入的语义与访问方式（`M.X` vs `M["X"]`）并补测试
