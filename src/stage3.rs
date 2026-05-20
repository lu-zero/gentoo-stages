use gentoo_core::Arch;
use std::path::{Path, PathBuf};

/// Information about a stage3 image
#[derive(Debug, Clone)]
pub struct Stage3 {
    pub name: String,
    pub url: String,
    pub size: u64,
    pub date: Option<String>,
    pub arch: Arch,
    pub variant: String,
    pub(crate) cache_dir: PathBuf,
}

impl Stage3 {
    /// Create a new Stage3 instance
    pub(crate) fn new(
        name: String,
        url: String,
        size: u64,
        date: Option<&str>,
        arch: Arch,
        variant: String,
        cache_dir: impl AsRef<Path>,
    ) -> Self {
        Self {
            name,
            url,
            size,
            date: date.map(|s| s.to_string()),
            arch,
            variant,
            cache_dir: cache_dir.as_ref().to_path_buf(),
        }
    }

    /// Check if this stage3 image is cached
    pub fn is_cached(&self) -> bool {
        self.file_path().exists()
    }

    /// Get the full path to the cached stage3 file
    pub fn file_path(&self) -> PathBuf {
        self.arch_cache_dir().join(&self.name)
    }

    /// Get the architecture-specific cache directory
    pub(crate) fn arch_cache_dir(&self) -> PathBuf {
        self.cache_dir.join("stages").join(self.arch.as_str())
    }
}
