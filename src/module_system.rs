use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ModuleContext {
    pub module_id: String,
    pub base_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct ResolvedModule {
    pub module_id: String,
    pub source: String,
    pub base_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum ModuleResolveError {
    ImportDisabled,
    InvalidSpecifier(String),
    NoBaseDir(String),
    NotFound(String),
    AccessDenied(String),
    IoError(String),
}

impl std::fmt::Display for ModuleResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleResolveError::ImportDisabled => write!(f, "Import is disabled"),
            ModuleResolveError::InvalidSpecifier(s) => write!(f, "Invalid module specifier: {s}"),
            ModuleResolveError::NoBaseDir(s) => {
                write!(f, "No base directory to resolve specifier: {s}")
            }
            ModuleResolveError::NotFound(s) => write!(f, "Module not found: {s}"),
            ModuleResolveError::AccessDenied(s) => write!(f, "Module access denied: {s}"),
            ModuleResolveError::IoError(s) => write!(f, "Module IO error: {s}"),
        }
    }
}

impl std::error::Error for ModuleResolveError {}

pub trait ModuleResolver {
    fn resolve(
        &self,
        specifier: &str,
        from: Option<&ModuleContext>,
    ) -> Result<ResolvedModule, ModuleResolveError>;
}

#[derive(Default, Debug, Clone)]
pub struct DisabledModuleResolver;

impl ModuleResolver for DisabledModuleResolver {
    fn resolve(
        &self,
        _specifier: &str,
        _from: Option<&ModuleContext>,
    ) -> Result<ResolvedModule, ModuleResolveError> {
        Err(ModuleResolveError::ImportDisabled)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FileSystemModuleResolver {
    /// Optional root directory; when set, resolved paths must be under this root.
    pub root_dir: Option<PathBuf>,
    /// Whether to allow absolute paths.
    pub allow_absolute: bool,
}

impl FileSystemModuleResolver {
    fn normalize_specifier(specifier: &str) -> Result<&str, ModuleResolveError> {
        if specifier.contains("://") {
            return Err(ModuleResolveError::InvalidSpecifier(specifier.to_string()));
        }
        Ok(specifier)
    }

    fn with_aether_extension(path: &Path) -> PathBuf {
        if path.extension().is_some() {
            path.to_path_buf()
        } else {
            let mut p = path.to_path_buf();
            p.set_extension("aether");
            p
        }
    }

    fn ensure_under_root(&self, path: &Path) -> Result<(), ModuleResolveError> {
        if let Some(root) = &self.root_dir {
            let root = match root.canonicalize() {
                Ok(p) => p,
                Err(e) => return Err(ModuleResolveError::IoError(e.to_string())),
            };
            let canon = match path.canonicalize() {
                Ok(p) => p,
                Err(e) => return Err(ModuleResolveError::IoError(e.to_string())),
            };
            if !canon.starts_with(&root) {
                return Err(ModuleResolveError::AccessDenied(
                    canon.display().to_string(),
                ));
            }
        }
        Ok(())
    }

    fn resolve_path(
        &self,
        specifier: &str,
        from: Option<&ModuleContext>,
    ) -> Result<PathBuf, ModuleResolveError> {
        let specifier = Self::normalize_specifier(specifier)?;

        let raw = PathBuf::from(specifier);

        if raw.is_absolute() {
            if !self.allow_absolute {
                return Err(ModuleResolveError::AccessDenied(specifier.to_string()));
            }
            return Ok(Self::with_aether_extension(&raw));
        }

        let base_dir = from
            .and_then(|c| c.base_dir.clone())
            .ok_or_else(|| ModuleResolveError::NoBaseDir(specifier.to_string()))?;

        Ok(Self::with_aether_extension(&base_dir.join(raw)))
    }
}

impl ModuleResolver for FileSystemModuleResolver {
    fn resolve(
        &self,
        specifier: &str,
        from: Option<&ModuleContext>,
    ) -> Result<ResolvedModule, ModuleResolveError> {
        let path = self.resolve_path(specifier, from)?;

        if !path.exists() {
            return Err(ModuleResolveError::NotFound(path.display().to_string()));
        }

        self.ensure_under_root(&path)?;

        let canon = path
            .canonicalize()
            .map_err(|e| ModuleResolveError::IoError(e.to_string()))?;

        let source = std::fs::read_to_string(&canon)
            .map_err(|e| ModuleResolveError::IoError(e.to_string()))?;

        let base_dir = canon.parent().map(|p| p.to_path_buf());

        Ok(ResolvedModule {
            module_id: canon.display().to_string(),
            source,
            base_dir,
        })
    }
}
