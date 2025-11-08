# Aether 类（Class）和接口（Interface）设计方案

## 目录

1. [概述](#概述)
2. [为什么需要类和接口](#为什么需要类和接口)
3. [设计原则](#设计原则)
4. [类（Class）设计](#类class设计)
5. [接口（Interface）设计](#接口interface设计)
6. [实现方案](#实现方案)
7. [语法示例](#语法示例)
8. [与现有特性的集成](#与现有特性的集成)
9. [实现路线图](#实现路线图)

---

## 概述

类（Class）和接口（Interface）是面向对象编程的核心概念。为 Aether 引入这些特性将：

- 提供更好的代码组织和封装
- 支持多态和抽象
- 增强代码复用性
- 提供类型约束和契约
- 更好地支持大型项目开发

## 为什么需要类和接口

### 当前 Aether 的局限性

1. **缺乏数据封装**：字典虽然可以模拟对象，但没有方法绑定和访问控制
2. **没有继承机制**：代码复用主要依赖函数组合
3. **缺少类型约束**：无法定义数据契约和行为约束
4. **状态管理困难**：复杂对象的状态管理需要手动维护

### 类和接口可以解决的问题

```aether
// 当前方式：使用字典模拟对象
Func CREATE_PERSON(NAME, AGE) {
    Return {
        "name": NAME,
        "age": AGE
    }
}

Func GREET_PERSON(PERSON) {
    Return "Hello, " + PERSON["name"]
}

// 使用类的方式：更清晰、更安全
Class Person {
    Var NAME
    Var AGE
    
    Func New(name, age) {
        Set This.NAME name
        Set This.AGE age
    }
    
    Func Greet() {
        Return "Hello, " + This.NAME
    }
}
```

---

## 设计原则

### 1. 保持简洁

Aether 是一门简洁的语言，类和接口的设计应该：

- 语法直观易懂
- 避免过度复杂的特性（如多重继承）
- 与现有语法风格保持一致

### 2. 兼容现有代码

- 不破坏现有的函数式编程范式
- 字典、数组等数据结构继续可用
- 可以渐进式地引入类

### 3. 性能考虑

- 类实例应该高效（类似字典的性能）
- 方法调用不应有显著开销
- 支持延迟计算和优化

### 4. 遵循 Aether 命名约定

- 类名：首字母大写驼峰（PascalCase）如 `Person`, `HttpClient`
- 方法名：首字母大写驼峰
- 实例变量：全大写+下划线（保持一致性）或首字母小写驼峰
- 关键字：首字母大写（`Class`, `Interface`, `Extends`, `Implements`）

---

## 类（Class）设计

### 核心特性

#### 1. 类定义

```aether
Class ClassName {
    // 实例变量声明
    Var INSTANCE_VAR1
    Var INSTANCE_VAR2
    
    // 构造函数
    Func New(param1, param2) {
        Set This.INSTANCE_VAR1 param1
        Set This.INSTANCE_VAR2 param2
    }
    
    // 实例方法
    Func MethodName(param) {
        // 方法体
        Return This.INSTANCE_VAR1 + param
    }
    
    // 静态方法（类方法）
    Static Func StaticMethod(param) {
        // 不能访问 This
        Return param * 2
    }
}
```

#### 2. 创建实例

```aether
// 使用 New 关键字创建实例
Set person Person.New("Alice", 30)

// 或者简化语法
Set person New Person("Alice", 30)
```

#### 3. 访问成员

```aether
// 访问属性
Println(person.NAME)

// 调用方法
Set greeting person.Greet()

// 调用静态方法
Set result Person.StaticMethod(10)
```

#### 4. 继承

```aether
Class Employee Extends Person {
    Var EMPLOYEE_ID
    Var SALARY
    
    Func New(name, age, id, salary) {
        // 调用父类构造函数
        Super.New(name, age)
        Set This.EMPLOYEE_ID id
        Set This.SALARY salary
    }
    
    // 重写方法
    Func Greet() {
        Set base_greeting Super.Greet()
        Return base_greeting + " (Employee #" + ToString(This.EMPLOYEE_ID) + ")"
    }
    
    // 新方法
    Func GetSalary() {
        Return This.SALARY
    }
}
```

#### 5. 访问控制

```aether
Class BankAccount {
    Public Var ACCOUNT_NUMBER
    Private Var BALANCE  // 只能在类内部访问
    
    Func New(account_number, initial_balance) {
        Set This.ACCOUNT_NUMBER account_number
        Set This.BALANCE initial_balance
    }
    
    Public Func Deposit(amount) {
        If (amount > 0) {
            Set This.BALANCE (This.BALANCE + amount)
            Return True
        }
        Return False
    }
    
    Public Func GetBalance() {
        Return This.BALANCE
    }
    
    Private Func ValidateTransaction(amount) {
        // 私有辅助方法
        Return amount > 0 && amount <= This.BALANCE
    }
}
```

---

## 接口（Interface）设计

### 核心特性

#### 1. 接口定义

```aether
Interface Drawable {
    // 方法签名（不包含实现）
    Func Draw()
    Func GetBounds()
}

Interface Movable {
    Func Move(x, y)
    Func GetPosition()
}
```

#### 2. 实现接口

```aether
Class Circle Implements Drawable, Movable {
    Var X
    Var Y
    Var RADIUS
    
    Func New(x, y, radius) {
        Set This.X x
        Set This.Y y
        Set This.RADIUS radius
    }
    
    // 实现 Drawable 接口
    Func Draw() {
        Println("Drawing circle at (" + ToString(This.X) + "," + ToString(This.Y) + ")")
    }
    
    Func GetBounds() {
        Return {
            "x": This.X - This.RADIUS,
            "y": This.Y - This.RADIUS,
            "width": This.RADIUS * 2,
            "height": This.RADIUS * 2
        }
    }
    
    // 实现 Movable 接口
    Func Move(x, y) {
        Set This.X (This.X + x)
        Set This.Y (This.Y + y)
    }
    
    Func GetPosition() {
        Return {"x": This.X, "y": This.Y}
    }
}
```

#### 3. 接口作为类型约束

```aether
// 函数接受任何实现了 Drawable 接口的对象
Func RenderShape(shape Drawable) {
    shape.Draw()
    Set bounds shape.GetBounds()
    Println("Bounds: " + ToString(bounds))
}

Set circle New Circle(10, 20, 5)
RenderShape(circle)  // 正确

Set point {"x": 10, "y": 20}
RenderShape(point)   // 错误：point 没有实现 Drawable 接口
```

#### 4. 接口继承

```aether
Interface Shape Extends Drawable {
    Func GetArea()
    Func GetPerimeter()
}

Class Rectangle Implements Shape {
    Var X
    Var Y
    Var WIDTH
    Var HEIGHT
    
    // 必须实现 Drawable 的方法
    Func Draw() {
        Println("Drawing rectangle")
    }
    
    Func GetBounds() {
        Return {"x": This.X, "y": This.Y, "width": This.WIDTH, "height": This.HEIGHT}
    }
    
    // 必须实现 Shape 的方法
    Func GetArea() {
        Return This.WIDTH * This.HEIGHT
    }
    
    Func GetPerimeter() {
        Return 2 * (This.WIDTH + This.HEIGHT)
    }
}
```

---

## 实现方案

### 方案1：基于字典的轻量级实现（推荐）

**优点**：

- 实现简单，性能好
- 与现有系统无缝集成
- 易于调试

**实现思路**：

```rust
// 在 Rust 代码中
pub enum Value {
    // ... 现有类型
    
    // 新增类实例类型
    Instance {
        class_name: String,
        fields: HashMap<String, Value>,
        methods: HashMap<String, Rc<Function>>,
    },
}
```

类定义在编译时被转换为：

1. 一个构造函数
2. 方法表
3. 字段描述

```aether
// 源代码
Class Point {
    Var X
    Var Y
    
    Func New(x, y) {
        Set This.X x
        Set This.Y y
    }
    
    Func Distance(other) {
        Set dx (This.X - other.X)
        Set dy (This.Y - other.Y)
        Return SQRT(dx * dx + dy * dy)
    }
}

// 内部转换为
Func Point_New(x, y) {
    Return {
        "__class__": "Point",
        "__methods__": {
            "Distance": Point_Distance
        },
        "X": x,
        "Y": y
    }
}

Func Point_Distance(this, other) {
    Set dx (this["X"] - other["X"])
    Set dy (this["Y"] - other["Y"])
    Return SQRT(dx * dx + dy * dy)
}
```

### 方案2：原生类系统

**优点**：

- 更好的性能优化空间
- 更强的类型检查
- 支持更高级的特性

**缺点**：

- 实现复杂度高
- 需要更多的测试

---

## 语法示例

### 完整示例：形状绘制系统

```aether
// 定义接口
Interface Drawable {
    Func Draw()
    Func GetArea()
}

// 基类
Class Shape {
    Var COLOR
    
    Func New(color) {
        Set This.COLOR color
    }
    
    Func Describe() {
        Return "A " + This.COLOR + " shape"
    }
}

// 派生类
Class Circle Extends Shape Implements Drawable {
    Var X
    Var Y
    Var RADIUS
    
    Func New(x, y, radius, color) {
        Super.New(color)
        Set This.X x
        Set This.Y y
        Set This.RADIUS radius
    }
    
    Func Draw() {
        Println("Drawing " + This.COLOR + " circle at (" + 
                ToString(This.X) + "," + ToString(This.Y) + 
                ") with radius " + ToString(This.RADIUS))
    }
    
    Func GetArea() {
        Return 3.14159 * This.RADIUS * This.RADIUS
    }
}

Class Rectangle Extends Shape Implements Drawable {
    Var X
    Var Y
    Var WIDTH
    Var HEIGHT
    
    Func New(x, y, width, height, color) {
        Super.New(color)
        Set This.X x
        Set This.Y y
        Set This.WIDTH width
        Set This.HEIGHT height
    }
    
    Func Draw() {
        Println("Drawing " + This.COLOR + " rectangle at (" + 
                ToString(This.X) + "," + ToString(This.Y) + 
                ") " + ToString(This.WIDTH) + "x" + ToString(This.HEIGHT))
    }
    
    Func GetArea() {
        Return This.WIDTH * This.HEIGHT
    }
}

// 使用
Set shapes []
Set shapes PUSH(shapes, New Circle(10, 20, 5, "red"))
Set shapes PUSH(shapes, New Rectangle(30, 40, 10, 15, "blue"))

Func DrawAll(shapes) {
    Set i 0
    While (i < LEN(shapes)) {
        Set shape shapes[i]
        shape.Draw()
        Println("Area: " + ToString(shape.GetArea()))
        Set i (i + 1)
    }
}

DrawAll(shapes)
```

---

## 与现有特性的集成

### 1. 与字典的互操作

```aether
// 类实例可以转换为字典
Set person New Person("Alice", 30)
Set dict person.ToDict()

// 字典可以作为类构造的参数
Set person2 Person.FromDict(dict)
```

### 2. 与函数式编程的结合

```aether
// 类的方法可以作为一等公民
Set greet_func person.Greet
greet_func()  // 自动绑定 this

// 高阶函数
Set people [person1, person2, person3]
Set greetings MAP(people, Func(p) { Return p.Greet() })
```

### 3. 与现有类型系统的集成

```aether
// TYPE 函数应该返回类名
Set person New Person("Alice", 30)
Println(TYPE(person))  // "Person"

// 类型检查
If (INSTANCEOF(person, Person)) {
    Println("person is a Person")
}
```

---

## 实现路线图

### 阶段1：基础类支持（MVP）

**目标**：实现最小可用的类系统

- [ ] 类定义语法（Class 关键字）
- [ ] 构造函数（New 方法）
- [ ] 实例变量
- [ ] 实例方法
- [ ] This 关键字
- [ ] 点语法访问成员

**时间估计**：2-3周

### 阶段2：继承和多态

- [ ] Extends 关键字
- [ ] Super 关键字
- [ ] 方法重写
- [ ] 虚方法调用

**时间估计**：2-3周

### 阶段3：接口支持

- [ ] Interface 关键字
- [ ] Implements 关键字
- [ ] 接口类型检查
- [ ] 多接口实现

**时间估计**：2周

### 阶段4：高级特性

- [ ] 静态方法和变量
- [ ] 访问控制（Public/Private）
- [ ] 属性（Getter/Setter）
- [ ] 抽象类和方法
- [ ] 运算符重载

**时间估计**：3-4周

### 阶段5：优化和工具

- [ ] 性能优化
- [ ] 更好的错误信息
- [ ] 类的序列化/反序列化
- [ ] 反射 API
- [ ] 类型推断改进

**时间估计**：2-3周

---

## 考虑的替代方案

### 方案A：原型继承（类似 JavaScript）

```aether
Set PersonProto {
    "greet": Func() {
        Return "Hello, " + This["name"]
    }
}

Func CreatePerson(name, age) {
    Set person {"name": name, "age": age}
    Set person["__proto__"] PersonProto
    Return person
}
```

**优点**：灵活，动态
**缺点**：不够直观，难以优化

### 方案B：Trait 系统（类似 Rust）

```aether
Trait Drawable {
    Func Draw()
}

Struct Circle {
    Var x
    Var y
    Var radius
}

Impl Drawable For Circle {
    Func Draw() {
        Println("Drawing circle")
    }
}
```

**优点**：组合优于继承，更灵活
**缺点**：学习曲线陡峭，可能过于复杂

---

## 结论

引入类和接口将显著增强 Aether 的表达能力和适用范围。推荐采用：

1. **基于字典的轻量级类实现**作为起点
2. **单继承 + 多接口**的设计模式
3. **渐进式引入**，保持向后兼容
4. **简洁的语法**，与 Aether 风格一致

这种设计既保持了 Aether 的简洁性，又提供了面向对象编程的强大功能，使其能够更好地支持大型项目开发。

---

## 附录：参考资料

- Python 的类系统（简洁直观）
- TypeScript 的接口设计（灵活强大）
- Rust 的 Trait 系统（组合优于继承）
- Go 的接口实现（隐式实现，鸭子类型）

**建议下一步**：

1. 创建详细的语法规范文档
2. 实现原型解析器支持
3. 编写测试用例
4. 收集社区反馈
