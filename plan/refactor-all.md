# API Refactoring Plan

## Goal
Redesign the gentoo-stages API to be more intuitive and follow Rust best practices.

## Current Issues
- `Stage3Fetcher` requires unnecessary parameters for simple operations
- Method names are verbose (`list_available_flavors`)
- Target/flavor coupling is inflexible
- Cache handling is not transparent

## New Design

### 1. Client Struct
```rust
pub struct Client {
    mirror_url: String,      // Default: "https://distfiles.gentoo.org"
    arch: Arch,              // Default: current host architecture
    cache_dir: Cache,        // Default: Temp(tempfile::TempDir::new()?)
}
```

### 2. Cache Enum
```rust
pub enum Cache {
    Temp(tempfile::TempDir),  // Default: temporary directory
    Path(PathBuf),           // User-specified path
}
```

### 3. Stage3 Struct (renamed from Stage3Info)
```rust
pub struct Stage3 {
    pub name: String,
    pub url: String,
    pub size: u64,
    pub date: String,
    pub arch: Arch,
    pub flavor: String,
    cached: bool,            // Private field
}

impl Stage3 {
    pub fn fetch(&mut self, client: &Client) -> Result<(), Stage3Error>;
    pub fn extract(&self, target_dir: impl AsRef<Path>) -> Result<(), Stage3Error>;
    pub fn is_cached(&self) -> bool;
    pub fn cache_path(&self, cache_dir: impl AsRef<Path>) -> PathBuf;
}
```

### 4. Client Methods
```rust
impl Client {
    // Constructors
    pub fn new() -> Result<Self, Stage3Error>;  // All defaults
    pub fn with_arch(arch: Arch) -> Result<Self, Stage3Error>;
    pub fn with_mirror(mirror_url: &str) -> Result<Self, Stage3Error>;
    pub fn with_cache(cache_dir: impl AsRef<Path>) -> Result<Self, Stage3Error>;

    // Main operations
    pub fn list(&self) -> Result<Vec<Stage3>, Stage3Error>;
    pub fn get(&self, flavor: &str) -> Result<Stage3, Stage3Error>;
}
```

## Implementation Steps

### Phase 1: Setup
1. Add `tempfile` dependency to Cargo.toml
2. Create backup of current files

### Phase 2: Core Types
3. Update `models.rs`:
   - Rename `Stage3Info` to `Stage3`
   - Add `cached` field
   - Add method implementations
   - Add `Cache` enum

### Phase 3: Client Implementation
4. Create new `client.rs` module:
   - Implement `Client` struct
   - Implement constructor methods
   - Implement `list()` method
   - Implement `get()` method

### Phase 4: Refactor Existing Code
5. Update `stage3.rs`:
   - Move relevant logic to new types
   - Keep internal helper functions
   - Update to work with new API

6. Update `lib.rs`:
   - Export new types
   - Maintain backward compatibility if needed

### Phase 5: Examples
7. Update `examples/list.rs`:
   - Use new `Client::list()` API
   - Show cache status

8. Update `examples/download.rs`:
   - Use new `Client::get()` API
   - Demonstrate extraction

### Phase 6: Testing
9. Run `cargo test`
10. Run `cargo clippy -- -D warnings`
11. Run `cargo fmt --check`
12. Test examples manually

### Phase 7: Documentation
13. Update README.md
14. Add doc comments to all public items
15. Add usage examples in documentation

## Backward Compatibility
Consider keeping `Stage3Fetcher` as a deprecated wrapper around new `Client` API for existing users.

## Benefits
- Simpler API surface
- Better separation of concerns
- Transparent caching
- More intuitive method names
- Better Rust idioms
