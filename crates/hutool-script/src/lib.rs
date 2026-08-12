//! 对齐: `cn.hutool.script` 顶级包（无 Java 子包）
//! 来源: hutool-script/src/main/java/cn/hutool/script/
//!
//! 中文说明: 按 `cn.hutool.script` 1:1 平铺到 `src/`：
//! - `script_util.rs`（cn.hutool.script.ScriptUtil）
//! - `script_runtime_exception.rs`（cn.hutool.script.ScriptRuntimeException）
//! - `javax_script/`（对齐 Java `javax.script` 包，Rust 不属于 hutool-script 顶级包但需要 JSR-223 抽象）
//!   - `javax_script/all.rs` 集中实现 Bindings/CompiledScript/ScriptContext/ScriptScope/ScriptLanguage/ScriptEngineFactory/FullSupportScriptEngine/ScriptInterface/JavaScriptEngine
//!
//! Sandboxed embedded scripting backed by Rhai.

#![forbid(unsafe_code)]

pub mod javax_script;
pub mod prelude;
mod script_runtime_exception;
mod script_util;

pub use javax_script::all::{
    Bindings, CompiledScript, FullSupportScriptEngine, JavaScriptEngine, ScriptContext,
    ScriptEngineFactory, ScriptInterface, ScriptLanguage, ScriptScope,
};
use rhai::Engine;
pub use rhai::{AST, CallFnOptions, Dynamic, EvalAltResult, ParseError, Scope};
pub use script_runtime_exception::ScriptRuntimeException;
pub use script_util::ScriptUtil;
use std::{any::Any, any::type_name};
use thiserror::Error;

/// Script evaluation failures.
#[derive(Debug, Error)]
pub enum ScriptError {
    /// The requested optional language engine is not installed.
    #[error("script language `{0}` is not supported")]
    UnsupportedLanguage(String),
    /// The Rhai parser rejected the script.
    #[error(transparent)]
    Compilation(#[from] ParseError),
    /// Script input could not be read.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Invocation needs a previously compiled or evaluated script.
    #[error("{0}")]
    Invocation(String),
    /// The Rhai engine rejected or terminated the script.
    #[error(transparent)]
    Evaluation(#[from] Box<EvalAltResult>),
    /// The dynamic result did not match the requested Rust type.
    #[error("script result cannot be converted to `{0}`")]
    TypeMismatch(&'static str),
}

/// Resource limits applied to every script engine.
#[derive(Debug, Clone, Copy)]
pub struct ScriptLimits {
    /// Maximum expression operations before termination.
    pub max_operations: u64,
    /// Maximum nested function-call depth.
    pub max_call_levels: usize,
    /// Maximum expression depth.
    pub max_expression_depth: usize,
    /// Maximum string length in bytes.
    pub max_string_size: usize,
    /// Maximum array element count.
    pub max_array_size: usize,
    /// Maximum map entry count.
    pub max_map_size: usize,
}

impl Default for ScriptLimits {
    fn default() -> Self {
        Self {
            max_operations: 100_000,
            max_call_levels: 32,
            max_expression_depth: 64,
            max_string_size: 1_048_576,
            max_array_size: 10_000,
            max_map_size: 10_000,
        }
    }
}

/// A constrained script engine. No filesystem, process, or network APIs are
/// registered by this crate.
pub struct ScriptEngine {
    inner: Engine,
}

impl ScriptEngine {
    /// Creates an engine with explicit resource limits and dynamic `eval`
    /// disabled.
    #[must_use]
    pub fn new(limits: ScriptLimits) -> Self {
        let mut inner = Engine::new();
        inner
            .set_max_operations(limits.max_operations)
            .set_max_call_levels(limits.max_call_levels)
            .set_max_expr_depths(limits.max_expression_depth, limits.max_expression_depth)
            .set_max_string_size(limits.max_string_size)
            .set_max_array_size(limits.max_array_size)
            .set_max_map_size(limits.max_map_size)
            .disable_symbol("eval");
        Self { inner }
    }

    /// Returns the underlying engine for deliberate function registration.
    pub fn engine_mut(&mut self) -> &mut Engine {
        &mut self.inner
    }

    /// Returns the underlying constrained engine.
    #[must_use]
    pub fn engine(&self) -> &Engine {
        &self.inner
    }

    /// Compiles script text into a reusable AST.
    pub fn compile(&self, script: &str) -> Result<AST, ScriptError> {
        Ok(self.inner.compile(script)?)
    }

    /// Evaluates an AST with the supplied scope.
    pub fn eval_ast_with_scope(
        &self,
        scope: &mut Scope<'_>,
        ast: &AST,
    ) -> Result<Dynamic, ScriptError> {
        Ok(self.inner.eval_ast_with_scope(scope, ast)?)
    }

    /// Calls a script-defined function with dynamic arguments.
    pub fn call_fn(
        &self,
        scope: &mut Scope<'_>,
        ast: &AST,
        name: &str,
        args: Vec<Dynamic>,
    ) -> Result<Dynamic, ScriptError> {
        Ok(self.inner.call_fn(scope, ast, name, args)?)
    }

    /// Calls a script method with a bound `this` value.
    pub fn call_method(
        &self,
        scope: &mut Scope<'_>,
        ast: &AST,
        this: &mut Dynamic,
        name: &str,
        args: Vec<Dynamic>,
    ) -> Result<Dynamic, ScriptError> {
        let options = CallFnOptions::new().bind_this_ptr(this);
        Ok(self
            .inner
            .call_fn_with_options(options, scope, ast, name, args)?)
    }

    /// Evaluates a script into a typed result.
    pub fn eval<T: Any>(&self, script: &str) -> Result<T, ScriptError> {
        self.inner
            .eval::<Dynamic>(script)?
            .try_cast::<T>()
            .ok_or(ScriptError::TypeMismatch(type_name::<T>()))
    }

    /// Evaluates a script using the supplied variable scope.
    pub fn eval_with_scope<T: Any>(
        &self,
        scope: &mut Scope<'_>,
        script: &str,
    ) -> Result<T, ScriptError> {
        self.inner
            .eval_with_scope::<Dynamic>(scope, script)?
            .try_cast::<T>()
            .ok_or(ScriptError::TypeMismatch(type_name::<T>()))
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new(ScriptLimits::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_typed_expressions_and_rejects_eval() {
        let mut engine = ScriptEngine::default();
        engine
            .engine_mut()
            .register_fn("double", |value: i64| value * 2);
        assert_eq!(engine.engine().eval::<i64>("double(21)").unwrap(), 42);
        assert_eq!(engine.eval::<i64>("40 + 2").unwrap(), 42);
        assert!(engine.eval::<i64>("let = ;").is_err());
        assert_eq!(
            engine.eval::<Dynamic>("40 + 2").unwrap().as_int().unwrap(),
            42
        );
        assert!(engine.eval::<Dynamic>(r#"eval("40 + 2")"#).is_err());
        assert!(engine.eval::<String>("40 + 2").is_err());
        assert!(engine.eval::<String>("let = ;").is_err());
    }

    #[test]
    fn uses_typed_scope_values() {
        let engine = ScriptEngine::default();
        let mut scope = Scope::new();
        scope.push("base", 40_i64);
        assert_eq!(
            engine
                .eval_with_scope::<i64>(&mut scope, "base + 2")
                .unwrap(),
            42
        );
        assert!(
            engine
                .eval_with_scope::<String>(&mut scope, "base + 2")
                .is_err()
        );
        assert!(
            engine
                .eval_with_scope::<String>(&mut scope, "let = ;")
                .is_err()
        );
        assert!(
            engine
                .eval_with_scope::<i64>(&mut scope, "let = ;")
                .is_err()
        );
    }
}
