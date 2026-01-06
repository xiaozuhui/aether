# Aether 模块系统设计（Import/Export + 可插拔 Resolver）

> 目标：让 Aether 在**通用语言模式（CLI/脚本）**具备可用的模块化能力（Import/Export），同时在**DSL 嵌入模式**下可默认禁用或沙箱化加载来源，避免扩大攻击面。
>
> 本文是实现前的设计草案（MVP 规格），用于约束语义与边界，避免边写边变。

---

## 1. 背景与动机

Aether 当前已支持 `Import/Export` 的**语法解析与 AST 节点**，但运行时尚未实现：执行到 Import/Export 会报错未实现。

Aether 的定位同时覆盖：

- **DSL 模式**：脚本能力受控，数据/函数主要由 Rust 宿主注入；通常不需要脚本主动加载文件。
- **通用语言模式**：脚本作者需要分文件组织、复用代码，`Import/Export` 是基础能力。

因此需要一个模块系统：

- 统一 `Import/Export` 的语义
- 把“模块源码从哪里来”抽象为可插拔的 **Resolver**
- 在 DSL 模式默认可禁用 Import（或仅允许宿主白名单来源）

---

## 2. 设计目标（Goals）

- **G1 语言级模块化**：
  - `Import ... From "..."` 能真正导入符号
  - `Export NAME` 能真正导出符号
  - 模块有边界：非导出符号不对外可见

- **G2 可插拔来源（Resolver）**：
  - CLI 默认使用文件系统 resolver
  - 嵌入式 DSL 默认禁用 resolver（或由宿主提供内存/DB resolver）

- **G3 安全与沙箱**：
  - Import 不应成为绕过权限的隐蔽通道
  - 支持禁用文件 IO 的“沙箱 resolver”策略（白名单/路径限制/禁止绝对路径等）

- **G4 可预测与可调试**：
  - 模块只执行一次（缓存）
  - 有清晰的错误类型（模块找不到、循环依赖、导出缺失等）

- **G5 性能**：
  - 缓存解析/执行结果（模块缓存）
  - 保持现有 AST cache 的优势

---

## 3. 非目标（Non-goals）

MVP 不做：

- 包管理器（如 npm/cargo 风格 registry）
- 复杂的模块搜索路径（可先支持最小 set：相对路径 + 扩展名）
- 热更新/动态卸载（可由宿主自行实现版本化 specifier）
- 复杂的命名空间/默认导出/星号导入等高级语法

---

## 4. 语法（当前已存在，保持兼容）

### 4.1 Import

- 单个名称：

```aether
Import FOO From "./foo.aether"
Import FOO As BAR From "./foo.aether"
```

- 多个名称：

```aether
Import {FOO, BAR} From "./foo.aether"
Import {FOO as F, BAR} From "./foo.aether"
```

### 4.2 Export

```aether
Export FOO
Export BAR
```

> 约束建议：`Export NAME` 只能用于“模块文件”的顶层语句（MVP 先不强制也可，但建议在实现时约束，以便语义清晰）。

---

## 5. 核心语义（Semantics）

### 5.1 模块边界

- 每个模块在加载时拥有独立的**模块环境**（Module Environment），不直接污染导入者。
- 导入者只能通过 `Export` 看到模块导出的符号。

### 5.2 模块执行（一次性）

- 模块源码会被解析为 Program 并在模块环境中执行。
- **同一模块 specifier 在同一 resolver 语境下只执行一次**（Module Cache）。
- 再次导入时直接复用已执行的模块导出表。

### 5.3 Export 表

- 模块内部维护导出表：`export_table: HashMap<String, Value>`。
- 执行到 `Export NAME` 时：
  - 在模块环境查找 `NAME`，若不存在 → 抛出运行时错误（建议错误类型：`ExportNotFound(NAME)`）。
  - 存在则将其值复制/引用到 `export_table`。

> Value 的共享语义：由于 Aether 目前 Value 多为可 clone 的持有结构，MVP 可使用 clone。若后续引入共享引用，需要明确生命周期与可变性。

### 5.4 Import 绑定

Import 流程（概念步骤）：

1. 使用 resolver 将 `path/specifier` 解析为模块源码（或模块 ID）。
2. 从模块缓存获取模块实例；若未加载：解析 + 执行模块，生成 export_table。
3. 将请求的符号从 export_table 取出，绑定到导入者当前环境：
   - `Import FOO From ...` 等价于 `Import {FOO} From ...`
   - `as` 别名则绑定为别名

导入缺失符号 → 运行时错误：`ImportNotExported(module, name)`。

### 5.5 循环依赖

- 若模块加载栈中出现重复（A → B → A），报错：`CircularImport(path_chain)`。
- MVP 直接报错即可；更高级的“部分初始化”语义（像 JS）先不做。

---

## 6. Resolver 抽象（关键）

### 6.1 为什么需要 Resolver

`Import ... From "..."` 的字符串不应默认隐式映射到真实文件读取，否则 DSL 会引入不必要的攻击面。

Resolver 将“怎么拿到模块源码”从语言中剥离：

- 通用语言：文件系统 resolver
- DSL：禁用 resolver 或仅允许宿主内存/DB resolver

### 6.2 Resolver 接口（概念）

MVP 建议接口返回“模块源码 + 模块标识”：

- 输入：`specifier: &str`（原始字符串）
- 输出：
  - `resolved_id: String`（用于缓存 key 的规范化 ID，例如绝对规范化路径或 `db://x@v1`）
  - `source: String`（模块源码）

可能的错误：

- `ModuleNotFound(specifier)`
- `ModuleAccessDenied(specifier)`
- `InvalidModuleSpecifier(specifier)`

### 6.3 三种推荐实现

- **FileSystemResolver（CLI 默认）**
  - 允许读取 `.aether` 文件
  - 解析相对路径，以当前脚本目录为 base
  - 可选：支持搜索路径（MVP 可不做）

- **DisabledResolver（DSL 默认）**
  - 任意 Import → 直接报错 `ImportDisabled`

- **HostResolver（嵌入式可选）**
  - 宿主提供一个 `HashMap<module_id, source>` 或回调到 DB
  - 可用于“模块来自 DB”，但仍由宿主决定白名单与版本

---

## 7. 安全与权限模型

### 7.1 原则

- **Import 不应成为绕过 IO 权限的隐蔽通道**。
- “从哪里加载模块”属于宿主能力，不属于脚本能力。

### 7.2 DSL 默认策略

- `Aether::new()`（DSL）默认：
  - `resolver = DisabledResolver`
  - 即使脚本写 Import 也无法加载任何模块

### 7.3 CLI 默认策略

- CLI 默认：
  - `resolver = FileSystemResolver`
  - 可选择强制启用 IO 权限或仅为 resolver 开放文件读取（取决于你希望的权限粒度）

建议的最小一致性：

- CLI 本来就“可信脚本”取向，默认启用文件 resolver 合理。
- DSL 默认禁用，保持安全。

### 7.4 沙箱 resolver（可禁文件 IO）

如果你希望 DSL 仍可 import（但来源受控），则使用 HostResolver：

- 只允许 `mem://`、`db://` 等 scheme
- 拒绝绝对路径与 `..` 路径穿越
- 对 module_id 做版本化（例如 `db://pricing@v3`）确保可复现

---

## 8. 缓存策略

### 8.1 模块缓存 Key

缓存 key 应使用 resolver 输出的 `resolved_id`（规范化后的稳定标识），而不是原始 specifier。

### 8.2 缓存内容

MVP 缓存建议存：

- 模块导出表（`export_table`）
- 可选：模块执行后环境（如需调试或支持 namespace import）

### 8.3 与现有 AST Cache 的关系

- 模块系统需要自己的 `ModuleCache`（按 module_id 缓存）
- 每个模块内部解析也可以复用现有 AST cache（如果 cache 的 key 能区分模块来源）

---

## 9. 错误模型（建议）

导入/导出相关错误最好独立出来（便于 CLI 显示和宿主处理）：

- `ImportDisabled`
- `ModuleNotFound(specifier)`
- `ModuleAccessDenied(specifier)`
- `InvalidModuleSpecifier(specifier)`
- `CircularImport(chain)`
- `ImportNotExported { module_id, name }`
- `ExportNotFound { name }`
- `ModuleParseError { module_id, err }`
- `ModuleRuntimeError { module_id, err }`

---

## 10. CLI 与嵌入式 API 规划（概念）

### 10.1 CLI

- `aether main.aether`：默认启用文件 resolver
- 将来可加：
  - `--module-root <dir>`
  - `--no-import`（强制禁用 import）

### 10.2 Rust 嵌入

- DSL：默认禁用 import
- 如果宿主需要模块化：
  - 构造时传入 resolver
  - 或提供 setter（例如 `engine.set_resolver(...)`）

（具体 API 形式可在实现阶段根据 Rust trait/object safety 决定。）

---

## 11. 与“宿主注入（B 方案）”的关系

宿主注入方案本质上是“模块系统的一种装配方式”：

- 宿主从 DB 拉取函数定义（模块源码片段）
- 宿主决定注入哪些函数、哪些数据
- 通过隔离作用域（如 `with_isolated_scope`）保证每次执行不污染

模块系统实现后：

- DSL 场景仍可继续使用宿主注入（无需 Import）
- 通用语言场景可使用 Import 分文件
- 如果需要“声明式依赖”，也可把 Import 设计为“触发 resolver 的导入”或“仅声明 + 宿主校验”两种模式

---

## 12. 测试计划（MVP）

- **基础导入导出**：
  - 模块导出函数/常量，主脚本导入并调用
- **别名**：`as` 绑定正确
- **缺失导出**：导入未导出符号报错
- **Export 未定义**：导出不存在符号报错
- **只执行一次**：模块顶层 `TRACE` 或计数器验证只执行一次
- **循环依赖**：A→B→A 报错链路清晰
- **DSL 禁用**：DisabledResolver 下 Import 直接失败

---

## 13. 分阶段落地建议

- **Phase 1（最小通用语言可用）**
  - FileSystemResolver（相对路径）
  - export_table + import binding
  - module cache + cycle detection

- **Phase 2（DSL 友好）**
  - DisabledResolver 默认
  - HostResolver（内存/DB）
  - 明确权限/沙箱规则

- **Phase 3（增强）**
  - 搜索路径、标准库作为模块导入、开发体验工具

---

## 14. Open Questions（需要你拍板的语义点）

1. `Import NAME From "path"`：
   - 解释为导入一个导出符号（语法糖）
   - 还是导入模块 namespace（`NAME.xxx`）？
  
2. 模块顶层是否允许任意语句？
   - 允许（更通用，但副作用要靠“只执行一次 + 隔离”控制）
   - 限制为声明（更安全，但限制能力）

3. CLI 的默认行为：
   - 是否强制要求文件系统权限开启才允许 Import（更一致）
   - 或把 Import 文件读取当成 CLI 自身能力（更简单）

---

如果你确认上述语义点（特别是第 1 条），这份 spec 就可以直接作为实现的验收标准。
