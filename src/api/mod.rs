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
}
