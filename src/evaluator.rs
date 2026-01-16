// src/evaluator.rs
//! Evaluator for executing Aether AST

use crate::ast::{BinOp, Expr, Program, Stmt, UnaryOp};
use crate::builtins::BuiltInRegistry;
use crate::environment::Environment;
use crate::module_system::{
    DisabledModuleResolver, ModuleContext, ModuleResolveError, ModuleResolver, ResolvedModule,
};
use crate::value::{GeneratorState, Value};
use serde_json::{Value as JsonValue, json};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct CallFrame {
    pub name: String,
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportErrorKind {
    ImportDisabled,
    InvalidSpecifier,
    NoBaseDir,
    NotFound,
    AccessDenied,
    IoError,
    NotExported,
    CircularImport,
    ParseFailed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportError {
    pub kind: ImportErrorKind,
    pub specifier: String,
    pub detail: Option<String>,
    pub symbol: Option<String>,
    pub module_id: Option<String>,
    pub import_chain: Vec<String>,
    pub cycle: Option<Vec<String>>,
}

impl ImportError {
    fn from_resolve_error(
        specifier: &str,
        err: ModuleResolveError,
        import_chain: Vec<String>,
    ) -> Self {
        match err {
            ModuleResolveError::ImportDisabled => ImportError {
                kind: ImportErrorKind::ImportDisabled,
                specifier: specifier.to_string(),
                detail: None,
                symbol: None,
                module_id: None,
                import_chain,
                cycle: None,
            },
            ModuleResolveError::InvalidSpecifier(s) => ImportError {
                kind: ImportErrorKind::InvalidSpecifier,
                specifier: specifier.to_string(),
                detail: Some(s),
                symbol: None,
                module_id: None,
                import_chain,
                cycle: None,
            },
            ModuleResolveError::NoBaseDir(s) => ImportError {
                kind: ImportErrorKind::NoBaseDir,
                specifier: specifier.to_string(),
                detail: Some(s),
                symbol: None,
                module_id: None,
                import_chain,
                cycle: None,
            },
            ModuleResolveError::NotFound(s) => ImportError {
                kind: ImportErrorKind::NotFound,
                specifier: specifier.to_string(),
                detail: Some(s),
                symbol: None,
                module_id: None,
                import_chain,
                cycle: None,
            },
            ModuleResolveError::AccessDenied(s) => ImportError {
                kind: ImportErrorKind::AccessDenied,
                specifier: specifier.to_string(),
                detail: Some(s),
                symbol: None,
                module_id: None,
                import_chain,
                cycle: None,
            },
            ModuleResolveError::IoError(s) => ImportError {
                kind: ImportErrorKind::IoError,
                specifier: specifier.to_string(),
                detail: Some(s),
                symbol: None,
                module_id: None,
                import_chain,
                cycle: None,
            },
        }
    }

    fn not_exported(specifier: &str, symbol: &str, import_chain: Vec<String>) -> Self {
        ImportError {
            kind: ImportErrorKind::NotExported,
            specifier: specifier.to_string(),
            detail: None,
            symbol: Some(symbol.to_string()),
            module_id: None,
            import_chain,
            cycle: None,
        }
    }

    fn circular(module_id: &str, cycle: Vec<String>, import_chain: Vec<String>) -> Self {
        ImportError {
            kind: ImportErrorKind::CircularImport,
            specifier: module_id.to_string(),
            detail: None,
            symbol: None,
            module_id: Some(module_id.to_string()),
            import_chain,
            cycle: Some(cycle),
        }
    }

    fn parse_failed(module_id: &str, detail: String, import_chain: Vec<String>) -> Self {
        ImportError {
            kind: ImportErrorKind::ParseFailed,
            specifier: module_id.to_string(),
            detail: Some(detail),
            symbol: None,
            module_id: Some(module_id.to_string()),
            import_chain,
            cycle: None,
        }
    }
}

/// Runtime errors
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    /// Variable not found
    UndefinedVariable(String),

    /// Type mismatch - simple message
    TypeError(String),

    /// Type mismatch - detailed version
    TypeErrorDetailed { expected: String, got: String },

    /// Invalid operation
    InvalidOperation(String),

    /// Division by zero
    DivisionByZero,

    /// Function not found or not callable
    NotCallable(String),

    /// Wrong number of arguments
    WrongArity { expected: usize, got: usize },

    /// Return statement (used for control flow)
    Return(Value),

    /// Yield statement (used for generators)
    Yield(Value),

    /// Break statement (used for loop control)
    Break,

    /// Continue statement (used for loop control)
    Continue,

    /// Throw statement (user-thrown error)
    Throw(Value),

    /// Structured Import/Export module errors (with import chain)
    ImportError(Box<ImportError>),

    /// Attach a captured call stack to an error.
    WithCallStack {
        error: Box<RuntimeError>,
        call_stack: Vec<CallFrame>,
    },

    /// Execution limit exceeded
    ExecutionLimit(crate::runtime::ExecutionLimitError),

    /// Custom error message (用于IO操作等)
    CustomError(String),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuntimeError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
            RuntimeError::TypeError(msg) => write!(f, "Type error: {}", msg),
            RuntimeError::TypeErrorDetailed { expected, got } => {
                write!(f, "Type error: expected {}, got {}", expected, got)
            }
            RuntimeError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            RuntimeError::DivisionByZero => write!(f, "Division by zero"),
            RuntimeError::NotCallable(name) => write!(f, "Not callable: {}", name),
            RuntimeError::WrongArity { expected, got } => {
                write!(
                    f,
                    "Wrong number of arguments: expected {}, got {}",
                    expected, got
                )
            }
            RuntimeError::Return(val) => write!(f, "Return: {}", val),
            RuntimeError::Yield(val) => write!(f, "Yield: {}", val),
            RuntimeError::Break => write!(f, "Break outside of loop"),
            RuntimeError::Continue => write!(f, "Continue outside of loop"),
            RuntimeError::Throw(val) => write!(f, "Throw: {}", val),
            RuntimeError::ImportError(e) => {
                let msg = match e.kind {
                    ImportErrorKind::ImportDisabled => "Import is disabled".to_string(),
                    ImportErrorKind::InvalidSpecifier => format!(
                        "Invalid module specifier: {}",
                        e.detail.clone().unwrap_or_else(|| e.specifier.clone())
                    ),
                    ImportErrorKind::NoBaseDir => format!(
                        "No base directory to resolve specifier: {}",
                        e.detail.clone().unwrap_or_else(|| e.specifier.clone())
                    ),
                    ImportErrorKind::NotFound => format!(
                        "Module not found: {}",
                        e.detail.clone().unwrap_or_else(|| e.specifier.clone())
                    ),
                    ImportErrorKind::AccessDenied => format!(
                        "Module access denied: {}",
                        e.detail.clone().unwrap_or_else(|| e.specifier.clone())
                    ),
                    ImportErrorKind::IoError => format!(
                        "Module IO error: {}",
                        e.detail.clone().unwrap_or_else(|| e.specifier.clone())
                    ),
                    ImportErrorKind::NotExported => format!(
                        "'{}' is not exported by module {}",
                        e.symbol.clone().unwrap_or_else(|| "<unknown>".to_string()),
                        e.specifier
                    ),
                    ImportErrorKind::CircularImport => {
                        let cycle = e.cycle.clone().unwrap_or_else(|| vec![e.specifier.clone()]);
                        format!("circular import detected: {}", cycle.join(" -> "))
                    }
                    ImportErrorKind::ParseFailed => format!(
                        "parse failed for module {}: {}",
                        e.module_id.clone().unwrap_or_else(|| e.specifier.clone()),
                        e.detail.clone().unwrap_or_else(|| "<unknown>".to_string())
                    ),
                };

                write!(f, "Import error: {}", msg)?;
                if !e.import_chain.is_empty() {
                    write!(f, "\nImport chain: {}", e.import_chain.join(" -> "))?;
                }
                Ok(())
            }
            RuntimeError::WithCallStack { error, call_stack } => {
                write!(f, "{}", error)?;
                if !call_stack.is_empty() {
                    let frames = call_stack
                        .iter()
                        .map(|fr| fr.signature.as_str())
                        .collect::<Vec<_>>()
                        .join(" -> ");
                    write!(f, "\nCall stack: {}", frames)?;
                }
                Ok(())
            }
            RuntimeError::CustomError(msg) => write!(f, "{}", msg),
            RuntimeError::ExecutionLimit(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for RuntimeError {}

pub type EvalResult = Result<Value, RuntimeError>;

/// A structured, machine-readable error report.
///
/// This is intended for CLI/host integrations that need stable fields
/// (instead of parsing human-readable error strings).
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorReport {
    pub phase: String,
    pub kind: String,
    pub message: String,
    pub import_chain: Vec<String>,
    pub call_stack: Vec<CallFrame>,
}

impl ErrorReport {
    pub fn io_error(message: impl Into<String>) -> Self {
        ErrorReport {
            phase: "io".to_string(),
            kind: "IoError".to_string(),
            message: message.into(),
            import_chain: Vec::new(),
            call_stack: Vec::new(),
        }
    }

    pub fn parse_error(message: impl Into<String>) -> Self {
        ErrorReport {
            phase: "parse".to_string(),
            kind: "ParseError".to_string(),
            message: message.into(),
            import_chain: Vec::new(),
            call_stack: Vec::new(),
        }
    }

    pub fn to_json_value(&self) -> JsonValue {
        let call_stack = self
            .call_stack
            .iter()
            .map(|fr| json!({"name": fr.name, "signature": fr.signature}))
            .collect::<Vec<_>>();

        json!({
            "phase": self.phase,
            "kind": self.kind,
            "message": self.message,
            "import_chain": self.import_chain,
            "call_stack": call_stack,
        })
    }

    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.to_json_value()).unwrap_or_else(|_| {
            "{\n  \"error\": \"failed to serialize ErrorReport\"\n}".to_string()
        })
    }
}

impl RuntimeError {
    fn peel_call_stack(&self) -> (&RuntimeError, Vec<CallFrame>) {
        let mut current = self;
        let mut frames: Vec<CallFrame> = Vec::new();

        while let RuntimeError::WithCallStack { error, call_stack } = current {
            if frames.is_empty() {
                frames = call_stack.clone();
            }
            current = error.as_ref();
        }

        (current, frames)
    }

    fn kind_name(&self) -> String {
        match self {
            RuntimeError::UndefinedVariable(_) => "UndefinedVariable",
            RuntimeError::TypeError(_) | RuntimeError::TypeErrorDetailed { .. } => "TypeError",
            RuntimeError::InvalidOperation(_) => "InvalidOperation",
            RuntimeError::DivisionByZero => "DivisionByZero",
            RuntimeError::NotCallable(_) => "NotCallable",
            RuntimeError::WrongArity { .. } => "WrongArity",
            RuntimeError::Return(_) => "Return",
            RuntimeError::Yield(_) => "Yield",
            RuntimeError::Break => "Break",
            RuntimeError::Continue => "Continue",
            RuntimeError::Throw(_) => "Throw",
            RuntimeError::ImportError(e) => match e.kind {
                ImportErrorKind::ImportDisabled => "ImportDisabled",
                ImportErrorKind::InvalidSpecifier => "InvalidSpecifier",
                ImportErrorKind::NoBaseDir => "NoBaseDir",
                ImportErrorKind::NotFound => "NotFound",
                ImportErrorKind::AccessDenied => "AccessDenied",
                ImportErrorKind::IoError => "IoError",
                ImportErrorKind::NotExported => "NotExported",
                ImportErrorKind::CircularImport => "CircularImport",
                ImportErrorKind::ParseFailed => "ParseFailed",
            },
            RuntimeError::WithCallStack { .. } => "WithCallStack",
            RuntimeError::ExecutionLimit(_) => "ExecutionLimit",
            RuntimeError::CustomError(_) => "CustomError",
        }
        .to_string()
    }

    fn base_message(&self) -> String {
        match self {
            RuntimeError::WithCallStack { error, .. } => error.base_message(),
            RuntimeError::ImportError(e) => {
                let msg = match e.kind {
                    ImportErrorKind::ImportDisabled => "Import is disabled".to_string(),
                    ImportErrorKind::InvalidSpecifier => format!(
                        "Invalid module specifier: {}",
                        e.detail.clone().unwrap_or_else(|| e.specifier.clone())
                    ),
                    ImportErrorKind::NoBaseDir => format!(
                        "No base directory to resolve specifier: {}",
                        e.detail.clone().unwrap_or_else(|| e.specifier.clone())
                    ),
                    ImportErrorKind::NotFound => format!(
                        "Module not found: {}",
                        e.detail.clone().unwrap_or_else(|| e.specifier.clone())
                    ),
                    ImportErrorKind::AccessDenied => format!(
                        "Module access denied: {}",
                        e.detail.clone().unwrap_or_else(|| e.specifier.clone())
                    ),
                    ImportErrorKind::IoError => format!(
                        "Module IO error: {}",
                        e.detail.clone().unwrap_or_else(|| e.specifier.clone())
                    ),
                    ImportErrorKind::NotExported => format!(
                        "'{}' is not exported by module {}",
                        e.symbol.clone().unwrap_or_else(|| "<unknown>".to_string()),
                        e.specifier
                    ),
                    ImportErrorKind::CircularImport => {
                        let cycle = e.cycle.clone().unwrap_or_else(|| vec![e.specifier.clone()]);
                        format!("circular import detected: {}", cycle.join(" -> "))
                    }
                    ImportErrorKind::ParseFailed => format!(
                        "parse failed for module {}: {}",
                        e.module_id.clone().unwrap_or_else(|| e.specifier.clone()),
                        e.detail.clone().unwrap_or_else(|| "<unknown>".to_string())
                    ),
                };

                format!("Import error: {msg}")
            }
            other => other.to_string(),
        }
    }

    pub fn to_error_report(&self) -> ErrorReport {
        let (base, call_stack) = self.peel_call_stack();

        let import_chain = match base {
            RuntimeError::ImportError(e) => e.import_chain.clone(),
            _ => Vec::new(),
        };

        ErrorReport {
            phase: "runtime".to_string(),
            kind: base.kind_name(),
            message: base.base_message(),
            import_chain,
            call_stack,
        }
    }
}

/// Evaluator for Aether programs
pub struct Evaluator {
    /// Global environment
    env: Rc<RefCell<Environment>>,
    /// Built-in function registry
    registry: BuiltInRegistry,
    /// In-memory trace buffer (for DSL-safe debugging; no stdout/files/network)
    trace: VecDeque<String>,
    /// Monotonic sequence for trace entries (starts at 1)
    trace_seq: u64,
    /// Structured trace entries (new in Stage 3.2)
    trace_entries: VecDeque<crate::runtime::TraceEntry>,
    /// Maximum number of trace entries to keep in buffer
    trace_buffer_size: usize,

    /// Module resolver (Import/Export). Defaults to disabled for DSL safety.
    module_resolver: Box<dyn ModuleResolver>,
    /// Module export cache: module_id -> exports
    module_cache: HashMap<String, HashMap<String, Value>>,
    /// Module load stack for cycle detection
    module_stack: Vec<String>,
    /// Current module export table stack (only when evaluating an imported module)
    export_stack: Vec<HashMap<String, Value>>,
    /// Optional base directory context for resolving relative imports (e.g. eval_file)
    import_base_stack: Vec<ModuleContext>,

    /// Call stack for better debugging (user functions + builtins)
    call_stack: Vec<CallFrame>,

    /// Execution limits configuration
    limits: crate::runtime::ExecutionLimits,
    /// Step counter (for step limit enforcement)
    step_counter: std::cell::Cell<usize>,
    /// Call stack depth counter (for recursion depth limit enforcement)
    call_stack_depth: std::cell::Cell<usize>,
    /// Execution start time (for timeout enforcement)
    start_time: std::cell::Cell<Option<std::time::Instant>>,
}

impl Evaluator {
    /// Default maximum number of trace entries to keep in buffer
    const DEFAULT_TRACE_BUFFER_SIZE: usize = 1024;

    fn register_builtins_into_env(registry: &BuiltInRegistry, env: &mut Environment) {
        for name in registry.names() {
            let arity = registry.get(&name).map(|(_, a)| a).unwrap_or(0);
            env.set(name.clone(), Value::BuiltIn { name, arity });
        }
    }

    /// Check and increment step counter
    fn eval_step(&self) -> Result<(), RuntimeError> {
        if let Some(limit) = self.limits.max_steps {
            let steps = self.step_counter.get();
            if steps >= limit {
                return Err(RuntimeError::ExecutionLimit(
                    crate::runtime::ExecutionLimitError::StepLimitExceeded { steps, limit },
                ));
            }
            self.step_counter.set(steps + 1);
        }
        Ok(())
    }

    /// Check execution timeout
    fn check_timeout(&self) -> Result<(), RuntimeError> {
        if let Some(limit_ms) = self.limits.max_duration_ms
            && let Some(start) = self.start_time.get()
        {
            let elapsed = start.elapsed().as_millis() as u64;
            if elapsed >= limit_ms {
                return Err(RuntimeError::ExecutionLimit(
                    crate::runtime::ExecutionLimitError::DurationExceeded {
                        duration_ms: elapsed,
                        limit: limit_ms,
                    },
                ));
            }
        }
        Ok(())
    }

    /// Enter function call (check recursion depth)
    fn enter_call(&self) -> Result<(), RuntimeError> {
        if let Some(limit) = self.limits.max_recursion_depth {
            let depth = self.call_stack_depth.get();
            if depth >= limit {
                return Err(RuntimeError::ExecutionLimit(
                    crate::runtime::ExecutionLimitError::RecursionDepthExceeded { depth, limit },
                ));
            }
            self.call_stack_depth.set(depth + 1);
        }
        Ok(())
    }

    /// Exit function call (decrement recursion depth)
    fn exit_call(&self) {
        if self.limits.max_recursion_depth.is_some() {
            let depth = self.call_stack_depth.get();
            self.call_stack_depth.set(depth.saturating_sub(1));
        }
    }

    /// Set execution limits (public API)
    pub fn set_limits(&mut self, limits: crate::runtime::ExecutionLimits) {
        self.limits = limits;
    }

    /// Get execution limits (public API)
    pub fn limits(&self) -> &crate::runtime::ExecutionLimits {
        &self.limits
    }

    fn is_control_flow_error(err: &RuntimeError) -> bool {
        matches!(
            err,
            RuntimeError::Return(_)
                | RuntimeError::Yield(_)
                | RuntimeError::Break
                | RuntimeError::Continue
        )
    }

    fn attach_call_stack_if_absent(&self, err: RuntimeError) -> RuntimeError {
        if Self::is_control_flow_error(&err) {
            return err;
        }
        match err {
            RuntimeError::WithCallStack { .. } => err,
            other => RuntimeError::WithCallStack {
                error: Box::new(other),
                call_stack: self.call_stack.clone(),
            },
        }
    }

    /// Create a new evaluator (默认禁用IO)
    pub fn new() -> Self {
        Self::with_permissions_and_trace_buffer(
            crate::builtins::IOPermissions::default(),
            Self::DEFAULT_TRACE_BUFFER_SIZE,
        )
    }

    /// Create a new evaluator with custom IO permissions
    pub fn with_permissions(permissions: crate::builtins::IOPermissions) -> Self {
        Self::with_permissions_and_trace_buffer(permissions, Self::DEFAULT_TRACE_BUFFER_SIZE)
    }

    /// Create a new evaluator with custom IO permissions and trace buffer size
    pub fn with_permissions_and_trace_buffer(
        permissions: crate::builtins::IOPermissions,
        trace_buffer_size: usize,
    ) -> Self {
        let env = Rc::new(RefCell::new(Environment::new()));

        // Register built-in functions with permissions
        let registry = BuiltInRegistry::with_permissions(permissions);
        Self::register_builtins_into_env(&registry, &mut env.borrow_mut());

        Evaluator {
            env,
            registry,
            trace: VecDeque::new(),
            trace_seq: 0,
            trace_entries: VecDeque::new(),
            trace_buffer_size,

            module_resolver: Box::new(DisabledModuleResolver),
            module_cache: HashMap::new(),
            module_stack: Vec::new(),
            export_stack: Vec::new(),
            import_base_stack: Vec::new(),

            call_stack: Vec::new(),

            limits: crate::runtime::ExecutionLimits::default(),
            step_counter: std::cell::Cell::new(0),
            call_stack_depth: std::cell::Cell::new(0),
            start_time: std::cell::Cell::new(None),
        }
    }

    /// Create evaluator with custom environment
    pub fn with_env(env: Rc<RefCell<Environment>>) -> Self {
        let registry = BuiltInRegistry::new();
        Evaluator {
            env,
            registry,
            trace: VecDeque::new(),
            trace_seq: 0,
            trace_entries: VecDeque::new(),
            trace_buffer_size: Self::DEFAULT_TRACE_BUFFER_SIZE,

            module_resolver: Box::new(DisabledModuleResolver),
            module_cache: HashMap::new(),
            module_stack: Vec::new(),
            export_stack: Vec::new(),
            import_base_stack: Vec::new(),

            call_stack: Vec::new(),

            limits: crate::runtime::ExecutionLimits::default(),
            step_counter: std::cell::Cell::new(0),
            call_stack_depth: std::cell::Cell::new(0),
            start_time: std::cell::Cell::new(None),
        }
    }

    /// Clear the call stack (used by top-level entry points like `Aether::eval`).
    pub fn clear_call_stack(&mut self) {
        self.call_stack.clear();
    }

    /// Configure the module resolver used for `Import/Export`.
    pub fn set_module_resolver(&mut self, resolver: Box<dyn ModuleResolver>) {
        self.module_resolver = resolver;
    }

    /// Push a base directory context for resolving relative imports.
    ///
    /// This is typically used by CLI `eval_file()` wrappers.
    pub fn push_import_base(&mut self, module_id: String, base_dir: Option<std::path::PathBuf>) {
        self.import_base_stack.push(ModuleContext {
            module_id,
            base_dir,
        });
    }

    /// Pop the most recent base directory context.
    pub fn pop_import_base(&mut self) {
        self.import_base_stack.pop();
    }

    /// Append a trace entry (host-readable; no IO side effects).
    pub fn trace_push(&mut self, msg: String) {
        self.trace_seq = self.trace_seq.saturating_add(1);
        let entry = format!("#{} {}", self.trace_seq, msg);

        if self.trace.len() >= self.trace_buffer_size {
            self.trace.pop_front();
        }
        self.trace.push_back(entry);
    }

    /// Push a structured trace entry (Stage 3.2)
    fn trace_push_entry(&mut self, entry: crate::runtime::TraceEntry) {
        self.trace_seq = self.trace_seq.saturating_add(1);

        // Add to structured entries
        if self.trace_entries.len() >= self.trace_buffer_size {
            self.trace_entries.pop_front();
        }
        self.trace_entries.push_back(entry.clone());

        // Also add to formatted trace (for backward compatibility)
        let formatted = entry.format();
        let msg = format!("#{} {}", self.trace_seq, formatted);
        if self.trace.len() >= self.trace_buffer_size {
            self.trace.pop_front();
        }
        self.trace.push_back(msg);
    }

    /// Drain the trace buffer.
    pub fn take_trace(&mut self) -> Vec<String> {
        std::mem::take(&mut self.trace).into_iter().collect()
    }

    /// Get all structured trace entries
    pub fn trace_records(&self) -> Vec<crate::runtime::TraceEntry> {
        self.trace_entries.iter().cloned().collect()
    }

    /// Filter trace entries by level
    pub fn trace_by_level(
        &self,
        level: crate::runtime::TraceLevel,
    ) -> Vec<crate::runtime::TraceEntry> {
        self.trace_entries
            .iter()
            .filter(|e| e.level == level)
            .cloned()
            .collect()
    }

    /// Filter trace entries by category
    pub fn trace_by_category(&self, category: &str) -> Vec<crate::runtime::TraceEntry> {
        self.trace_entries
            .iter()
            .filter(|e| e.category == category)
            .cloned()
            .collect()
    }

    /// Filter trace entries by label
    pub fn trace_by_label(&self, label: &str) -> Vec<crate::runtime::TraceEntry> {
        self.trace_entries
            .iter()
            .filter(|e| e.label.as_deref() == Some(label))
            .cloned()
            .collect()
    }

    /// Filter trace entries by time range (since)
    pub fn trace_since(&self, since: std::time::Instant) -> Vec<crate::runtime::TraceEntry> {
        self.trace_entries
            .iter()
            .filter(|e| e.timestamp >= since)
            .cloned()
            .collect()
    }

    /// Apply complex filter to trace entries
    pub fn trace_filter(
        &self,
        filter: &crate::runtime::TraceFilter,
    ) -> Vec<crate::runtime::TraceEntry> {
        self.trace_entries
            .iter()
            .filter(|e| filter.matches(e))
            .cloned()
            .collect()
    }

    /// Get trace statistics
    pub fn trace_stats(&self) -> crate::runtime::TraceStats {
        use std::collections::HashMap;

        let mut by_level = HashMap::new();
        let mut by_category = HashMap::new();

        for entry in &self.trace_entries {
            *by_level.entry(entry.level).or_insert(0) += 1;
            *by_category.entry(entry.category.clone()).or_insert(0) += 1;
        }

        crate::runtime::TraceStats {
            total_entries: self.trace_entries.len(),
            by_level,
            by_category,
            buffer_size: self.trace_buffer_size,
            buffer_full: self.trace_entries.len() >= self.trace_buffer_size,
        }
    }

    /// Clear the trace buffer.
    pub fn clear_trace(&mut self) {
        self.trace.clear();
        self.trace_entries.clear();
        self.trace_seq = 0;
    }

    /// Set the maximum number of trace entries to keep in buffer
    ///
    /// If the new size is smaller than the current number of entries,
    /// excess entries will be removed from the front of the buffer.
    pub fn set_trace_buffer_size(&mut self, size: usize) {
        self.trace_buffer_size = size;

        // Trim existing buffers if necessary
        while self.trace.len() > size {
            self.trace.pop_front();
        }
        while self.trace_entries.len() > size {
            self.trace_entries.pop_front();
        }
    }

    /// Reset the environment (clear all variables and re-register built-ins)
    ///
    /// This is useful for engine pooling and global singleton patterns
    /// where you want to reuse an engine instance but ensure isolation.
    pub fn reset_env(&mut self) {
        // Create new environment
        self.env = Rc::new(RefCell::new(Environment::new()));

        // Avoid leaking trace across pooled executions
        self.trace.clear();
        self.trace_entries.clear();
        self.trace_seq = 0;

        // Reset module contexts (cache is kept; can be cleared explicitly by host if needed)
        self.import_base_stack.clear();
        self.export_stack.clear();
        self.module_stack.clear();

        // Avoid leaking call stack across pooled executions
        self.call_stack.clear();

        // Re-register built-in functions
        Self::register_builtins_into_env(&self.registry, &mut self.env.borrow_mut());
    }

    /// Set a global variable from the host (without requiring `eval`).
    pub fn set_global(&mut self, name: impl Into<String>, value: Value) {
        self.env.borrow_mut().set(name.into(), value);
    }

    /// Get a global variable value from the environment
    pub fn get_global(&self, name: &str) -> Option<Value> {
        self.env.borrow().get(name)
    }

    /// Enter a child scope (new environment whose parent is the current env).
    ///
    /// Returns the previous environment handle; pass it back to `restore_env()`.
    pub fn enter_child_scope(&mut self) -> Rc<RefCell<Environment>> {
        let prev = Rc::clone(&self.env);
        let child = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&prev))));
        self.env = child;
        prev
    }

    /// Restore a previously saved environment handle (typically from `enter_child_scope()`).
    pub fn restore_env(&mut self, prev: Rc<RefCell<Environment>>) {
        self.env = prev;
    }

    /// Evaluate a program
    pub fn eval_program(&mut self, program: &Program) -> EvalResult {
        // Record start time for timeout checking
        if self.limits.max_duration_ms.is_some() {
            self.start_time.set(Some(std::time::Instant::now()));
        }

        let mut result = Value::Null;

        for stmt in program {
            result = self.eval_statement(stmt)?;
        }

        Ok(result)
    }

    /// Evaluate a statement
    pub fn eval_statement(&mut self, stmt: &Stmt) -> EvalResult {
        // Check execution limits before each statement
        self.eval_step()?;
        self.check_timeout()?;

        match stmt {
            Stmt::Set { name, value } => {
                let val = self.eval_expression(value)?;
                self.env.borrow_mut().set(name.clone(), val.clone());
                Ok(val)
            }

            Stmt::SetIndex {
                object,
                index,
                value,
            } => {
                // Evaluate the value to be assigned
                let val = self.eval_expression(value)?;

                // For simple identifier objects, we can modify in place
                if let Expr::Identifier(name) = object.as_ref() {
                    // Get the object from environment
                    let obj = self
                        .env
                        .borrow()
                        .get(name)
                        .ok_or_else(|| RuntimeError::UndefinedVariable(name.clone()))?;

                    // Evaluate the index
                    let idx_val = self.eval_expression(index)?;

                    // Modify based on object type
                    let new_obj = match (obj, idx_val) {
                        (Value::Array(mut arr), Value::Number(n)) => {
                            let idx = n as usize;
                            if idx >= arr.len() {
                                return Err(RuntimeError::InvalidOperation(format!(
                                    "Index {} out of bounds (array length: {})",
                                    idx,
                                    arr.len()
                                )));
                            }
                            arr[idx] = val.clone();
                            Value::Array(arr)
                        }
                        (Value::Dict(mut dict), Value::String(key)) => {
                            dict.insert(key, val.clone());
                            Value::Dict(dict)
                        }
                        (obj, idx) => {
                            return Err(RuntimeError::TypeError(format!(
                                "Cannot index {} with {}",
                                obj.type_name(),
                                idx.type_name()
                            )));
                        }
                    };

                    // Update the variable in environment
                    self.env.borrow_mut().set(name.clone(), new_obj);
                    Ok(val)
                } else {
                    // For complex expressions, we can't modify in place
                    Err(RuntimeError::InvalidOperation(
                        "Can only assign to simple variable indices (e.g., dict[key], not expr[key])"
                            .to_string(),
                    ))
                }
            }

            Stmt::FuncDef { name, params, body } => {
                let func = Value::Function {
                    name: Some(name.clone()),
                    params: params.clone(),
                    body: body.clone(),
                    env: Rc::clone(&self.env),
                };
                self.env.borrow_mut().set(name.clone(), func.clone());
                Ok(func)
            }

            Stmt::GeneratorDef { name, params, body } => {
                let r#gen = Value::Generator {
                    params: params.clone(),
                    body: body.clone(),
                    env: Rc::clone(&self.env),
                    state: GeneratorState::NotStarted,
                };
                self.env.borrow_mut().set(name.clone(), r#gen.clone());
                Ok(r#gen)
            }

            Stmt::LazyDef { name, expr } => {
                let lazy = Value::Lazy {
                    expr: expr.clone(),
                    env: Rc::clone(&self.env),
                    cached: None,
                };
                self.env.borrow_mut().set(name.clone(), lazy.clone());
                Ok(lazy)
            }

            Stmt::Return(expr) => {
                let val = self.eval_expression(expr)?;
                Err(RuntimeError::Return(val))
            }

            Stmt::Yield(expr) => {
                let val = self.eval_expression(expr)?;
                Err(RuntimeError::Yield(val))
            }

            Stmt::Break => Err(RuntimeError::Break),

            Stmt::Continue => Err(RuntimeError::Continue),

            Stmt::While { condition, body } => {
                let mut result = Value::Null;

                loop {
                    let cond = self.eval_expression(condition)?;
                    if !cond.is_truthy() {
                        break;
                    }

                    let mut should_break = false;
                    for stmt in body {
                        match self.eval_statement(stmt) {
                            Ok(val) => result = val,
                            Err(RuntimeError::Break) => {
                                should_break = true;
                                break;
                            }
                            Err(RuntimeError::Continue) => break,
                            Err(e) => return Err(e),
                        }
                    }

                    if should_break {
                        break;
                    }
                }

                Ok(result)
            }

            Stmt::For {
                var,
                iterable,
                body,
            } => {
                let iter_val = self.eval_expression(iterable)?;
                let mut result = Value::Null;

                match iter_val {
                    Value::Array(arr) => {
                        let mut should_break = false;
                        for item in arr {
                            self.env.borrow_mut().set(var.clone(), item);
                            for stmt in body {
                                match self.eval_statement(stmt) {
                                    Ok(val) => result = val,
                                    Err(RuntimeError::Break) => {
                                        should_break = true;
                                        break;
                                    }
                                    Err(RuntimeError::Continue) => break,
                                    Err(e) => return Err(e),
                                }
                            }
                            if should_break {
                                break;
                            }
                        }
                    }
                    _ => {
                        return Err(RuntimeError::TypeError(format!(
                            "Cannot iterate over {}",
                            iter_val.type_name()
                        )));
                    }
                }

                Ok(result)
            }

            Stmt::ForIndexed {
                index_var,
                value_var,
                iterable,
                body,
            } => {
                let iter_val = self.eval_expression(iterable)?;
                let mut result = Value::Null;

                match iter_val {
                    Value::Array(arr) => {
                        let mut should_break = false;
                        for (idx, item) in arr.iter().enumerate() {
                            self.env
                                .borrow_mut()
                                .set(index_var.clone(), Value::Number(idx as f64));
                            self.env.borrow_mut().set(value_var.clone(), item.clone());
                            for stmt in body {
                                match self.eval_statement(stmt) {
                                    Ok(val) => result = val,
                                    Err(RuntimeError::Break) => {
                                        should_break = true;
                                        break;
                                    }
                                    Err(RuntimeError::Continue) => break,
                                    Err(e) => return Err(e),
                                }
                            }
                            if should_break {
                                break;
                            }
                        }
                    }
                    _ => {
                        return Err(RuntimeError::TypeError(format!(
                            "Cannot iterate over {}",
                            iter_val.type_name()
                        )));
                    }
                }

                Ok(result)
            }

            Stmt::Switch {
                expr,
                cases,
                default,
            } => {
                let val = self.eval_expression(expr)?;

                for (case_expr, case_body) in cases {
                    let case_val = self.eval_expression(case_expr)?;
                    if val.equals(&case_val) {
                        let mut result = Value::Null;
                        for stmt in case_body {
                            result = self.eval_statement(stmt)?;
                        }
                        return Ok(result);
                    }
                }

                if let Some(default_body) = default {
                    let mut result = Value::Null;
                    for stmt in default_body {
                        result = self.eval_statement(stmt)?;
                    }
                    return Ok(result);
                }

                Ok(Value::Null)
            }

            Stmt::Import {
                names,
                path,
                aliases,
                namespace,
            } => self.eval_import(names, path, aliases, namespace.as_ref()),

            Stmt::Export(name) => self.eval_export(name),

            Stmt::Throw(expr) => {
                let val = self.eval_expression(expr)?;
                Err(RuntimeError::Throw(val))
            }

            Stmt::Expression(expr) => self.eval_expression(expr),
        }
    }

    /// Evaluate an expression
    pub fn eval_expression(&mut self, expr: &Expr) -> EvalResult {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),

            Expr::BigInteger(s) => {
                // 将大整数字符串转换为 Fraction (分母为1的分数)
                use num_bigint::BigInt;
                use num_rational::Ratio;

                match s.parse::<BigInt>() {
                    Ok(big_int) => Ok(Value::Fraction(Ratio::new(big_int, BigInt::from(1)))),
                    Err(_) => Err(RuntimeError::InvalidOperation(format!(
                        "Invalid big integer: {}",
                        s
                    ))),
                }
            }

            Expr::String(s) => Ok(Value::String(s.clone())),

            Expr::Boolean(b) => Ok(Value::Boolean(*b)),

            Expr::Null => Ok(Value::Null),

            Expr::Identifier(name) => self
                .env
                .borrow()
                .get(name)
                .ok_or_else(|| RuntimeError::UndefinedVariable(name.clone())),

            Expr::Binary { left, op, right } => {
                // Short-circuit evaluation for And and Or
                match op {
                    BinOp::And => {
                        let left_val = self.eval_expression(left)?;
                        if !left_val.is_truthy() {
                            // Short-circuit: left is falsy, return left without evaluating right
                            Ok(left_val)
                        } else {
                            // left is truthy, return right value
                            self.eval_expression(right)
                        }
                    }
                    BinOp::Or => {
                        let left_val = self.eval_expression(left)?;
                        if left_val.is_truthy() {
                            // Short-circuit: left is truthy, return left without evaluating right
                            Ok(left_val)
                        } else {
                            // left is falsy, return right value
                            self.eval_expression(right)
                        }
                    }
                    // For other operators, evaluate both sides
                    _ => {
                        let left_val = self.eval_expression(left)?;
                        let right_val = self.eval_expression(right)?;
                        self.eval_binary_op(&left_val, op, &right_val)
                    }
                }
            }

            Expr::Unary { op, expr } => {
                let val = self.eval_expression(expr)?;
                self.eval_unary_op(op, &val)
            }

            Expr::Call { func, args } => {
                let name_hint = match func.as_ref() {
                    Expr::Identifier(name) => Some(name.clone()),
                    _ => None,
                };
                let func_val = self.eval_expression(func)?;
                let arg_vals: Result<Vec<_>, _> =
                    args.iter().map(|arg| self.eval_expression(arg)).collect();
                let arg_vals = arg_vals?;

                self.call_function(name_hint.as_deref(), &func_val, arg_vals)
            }

            Expr::Array(elements) => {
                let vals: Result<Vec<_>, _> =
                    elements.iter().map(|e| self.eval_expression(e)).collect();
                Ok(Value::Array(vals?))
            }

            Expr::Dict(pairs) => {
                let mut map = std::collections::HashMap::new();
                for (key, value_expr) in pairs {
                    let value = self.eval_expression(value_expr)?;
                    map.insert(key.clone(), value);
                }
                Ok(Value::Dict(map))
            }

            Expr::Index { object, index } => {
                let obj_val = self.eval_expression(object)?;
                let idx_val = self.eval_expression(index)?;

                match (obj_val, idx_val) {
                    (Value::Array(arr), Value::Number(n)) => {
                        let idx = n as usize;
                        arr.get(idx).cloned().ok_or_else(|| {
                            RuntimeError::InvalidOperation(format!("Index {} out of bounds", idx))
                        })
                    }
                    (Value::String(s), Value::Number(n)) => {
                        let idx = n as usize;
                        let chars: Vec<char> = s.chars().collect();
                        chars
                            .get(idx)
                            .cloned()
                            .map(|ch| Value::String(ch.to_string()))
                            .ok_or_else(|| {
                                RuntimeError::InvalidOperation(format!(
                                    "Index {} out of bounds (string length: {})",
                                    idx,
                                    chars.len()
                                ))
                            })
                    }
                    (Value::Dict(dict), Value::String(key)) => {
                        dict.get(&key).cloned().ok_or_else(|| {
                            RuntimeError::InvalidOperation(format!("Key '{}' not found", key))
                        })
                    }
                    (obj, idx) => Err(RuntimeError::TypeError(format!(
                        "Cannot index {} with {}",
                        obj.type_name(),
                        idx.type_name()
                    ))),
                }
            }

            Expr::If {
                condition,
                then_branch,
                elif_branches,
                else_branch,
            } => {
                let cond = self.eval_expression(condition)?;

                if cond.is_truthy() {
                    let mut result = Value::Null;
                    for stmt in then_branch {
                        result = self.eval_statement(stmt)?;
                    }
                    return Ok(result);
                }

                for (elif_cond, elif_body) in elif_branches {
                    let cond = self.eval_expression(elif_cond)?;
                    if cond.is_truthy() {
                        let mut result = Value::Null;
                        for stmt in elif_body {
                            result = self.eval_statement(stmt)?;
                        }
                        return Ok(result);
                    }
                }

                if let Some(else_body) = else_branch {
                    let mut result = Value::Null;
                    for stmt in else_body {
                        result = self.eval_statement(stmt)?;
                    }
                    return Ok(result);
                }

                Ok(Value::Null)
            }

            Expr::Lambda { params, body } => {
                // Create a closure by capturing the current environment
                Ok(Value::Function {
                    name: None,
                    params: params.clone(),
                    body: body.clone(),
                    env: Rc::clone(&self.env),
                })
            }
        }
    }

    /// Evaluate binary operation
    fn eval_binary_op(&self, left: &Value, op: &BinOp, right: &Value) -> EvalResult {
        match op {
            BinOp::Add => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                (Value::Fraction(a), Value::Fraction(b)) => Ok(Value::Fraction(a + b)),
                (Value::Number(a), Value::Fraction(b)) | (Value::Fraction(b), Value::Number(a)) => {
                    use num_bigint::BigInt;
                    use num_rational::Ratio;
                    if a.fract() == 0.0 {
                        let a_frac = Ratio::new(BigInt::from(*a as i64), BigInt::from(1));
                        Ok(Value::Fraction(a_frac + b))
                    } else {
                        // 浮点数和分数混合运算，转换为浮点数
                        use num_traits::ToPrimitive;
                        let b_float =
                            b.numer().to_f64().unwrap_or(0.0) / b.denom().to_f64().unwrap_or(1.0);
                        Ok(Value::Number(a + b_float))
                    }
                }
                _ => Err(RuntimeError::TypeError(format!(
                    "Cannot add {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },

            BinOp::Subtract => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                (Value::Fraction(a), Value::Fraction(b)) => Ok(Value::Fraction(a - b)),
                (Value::Number(a), Value::Fraction(b)) => {
                    use num_bigint::BigInt;
                    use num_rational::Ratio;
                    if a.fract() == 0.0 {
                        let a_frac = Ratio::new(BigInt::from(*a as i64), BigInt::from(1));
                        Ok(Value::Fraction(a_frac - b))
                    } else {
                        use num_traits::ToPrimitive;
                        let b_float =
                            b.numer().to_f64().unwrap_or(0.0) / b.denom().to_f64().unwrap_or(1.0);
                        Ok(Value::Number(a - b_float))
                    }
                }
                (Value::Fraction(a), Value::Number(b)) => {
                    use num_bigint::BigInt;
                    use num_rational::Ratio;
                    if b.fract() == 0.0 {
                        let b_frac = Ratio::new(BigInt::from(*b as i64), BigInt::from(1));
                        Ok(Value::Fraction(a - b_frac))
                    } else {
                        use num_traits::ToPrimitive;
                        let a_float =
                            a.numer().to_f64().unwrap_or(0.0) / a.denom().to_f64().unwrap_or(1.0);
                        Ok(Value::Number(a_float - b))
                    }
                }
                _ => Err(RuntimeError::TypeError(format!(
                    "Cannot subtract {} from {}",
                    right.type_name(),
                    left.type_name()
                ))),
            },

            BinOp::Multiply => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    // 如果两个数都是整数，且足够大，使用精确计算
                    if a.fract() == 0.0 && b.fract() == 0.0 {
                        // 检查是否超过 f64 的安全整数范围 (2^53)
                        let max_safe = 9007199254740992.0; // 2^53
                        if a.abs() > max_safe || b.abs() > max_safe {
                            // 使用 Fraction (BigInt) 进行精确计算
                            use num_bigint::BigInt;
                            use num_rational::Ratio;

                            // 将 f64 转换为字符串再转为 BigInt，避免精度损失
                            let a_str = format!("{:.0}", a);
                            let b_str = format!("{:.0}", b);

                            if let (Ok(a_big), Ok(b_big)) =
                                (a_str.parse::<BigInt>(), b_str.parse::<BigInt>())
                            {
                                let result_big = a_big * b_big;
                                let frac = Ratio::new(result_big, BigInt::from(1));
                                return Ok(Value::Fraction(frac));
                            }
                        }
                    }
                    Ok(Value::Number(a * b))
                }
                (Value::Fraction(a), Value::Fraction(b)) => Ok(Value::Fraction(a * b)),
                (Value::Number(a), Value::Fraction(b)) | (Value::Fraction(b), Value::Number(a)) => {
                    use num_bigint::BigInt;
                    use num_rational::Ratio;
                    if a.fract() == 0.0 {
                        let a_frac = Ratio::new(BigInt::from(*a as i64), BigInt::from(1));
                        Ok(Value::Fraction(a_frac * b))
                    } else {
                        Err(RuntimeError::TypeError(
                            "Cannot multiply non-integer Number with Fraction".to_string(),
                        ))
                    }
                }
                _ => Err(RuntimeError::TypeError(format!(
                    "Cannot multiply {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },

            BinOp::Divide => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    if *b == 0.0 {
                        Err(RuntimeError::DivisionByZero)
                    } else {
                        Ok(Value::Number(a / b))
                    }
                }
                (Value::Fraction(a), Value::Fraction(b)) => {
                    use num_traits::Zero;
                    if b.is_zero() {
                        Err(RuntimeError::DivisionByZero)
                    } else {
                        Ok(Value::Fraction(a / b))
                    }
                }
                (Value::Number(a), Value::Fraction(b)) => {
                    use num_bigint::BigInt;
                    use num_rational::Ratio;
                    use num_traits::Zero;
                    if b.is_zero() {
                        Err(RuntimeError::DivisionByZero)
                    } else if a.fract() == 0.0 {
                        let a_frac = Ratio::new(BigInt::from(*a as i64), BigInt::from(1));
                        Ok(Value::Fraction(a_frac / b))
                    } else {
                        use num_traits::ToPrimitive;
                        let b_float =
                            b.numer().to_f64().unwrap_or(0.0) / b.denom().to_f64().unwrap_or(1.0);
                        Ok(Value::Number(a / b_float))
                    }
                }
                (Value::Fraction(a), Value::Number(b)) => {
                    use num_bigint::BigInt;
                    use num_rational::Ratio;
                    if *b == 0.0 {
                        Err(RuntimeError::DivisionByZero)
                    } else if b.fract() == 0.0 {
                        let b_frac = Ratio::new(BigInt::from(*b as i64), BigInt::from(1));
                        Ok(Value::Fraction(a / b_frac))
                    } else {
                        use num_traits::ToPrimitive;
                        let a_float =
                            a.numer().to_f64().unwrap_or(0.0) / a.denom().to_f64().unwrap_or(1.0);
                        Ok(Value::Number(a_float / b))
                    }
                }
                _ => Err(RuntimeError::TypeError(format!(
                    "Cannot divide {} by {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },

            BinOp::Modulo => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    if *b == 0.0 {
                        Err(RuntimeError::DivisionByZero)
                    } else {
                        Ok(Value::Number(a % b))
                    }
                }
                _ => Err(RuntimeError::TypeError(format!(
                    "Cannot modulo {} by {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },

            BinOp::Equal => Ok(Value::Boolean(left.equals(right))),

            BinOp::NotEqual => Ok(Value::Boolean(!left.equals(right))),

            BinOp::Less => match left.compare(right) {
                Some(ord) => Ok(Value::Boolean(ord == std::cmp::Ordering::Less)),
                None => Err(RuntimeError::TypeError(format!(
                    "Cannot compare {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },

            BinOp::LessEqual => match left.compare(right) {
                Some(ord) => Ok(Value::Boolean(ord != std::cmp::Ordering::Greater)),
                None => Err(RuntimeError::TypeError(format!(
                    "Cannot compare {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },

            BinOp::Greater => match left.compare(right) {
                Some(ord) => Ok(Value::Boolean(ord == std::cmp::Ordering::Greater)),
                None => Err(RuntimeError::TypeError(format!(
                    "Cannot compare {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },

            BinOp::GreaterEqual => match left.compare(right) {
                Some(ord) => Ok(Value::Boolean(ord != std::cmp::Ordering::Less)),
                None => Err(RuntimeError::TypeError(format!(
                    "Cannot compare {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },

            BinOp::And => {
                if !left.is_truthy() {
                    Ok(left.clone())
                } else {
                    Ok(right.clone())
                }
            }

            BinOp::Or => {
                if left.is_truthy() {
                    Ok(left.clone())
                } else {
                    Ok(right.clone())
                }
            }
        }
    }

    /// Evaluate unary operation
    fn eval_unary_op(&self, op: &UnaryOp, val: &Value) -> EvalResult {
        match op {
            UnaryOp::Minus => match val {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(RuntimeError::TypeError(format!(
                    "Cannot negate {}",
                    val.type_name()
                ))),
            },

            UnaryOp::Not => Ok(Value::Boolean(!val.is_truthy())),
        }
    }

    /// Call a function with arguments
    fn call_function(
        &mut self,
        name_hint: Option<&str>,
        func: &Value,
        args: Vec<Value>,
    ) -> EvalResult {
        // Check recursion depth limit
        self.enter_call()?;

        let frame = match func {
            Value::Function { name, params, .. } => {
                let display_name = name_hint
                    .map(|s| s.to_string())
                    .or_else(|| name.clone())
                    .unwrap_or_else(|| "<lambda>".to_string());
                let signature = format!("{}({})", display_name, params.join(", "));
                CallFrame {
                    name: display_name.clone(),
                    signature,
                }
            }
            Value::BuiltIn { name, .. } => {
                let arity = self.registry.get(name).map(|(_, a)| a).unwrap_or(0);
                let params = if arity == 0 {
                    String::new()
                } else {
                    (1..=arity)
                        .map(|i| format!("arg{}", i))
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                let signature = format!("{}({})", name, params);
                CallFrame {
                    name: name.clone(),
                    signature,
                }
            }
            other => {
                let name = name_hint.unwrap_or("<call>").to_string();
                let signature = format!("{}(<{}>)", name, other.type_name());
                CallFrame { name, signature }
            }
        };

        self.call_stack.push(frame);

        match func {
            Value::Function {
                params, body, env, ..
            } => {
                if params.len() != args.len() {
                    let err = RuntimeError::WrongArity {
                        expected: params.len(),
                        got: args.len(),
                    };
                    let err = self.attach_call_stack_if_absent(err);
                    let _ = self.call_stack.pop();
                    self.exit_call();
                    return Err(err);
                }

                // Create new environment for function execution
                let func_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(env))));

                // Bind parameters
                for (param, arg) in params.iter().zip(args.iter()) {
                    func_env.borrow_mut().set(param.clone(), arg.clone());
                }

                // Execute function body
                let prev_env = Rc::clone(&self.env);
                self.env = func_env;

                let mut result = Value::Null;
                for stmt in body {
                    match self.eval_statement(stmt) {
                        Ok(val) => result = val,
                        Err(RuntimeError::Return(val)) => {
                            result = val;
                            break;
                        }
                        Err(e) => {
                            self.env = prev_env;
                            let e = self.attach_call_stack_if_absent(e);
                            let _ = self.call_stack.pop();
                            self.exit_call();
                            return Err(e);
                        }
                    }
                }

                self.env = prev_env;
                let _ = self.call_stack.pop();
                self.exit_call();
                Ok(result)
            }

            Value::BuiltIn { name, .. } => {
                // Special handling for TRACE functions
                let res = match name.as_str() {
                    "TRACE" => {
                        if args.is_empty() {
                            return {
                                let err = RuntimeError::WrongArity {
                                    expected: 1,
                                    got: 0,
                                };
                                let err = self.attach_call_stack_if_absent(err);
                                let _ = self.call_stack.pop();
                                self.exit_call();
                                Err(err)
                            };
                        }

                        // Optional label: TRACE("label", x, y)
                        // If only one argument is provided, treat it as the payload (backward compatible).
                        let (label, payload_args) = if args.len() >= 2 {
                            match &args[0] {
                                Value::String(s) => (Some(s.as_str()), &args[1..]),
                                _ => (None, args.as_slice()),
                            }
                        } else {
                            (None, args.as_slice())
                        };

                        let payload = payload_args
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<_>>()
                            .join(" ");

                        let msg = match label {
                            Some(l) => format!("[{}] {}", l, payload),
                            None => payload,
                        };

                        self.trace_push(msg);
                        Ok(Value::Null)
                    }
                    "TRACE_DEBUG" | "TRACE_INFO" | "TRACE_WARN" | "TRACE_ERROR" => {
                        // Structured TRACE functions (Stage 3.2)
                        // Usage: TRACE_DEBUG("category", value1, value2, ...)
                        if args.len() < 2 {
                            return {
                                let err = RuntimeError::WrongArity {
                                    expected: 2,
                                    got: args.len(),
                                };
                                let err = self.attach_call_stack_if_absent(err);
                                let _ = self.call_stack.pop();
                                self.exit_call();
                                Err(err)
                            };
                        }

                        // Parse level from function name
                        let level = match name.as_str() {
                            "TRACE_DEBUG" => crate::runtime::TraceLevel::Debug,
                            "TRACE_INFO" => crate::runtime::TraceLevel::Info,
                            "TRACE_WARN" => crate::runtime::TraceLevel::Warn,
                            "TRACE_ERROR" => crate::runtime::TraceLevel::Error,
                            _ => unreachable!(),
                        };

                        // Parse category
                        let category = match &args[0] {
                            Value::String(s) => s.clone(),
                            _ => {
                                return {
                                    let err = RuntimeError::CustomError(format!(
                                        "TRACE category must be a string, got {}",
                                        args[0].type_name()
                                    ));
                                    let err = self.attach_call_stack_if_absent(err);
                                    let _ = self.call_stack.pop();
                                    self.exit_call();
                                    Err(err)
                                };
                            }
                        };

                        // Collect values (args[1..])
                        let values = args[1..].to_vec();

                        // Create and push structured entry
                        let entry = crate::runtime::TraceEntry::new(level, category, values);
                        self.trace_push_entry(entry);

                        Ok(Value::Null)
                    }
                    "MAP" => self.builtin_map(&args),
                    "FILTER" => self.builtin_filter(&args),
                    "REDUCE" => self.builtin_reduce(&args),
                    _ => {
                        // Get the built-in function from the registry
                        if let Some((func, _arity)) = self.registry.get(name) {
                            // Call the built-in function
                            func(&args)
                        } else {
                            Err(RuntimeError::NotCallable(format!(
                                "Built-in function '{}' not found",
                                name
                            )))
                        }
                    }
                };

                let _ = self.call_stack.pop();
                self.exit_call();
                match res {
                    Ok(v) => Ok(v),
                    Err(e) => Err(self.attach_call_stack_if_absent(e)),
                }
            }

            _ => {
                let err = RuntimeError::NotCallable(func.type_name().to_string());
                let err = self.attach_call_stack_if_absent(err);
                let _ = self.call_stack.pop();
                self.exit_call();
                Err(err)
            }
        }
    }

    // 实现 MAP 内置函数
    fn builtin_map(&mut self, args: &[Value]) -> EvalResult {
        if args.len() != 2 {
            return Err(RuntimeError::WrongArity {
                expected: 2,
                got: args.len(),
            });
        }

        let arr = match &args[0] {
            Value::Array(a) => a,
            other => {
                return Err(RuntimeError::TypeErrorDetailed {
                    expected: "Array".to_string(),
                    got: format!("{:?}", other),
                });
            }
        };

        let func = &args[1];

        let mut result = Vec::new();
        for item in arr {
            let mapped = self.call_function(None, func, vec![item.clone()])?;
            result.push(mapped);
        }

        Ok(Value::Array(result))
    }

    // 实现 FILTER 内置函数
    fn builtin_filter(&mut self, args: &[Value]) -> EvalResult {
        if args.len() != 2 {
            return Err(RuntimeError::WrongArity {
                expected: 2,
                got: args.len(),
            });
        }

        let arr = match &args[0] {
            Value::Array(a) => a,
            other => {
                return Err(RuntimeError::TypeErrorDetailed {
                    expected: "Array".to_string(),
                    got: format!("{:?}", other),
                });
            }
        };

        let predicate = &args[1];

        let mut result = Vec::new();
        for item in arr {
            let test_result = self.call_function(None, predicate, vec![item.clone()])?;
            if test_result.is_truthy() {
                result.push(item.clone());
            }
        }

        Ok(Value::Array(result))
    }

    // 实现 REDUCE 内置函数
    fn builtin_reduce(&mut self, args: &[Value]) -> EvalResult {
        if args.len() != 3 {
            return Err(RuntimeError::WrongArity {
                expected: 3,
                got: args.len(),
            });
        }

        let arr = match &args[0] {
            Value::Array(a) => a,
            other => {
                return Err(RuntimeError::TypeErrorDetailed {
                    expected: "Array".to_string(),
                    got: format!("{:?}", other),
                });
            }
        };

        let func = match &args[1] {
            Value::Function { .. } | Value::BuiltIn { .. } => &args[1],
            other => {
                return Err(RuntimeError::TypeErrorDetailed {
                    expected: "Function".to_string(),
                    got: format!("{:?}", other),
                });
            }
        };

        let mut accumulator = args[2].clone();

        for (idx, item) in arr.iter().enumerate() {
            let arg_count = match func {
                Value::Function { params, .. } => params.len(),
                Value::BuiltIn { arity, .. } => *arity,
                _ => 0,
            };

            let mut call_args = Vec::new();
            call_args.push(accumulator);
            call_args.push(item.clone());
            if arg_count >= 3 {
                call_args.push(Value::Number(idx as f64));
            }

            if arg_count < 2 {
                return Err(RuntimeError::WrongArity {
                    expected: 2,
                    got: arg_count,
                });
            }

            accumulator = self.call_function(None, func, call_args)?;
        }

        Ok(accumulator)
    }
}

impl Evaluator {
    fn import_chain(&self) -> Vec<String> {
        self.import_base_stack
            .iter()
            .map(|c| c.module_id.clone())
            .collect()
    }

    fn import_chain_with(&self, leaf: impl Into<String>) -> Vec<String> {
        let mut chain = self.import_chain();
        chain.push(leaf.into());
        chain
    }

    fn current_import_context(&self) -> Option<&ModuleContext> {
        self.import_base_stack.last()
    }

    fn eval_import(
        &mut self,
        names: &[String],
        specifier: &str,
        aliases: &[Option<String>],
        namespace: Option<&String>,
    ) -> EvalResult {
        let from_ctx = self.current_import_context();

        let chain_for_resolve = self.import_chain_with(specifier.to_string());

        let resolved = self
            .module_resolver
            .resolve(specifier, from_ctx)
            .map_err(|e| {
                RuntimeError::ImportError(Box::new(ImportError::from_resolve_error(
                    specifier,
                    e,
                    chain_for_resolve,
                )))
            })?;

        let exports = self.load_module(resolved)?;

        if let Some(ns) = namespace {
            self.env.borrow_mut().set(ns.clone(), Value::Dict(exports));
            return Ok(Value::Null);
        }

        for (i, name) in names.iter().enumerate() {
            let alias = aliases
                .get(i)
                .and_then(|a| a.clone())
                .unwrap_or_else(|| name.clone());
            let v = exports.get(name).cloned().ok_or_else(|| {
                RuntimeError::ImportError(Box::new(ImportError::not_exported(
                    specifier,
                    name,
                    self.import_chain_with(specifier.to_string()),
                )))
            })?;
            self.env.borrow_mut().set(alias, v);
        }

        Ok(Value::Null)
    }

    fn eval_export(&mut self, name: &str) -> EvalResult {
        let exports = self.export_stack.last_mut().ok_or_else(|| {
            RuntimeError::CustomError("Export error: Export used outside of a module".to_string())
        })?;

        let val = self.env.borrow().get(name).ok_or_else(|| {
            RuntimeError::CustomError(format!("Export error: '{}' is not defined", name))
        })?;

        exports.insert(name.to_string(), val);
        Ok(Value::Null)
    }

    fn load_module(
        &mut self,
        resolved: ResolvedModule,
    ) -> Result<HashMap<String, Value>, RuntimeError> {
        let import_chain = self.import_chain_with(resolved.module_id.clone());

        if let Some(cached) = self.module_cache.get(&resolved.module_id) {
            return Ok(cached.clone());
        }

        if self.module_stack.contains(&resolved.module_id) {
            let mut chain = self.module_stack.clone();
            chain.push(resolved.module_id.clone());
            return Err(RuntimeError::ImportError(Box::new(ImportError::circular(
                &resolved.module_id,
                chain,
                import_chain,
            ))));
        }

        self.module_stack.push(resolved.module_id.clone());

        // Parse module
        let mut parser = crate::parser::Parser::new(&resolved.source);
        let program = match parser.parse_program() {
            Ok(p) => p,
            Err(e) => {
                let _ = self.module_stack.pop();
                return Err(RuntimeError::ImportError(Box::new(
                    ImportError::parse_failed(&resolved.module_id, e.to_string(), import_chain),
                )));
            }
        };

        // Evaluate in an isolated environment with builtins registered.
        let prev_env = Rc::clone(&self.env);
        let module_env = Rc::new(RefCell::new(Environment::new()));
        Self::register_builtins_into_env(&self.registry, &mut module_env.borrow_mut());
        self.env = module_env;

        // Push module import base (for relative imports inside the module)
        self.import_base_stack.push(ModuleContext {
            module_id: resolved.module_id.clone(),
            base_dir: resolved.base_dir.clone(),
        });

        // Push export table
        self.export_stack.push(HashMap::new());

        let eval_res = self.eval_program(&program);

        // Pop stacks and restore env (must happen even on error)
        let exports = self.export_stack.pop().unwrap_or_default();
        self.import_base_stack.pop();
        self.env = prev_env;

        // Pop module stack
        let _ = self.module_stack.pop();

        // Propagate module evaluation error (cleanup already done)
        let _ = eval_res.map_err(|e| self.attach_call_stack_if_absent(e))?;

        self.module_cache
            .insert(resolved.module_id.clone(), exports.clone());
        Ok(exports)
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
