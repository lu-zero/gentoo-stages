use gentoo_core::Arch;
use std::path::{Path, PathBuf};

/// Information about a stage3 image
#[derive(Debug, Clone)]
pub struct Stage3 {
    pub name: String,         // e.g., "stage3-riscv64-openrc-20231018T010001Z.tar.xz"
    pub url: String,          // Full download URL
    pub size: u64,            // Size in bytes
    pub date: Option<String>, // Build datetime (extracted from filename)
    pub arch: Arch,           // Architecture
    pub variant: String,      // Variant (e.g., "rv64_lp64d-openrc", "rv32_ilp32d_musl")
    pub(crate) cache_dir: PathBuf, // Base cache directory (e.g., "/path/to/cache")
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
        self.cache_dir.join("stages").join(self.arch.as_keyword())
    }
}
