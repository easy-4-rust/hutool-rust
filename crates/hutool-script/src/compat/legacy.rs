//! 兼容层过渡模块。
//!
//! 当前保留尚未拆分的 Hutool 脚本兼容对象，后续会继续按对象拆分。

use super::ScriptRuntimeException;
use crate::{AST, Dynamic, Scope, ScriptEngine, ScriptError};
use std::{
    collections::BTreeMap,
    io::Read,
    ops::{Deref, DerefMut},
};

const MAX_READER_BYTES: u64 = 1_048_577;

/// Script languages recognized by the compatibility facade.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptLanguage {
    /// Sandboxed Rhai, also used for the JavaScript-shaped compatibility facade.
    Rhai,
    /// Optional Python engine; not linked by the base crate.
    Python,
    /// Optional Lua engine; not linked by the base crate.
    Lua,
    /// Optional Groovy engine; not linked by the base crate.
    Groovy,
}

impl ScriptLanguage {
    fn resolve(value: &str) -> Result<Self, ScriptError> {
        match value.to_ascii_lowercase().as_str() {
            "rhai" | "js" | "javascript" | "application/javascript" | "text/javascript" => {
                Ok(Self::Rhai)
            }
            "python" | "py" => Ok(Self::Python),
            "lua" => Ok(Self::Lua),
            "groovy" => Ok(Self::Groovy),
            _ => Err(ScriptError::UnsupportedLanguage(value.to_owned())),
        }
    }
}

/// Bindings passed into a script scope.
#[derive(Debug, Clone, Default)]
pub struct Bindings {
    values: BTreeMap<String, Dynamic>,
}

impl Bindings {
    /// Creates empty bindings.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    /// Inserts or replaces one dynamic value.
    pub fn insert(
        &mut self,
        name: impl Into<String>,
        value: impl Into<Dynamic>,
    ) -> Option<Dynamic> {
        self.values.insert(name.into(), value.into())
    }

    /// Returns a bound value.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Dynamic> {
        self.values.get(name)
    }

    /// Removes a bound value.
    pub fn remove(&mut self, name: &str) -> Option<Dynamic> {
        self.values.remove(name)
    }

    /// Returns the number of bindings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether the binding set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn append_to_scope(&self, scope: &mut Scope<'_>) {
        for (name, value) in &self.values {
            if scope.contains(name) {
                scope.set_value(name, value.clone());
            } else {
                scope.push_dynamic(name.clone(), value.clone());
            }
        }
    }

    fn replace_from_scope(&mut self, scope: &Scope<'_>) {
        self.values = scope
            .iter_raw()
            .map(|(name, _, value)| (name.to_owned(), value.clone()))
            .collect();
    }
}

/// Binding scopes corresponding to JSR-223 engine and global scopes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptScope {
    /// Per-engine values, taking precedence over globals.
    Engine,
    /// Shared values copied into an evaluation context.
    Global,
}

/// Explicit script context; no process-global bindings are used.
#[derive(Debug, Clone, Default)]
pub struct ScriptContext {
    engine: Bindings,
    global: Bindings,
    source_name: Option<String>,
}

impl ScriptContext {
    /// Creates an empty context.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            engine: Bindings::new(),
            global: Bindings::new(),
            source_name: None,
        }
    }

    /// Returns bindings for one scope.
    #[must_use]
    pub fn bindings(&self, scope: ScriptScope) -> &Bindings {
        match scope {
            ScriptScope::Engine => &self.engine,
            ScriptScope::Global => &self.global,
        }
    }

    /// Returns mutable bindings for one scope.
    pub fn bindings_mut(&mut self, scope: ScriptScope) -> &mut Bindings {
        match scope {
            ScriptScope::Engine => &mut self.engine,
            ScriptScope::Global => &mut self.global,
        }
    }

    /// Replaces bindings for one scope.
    pub fn set_bindings(&mut self, scope: ScriptScope, bindings: Bindings) {
        *self.bindings_mut(scope) = bindings;
    }

    /// Sets a diagnostic source name.
    pub fn set_source_name(&mut self, source_name: impl Into<String>) {
        self.source_name = Some(source_name.into());
    }

    /// Returns the diagnostic source name.
    #[must_use]
    pub fn source_name(&self) -> Option<&str> {
        self.source_name.as_deref()
    }

    fn scope(&self) -> Scope<'static> {
        let mut scope = Scope::new();
        self.global.append_to_scope(&mut scope);
        self.engine.append_to_scope(&mut scope);
        scope
    }

    fn update_engine(&mut self, scope: &Scope<'_>) {
        self.engine.replace_from_scope(scope);
    }
}

/// Compiled, reusable script AST.
#[derive(Debug, Clone)]
pub struct CompiledScript {
    ast: AST,
    source_name: Option<String>,
}

impl CompiledScript {
    /// Returns the compiled AST.
    #[must_use]
    pub const fn ast(&self) -> &AST {
        &self.ast
    }

    /// Evaluates this script using an engine and context.
    pub fn eval(
        &self,
        engine: &ScriptEngine,
        context: &mut ScriptContext,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        let mut scope = context.scope();
        let result = engine
            .eval_ast_with_scope(&mut scope, &self.ast)
            .map_err(|error| {
                ScriptRuntimeException::from_error(&error, self.source_name.as_deref())
            })?;
        context.update_engine(&scope);
        Ok(result)
    }
}

/// Metadata for the built-in constrained engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScriptEngineFactory;

impl ScriptEngineFactory {
    /// Engine display name.
    #[must_use]
    pub const fn engine_name(self) -> &'static str {
        "HiTool Rhai"
    }

    /// Language name.
    #[must_use]
    pub const fn language_name(self) -> &'static str {
        "Rhai"
    }

    /// Accepted aliases.
    #[must_use]
    pub const fn names(self) -> &'static [&'static str] {
        &["rhai", "js", "javascript"]
    }
}

/// Full compile/evaluate/invoke facade over a constrained engine.
pub struct FullSupportScriptEngine {
    engine: ScriptEngine,
    context: ScriptContext,
    active_ast: Option<AST>,
}

impl FullSupportScriptEngine {
    /// Wraps an explicitly configured engine.
    #[must_use]
    pub fn from_engine(engine: ScriptEngine) -> Self {
        Self {
            engine,
            context: ScriptContext::new(),
            active_ast: None,
        }
    }

    /// Creates a supported named engine.
    pub fn new(name_or_extension_or_mime: &str) -> Result<Self, ScriptRuntimeException> {
        match ScriptLanguage::resolve(name_or_extension_or_mime)
            .map_err(|error| ScriptRuntimeException::from_error(&error, None))?
        {
            ScriptLanguage::Rhai => Ok(Self::from_engine(ScriptEngine::default())),
            language => Err(ScriptRuntimeException::new(format!(
                "script language `{language:?}` requires an optional engine"
            ))),
        }
    }

    /// Compiles script text.
    pub fn compile(&self, script: &str) -> Result<CompiledScript, ScriptRuntimeException> {
        self.engine
            .compile(script)
            .map(|ast| CompiledScript {
                ast,
                source_name: self.context.source_name.clone(),
            })
            .map_err(|error| ScriptRuntimeException::from_error(&error, self.context.source_name()))
    }

    /// Compiles script text read from a bounded reader.
    pub fn compile_reader(
        &self,
        reader: impl Read,
    ) -> Result<CompiledScript, ScriptRuntimeException> {
        let script = read_script(reader)?;
        self.compile(&script)
    }

    /// Evaluates text in the engine's current context.
    pub fn eval(&mut self, script: &str) -> Result<Dynamic, ScriptRuntimeException> {
        let compiled = self.compile(script)?;
        let result = compiled.eval(&self.engine, &mut self.context)?;
        self.active_ast = Some(compiled.ast);
        Ok(result)
    }

    /// Evaluates bounded reader input in the current context.
    pub fn eval_reader(&mut self, reader: impl Read) -> Result<Dynamic, ScriptRuntimeException> {
        let script = read_script(reader)?;
        self.eval(&script)
    }

    /// Evaluates text with an explicit context.
    pub fn eval_with_context(
        &mut self,
        script: &str,
        context: &mut ScriptContext,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        let compiled = self.compile(script)?;
        let result = compiled.eval(&self.engine, context)?;
        self.active_ast = Some(compiled.ast);
        Ok(result)
    }

    /// Evaluates reader input with an explicit context.
    pub fn eval_reader_with_context(
        &mut self,
        reader: impl Read,
        context: &mut ScriptContext,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        self.eval_with_context(&read_script(reader)?, context)
    }

    /// Evaluates text with temporary engine bindings.
    pub fn eval_with_bindings(
        &mut self,
        script: &str,
        bindings: Bindings,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        let mut context = ScriptContext::new();
        context.set_bindings(ScriptScope::Engine, bindings);
        self.eval_with_context(script, &mut context)
    }

    /// Evaluates reader input with temporary bindings.
    pub fn eval_reader_with_bindings(
        &mut self,
        reader: impl Read,
        bindings: Bindings,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        self.eval_with_bindings(&read_script(reader)?, bindings)
    }

    /// Invokes a function in the most recently compiled/evaluated AST.
    pub fn invoke_function(
        &self,
        name: &str,
        args: Vec<Dynamic>,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        let ast = self.active_ast.as_ref().ok_or_else(|| {
            ScriptRuntimeException::new("no compiled script is active for invocation")
        })?;
        let mut scope = self.context.scope();
        self.engine
            .call_fn(&mut scope, ast, name, args)
            .map_err(|error| ScriptRuntimeException::from_error(&error, self.context.source_name()))
    }

    /// Invokes a method with a bound dynamic `this` value.
    pub fn invoke_method(
        &self,
        this: &mut Dynamic,
        name: &str,
        args: Vec<Dynamic>,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        let ast = self.active_ast.as_ref().ok_or_else(|| {
            ScriptRuntimeException::new("no compiled script is active for invocation")
        })?;
        let mut scope = self.context.scope();
        self.engine
            .call_method(&mut scope, ast, this, name, args)
            .map_err(|error| ScriptRuntimeException::from_error(&error, self.context.source_name()))
    }

    /// Returns a typed Rust invocation facade.
    pub fn interface(&mut self) -> ScriptInterface<'_> {
        ScriptInterface {
            engine: self,
            this: None,
        }
    }

    /// Returns an invocation facade bound to a `this` value.
    pub fn interface_on(&mut self, this: Dynamic) -> ScriptInterface<'_> {
        ScriptInterface {
            engine: self,
            this: Some(this),
        }
    }

    /// Stores an engine-scope value.
    pub fn put(&mut self, key: impl Into<String>, value: impl Into<Dynamic>) -> Option<Dynamic> {
        self.context.engine.insert(key, value)
    }

    /// Returns an engine-scope value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Dynamic> {
        self.context.engine.get(key)
    }

    /// Returns bindings for one scope.
    #[must_use]
    pub fn bindings(&self, scope: ScriptScope) -> &Bindings {
        self.context.bindings(scope)
    }

    /// Replaces bindings for one scope.
    pub fn set_bindings(&mut self, bindings: Bindings, scope: ScriptScope) {
        self.context.set_bindings(scope, bindings);
    }

    /// Creates independent empty bindings.
    #[must_use]
    pub const fn create_bindings(&self) -> Bindings {
        Bindings::new()
    }

    /// Returns the current context.
    #[must_use]
    pub const fn context(&self) -> &ScriptContext {
        &self.context
    }

    /// Replaces the current context.
    pub fn set_context(&mut self, context: ScriptContext) {
        self.context = context;
    }

    /// Returns engine metadata.
    #[must_use]
    pub const fn factory(&self) -> ScriptEngineFactory {
        ScriptEngineFactory
    }
}

/// Rust replacement for Java interface proxies over invocable scripts.
pub struct ScriptInterface<'a> {
    engine: &'a mut FullSupportScriptEngine,
    this: Option<Dynamic>,
}

impl ScriptInterface<'_> {
    /// Invokes a function or a method when this interface is object-bound.
    pub fn invoke(
        &mut self,
        name: &str,
        args: Vec<Dynamic>,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        match &mut self.this {
            Some(this) => self.engine.invoke_method(this, name, args),
            None => self.engine.invoke_function(name, args),
        }
    }

    /// Returns the current bound object after method mutations.
    #[must_use]
    pub fn bound_this(&self) -> Option<&Dynamic> {
        self.this.as_ref()
    }
}

/// JavaScript-named compatibility wrapper backed by sandboxed Rhai.
pub struct JavaScriptEngine(FullSupportScriptEngine);

impl JavaScriptEngine {
    /// Creates a fresh constrained engine.
    #[must_use]
    pub fn new() -> Self {
        Self(FullSupportScriptEngine::from_engine(ScriptEngine::default()))
    }

    /// Creates a fresh instance, mirroring Hutool's non-singleton method.
    #[must_use]
    pub fn instance() -> Self {
        Self::new()
    }
}

impl Default for JavaScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for JavaScriptEngine {
    type Target = FullSupportScriptEngine;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JavaScriptEngine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn read_script(reader: impl Read) -> Result<String, ScriptRuntimeException> {
    let mut bytes = Vec::new();
    reader
        .take(MAX_READER_BYTES)
        .read_to_end(&mut bytes)
        .map_err(|error| ScriptRuntimeException::from_error(&error.into(), None))?;
    if bytes.len() as u64 >= MAX_READER_BYTES {
        return Err(ScriptRuntimeException::new("script reader exceeds 1 MiB"));
    }
    String::from_utf8(bytes).map_err(|error| ScriptRuntimeException::new(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ScriptLimits, ScriptRuntimeException, ScriptUtil};
    use std::io::{self, Cursor};

    struct TestReader {
        cursor: Cursor<Vec<u8>>,
        fail: bool,
    }

    impl TestReader {
        fn good(bytes: impl Into<Vec<u8>>) -> Self {
            Self {
                cursor: Cursor::new(bytes.into()),
                fail: false,
            }
        }

        fn failing() -> Self {
            Self {
                cursor: Cursor::new(Vec::new()),
                fail: true,
            }
        }
    }

    impl Read for TestReader {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            if self.fail {
                Err(io::Error::other("injected"))
            } else {
                self.cursor.read(buffer)
            }
        }
    }

    #[test]
    fn bindings_and_context_keep_scopes_explicit() {
        let mut bindings = Bindings::new();
        assert!(bindings.is_empty());
        assert!(bindings.insert("value", 40_i64).is_none());
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings.get("value").unwrap().as_int().unwrap(), 40);
        assert_eq!(
            bindings.insert("value", 41_i64).unwrap().as_int().unwrap(),
            40
        );
        assert_eq!(bindings.remove("value").unwrap().as_int().unwrap(), 41);

        let mut context = ScriptContext::new();
        context
            .bindings_mut(ScriptScope::Global)
            .insert("value", 1_i64);
        let mut engine_bindings = Bindings::new();
        engine_bindings.insert("value", 40_i64);
        context.set_bindings(ScriptScope::Engine, engine_bindings);
        context.set_source_name("bindings.rhai");
        assert_eq!(context.source_name(), Some("bindings.rhai"));
        assert_eq!(
            context
                .bindings(ScriptScope::Global)
                .get("value")
                .unwrap()
                .as_int()
                .unwrap(),
            1
        );

        let engine = ScriptEngine::default();
        let compiled = CompiledScript {
            ast: engine.compile("value += 2; value").unwrap(),
            source_name: Some("bindings.rhai".into()),
        };
        let _ = compiled.ast();
        assert_eq!(
            compiled
                .eval(&engine, &mut context)
                .unwrap()
                .as_int()
                .unwrap(),
            42
        );
        assert_eq!(
            context
                .bindings(ScriptScope::Engine)
                .get("value")
                .unwrap()
                .as_int()
                .unwrap(),
            42
        );
    }

    #[test]
    fn runtime_exception_constructors_and_locations_match_hutool() {
        let plain = ScriptRuntimeException::new("broken");
        assert_eq!(plain.base_message(), "broken");
        assert_eq!(plain.to_string(), "broken");
        assert_eq!(plain.file_name(), None);
        assert_eq!(plain.line_number(), None);
        assert_eq!(plain.column_number(), None);

        let located = ScriptRuntimeException::with_location("broken", "job.rhai", 3, Some(7));
        assert_eq!(located.file_name(), Some("job.rhai"));
        assert_eq!(located.line_number(), Some(3));
        assert_eq!(located.column_number(), Some(7));
        assert_eq!(
            located.to_string(),
            "broken in job.rhai at line number 3 at column number 7"
        );
        assert_eq!(
            ScriptRuntimeException::with_location("broken", "job.rhai", 3, None).to_string(),
            "broken in job.rhai at line number 3"
        );
        let first = 1;
        let second = "two";
        assert_eq!(
            ScriptRuntimeException::formatted("{} + {}", &[&first, &second]).to_string(),
            "1 + two"
        );

        let mut engine = FullSupportScriptEngine::new("rhai").unwrap();
        let mut context = ScriptContext::new();
        context.set_source_name("bad.rhai");
        engine.set_context(context);
        let error = engine.compile("let = ;").unwrap_err();
        assert_eq!(error.file_name(), Some("bad.rhai"));
        assert!(error.line_number().is_some());
        let error = engine.eval("throw(\"no\")").unwrap_err();
        assert!(error.line_number().is_some());
    }

    #[test]
    fn full_engine_compiles_evaluates_readers_bindings_and_contexts() {
        let configured = ScriptEngine::new(ScriptLimits::default());
        let mut engine = FullSupportScriptEngine::from_engine(configured);
        assert!(engine.put("base", 40_i64).is_none());
        assert_eq!(engine.get("base").unwrap().as_int().unwrap(), 40);
        assert_eq!(engine.eval("base + 2").unwrap().as_int().unwrap(), 42);
        assert_eq!(
            engine
                .eval_reader(TestReader::good("base + 3"))
                .unwrap()
                .as_int()
                .unwrap(),
            43
        );
        assert!(engine.create_bindings().is_empty());

        let compiled = engine.compile_reader(TestReader::good("base + 4")).unwrap();
        let mut context = engine.context().clone();
        assert_eq!(
            compiled
                .eval(&ScriptEngine::default(), &mut context)
                .unwrap()
                .as_int()
                .unwrap(),
            44
        );

        let mut external = ScriptContext::new();
        external
            .bindings_mut(ScriptScope::Engine)
            .insert("value", 10_i64);
        assert_eq!(
            engine
                .eval_with_context("value * 2", &mut external)
                .unwrap()
                .as_int()
                .unwrap(),
            20
        );
        assert_eq!(
            engine
                .eval_reader_with_context(TestReader::good("value * 3"), &mut external)
                .unwrap()
                .as_int()
                .unwrap(),
            30
        );

        let mut temporary = Bindings::new();
        temporary.insert("number", 7_i64);
        assert_eq!(
            engine
                .eval_with_bindings("number * 6", temporary.clone())
                .unwrap()
                .as_int()
                .unwrap(),
            42
        );
        assert_eq!(
            engine
                .eval_reader_with_bindings(TestReader::good("number + 1"), temporary)
                .unwrap()
                .as_int()
                .unwrap(),
            8
        );

        let mut replacement = ScriptContext::new();
        replacement
            .bindings_mut(ScriptScope::Global)
            .insert("global", 9_i64);
        engine.set_context(replacement);
        assert_eq!(engine.bindings(ScriptScope::Global).len(), 1);
        let mut new_bindings = Bindings::new();
        new_bindings.insert("engine", 11_i64);
        engine.set_bindings(new_bindings, ScriptScope::Engine);
        assert_eq!(engine.get("engine").unwrap().as_int().unwrap(), 11);

        assert!(engine.compile_reader(TestReader::failing()).is_err());
        assert!(engine.eval_reader(TestReader::failing()).is_err());
        assert!(
            engine
                .eval_reader_with_context(TestReader::failing(), &mut external)
                .is_err()
        );
        assert!(
            engine
                .eval_reader_with_bindings(TestReader::failing(), Bindings::new())
                .is_err()
        );
        assert!(engine.eval("let = ;").is_err());
        assert!(engine.eval_with_context("let = ;", &mut external).is_err());
        assert!(
            engine
                .eval_with_context("throw(\"no\")", &mut external)
                .is_err()
        );

        let factory = engine.factory();
        assert_eq!(factory.engine_name(), "HiTool Rhai");
        assert_eq!(factory.language_name(), "Rhai");
        assert_eq!(factory.names(), &["rhai", "js", "javascript"]);
    }

    #[test]
    fn invocable_functions_methods_and_interfaces_share_the_active_ast() {
        let mut engine = FullSupportScriptEngine::new("js").unwrap();
        assert!(engine.invoke_function("missing", Vec::new()).is_err());
        let _ = engine
            .eval("fn add(x, y) { x + y } fn bump(x) { this += x; this }")
            .unwrap();
        assert_eq!(
            engine
                .invoke_function("add", vec![20_i64.into(), 22_i64.into()])
                .unwrap()
                .as_int()
                .unwrap(),
            42
        );
        let mut this: Dynamic = 40_i64.into();
        assert_eq!(
            engine
                .invoke_method(&mut this, "bump", vec![2_i64.into()])
                .unwrap()
                .as_int()
                .unwrap(),
            42
        );
        assert_eq!(this.as_int().unwrap(), 42);
        assert!(engine.invoke_function("missing", Vec::new()).is_err());
        assert!(
            engine
                .invoke_method(&mut this, "missing", Vec::new())
                .is_err()
        );

        let mut interface = engine.interface();
        assert_eq!(
            interface
                .invoke("add", vec![1_i64.into(), 2_i64.into()])
                .unwrap()
                .as_int()
                .unwrap(),
            3
        );
        assert!(interface.bound_this().is_none());
        drop(interface);

        let mut interface = engine.interface_on(5_i64.into());
        assert_eq!(
            interface
                .invoke("bump", vec![3_i64.into()])
                .unwrap()
                .as_int()
                .unwrap(),
            8
        );
        assert_eq!(interface.bound_this().unwrap().as_int().unwrap(), 8);

        let empty = FullSupportScriptEngine::new("rhai").unwrap();
        let mut object = Dynamic::from(1_i64);
        assert!(
            empty
                .invoke_method(&mut object, "missing", Vec::new())
                .is_err()
        );
    }

    #[test]
    fn javascript_wrapper_and_util_cover_supported_and_optional_languages() {
        let mut javascript = JavaScriptEngine::new();
        assert_eq!(javascript.eval("40 + 2").unwrap().as_int().unwrap(), 42);
        let mut instance = JavaScriptEngine::instance();
        assert_eq!(instance.eval("6 * 7").unwrap().as_int().unwrap(), 42);
        let mut default = JavaScriptEngine::default();
        assert_eq!(default.eval("41 + 1").unwrap().as_int().unwrap(), 42);

        for alias in [
            "rhai",
            "js",
            "javascript",
            "application/javascript",
            "text/javascript",
        ] {
            assert!(FullSupportScriptEngine::new(alias).is_ok());
        }
        assert!(FullSupportScriptEngine::new("unknown").is_err());
        for language in ["python", "py", "lua", "groovy"] {
            assert!(FullSupportScriptEngine::new(language).is_err());
        }

        assert!(ScriptUtil::get_script("rhai").is_ok());
        assert!(ScriptUtil::create_script("js").is_ok());
        assert!(ScriptUtil::get_js_engine().is_ok());
        assert!(ScriptUtil::create_js_engine().is_ok());
        assert_eq!(
            ScriptUtil::get_java_script_engine()
                .eval("21 * 2")
                .unwrap()
                .as_int()
                .unwrap(),
            42
        );
        let javascript = ScriptUtil::get_java_script_engine();
        let full: &FullSupportScriptEngine = &javascript;
        assert_eq!(full.factory().language_name(), "Rhai");
        assert!(ScriptUtil::get_python_engine().is_err());
        assert!(ScriptUtil::create_python_engine().is_err());
        assert!(ScriptUtil::get_lua_engine().is_err());
        assert!(ScriptUtil::create_lua_engine().is_err());
        assert!(ScriptUtil::get_groovy_engine().is_err());
        assert!(ScriptUtil::create_groovy_engine().is_err());

        assert_eq!(ScriptUtil::eval("40 + 2").unwrap().as_int().unwrap(), 42);
        let mut context = ScriptContext::new();
        context
            .bindings_mut(ScriptScope::Engine)
            .insert("x", 40_i64);
        assert_eq!(
            ScriptUtil::eval_with_context("x + 2", &mut context)
                .unwrap()
                .as_int()
                .unwrap(),
            42
        );
        let mut bindings = Bindings::new();
        bindings.insert("x", 40_i64);
        assert_eq!(
            ScriptUtil::eval_with_bindings("x + 2", bindings)
                .unwrap()
                .as_int()
                .unwrap(),
            42
        );
        let function = "fn answer(x) { x + 2 }";
        assert_eq!(
            ScriptUtil::invoke(function, "answer", vec![40_i64.into()])
                .unwrap()
                .as_int()
                .unwrap(),
            42
        );
        let compiled = ScriptUtil::compile("40 + 2").unwrap();
        assert_eq!(
            compiled
                .eval(&ScriptEngine::default(), &mut ScriptContext::new())
                .unwrap()
                .as_int()
                .unwrap(),
            42
        );
        let engine = FullSupportScriptEngine::new("rhai").unwrap();
        assert!(ScriptUtil::compile_with_engine(&engine, "40 + 2").is_ok());
        assert!(ScriptUtil::eval_invocable("let = ;").is_err());
        assert!(ScriptUtil::eval("let = ;").is_err());
        assert!(ScriptUtil::eval_with_context("let = ;", &mut context).is_err());
        assert!(ScriptUtil::eval_with_bindings("let = ;", Bindings::new()).is_err());
        assert!(ScriptUtil::invoke("let = ;", "x", Vec::new()).is_err());
        assert!(ScriptUtil::invoke("fn ok() {}", "missing", Vec::new()).is_err());
        assert!(ScriptUtil::compile("let = ;").is_err());
        assert!(ScriptUtil::compile_with_engine(&engine, "let = ;").is_err());
    }

    #[test]
    fn bounded_reader_rejects_io_invalid_utf8_and_oversized_scripts() {
        assert!(read_script(TestReader::failing()).is_err());
        assert!(read_script(TestReader::good(vec![0xff])).is_err());
        assert!(read_script(TestReader::good(vec![b'x'; 1_048_577])).is_err());
        assert_eq!(read_script(TestReader::good("40 + 2")).unwrap(), "40 + 2");
    }
}
