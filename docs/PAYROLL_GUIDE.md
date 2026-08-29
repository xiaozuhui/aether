# 薪酬计算模块使用指南

## 概述

Aether 薪酬计算模块提供 72 个函数，覆盖薪资折算、加班费、个税、社保公积金、考勤扣款、奖金、津贴与统计场景。折算标准基于中国劳动法的月度计薪天数 21.75 天（(365 − 104) ÷ 12）。

本文所有函数名都以反引号标注；示例使用 `Set` 赋值、`//` 行注释。函数参数按位置传入，多个同名参数含义在各节说明。

## 模块结构

### 1. 基本工资计算（7 个）

| 函数 | 参数 | 说明 |
|------|------|------|
| `CALC_HOURLY_PAY` | 月薪[, 月工时=174] | 时薪（月薪 ÷ 月工时） |
| `CALC_DAILY_PAY` | 月薪[, 计薪天数=21.75] | 日薪 |
| `CALC_MONTHLY_FROM_HOURLY` | 时薪[, 月工时=174] | 由时薪折算月薪 |
| `CALC_ANNUAL_SALARY` | 月薪[, 月数=12] | 年薪 |
| `CALC_BASE_SALARY` | 基本工资, 实际出勤天数, 应出勤天数 | 按出勤率折算的基本工资 |
| `CALC_GROSS_SALARY` | 基本工资, 加班费, 奖金, 津贴 | 应发工资（四项求和） |
| `CALC_NET_SALARY` | 应发工资, 社保, 公积金, 个税, 其他扣款 | 实发工资（逐项扣减，不为负） |

```aether
Set MONTHLY 10000
Set HOURLY CALC_HOURLY_PAY(MONTHLY, 174)      // 月标准工时 174 小时
Set DAILY CALC_DAILY_PAY(MONTHLY, 21.75)
Set ANNUAL CALC_ANNUAL_SALARY(MONTHLY)
Set GROSS CALC_GROSS_SALARY(MONTHLY, 800, 0, 1500)   // 基本工资+加班费+奖金+津贴
Set NET CALC_NET_SALARY(GROSS, 2250, 0, 180, 0)       // 应发 − 社保 − 公积金 − 个税 − 其他
```

### 2. 加班费计算（5 个）

| 函数 | 参数 | 说明 |
|------|------|------|
| `CALC_OVERTIME_PAY` | 月薪, 加班小时 | 通用加班费（默认 1.5 倍） |
| `CALC_WEEKDAY_OVERTIME` | 月薪, 加班小时 | 工作日延时（1.5 倍） |
| `CALC_WEEKEND_OVERTIME` | 月薪, 加班小时 | 休息日（2 倍） |
| `CALC_HOLIDAY_OVERTIME` | 月薪, 加班小时 | 法定节假日（3 倍） |
| `CALC_TOTAL_OVERTIME` | 月薪, 工作日时, 休息日时, 节假日时 | 三类合计 |

```aether
Set TOTAL CALC_TOTAL_OVERTIME(10000, 10, 8, 4)   // 1.5x + 2x + 3x
```

### 3. 个人所得税（6 个）

**口径说明（重要，勿混用）**：

- `CALC_PERSONAL_TAX` 使用**年度累计**税率表（综合所得 7 档超额累进，下表）。月度应纳税所得额不能直接套用——月度按 5000 元/月起征预扣（用 `CALC_TAXABLE_INCOME`），年度汇算时再按累计应纳税所得额套用本表。
- `CALC_TAXABLE_INCOME` 是**月度**口径：入参为（应发工资, 社保, 公积金[, 专项附加扣除]），公式 max(0, 应发 − 社保 − 公积金 − 5000 起征点 − 专项附加)。
- `CALC_GROSS_FROM_NET` 的反推同样基于上述 7 档表与 5000 元/月起征点，用二分法求解，往返误差 < 0.01 元。

| 函数 | 参数 | 说明 |
|------|------|------|
| `CALC_PERSONAL_TAX` | 应纳税所得额 | 按 7 档年度表计算税额 |
| `CALC_TAXABLE_INCOME` | 应发工资, 社保, 公积金[, 专项附加] | max(0, 应发−社保−公积金−5000−专项附加) |
| `CALC_ANNUAL_BONUS_TAX` | 年终奖金额 | 单独计税：除以 12 定档，税率×全额 − 速算扣除×12 |
| `CALC_EFFECTIVE_TAX_RATE` | 税额, 总收入 | 实际税率（税额 ÷ 总收入） |
| `CALC_GROSS_FROM_NET` | 税后净额, 社保, 公积金 | 由税后反推税前（二分） |
| `CALC_TAX_REFUND` | 已缴税额, 应缴税额 | 退税额（正为退、负为补） |

年度 7 档税率表（单位：元）：

| 累计应纳税所得额 | 税率 | 速算扣除数 |
|------|------|------|
| ≤ 36,000 | 3% | 0 |
| 36,000 − 144,000 | 10% | 2,520 |
| 144,000 − 300,000 | 20% | 16,920 |
| 300,000 − 420,000 | 25% | 31,920 |
| 420,000 − 660,000 | 30% | 52,920 |
| 660,000 − 960,000 | 35% | 85,920 |
| > 960,000 | 45% | 181,920 |

```aether
// 月度预扣：月收入 15000，社保+公积金合计 2625
Set TAXABLE CALC_TAXABLE_INCOME(15000, 2625, 0)    // 7375 = 15000 − 2625 − 5000
// 年度汇算：累计应纳税所得额
Set ANNUAL_TAX CALC_PERSONAL_TAX(120000)           // 年度表
// 年终奖单独计税
Set BONUS_TAX CALC_ANNUAL_BONUS_TAX(36000)         // 36000/12=3000 → 3% 档
// 税后反推税前：目标净额 12000，社保 2100，公积金 1200
Set GROSS CALC_GROSS_FROM_NET(12000, 2100, 1200)
```

### 4. 社保公积金（10 个）

个人缴费比例：养老 8%、医疗 2%、失业 0.5%、公积金 12%。企业部分另计（工伤、生育仅企业缴纳）。

| 函数 | 参数 | 说明 |
|------|------|------|
| `CALC_PENSION_INSURANCE` | 缴费基数 | 养老（个人 8%） |
| `CALC_MEDICAL_INSURANCE` | 缴费基数 | 医疗（个人 2%） |
| `CALC_UNEMPLOYMENT_INSURANCE` | 缴费基数 | 失业（个人 0.5%） |
| `CALC_HOUSING_FUND` | 缴费基数 | 公积金（个人 12%） |
| `CALC_SOCIAL_INSURANCE` | 缴费基数 | 个人四项合计（22.5%） |
| `ADJUST_SOCIAL_BASE` | 实际工资, 下限, 上限 | 基数钳制到上下限 |
| `CALC_SOCIAL_BASE_LOWER` | 平均工资, 倍率 | 缴费下限（平均×倍率） |
| `CALC_SOCIAL_BASE_UPPER` | 平均工资, 倍率 | 缴费上限（平均×倍率） |
| `CALC_INJURY_INSURANCE` | 缴费基数 | 工伤（企业缴纳） |
| `CALC_MATERNITY_INSURANCE` | 缴费基数 | 生育（企业缴纳） |

```aether
Set BASE ADJUST_SOCIAL_BASE(18000, 4000, 20000)    // 超上限则取 20000
Set SOCIAL CALC_SOCIAL_INSURANCE(BASE)             // 养老+医疗+失业+公积金
```

### 5. 考勤扣款（7 个）

| 函数 | 参数 | 说明 |
|------|------|------|
| `CALC_ATTENDANCE_RATE` | 实际出勤天数, 应出勤天数 | 出勤率 |
| `CALC_LATE_DEDUCTION` | 迟到次数, 单次扣款 | 迟到扣款 |
| `CALC_EARLY_LEAVE_DEDUCTION` | 早退次数, 单次扣款 | 早退扣款 |
| `CALC_ABSENT_DEDUCTION` | 月薪, 旷工天数 | 旷工扣款（按 21.75 天折算日薪扣除） |
| `CALC_LEAVE_DEDUCTION` | 月薪, 请假天数 | 事假扣款 |
| `CALC_SICK_LEAVE_PAY` | 月薪, 病假天数, 工龄 | 病假工资（工龄越长比例越高，60%−100%） |
| `CALC_UNPAID_LEAVE_DEDUCTION` | 月薪, 旷工天数 | 无薪假扣款 |

```aether
Set LATE_FEE CALC_LATE_DEDUCTION(3, 50)             // 迟到 3 次，每次 50 元
Set SICK CALC_SICK_LEAVE_PAY(10000, 5, 3)           // 工龄 3 年 → 70% 档
```

### 6. 奖金计算（6 个）

| 函数 | 参数 | 说明 |
|------|------|------|
| `CALC_PERFORMANCE_PAY` | 基本工资, 绩效系数 | 绩效工资 |
| `CALC_ANNUAL_BONUS` | 月薪[, 月数=12][, 绩效系数=1] | 年终奖（后两个参数可省略） |
| `CALC_ATTENDANCE_BONUS` | 奖金基数, 出勤率 | 全勤奖（按出勤率折算） |
| `CALC_SALES_COMMISSION` | 销售额, 提成比例 | 销售提成 |
| `CALC_PROJECT_BONUS` | 项目金额, 分配比例 | 项目奖金 |
| `CALC_13TH_SALARY` | 月薪, 工作月数 | 13 薪 |

```aether
Set BONUS CALC_ANNUAL_BONUS(15000, 12, 1.2)        // 月薪×12×绩效系数
Set THIRTEENTH CALC_13TH_SALARY(15000, 12)
```

### 7. 津贴补贴（7 个）

全部为二元函数：

| 函数 | 参数 | 说明 |
|------|------|------|
| `CALC_MEAL_ALLOWANCE` | 每日标准, 出勤天数 | 餐补 |
| `CALC_TRANSPORT_ALLOWANCE` | 每日标准, 出勤天数 | 交通补贴 |
| `CALC_COMMUNICATION_ALLOWANCE` | 月度标准, 在职月数 | 通讯补贴 |
| `CALC_HOUSING_ALLOWANCE` | 月度标准, 在职月数 | 住房补贴 |
| `CALC_HIGH_TEMP_ALLOWANCE` | 月度标准, 高温天数 | 高温津贴 |
| `CALC_NIGHT_SHIFT_ALLOWANCE` | 单班标准, 夜班次数 | 夜班津贴 |
| `CALC_POSITION_ALLOWANCE` | 岗位基数, 岗位系数 | 岗位津贴 |

### 8. 薪资折算转换（12 个）

**单位转换**（21.75 天、8 小时/天标准）：

| 函数 | 说明 |
|------|------|
| `ANNUAL_TO_MONTHLY` / `MONTHLY_TO_ANNUAL` | 年 ↔ 月 |
| `MONTHLY_TO_DAILY` / `DAILY_TO_MONTHLY` | 月 ↔ 日 |
| `MONTHLY_TO_HOURLY` / `HOURLY_TO_MONTHLY` | 月 ↔ 时 |

**按方式折算**：

| 函数 | 参数 | 说明 |
|------|------|------|
| `PRORATE_BY_NATURAL_DAYS` | 月薪, 工作天数, 自然总天数 | 按自然天折算 |
| `PRORATE_BY_LEGAL_DAYS` | 月薪, 工作天数 | 按 21.75 天折算（法定标准） |
| `PRORATE_BY_WORKDAYS` | 月薪, 工作天数, 应出勤天数 | 按工作日折算 |

**入离职场景**（折算方式：0 = 自然天，1 = 21.75 天，2 = 工作日）：

| 函数 | 参数 | 说明 |
|------|------|------|
| `CALC_ONBOARDING_SALARY` | 月薪, 入职日(几号), 当月天数, 折算方式 | 入职当月工资 |
| `CALC_RESIGNATION_SALARY` | 月薪, 离职日(几号), 当月天数, 折算方式 | 离职当月工资 |
| `CALC_14TH_SALARY` | 月薪, 工作月数 | 14 薪 |

```aether
// 15 号入职，按 21.75 天标准折算
Set ONBOARD CALC_ONBOARDING_SALARY(10000, 15, 30, 1)
// 按自然天折算
Set SALARY PRORATE_BY_NATURAL_DAYS(10000, 16, 30)
```

### 9. 日期时间计算（6 个）

本模块只提供**可由星期与天数真实推算**的函数。历史上存在过一批"法定节假日"函数，但它们无法在无法定节假日数据源的情况下真实计算，已随 0.6.0 删除——节假日判断请由宿主程序给出布尔值后传入 `IS_WORKDAY`。

| 函数 | 参数 | 说明 |
|------|------|------|
| `CALC_NATURAL_DAYS` | 开始日(几号), 结束日(几号) | 区间自然天数（含首尾） |
| `CALC_WEEKEND_DAYS` | 总天数, 起始星期 | 区间周末天数 |
| `IS_WORKDAY` | 星期, 是否节假日 | 是否工作日（周一至周五且非节假日） |
| `IS_WEEKEND` | 星期 | 是否周末（周六=6、周日=7） |
| `CALC_WORK_HOURS` | 工作天数 | 工时（每天 8 小时） |
| `CALC_MONTHLY_WORK_HOURS` | （无参数） | 月标准工时 174 小时（21.75 × 8） |

```aether
// 2026-08（周六起）：本月 9 个周末日
Set WEEKENDS CALC_WEEKEND_DAYS(31, 6)
// 宿主已知 8 号是调休节假日时
Set IS_WORK IS_WORKDAY(1, True)     // 周一但放假 → False
```

### 10. 统计分析（6 个）

统计函数接受**多个数值参数**（逐个列出，不是数组）：

| 函数 | 参数 | 说明 |
|------|------|------|
| `CALC_SALARY_AVERAGE` | 薪资1, 薪资2, … | 平均薪资 |
| `CALC_SALARY_MEDIAN` | 薪资1, 薪资2, … | 中位数 |
| `CALC_SALARY_RANGE` | 薪资1, 薪资2, … | 极差（最大 − 最小） |
| `CALC_SALARY_STD_DEV` | 薪资1, 薪资2, … | 标准差 |
| `CALC_PERCENTILE` | 百分位, 薪资1, 薪资2, … | 分位值（如 25 → P25） |
| `CALC_SALARY_DISTRIBUTION` | 薪资1, 薪资2, … | 分档分布（区间计数） |

```aether
Set AVG CALC_SALARY_AVERAGE(8000, 12000, 15000)
Set P25 CALC_PERCENTILE(25, 8000, 12000, 15000)
```

## 完整示例：月度工资计算

```aether
Func CALC_MONTHLY_PAYROLL(BASE_SALARY, OVERTIME_HOURS, ATTENDANCE_DAYS) {
    PRINTLN("=== 月度工资计算 ===")

    // 1. 加班费（工作日 1.5 倍）
    Set OVERTIME_PAY CALC_WEEKDAY_OVERTIME(BASE_SALARY, OVERTIME_HOURS)

    // 2. 餐补与交通补贴
    Set MEAL CALC_MEAL_ALLOWANCE(20, ATTENDANCE_DAYS)
    Set TRANSPORT CALC_TRANSPORT_ALLOWANCE(10, ATTENDANCE_DAYS)

    // 3. 应发工资
    Set GROSS CALC_GROSS_SALARY(BASE_SALARY, OVERTIME_PAY, 0, MEAL + TRANSPORT)
    PRINTLN("应发工资: " + TO_STRING(GROSS))

    // 4. 社保公积金（个人部分，含公积金 12%）
    Set SOCIAL CALC_SOCIAL_INSURANCE(BASE_SALARY)

    // 5. 个税（月度预扣口径：5000 元/月起征）
    // 社保合计（含公积金）拆成社保/公积金两处传入
    Set TAXABLE CALC_TAXABLE_INCOME(GROSS, SOCIAL, 0)
    Set TAX 0
    If (TAXABLE > 0) {
        Set TAX CALC_PERSONAL_TAX(TAXABLE)
    }
    PRINTLN("个税: " + TO_STRING(TAX))

    // 6. 实发工资
    Set NET CALC_NET_SALARY(GROSS, SOCIAL, 0, TAX, 0)
    PRINTLN("实发工资: " + TO_STRING(NET))

    Return NET
}

CALC_MONTHLY_PAYROLL(12000, 20, 22)
```

> 提示：`CALC_PERSONAL_TAX` 使用年度累计表；月度预扣与年度汇算存在口径差异，年度终了需汇算多退少补（见 `CALC_TAX_REFUND`）。

## 完整示例：年终奖与税金

```aether
Func CALC_YEAR_END_BONUS(MONTHLY_SALARY, PERFORMANCE) {
    Set BONUS CALC_ANNUAL_BONUS(MONTHLY_SALARY, 12, PERFORMANCE)
    PRINTLN("年终奖: " + TO_STRING(BONUS))

    // 年终奖单独计税：除以 12 定档
    Set BONUS_TAX CALC_ANNUAL_BONUS_TAX(BONUS)
    PRINTLN("年终奖个税: " + TO_STRING(BONUS_TAX))

    // 税后年终奖
    Set NET_BONUS BONUS - BONUS_TAX
    PRINTLN("税后年终奖: " + TO_STRING(NET_BONUS))

    // 13 薪
    Set THIRTEENTH CALC_13TH_SALARY(MONTHLY_SALARY, 12)
    PRINTLN("13薪: " + TO_STRING(THIRTEENTH))

    Return NET_BONUS
}

CALC_YEAR_END_BONUS(15000, 1.2)
```

## 重要说明

### 1. 21.75 天标准

月度计薪天数 21.75 天的由来：

```
(365 天 − 104 天周末) ÷ 12 个月 = 21.75 天
```

用于：日薪/时薪折算、月度薪资折算（`PRORATE_BY_LEGAL_DAYS`）、加班费基数计算。

### 2. 个税口径速查

- 月度预扣：月收入 − 5000 起征点 − 专项扣除（社保公积金）− 专项附加扣除 → 月度应纳税所得额。
- 年度汇算：按累计应纳税所得额套 7 档年度表（`CALC_PERSONAL_TAX`）。
- 年终奖：可单独计税（`CALC_ANNUAL_BONUS_TAX`，除以 12 定档），也可并入综合所得——两种方式税额可能不同。
- 专项附加扣除项目（子女教育、住房贷款利息、赡养老人等）由宿主计算后并入收入扣减，本模块不内置其规则。

### 3. 社保缴费比例（参考值，各地不同）

**个人缴费**：养老 8%、医疗 2%、失业 0.5%、公积金 12%——对应 `CALC_SOCIAL_INSURANCE` 的 22.5%。

**企业缴费**：养老 16%、医疗 10%、失业 0.5%、工伤 0.2−1.9%（`CALC_INJURY_INSURANCE`）、生育 0.8%（`CALC_MATERNITY_INSURANCE`）、公积金 12%。

缴费基数受上下限约束（`ADJUST_SOCIAL_BASE`、`CALC_SOCIAL_BASE_LOWER`、`CALC_SOCIAL_BASE_UPPER`）。

### 4. 加班费倍数

- 工作日延时：1.5 倍
- 休息日：2 倍
- 法定节假日：3 倍

## 相关文档

- [README 语言参考](../README.md)
- [错误报告指南](ERROR_REPORTING.md)
- [开发指南](../DEVELOPMENT.md)
- [变更日志](../CHANGELOG.md)
