use gentoo_core::Arch;
use gentoo_stages::Client;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Parse architecture from command line argument
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <architecture>", args[0]);
        eprintln!(
            "Supported architectures: arm, aarch64, x86, amd64, riscv32, riscv64, ppc, ppc64"
        );
        std::process::exit(1);
    }

    let arch_str = &args[1];
    let arch = match arch_str.parse::<Arch>() {
        Ok(a) => a,
        Err(_) => {
            eprintln!("Error: Unknown architecture '{}'", arch_str);
            eprintln!(
                "Supported architectures: arm, aarch64, x86, amd64, riscv32, riscv64, ppc, ppc64"
            );
            std::process::exit(1);
        }
    };

    // Create client for the specified architecture
    let client = Client::with_arch(arch)?;

    println!("Fetching available stage3 images for {}...", arch);
    let stage3_list = client.list()?;

    println!("Available stage3 images for {}:", arch);
    for stage3 in stage3_list {
        let cached_status = if stage3.is_cached() { "[cached]" } else { "" };
        let date_display = stage3.date.as_deref().unwrap_or("unknown");
        println!(
            "- {} {} ({} bytes, {})",
            stage3.variant, cached_status, stage3.size, date_display
        );
    }

    Ok(())
}
