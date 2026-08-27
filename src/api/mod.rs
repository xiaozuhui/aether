use crate::cache::ASTCache;
use crate::evaluator::Evaluator;
use crate::optimizer::Optimizer;

mod cache;
mod constructors;
mod eval;
mod limits;
mod stdlib;
mod trace;

/// 主要的 Aether 引擎结构体
pub struct Aether {
    pub(crate) evaluator: Evaluator,
    pub(crate) cache: ASTCache,
    pub(crate) optimizer: Optimizer,
    /// 整数字面量超过该位数后切换为 BigInteger（传给解析期 Lexer）
    pub(crate) bigint_threshold: usize,
}
