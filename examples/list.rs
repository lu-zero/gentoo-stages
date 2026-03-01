use gentoo_core::Arch;
use gentoo_stages::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let arch = std::env::args()
        .nth(1)
        .map_or_else(Arch::current, |a| a.parse::<Arch>())?;

    // Create client for the specified architecture with persistent cache
    let client = Client::builder().arch(arch).cache_dir("./cache").build()?;

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
