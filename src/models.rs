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

// Implement Into<Cache> for various path types to enable convenient builder usage
impl<T> From<T> for Cache
where
    T: Into<PathBuf>,
{
    fn from(path: T) -> Self {
        Cache::Path(path.into())
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
    cache_dir: PathBuf,       // Cache directory where this image is stored
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
        self.cache_dir.join(&self.name).exists()
    }

    /// Get the cache path for this stage3 image
    pub fn cache_path(&self) -> &Path {
        &self.cache_dir
    }

    /// Get the full path to the cached stage3 file
    pub fn file_path(&self) -> PathBuf {
        self.cache_dir.join(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_cache_path_method() {
        let temp_cache = Cache::Temp(tempfile::tempdir().unwrap());
        assert!(temp_cache.path().exists());

        let path_cache = Cache::Path(PathBuf::from("./test_cache"));
        assert_eq!(path_cache.path(), Path::new("./test_cache"));
    }

    #[test]
    fn test_into_cache_conversions() {
        // Test PathBuf -> Cache
        let path_buf: PathBuf = PathBuf::from("./test_cache");
        let cache_from_pathbuf: Cache = path_buf.into();
        match cache_from_pathbuf {
            Cache::Path(p) => assert_eq!(p, PathBuf::from("./test_cache")),
            Cache::Temp(_) => panic!("Expected Path variant"),
        }

        // Test &str -> Cache
        let cache_from_str: Cache = "./test_cache".into();
        match cache_from_str {
            Cache::Path(p) => assert_eq!(p, PathBuf::from("./test_cache")),
            Cache::Temp(_) => panic!("Expected Path variant"),
        }

        // Test String -> Cache
        let string_path = "./test_cache".to_string();
        let cache_from_string: Cache = string_path.into();
        match cache_from_string {
            Cache::Path(p) => assert_eq!(p, PathBuf::from("./test_cache")),
            Cache::Temp(_) => panic!("Expected Path variant"),
        }

        // Test &Path -> Cache
        let path = Path::new("./test_cache");
        let cache_from_path: Cache = path.into();
        match cache_from_path {
            Cache::Path(p) => assert_eq!(p, PathBuf::from("./test_cache")),
            Cache::Temp(_) => panic!("Expected Path variant"),
        }
    }
}
