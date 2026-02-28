use gentoo_core::Arch;
use gentoo_stages::{Stage3Fetcher, Target};
use std::env;
use std::path::PathBuf;

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

    // Use a default flavor for the architecture
    let default_flavor = match arch {
        Arch::Arm => "armv7a-openrc",
        Arch::AArch64 => "arm64-openrc",
        Arch::X86 => "x86-openrc",
        Arch::X86_64 => "amd64-openrc",
        Arch::Riscv32 => "rv32_ilp32d-openrc",
        Arch::Riscv64 => "rv64_lp64d-openrc",
        Arch::Powerpc => "ppc-openrc",
        Arch::Powerpc64 => "ppc64-openrc",
    };

    let target = Target {
        arch,
        flavor: default_flavor.to_string(),
    };

    let cache_dir = PathBuf::from("./cache");
    let mirror_url = "https://distfiles.gentoo.org";

    let fetcher = Stage3Fetcher::new(target, cache_dir, mirror_url);

    println!("Fetching available stage3 flavors for {}...", arch);
    let flavors = fetcher.list_available_flavors()?;

    println!("Available flavors for {}:", arch);
    for flavor in flavors {
        println!("- {}", flavor);
    }

    Ok(())
}
