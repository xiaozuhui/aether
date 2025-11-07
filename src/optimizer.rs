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
    fn fold_expr(&self, expr: Expr) -> Expr {
        match expr {
            // 二元运算常量折叠
            Expr::Binary { left, op, right } => {
                let left = self.fold_expr(*left);
                let right = self.fold_expr(*right);

                // 如果两边都是常量,直接计算结果
                if let (Expr::Number(l), Expr::Number(r)) = (&left, &right) {
                    if let Some(result) = Self::eval_const_binary(*l, &op, *r) {
                        return Expr::Number(result);
                    }
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

        // 检查最后一条语句
        if let Some(last_stmt) = body.last() {
            match last_stmt {
                Stmt::Return(expr) => self.is_tail_call(func_name, expr),
                _ => false,
            }
        } else {
            false
        }
    }

    /// 检查表达式是否为尾调用
    fn is_tail_call(&self, func_name: &str, expr: &Expr) -> bool {
        match expr {
            Expr::Call { func, .. } => {
                if let Expr::Identifier(name) = &**func {
                    name == func_name
                } else {
                    false
                }
            }
            // 如果if表达式的所有分支都是尾调用,也算尾递归
            Expr::If {
                then_branch,
                elif_branches,
                else_branch,
                ..
            } => {
                let then_is_tail = then_branch
                    .last()
                    .map(|s| matches!(s, Stmt::Return(e) if self.is_tail_call(func_name, e)))
                    .unwrap_or(false);

                let elif_is_tail = elif_branches.iter().all(|(_, body)| {
                    body.last()
                        .map(|s| matches!(s, Stmt::Return(e) if self.is_tail_call(func_name, e)))
                        .unwrap_or(false)
                });

                let else_is_tail = else_branch
                    .as_ref()
                    .and_then(|b| b.last())
                    .map(|s| matches!(s, Stmt::Return(e) if self.is_tail_call(func_name, e)))
                    .unwrap_or(true); // 没有else分支也算

                then_is_tail && elif_is_tail && else_is_tail
            }
            _ => false,
        }
    }

    /// 将尾递归转换为循环 (简化实现)
    fn convert_tail_recursion_to_loop(
        &self,
        _func_name: &str,
        _params: &[String],
        body: Vec<Stmt>,
    ) -> Vec<Stmt> {
        // 这是一个简化的实现
        // 完整的尾递归优化需要更复杂的转换逻辑
        // 包括识别递归调用点,提取参数更新逻辑等

        // 当前先保持原样,标记为已优化
        // TODO: 完整实现尾递归到循环的转换
        body
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
}
