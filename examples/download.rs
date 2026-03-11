use gentoo_core::{Arch, KnownArch};
use gentoo_stages::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client = Client::builder()
        .arch(Arch::Known(KnownArch::Riscv64))
        .cache_dir("./cache")
        .build()?;

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

    // The stage3 image is now downloaded and cached at:
    println!("\nStage3 image cached at: {}", stage3.file_path().display());
    println!(
        "You can extract it manually using: tar -xJpf {}",
        stage3.file_path().display()
    );

    Ok(())
}
