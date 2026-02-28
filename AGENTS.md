# Agents Configuration for Gentoo Stages Vibe

This file documents the agent configuration and usage patterns for the gentoo-stages-vibe crate.

## Agent Capabilities

The `gentoo-stages-vibe` crate provides the following agent capabilities:

### Stage3Fetcher Agent

**Purpose:** Fetch and manage Gentoo stage3 images

**Capabilities:**
- `list_available_flavors()` - List all available stage3 flavors for an architecture
- `fetch_latest()` - Fetch the latest stage3 image for a specific target
- `extract_stage3()` - Extract a downloaded stage3 image to a target directory

**Usage Pattern:**
```rust
use gentoo_stages::{Stage3Fetcher, Target};
use gentoo_core::Arch;

let target = Target {
    arch: Arch::Riscv64,
    flavor: "rv64_lp64d-openrc".to_string(),
};

let fetcher = Stage3Fetcher::new(target, "./cache", "https://distfiles.gentoo.org");
let stage3 = fetcher.fetch_latest()?;
```

## Integration Patterns

### With Cross-Compilation Tools

The agent can be integrated with cross-compilation toolchains:

```rust
// Fetch stage3 for cross-compilation target
let target = Target {
    arch: Arch::Riscv64,
    flavor: "rv64_lp64d-openrc".to_string(),
};

let fetcher = Stage3Fetcher::new(target, "./cache", "https://distfiles.gentoo.org");
let stage3 = fetcher.fetch_latest()?;

// Extract to cross-compilation root
fetcher.extract_stage3(&stage3, "/path/to/cross/root")?;
```

### With CI/CD Pipelines

Use in continuous integration for setting up build environments:

```rust
// In CI setup script
let fetcher = Stage3Fetcher::new(
    Target { arch: Arch::X86_64, flavor: "amd64-openrc".to_string() },
    "./cache",
    "https://distfiles.gentoo.org"
);

// Get latest stage3 for CI environment
let stage3 = fetcher.fetch_latest()?;
println!("Using stage3: {}", stage3.name);
```

## Error Handling

The agent provides comprehensive error handling:

```rust
use gentoo_stages_vibe::Stage3Error;

match fetcher.fetch_latest() {
    Ok(stage3) => println!("Success: {}", stage3.name),
    Err(Stage3Error::FetchError(e)) => eprintln!("Fetch failed: {}", e),
    Err(Stage3Error::DownloadError(e)) => eprintln!("Download failed: {}", e),
    Err(Stage3Error::VerifyError(e)) => eprintln!("Verification failed: {}", e),
    Err(Stage3Error::IoError(e)) => eprintln!("IO error: {}", e),
}
```

## Configuration Options

### Mirror Configuration

Configure different Gentoo mirrors:

```rust
// Use official Gentoo mirror
let fetcher = Stage3Fetcher::new(target, cache_dir, "https://distfiles.gentoo.org");

// Use alternative mirror
let fetcher = Stage3Fetcher::new(target, cache_dir, "https://mirror.example.com/gentoo");
```

### Cache Management

Control caching behavior:

```rust
// Disable caching by using temporary directory
let temp_cache = tempfile::tempdir()?;
let fetcher = Stage3Fetcher::new(target, temp_cache.path(), mirror_url);

// Use persistent cache
let fetcher = Stage3Fetcher::new(target, "/var/cache/gentoo-stages", mirror_url);
```

## Performance Considerations

### Caching
- Stage3 images are cached locally to avoid repeated downloads
- Cache location is configurable via the `cache_dir` parameter
- Cache files are verified before use

### Network Efficiency
- Uses `curl` for efficient HTTP downloads
- Supports resumable downloads via HTTP range requests
- Minimal metadata fetching for flavor listing

## Security

### Verification
- Download size verification
- File existence checks
- PGP signature support (via external tools)

### Best Practices
- Use HTTPS mirrors
- Verify downloaded files
- Use dedicated cache directories
- Clean up old cache files periodically

## Examples

See the `examples/` directory for complete working examples:
- `list.rs` - List available flavors
- `download.rs` - Download latest stage3

## Related Agents

- **gentoo-core** - Provides architecture types and utilities
- **crossdev-stages** - Higher-level cross-compilation management
- **gentoo-image** - Image building capabilities

## Troubleshooting

### Common Issues

**Network connectivity:**
- Ensure `curl` is available in PATH
- Check mirror URL accessibility
- Verify network connectivity

**Cache issues:**
- Check cache directory permissions
- Verify sufficient disk space
- Clear cache if corrupted

**Parsing errors:**
- Check Gentoo mirror format compatibility
- Verify mirror provides expected metadata files
- Update crate if mirror format changes

## Future Enhancements

Potential future agent capabilities:
- PGP signature verification
- Parallel downloads
- Progress reporting
- Mirror failover
- Cache expiration policies
