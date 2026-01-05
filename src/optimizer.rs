// src/optimizer.rs
//! 代码优化器 - 包含尾递归优化、常量折叠等

use crate::ast::{BinOp, Expr, Program, Stmt, UnaryOp};

/// 代码优化器
pub struct Optimizer {
    /// 是否启用尾递归优化
    pub tail_recursion: bool,
    /// 是否启用常量折叠
    pub constant_folding: bool,
    /// 是否启用死代码消除
    pub dead_code_elimination: bool,
}

impl Optimizer {
    /// 创建新的优化器,所有优化默认启用
    pub fn new() -> Self {
        Optimizer {
            tail_recursion: true,
            constant_folding: true,
            dead_code_elimination: true,
        }
    }

    /// 优化整个程序
    pub fn optimize_program(&self, program: &Program) -> Program {
        let mut optimized = program.clone();

        // 常量折叠
        if self.constant_folding {
            optimized = self.fold_constants(optimized);
        }

        // 死代码消除
        if self.dead_code_elimination {
            optimized = self.eliminate_dead_code(optimized);
        }

        // 尾递归优化
        if self.tail_recursion {
            optimized = self.optimize_tail_recursion(optimized);
        }

        optimized
    }

    /// 常量折叠优化
    fn fold_constants(&self, program: Program) -> Program {
        program
            .into_iter()
            .map(|stmt| self.fold_stmt(stmt))
            .collect()
    }

    /// 折叠语句中的常量
    fn fold_stmt(&self, stmt: Stmt) -> Stmt {
        match stmt {
            Stmt::Set { name, value } => Stmt::Set {
                name,
                value: self.fold_expr(value),
            },
            Stmt::FuncDef { name, params, body } => Stmt::FuncDef {
                name,
                params,
                body: body.into_iter().map(|s| self.fold_stmt(s)).collect(),
            },
            Stmt::GeneratorDef { name, params, body } => Stmt::GeneratorDef {
                name,
                params,
                body: body.into_iter().map(|s| self.fold_stmt(s)).collect(),
            },
            Stmt::Return(expr) => Stmt::Return(self.fold_expr(expr)),
            Stmt::Yield(expr) => Stmt::Yield(self.fold_expr(expr)),
            Stmt::While { condition, body } => Stmt::While {
                condition: self.fold_expr(condition),
                body: body.into_iter().map(|s| self.fold_stmt(s)).collect(),
            },
            Stmt::For {
                var,
                iterable,
                body,
            } => Stmt::For {
                var,
                iterable: self.fold_expr(iterable),
                body: body.into_iter().map(|s| self.fold_stmt(s)).collect(),
            },
            Stmt::ForIndexed {
                index_var,
                value_var,
                iterable,
                body,
            } => Stmt::ForIndexed {
                index_var,
                value_var,
                iterable: self.fold_expr(iterable),
                body: body.into_iter().map(|s| self.fold_stmt(s)).collect(),
            },
            Stmt::Expression(expr) => Stmt::Expression(self.fold_expr(expr)),
            other => other,
        }
    }

    /// 折叠表达式中的常量
    #[allow(clippy::only_used_in_recursion)]
    fn fold_expr(&self, expr: Expr) -> Expr {
        match expr {
            // 二元运算常量折叠
            Expr::Binary { left, op, right } => {
                let left = self.fold_expr(*left);
                let right = self.fold_expr(*right);

                // 如果两边都是常量,直接计算结果
                if let (Expr::Number(l), Expr::Number(r)) = (&left, &right)
                    && let Some(result) = Self::eval_const_binary(*l, &op, *r)
                {
                    return Expr::Number(result);
                }

                Expr::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                }
            }

            // 一元运算常量折叠
            Expr::Unary { op, expr } => {
                let expr = self.fold_expr(*expr);

                if let Expr::Number(n) = expr {
                    match op {
                        UnaryOp::Minus => return Expr::Number(-n),
                        UnaryOp::Not => return Expr::Boolean(n == 0.0),
                    }
                }

                if let (UnaryOp::Not, Expr::Boolean(b)) = (&op, &expr) {
                    return Expr::Boolean(!b);
                }

                Expr::Unary {
                    op,
                    expr: Box::new(expr),
                }
            }

            // 递归处理其他表达式
            Expr::Call { func, args } => Expr::Call {
                func: Box::new(self.fold_expr(*func)),
                args: args.into_iter().map(|e| self.fold_expr(e)).collect(),
            },

            Expr::Array(elements) => {
                Expr::Array(elements.into_iter().map(|e| self.fold_expr(e)).collect())
            }

            Expr::Index { object, index } => Expr::Index {
                object: Box::new(self.fold_expr(*object)),
                index: Box::new(self.fold_expr(*index)),
            },

            other => other,
        }
    }

    /// 计算常量二元运算
    fn eval_const_binary(left: f64, op: &BinOp, right: f64) -> Option<f64> {
        match op {
            BinOp::Add => Some(left + right),
            BinOp::Subtract => Some(left - right),
            BinOp::Multiply => Some(left * right),
            BinOp::Divide if right != 0.0 => Some(left / right),
            BinOp::Modulo if right != 0.0 => Some(left % right),
            _ => None,
        }
    }

    /// 死代码消除
    fn eliminate_dead_code(&self, program: Program) -> Program {
        program
            .into_iter()
            .filter_map(|stmt| self.eliminate_dead_stmt(stmt))
            .collect()
    }

    /// 消除死语句
    fn eliminate_dead_stmt(&self, stmt: Stmt) -> Option<Stmt> {
        match stmt {
            // While循环的常量条件
            Stmt::While { condition, body } => {
                if let Expr::Boolean(false) = condition {
                    // 永远不执行的循环可以删除
                    return None;
                }

                Some(Stmt::While {
                    condition,
                    body: body
                        .into_iter()
                        .filter_map(|s| self.eliminate_dead_stmt(s))
                        .collect(),
                })
            }

            // 函数定义递归处理
            Stmt::FuncDef { name, params, body } => Some(Stmt::FuncDef {
                name,
                params,
                body: body
                    .into_iter()
                    .filter_map(|s| self.eliminate_dead_stmt(s))
                    .collect(),
            }),

            Stmt::GeneratorDef { name, params, body } => Some(Stmt::GeneratorDef {
                name,
                params,
                body: body
                    .into_iter()
                    .filter_map(|s| self.eliminate_dead_stmt(s))
                    .collect(),
            }),

            // 表达式语句中可能包含If表达式
            Stmt::Expression(expr) => Some(Stmt::Expression(self.eliminate_dead_expr(expr))),

            other => Some(other),
        }
    }

    /// 消除表达式中的死代码
    fn eliminate_dead_expr(&self, expr: Expr) -> Expr {
        match expr {
            Expr::If {
                condition,
                then_branch,
                elif_branches,
                else_branch,
            } => {
                if let Expr::Boolean(true) = *condition {
                    // 条件永远为真,简化为then分支
                    return Expr::If {
                        condition: Box::new(Expr::Boolean(true)),
                        then_branch,
                        elif_branches: vec![],
                        else_branch: None,
                    };
                }

                if let Expr::Boolean(false) = *condition {
                    // 条件永远为假,检查elif或else
                    if let Some(else_body) = else_branch {
                        // 简化为else块
                        return Expr::If {
                            condition: Box::new(Expr::Boolean(true)),
                            then_branch: else_body,
                            elif_branches: vec![],
                            else_branch: None,
                        };
                    }
                    // 没有else,返回null
                    return Expr::Null;
                }

                // 递归处理分支
                Expr::If {
                    condition,
                    then_branch: then_branch
                        .into_iter()
                        .filter_map(|s| self.eliminate_dead_stmt(s))
                        .collect(),
                    elif_branches: elif_branches
                        .into_iter()
                        .map(|(c, b)| {
                            (
                                self.eliminate_dead_expr(c),
                                b.into_iter()
                                    .filter_map(|s| self.eliminate_dead_stmt(s))
                                    .collect(),
                            )
                        })
                        .collect(),
                    else_branch: else_branch.map(|b| {
                        b.into_iter()
                            .filter_map(|s| self.eliminate_dead_stmt(s))
                            .collect()
                    }),
                }
            }
            other => other,
        }
    }

    /// 尾递归优化
    fn optimize_tail_recursion(&self, program: Program) -> Program {
        program
            .into_iter()
            .map(|stmt| self.optimize_tail_recursive_stmt(stmt))
            .collect()
    }

    /// 优化尾递归语句
    fn optimize_tail_recursive_stmt(&self, stmt: Stmt) -> Stmt {
        match stmt {
            Stmt::FuncDef { name, params, body } => {
                // 检查函数体是否包含尾递归
                if self.is_tail_recursive(&name, &body) {
                    // 转换为迭代形式
                    Stmt::FuncDef {
                        name: name.clone(),
                        params: params.clone(),
                        body: self.convert_tail_recursion_to_loop(&name, &params, body),
                    }
                } else {
                    Stmt::FuncDef { name, params, body }
                }
            }
            other => other,
        }
    }

    /// 检查是否为尾递归
    fn is_tail_recursive(&self, func_name: &str, body: &[Stmt]) -> bool {
        if body.is_empty() {
            return false;
        }

        // 递归检查所有可能的返回路径
        self.has_tail_recursion_in_body(func_name, body)
    }

    /// 检查函数体中是否包含尾递归
    fn has_tail_recursion_in_body(&self, func_name: &str, body: &[Stmt]) -> bool {
        // 至少需要有一条return语句包含尾递归调用
        body.iter()
            .any(|stmt| self.stmt_has_tail_recursion(func_name, stmt))
    }

    /// 检查语句是否包含尾递归
    fn stmt_has_tail_recursion(&self, func_name: &str, stmt: &Stmt) -> bool {
        match stmt {
            Stmt::Return(expr) => self.is_tail_call(func_name, expr),
            Stmt::Expression(expr) => self.expr_has_tail_recursion(func_name, expr),
            Stmt::While { body, .. } => self.has_tail_recursion_in_body(func_name, body),
            Stmt::For { body, .. } => self.has_tail_recursion_in_body(func_name, body),
            Stmt::ForIndexed { body, .. } => self.has_tail_recursion_in_body(func_name, body),
            _ => false,
        }
    }

    /// 检查表达式是否包含尾递归
    fn expr_has_tail_recursion(&self, func_name: &str, expr: &Expr) -> bool {
        match expr {
            Expr::If {
                then_branch,
                elif_branches,
                else_branch,
                ..
            } => {
                // 检查所有分支
                let then_tail = self.has_tail_recursion_in_body(func_name, then_branch);
                let elif_tail = elif_branches
                    .iter()
                    .any(|(_, body)| self.has_tail_recursion_in_body(func_name, body));
                let else_tail = else_branch
                    .as_ref()
                    .map(|body| self.has_tail_recursion_in_body(func_name, body))
                    .unwrap_or(false);

                then_tail || elif_tail || else_tail
            }
            _ => false,
        }
    }

    /// 检查表达式是否为尾调用（增强版）
    fn is_tail_call(&self, func_name: &str, expr: &Expr) -> bool {
        match expr {
            // 直接的递归调用
            Expr::Call { func, .. } => {
                if let Expr::Identifier(name) = &**func {
                    name == func_name
                } else {
                    false
                }
            }
            // 条件表达式中的尾调用
            Expr::If {
                then_branch,
                elif_branches,
                else_branch,
                ..
            } => {
                // 所有分支都必须是尾调用或没有返回值
                let then_is_tail = self.branch_ends_with_tail_call(func_name, then_branch);

                let elif_all_tail = elif_branches
                    .iter()
                    .all(|(_, body)| self.branch_ends_with_tail_call(func_name, body));

                let else_is_tail = else_branch
                    .as_ref()
                    .map(|body| self.branch_ends_with_tail_call(func_name, body))
                    .unwrap_or(true);

                then_is_tail && elif_all_tail && else_is_tail
            }
            _ => false,
        }
    }

    /// 检查分支是否以尾调用结束
    fn branch_ends_with_tail_call(&self, func_name: &str, branch: &[Stmt]) -> bool {
        if let Some(last_stmt) = branch.last() {
            match last_stmt {
                Stmt::Return(expr) => self.is_tail_call(func_name, expr),
                Stmt::Expression(expr) => {
                    // 表达式可能是If表达式
                    self.is_tail_call(func_name, expr)
                }
                _ => false,
            }
        } else {
            false
        }
    }

    /// 将尾递归转换为循环 (完整实现)
    fn convert_tail_recursion_to_loop(
        &self,
        func_name: &str,
        params: &[String],
        body: Vec<Stmt>,
    ) -> Vec<Stmt> {
        // 步骤1: 为每个参数创建临时变量
        let mut new_body = Vec::new();

        // 初始化临时变量
        for param in params {
            new_body.push(Stmt::Set {
                name: format!("_loop_{}", param),
                value: Expr::Identifier(param.clone()),
            });
        }

        // 步骤2: 创建循环标志
        new_body.push(Stmt::Set {
            name: "_loop_continue".to_string(),
            value: Expr::Boolean(true),
        });

        // 步骤3: 转换函数体为while循环
        let loop_body = self.transform_body_to_loop(func_name, params, body);

        // 步骤4: 创建while循环
        new_body.push(Stmt::While {
            condition: Expr::Identifier("_loop_continue".to_string()),
            body: loop_body,
        });

        new_body
    }

    /// 转换函数体为循环体
    fn transform_body_to_loop(
        &self,
        func_name: &str,
        params: &[String],
        body: Vec<Stmt>,
    ) -> Vec<Stmt> {
        let mut loop_body = Vec::new();

        for stmt in body {
            match stmt {
                Stmt::Return(expr) => {
                    // 检查是否为尾递归调用
                    if let Some(new_args) = self.extract_tail_call_args(func_name, &expr) {
                        // 这是尾递归调用，转换为参数更新
                        for (i, param) in params.iter().enumerate() {
                            if let Some(arg) = new_args.get(i) {
                                loop_body.push(Stmt::Set {
                                    name: format!("_loop_{}", param),
                                    value: arg.clone(),
                                });
                            }
                        }

                        // 更新参数值
                        for param in params {
                            loop_body.push(Stmt::Set {
                                name: param.clone(),
                                value: Expr::Identifier(format!("_loop_{}", param)),
                            });
                        }

                        // 继续循环
                    } else {
                        // 这不是尾递归调用，正常返回
                        loop_body.push(Stmt::Set {
                            name: "_loop_continue".to_string(),
                            value: Expr::Boolean(false),
                        });
                        loop_body.push(Stmt::Return(expr));
                    }
                }
                _ => {
                    // 其他语句递归转换
                    loop_body.push(self.transform_stmt_for_loop(func_name, params, stmt));
                }
            }
        }

        loop_body
    }

    /// 提取尾调用的参数
    fn extract_tail_call_args(&self, func_name: &str, expr: &Expr) -> Option<Vec<Expr>> {
        match expr {
            Expr::Call { func, args } => {
                if let Expr::Identifier(name) = &**func
                    && name == func_name
                {
                    return Some(args.clone());
                }
                None
            }
            _ => None,
        }
    }

    /// 转换语句以适应循环结构
    fn transform_stmt_for_loop(&self, func_name: &str, params: &[String], stmt: Stmt) -> Stmt {
        match stmt {
            Stmt::Expression(expr) => {
                // 处理If表达式
                Stmt::Expression(self.transform_expr_for_loop(func_name, params, expr))
            }
            Stmt::While { condition, body } => Stmt::While {
                condition,
                body: self.transform_body_to_loop(func_name, params, body),
            },
            Stmt::For {
                var,
                iterable,
                body,
            } => Stmt::For {
                var,
                iterable,
                body: self.transform_body_to_loop(func_name, params, body),
            },
            Stmt::ForIndexed {
                index_var,
                value_var,
                iterable,
                body,
            } => Stmt::ForIndexed {
                index_var,
                value_var,
                iterable,
                body: self.transform_body_to_loop(func_name, params, body),
            },
            other => other,
        }
    }

    /// 转换表达式以适应循环结构
    fn transform_expr_for_loop(&self, func_name: &str, params: &[String], expr: Expr) -> Expr {
        match expr {
            Expr::If {
                condition,
                then_branch,
                elif_branches,
                else_branch,
            } => Expr::If {
                condition,
                then_branch: self.transform_body_to_loop(func_name, params, then_branch),
                elif_branches: elif_branches
                    .into_iter()
                    .map(|(cond, body)| {
                        (cond, self.transform_body_to_loop(func_name, params, body))
                    })
                    .collect(),
                else_branch: else_branch
                    .map(|body| self.transform_body_to_loop(func_name, params, body)),
            },
            other => other,
        }
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_folding() {
        let optimizer = Optimizer::new();

        // 测试: 2 + 3 应该折叠为 5
        let expr = Expr::Binary {
            left: Box::new(Expr::Number(2.0)),
            op: BinOp::Add,
            right: Box::new(Expr::Number(3.0)),
        };

        let folded = optimizer.fold_expr(expr);
        assert_eq!(folded, Expr::Number(5.0));
    }

    #[test]
    fn test_dead_code_elimination() {
        let optimizer = Optimizer::new();

        // While False 应该被删除
        let stmt = Stmt::While {
            condition: Expr::Boolean(false),
            body: vec![Stmt::Set {
                name: "x".to_string(),
                value: Expr::Number(10.0),
            }],
        };

        let result = optimizer.eliminate_dead_stmt(stmt);
        assert!(result.is_none());
    }

    #[test]
    fn test_optimize_program() {
        let optimizer = Optimizer::new();

        let program: Program = vec![Stmt::Set {
            name: "x".to_string(),
            value: Expr::Binary {
                left: Box::new(Expr::Number(2.0)),
                op: BinOp::Add,
                right: Box::new(Expr::Number(3.0)),
            },
        }];

        let optimized = optimizer.optimize_program(&program);

        // 验证常量折叠
        if let Some(Stmt::Set { value, .. }) = optimized.first() {
            assert_eq!(*value, Expr::Number(5.0));
        }
    }

    #[test]
    fn test_tail_recursion_detection() {
        let optimizer = Optimizer::new();

        // 测试简单的尾递归
        let body = vec![Stmt::Return(Expr::Call {
            func: Box::new(Expr::Identifier("factorial".to_string())),
            args: vec![
                Expr::Binary {
                    left: Box::new(Expr::Identifier("n".to_string())),
                    op: BinOp::Subtract,
                    right: Box::new(Expr::Number(1.0)),
                },
                Expr::Binary {
                    left: Box::new(Expr::Identifier("acc".to_string())),
                    op: BinOp::Multiply,
                    right: Box::new(Expr::Identifier("n".to_string())),
                },
            ],
        })];

        assert!(optimizer.is_tail_recursive("factorial", &body));
    }

    #[test]
    fn test_non_tail_recursion_detection() {
        let optimizer = Optimizer::new();

        // 测试非尾递归（递归调用后还有操作）
        let body = vec![Stmt::Return(Expr::Binary {
            left: Box::new(Expr::Identifier("n".to_string())),
            op: BinOp::Multiply,
            right: Box::new(Expr::Call {
                func: Box::new(Expr::Identifier("factorial".to_string())),
                args: vec![Expr::Binary {
                    left: Box::new(Expr::Identifier("n".to_string())),
                    op: BinOp::Subtract,
                    right: Box::new(Expr::Number(1.0)),
                }],
            }),
        })];

        assert!(!optimizer.is_tail_recursive("factorial", &body));
    }

    #[test]
    fn test_tail_recursion_in_if() {
        let optimizer = Optimizer::new();

        // 测试If表达式中的尾递归
        // 实际上Aether中Return语句后面跟的是表达式，而If是表达式
        // 所以我们需要Return一个If表达式
        let body = vec![Stmt::Expression(Expr::If {
            condition: Box::new(Expr::Binary {
                left: Box::new(Expr::Identifier("n".to_string())),
                op: BinOp::LessEqual,
                right: Box::new(Expr::Number(0.0)),
            }),
            then_branch: vec![Stmt::Return(Expr::Identifier("acc".to_string()))],
            elif_branches: vec![],
            else_branch: Some(vec![Stmt::Return(Expr::Call {
                func: Box::new(Expr::Identifier("sum".to_string())),
                args: vec![
                    Expr::Binary {
                        left: Box::new(Expr::Identifier("n".to_string())),
                        op: BinOp::Subtract,
                        right: Box::new(Expr::Number(1.0)),
                    },
                    Expr::Binary {
                        left: Box::new(Expr::Identifier("acc".to_string())),
                        op: BinOp::Add,
                        right: Box::new(Expr::Identifier("n".to_string())),
                    },
                ],
            })]),
        })];

        assert!(optimizer.is_tail_recursive("sum", &body));
    }

    #[test]
    fn test_tail_recursion_optimization_transform() {
        let optimizer = Optimizer::new();

        // 创建一个简单的尾递归函数
        let func_def = Stmt::FuncDef {
            name: "factorial".to_string(),
            params: vec!["n".to_string(), "acc".to_string()],
            body: vec![Stmt::Return(Expr::Call {
                func: Box::new(Expr::Identifier("factorial".to_string())),
                args: vec![
                    Expr::Binary {
                        left: Box::new(Expr::Identifier("n".to_string())),
                        op: BinOp::Subtract,
                        right: Box::new(Expr::Number(1.0)),
                    },
                    Expr::Binary {
                        left: Box::new(Expr::Identifier("acc".to_string())),
                        op: BinOp::Multiply,
                        right: Box::new(Expr::Identifier("n".to_string())),
                    },
                ],
            })],
        };

        let optimized = optimizer.optimize_tail_recursive_stmt(func_def);

        // 验证转换后包含While循环
        if let Stmt::FuncDef { body, .. } = optimized {
            // 应该包含临时变量初始化、循环标志和while循环
            // 2个参数 = 2个临时变量 + 1个循环标志 + 1个while循环 = 4个语句
            assert!(
                body.len() >= 3,
                "Expected at least 3 statements, got {}",
                body.len()
            );

            // 最后一个语句应该是While循环
            if let Some(Stmt::While { .. }) = body.last() {
                // 成功转换为循环
            } else {
                panic!("Expected While loop at the end of optimized function body");
            }
        } else {
            panic!("Expected FuncDef");
        }
    }
}
