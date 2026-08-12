//! `ScriptUtil` 兼容对象。
//!
//! 对应 Java 类：`cn.hutool.script.ScriptUtil`
//! Java 来源：`hutool-script/src/main/java/cn/hutool/script/ScriptUtil.java`

use super::{
    Bindings, CompiledScript, FullSupportScriptEngine, JavaScriptEngine, ScriptContext,
    ScriptRuntimeException,
};
use crate::{Dynamic, ScriptEngine};

/// 静态脚本工具门面，对齐 Hutool 的 `ScriptUtil` 能力。
pub struct ScriptUtil;

impl ScriptUtil {
    /// 创建具名引擎。Rust 版本不做全局缓存。
    pub fn get_script(name: &str) -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::create_script(name)
    }

    /// 创建具名引擎。
    pub fn create_script(name: &str) -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        FullSupportScriptEngine::new(name)
    }

    /// 创建 JavaScript 兼容包装器。
    #[must_use]
    pub fn get_java_script_engine() -> JavaScriptEngine {
        JavaScriptEngine::new()
    }

    /// 创建受支持的 JS/Rhai 引擎。
    pub fn get_js_engine() -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::get_script("js")
    }

    /// 创建独立的 JS/Rhai 引擎。
    pub fn create_js_engine() -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::create_script("js")
    }

    /// 返回未链接 Python 引擎的兼容错误。
    pub fn get_python_engine() -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::create_script("python")
    }

    /// 返回未链接 Python 引擎的兼容错误。
    pub fn create_python_engine() -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::create_script("python")
    }

    /// 返回未链接 Lua 引擎的兼容错误。
    pub fn get_lua_engine() -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::create_script("lua")
    }

    /// 返回未链接 Lua 引擎的兼容错误。
    pub fn create_lua_engine() -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::create_script("lua")
    }

    /// 返回未链接 Groovy 引擎的兼容错误。
    pub fn get_groovy_engine() -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::create_script("groovy")
    }

    /// 返回未链接 Groovy 引擎的兼容错误。
    pub fn create_groovy_engine() -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::create_script("groovy")
    }

    /// 先执行脚本，再返回可调用引擎。
    pub fn eval_invocable(script: &str) -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        let mut engine = FullSupportScriptEngine::from_engine(ScriptEngine::default());
        let _ = engine.eval(script)?;
        Ok(engine)
    }

    /// 使用全新引擎执行脚本。
    pub fn eval(script: &str) -> Result<Dynamic, ScriptRuntimeException> {
        FullSupportScriptEngine::from_engine(ScriptEngine::default()).eval(script)
    }

    /// 使用显式上下文执行脚本。
    pub fn eval_with_context(
        script: &str,
        context: &mut ScriptContext,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        FullSupportScriptEngine::from_engine(ScriptEngine::default())
            .eval_with_context(script, context)
    }

    /// 使用显式绑定执行脚本。
    pub fn eval_with_bindings(
        script: &str,
        bindings: Bindings,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        FullSupportScriptEngine::from_engine(ScriptEngine::default())
            .eval_with_bindings(script, bindings)
    }

    /// 执行脚本并调用指定函数。
    pub fn invoke(
        script: &str,
        function: &str,
        args: Vec<Dynamic>,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        Self::eval_invocable(script)?.invoke_function(function, args)
    }

    /// 使用全新引擎编译脚本。
    pub fn compile(script: &str) -> Result<CompiledScript, ScriptRuntimeException> {
        FullSupportScriptEngine::from_engine(ScriptEngine::default()).compile(script)
    }

    /// 使用显式配置的引擎编译脚本。
    pub fn compile_with_engine(
        engine: &FullSupportScriptEngine,
        script: &str,
    ) -> Result<CompiledScript, ScriptRuntimeException> {
        engine.compile(script)
    }
}
