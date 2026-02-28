use gentoo_stages::{Stage3Fetcher, Target};
use gentoo_core::Arch;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Example: Download latest stage3 for riscv64
    let target = Target {
        arch: Arch::Riscv64,
        flavor: "rv64_lp64d-openrc".to_string(),
    };

    let cache_dir = PathBuf::from("./cache");
    let mirror_url = "https://distfiles.gentoo.org";

    let fetcher = Stage3Fetcher::new(target, cache_dir, mirror_url);

    println!("Fetching latest stage3 image...");
    let stage3 = fetcher.fetch_latest()?;

    println!("Downloaded stage3 image:");
    println!("  Name: {}", stage3.name);
    println!("  URL: {}", stage3.url);
    println!("  Size: {} bytes", stage3.size);
    println!("  Date: {}", stage3.date);
    println!("  Arch: {}", stage3.arch);
    println!("  Flavor: {}", stage3.flavor);

    Ok(())
}
