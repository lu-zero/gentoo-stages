# Gentoo Stages

[![Crates.io](https://img.shields.io/crates/v/gentoo-stages.svg)](https://crates.io/crates/gentoo-stages)
[![Docs.rs](https://docs.rs/gentoo-stages/badge.svg)](https://docs.rs/gentoo-stages)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A Rust crate for fetching and managing Gentoo Linux stage3 images.

## Overview

`gentoo-stages-vibe` provides functionality for working with Gentoo Linux stage3 tarballs, including:

- Listing available stage3 flavors for architectures
- Fetching latest stage3 images from Gentoo mirrors
- Downloading and caching stage3 images
- Extracting stage3 images to target directories

## Features

- **Standalone crate** following gentoo-core patterns
- **No external dependencies** beyond core Rust ecosystem
- **Clean API** designed for easy integration
- **Comprehensive error handling** with thiserror
- **Logging support** via log crate

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
gentoo-stages = "0.1.0"
gentoo-core = "0.1.0"
```

### Example: List Available Flavors

```rust
use gentoo_stages::{Stage3Fetcher, Target};
use gentoo_core::Arch;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = Target {
        arch: Arch::Riscv64,
        flavor: "rv64_lp64d-openrc".to_string(),
    };

    let cache_dir = PathBuf::from("./cache");
    let mirror_url = "https://distfiles.gentoo.org";

    let fetcher = Stage3Fetcher::new(target, cache_dir, mirror_url);
    let flavors = fetcher.list_available_flavors()?;

    println!("Available flavors:");
    for flavor in flavors {
        println!("- {}", flavor);
    }

    Ok(())
}
```

### Example: Download Latest Stage3

```rust
use gentoo_stages::{Stage3Fetcher, Target};
use gentoo_core::Arch;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = Target {
        arch: Arch::Riscv64,
        flavor: "rv64_lp64d-openrc".to_string(),
    };

    let cache_dir = PathBuf::from("./cache");
    let mirror_url = "https://distfiles.gentoo.org";

    let fetcher = Stage3Fetcher::new(target, cache_dir, mirror_url);
    let stage3 = fetcher.fetch_latest()?;

    println!("Downloaded: {}", stage3.name);
    println!("Size: {} bytes", stage3.size);

    Ok(())
}
```

## Examples

The crate includes working examples:

- `list.rs` - List available stage3 flavors for a given architecture
- `download.rs` - Download latest stage3 image

Run examples with:

```bash
# List flavors for riscv64
cargo run --example list_stages -- riscv64

# List flavors for amd64
cargo run --example list_stages -- amd64

# Download latest stage3
cargo run --example download_stage
```

## Architecture Support

Supports all Gentoo architectures via `gentoo-core::Arch`:

- ARM (arm, arm64)
- x86 (x86, amd64)
- RISC-V (riscv32, riscv64)
- PowerPC (ppc, ppc64)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Related Projects

- [gentoo-core](https://github.com/lu-zero/gentoo-core) - Core Gentoo types and utilities
- [crossdev-stages-rust](https://github.com/lu-zero/crossdev-stages-rust) - Cross-compilation tooling

## Acknowledgments

- Inspired by the original crossdev-stage3 functionality
- Built on the foundation of gentoo-core
- Uses Gentoo's official distribution infrastructure
