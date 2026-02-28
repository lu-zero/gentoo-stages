use gentoo_core::Arch;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Cache configuration for stage3 images
#[derive(Debug)]
pub enum Cache {
    /// Temporary cache that will be automatically cleaned up
    Temp(TempDir),
    /// Persistent cache at a specific path
    Path(PathBuf),
}

impl Cache {
    /// Get the cache directory path
    pub fn path(&self) -> &Path {
        match self {
            Cache::Temp(temp_dir) => temp_dir.path(),
            Cache::Path(path) => path,
        }
    }
}

/// Information about a stage3 image
#[derive(Debug, Clone)]
pub struct Stage3 {
    pub name: String,         // e.g., "stage3-riscv64-openrc-20231018T010001Z.tar.xz"
    pub url: String,          // Full download URL
    pub size: u64,            // Size in bytes
    pub date: Option<String>, // Build datetime (extracted from filename)
    pub arch: Arch,           // Architecture
    pub variant: String,      // Variant (e.g., "rv64_lp64d-openrc", "rv32_ilp32d_musl")
    cached: bool,             // Whether this image is already cached
}

impl Stage3 {
    /// Create a new Stage3 instance
    pub fn new(
        name: String,
        url: String,
        size: u64,
        date: Option<&str>,
        arch: Arch,
        variant: String,
    ) -> Self {
        Self {
            name,
            url,
            size,
            date: date.map(|s| s.to_string()),
            arch,
            variant,
            cached: false,
        }
    }

    /// Check if this stage3 image is cached
    pub fn is_cached(&self) -> bool {
        self.cached
    }

    /// Get the cache path for this stage3 image
    pub fn cache_path(&self, cache_dir: impl AsRef<Path>) -> PathBuf {
        cache_dir.as_ref().join(&self.name)
    }

    /// Set the cached status
    pub(crate) fn set_cached(&mut self, cached: bool) {
        self.cached = cached;
    }
}
