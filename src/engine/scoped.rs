//! 闭包模式引擎
//!
//! 使用闭包风格的API，每次都创建新引擎，完全隔离。
//! 类似于 Py3o 的使用方式，适合临时执行或需要完全隔离的场景。

use crate::{Aether, Value};

/// 闭包模式引擎
///
/// # 使用场景
///
/// - ✅ 临时脚本执行
/// - ✅ 需要完全隔离（每次都是新引擎）
/// - ✅ 偶尔使用（不频繁）
/// - ✅ 简单API（类似 Py3o）
/// - ❌ 高频调用（性能较低，请使用 GlobalEngine 或 PooledEngine）
///
/// # 特点
///
/// - **完全隔离**：每次都创建新引擎实例
/// - **简洁API**：闭包风格，自动管理生命周期
/// - **无缓存**：无法利用 AST 缓存（每次都是新引擎）
/// - **线程安全**：每个线程独立创建引擎
///
/// # 示例
///
/// ```rust
/// use aether::engine::ScopedEngine;
///
/// // 基本使用
/// let result = ScopedEngine::with(|engine| {
///     engine.eval("Set X 10\n(X + 20)")
/// }).unwrap();
/// assert_eq!(result.to_string(), "30");
///
/// // 多步骤执行
/// let result = ScopedEngine::with(|engine| {
///     engine.eval("Set X 10")?;
///     engine.eval("Set Y 20")?;
///     engine.eval("(X + Y)")
/// }).unwrap();
/// assert_eq!(result.to_string(), "30");
///
/// // 简化版：直接执行单行代码
/// let result = ScopedEngine::eval("(10 + 20)").unwrap();
/// assert_eq!(result.to_string(), "30");
/// ```
///
/// # 与其他模式对比
///
/// ```rust
/// use aether::engine::{GlobalEngine, EnginePool, ScopedEngine};
///
/// // GlobalEngine - 性能最优，但单线程
/// let result1 = GlobalEngine::eval_isolated("Set X 10\n(X + 20)").unwrap();
///
/// // PooledEngine - 多线程，性能好
/// let pool = EnginePool::new(4);
/// let result2 = pool.acquire().eval("Set X 10\n(X + 20)").unwrap();
///
/// // ScopedEngine - API最简洁，完全隔离
/// let result3 = ScopedEngine::eval("Set X 10\n(X + 20)").unwrap();
///
/// assert_eq!(result1, result2);
/// assert_eq!(result2, result3);
/// ```
pub struct ScopedEngine;

impl ScopedEngine {
    /// 使用闭包执行代码（完全隔离）
    ///
    /// 每次调用都会创建新的引擎实例，执行完毕后自动销毁。
    ///
    /// # 参数
    ///
    /// - `f`: 接收 `&mut Aether` 的闭包，返回 `Result<T, String>`
    ///
    /// # 返回
    ///
    /// - `Ok(T)`: 闭包的返回值
    /// - `Err(String)`: 错误信息
    ///
    /// # 类型参数
    ///
    /// - `F`: 闭包类型
    /// - `T`: 返回值类型
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aether::engine::ScopedEngine;
    ///
    /// // 单步执行
    /// let result = ScopedEngine::with(|engine| {
    ///     engine.eval("(10 + 20)")
    /// }).unwrap();
    ///
    /// // 多步执行
    /// let result = ScopedEngine::with(|engine| {
    ///     engine.eval("Set X 10")?;
    ///     engine.eval("Set Y (X * 2)")?;
    ///     engine.eval("(X + Y)")
    /// }).unwrap();
    ///
    /// // 自定义返回类型
    /// let (x, y) = ScopedEngine::with(|engine| {
    ///     engine.eval("Set X 10")?;
    ///     engine.eval("Set Y 20")?;
    ///     let x = engine.eval("X")?;
    ///     let y = engine.eval("Y")?;
    ///     Ok((x, y))
    /// }).unwrap();
    /// ```
    pub fn with<F, T>(f: F) -> Result<T, String>
    where
        F: FnOnce(&mut Aether) -> Result<T, String>,
    {
        let mut engine = Aether::new();
        f(&mut engine)
    }

    /// 使用闭包执行代码（启用所有权限）
    ///
    /// 创建的引擎会启用所有 IO 权限（文件系统、网络等）。
    ///
    /// # 警告
    ///
    /// 仅在信任代码来源时使用。用户提供的代码可能包含恶意操作。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aether::engine::ScopedEngine;
    ///
    /// let result = ScopedEngine::with_all_permissions(|engine| {
    ///     // 可以使用文件系统操作
    ///     engine.eval(r#"
    ///         Set Content (ReadFile "config.txt")
    ///         (Length Content)
    ///     "#)
    /// });
    /// ```
    pub fn with_all_permissions<F, T>(f: F) -> Result<T, String>
    where
        F: FnOnce(&mut Aether) -> Result<T, String>,
    {
        let mut engine = Aether::with_all_permissions();
        f(&mut engine)
    }

    /// 直接执行代码（简化版）
    ///
    /// 创建新引擎，执行代码，返回结果。
    /// 适合简单的单行代码执行。
    ///
    /// # 参数
    ///
    /// - `code`: 要执行的 Aether 代码
    ///
    /// # 返回
    ///
    /// - `Ok(Value)`: 执行结果
    /// - `Err(String)`: 错误信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aether::engine::ScopedEngine;
    ///
    /// // 简单表达式
    /// let result = ScopedEngine::eval("(10 + 20)").unwrap();
    /// assert_eq!(result.to_string(), "30");
    ///
    /// // 多行代码
    /// let result = ScopedEngine::eval("Set X 10\nSet Y 20\n(X + Y)").unwrap();
    /// assert_eq!(result.to_string(), "30");
    /// ```
    pub fn eval(code: &str) -> Result<Value, String> {
        Self::with(|engine| engine.eval(code))
    }

    /// 直接执行代码（启用所有权限）
    ///
    /// 创建启用所有 IO 权限的引擎，执行代码。
    ///
    /// # 警告
    ///
    /// 仅在信任代码来源时使用。
    pub fn eval_with_all_permissions(code: &str) -> Result<Value, String> {
        Self::with_all_permissions(|engine| engine.eval(code))
    }

    /// 使用闭包异步执行代码（requires "async" feature）
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use aether::engine::ScopedEngine;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let result = ScopedEngine::with_async(|engine| async move {
    ///         engine.eval("Set X 10")?;
    ///         engine.eval("(X + 20)")
    ///     }).await.unwrap();
    ///     println!("Result: {}", result);
    /// }
    /// ```
    #[cfg(feature = "async")]
    pub async fn with_async<F, Fut, T>(f: F) -> Result<T, String>
    where
        F: FnOnce(&mut Aether) -> Fut,
        Fut: std::future::Future<Output = Result<T, String>>,
    {
        tokio::task::yield_now().await;
        let mut engine = Aether::new();
        f(&mut engine).await
    }

    /// 异步执行代码（简化版，requires "async" feature）
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use aether::engine::ScopedEngine;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let result = ScopedEngine::eval_async("Set X 10\n(X + 20)").await.unwrap();
    ///     println!("Result: {}", result);
    /// }
    /// ```
    #[cfg(feature = "async")]
    pub async fn eval_async(code: &str) -> Result<Value, String> {
        tokio::task::yield_now().await;
        Self::eval(code)
    }

    /// 异步执行代码（启用所有权限，requires "async" feature）
    #[cfg(feature = "async")]
    pub async fn eval_with_all_permissions_async(code: &str) -> Result<Value, String> {
        tokio::task::yield_now().await;
        Self::eval_with_all_permissions(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scoped_engine_basic() {
        let result = ScopedEngine::eval("Set X 10\n(X + 20)").unwrap();
        assert_eq!(result.to_string(), "30");
    }

    #[test]
    fn test_scoped_engine_isolation() {
        // 第一次执行
        ScopedEngine::eval("Set X 10").unwrap();

        // 第二次执行（新引擎，X 不存在）
        let result = ScopedEngine::eval("X");
        assert!(result.is_err());
    }

    #[test]
    fn test_scoped_engine_with_closure() {
        let result = ScopedEngine::with(|engine| {
            engine.eval("Set X 10")?;
            engine.eval("Set Y 20")?;
            engine.eval("(X + Y)")
        })
        .unwrap();

        assert_eq!(result.to_string(), "30");
    }

    #[test]
    fn test_scoped_engine_custom_return() {
        let (x, y) = ScopedEngine::with(|engine| {
            engine.eval("Set X 10")?;
            engine.eval("Set Y 20")?;
            let x = engine.eval("X")?;
            let y = engine.eval("Y")?;
            Ok((x, y))
        })
        .unwrap();

        assert_eq!(x.to_string(), "10");
        assert_eq!(y.to_string(), "20");
    }
}
