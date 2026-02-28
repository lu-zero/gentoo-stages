use gentoo_core::Arch;

/// Information about a stage3 image
#[derive(Debug, Clone)]
pub struct Stage3Info {
    pub name: String,   // e.g., "stage3-riscv64-openrc-20231018T010001Z.tar.xz"
    pub url: String,    // Full download URL
    pub size: u64,      // Size in bytes
    pub date: String,   // Build date (extracted from filename)
    pub arch: Arch,     // Architecture
    pub flavor: String, // Flavor (e.g., "rv64_lp64d-openrc")
}

/// Target configuration for stage3 operations
#[derive(Debug, Clone)]
pub struct Target {
    pub arch: Arch,
    pub flavor: String,
}
