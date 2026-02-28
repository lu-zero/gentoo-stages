use gentoo_core::Arch;
use gentoo_stages::Client;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Example: Download latest stage3 for riscv64
    let client = Client::with_arch(Arch::Riscv64)?;

    println!("Fetching latest stage3 image for riscv64...");
    let stage3 = client.get("rv64_lp64d-openrc")?;

    println!("Stage3 image information:");
    println!("  Name: {}", stage3.name);
    println!("  URL: {}", stage3.url);
    println!("  Size: {} bytes", stage3.size);
    println!("  Date: {}", stage3.date.as_deref().unwrap_or("unknown"));
    println!("  Arch: {}", stage3.arch);
    println!("  Variant: {}", stage3.variant);
    println!("  Cached: {}", stage3.is_cached());

    // Example: Extract to a target directory
    let target_dir = PathBuf::from("./extracted_stage3");
    println!("\nExtracting to {}...", target_dir.display());
    client.extract(&stage3, target_dir)?;
    println!("Extraction complete!");

    Ok(())
}
