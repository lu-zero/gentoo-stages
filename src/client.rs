use crate::{
    error::Error,
    models::{Cache, Stage3},
};
use gentoo_core::Arch;
use log::info;
use std::io::copy;
use std::path::Path;

/// Client for interacting with Gentoo distfiles mirrors
pub struct Client {
    mirror_url: String,
    arch: Arch,
    cache_dir: Cache,
}

impl Client {
    /// Create a new Client with default settings
    ///
    /// Uses distfiles.gentoo.org mirror, host architecture,
    /// and a temporary cache directory.
    pub fn new() -> Result<Self, Error> {
        let cache_dir = Cache::Temp(tempfile::tempdir()?);

        Ok(Self {
            mirror_url: "https://distfiles.gentoo.org".to_string(),
            arch: Arch::current()?,
            cache_dir,
        })
    }

    /// Create a new Client with specified architecture
    pub fn with_arch(arch: Arch) -> Result<Self, Error> {
        let mut client = Self::new()?;
        client.arch = arch;
        Ok(client)
    }

    /// Create a new Client with specified mirror URL
    pub fn with_mirror(mirror_url: &str) -> Result<Self, Error> {
        let mut client = Self::new()?;
        client.mirror_url = mirror_url.to_string();
        Ok(client)
    }

    /// Create a new Client with specified cache directory
    pub fn with_cache(cache_dir: impl AsRef<Path>) -> Result<Self, Error> {
        let mut client = Self::new()?;
        client.cache_dir = Cache::Path(cache_dir.as_ref().to_path_buf());
        Ok(client)
    }

    /// List all available stage3 images for the configured architecture
    pub fn list(&self) -> Result<Vec<Stage3>, Error> {
        let stage3_list = self.fetch_all_stage3_flavors()?;

        // Check cache status for each image
        let mut result = Vec::new();
        for stage3 in stage3_list {
            let _cache_path = stage3.cache_path();

            result.push(stage3);
        }

        // Sort by date (newest first)
        result.sort_by(|a, b| {
            let a_ts = extract_timestamp(&a.name);
            let b_ts = extract_timestamp(&b.name);
            b_ts.cmp(&a_ts) // Descending order
        });

        Ok(result)
    }

    /// Get a specific stage3 variant (downloads if not cached)
    pub fn get(&self, variant: &str) -> Result<Stage3, Error> {
        let stage3 = self
            .list()?
            .into_iter()
            .find(|s| s.variant == variant)
            .ok_or_else(|| Error::VariantNotFound(variant.to_string()))?;

        if !stage3.is_cached() {
            self.download_stage3(&stage3)?;
        }

        Ok(stage3)
    }

    /// Fetch the list of all available stage3 images for the architecture
    fn fetch_all_stage3_flavors(&self) -> Result<Vec<Stage3>, Error> {
        let latest_url = format!(
            "{}/releases/{}/autobuilds/latest-stage3.txt",
            self.mirror_url.trim_end_matches('/'),
            self.arch.as_keyword()
        );

        info!("Fetching all stage3 variants from: {}", latest_url);

        // Use reqwest instead of curl
        let content = reqwest::blocking::get(&latest_url)?.text()?;
        self.parse_all_flavors_list(&content)
    }

    /// Parse stage3 list content into Stage3 structures (for all flavors)
    fn parse_all_flavors_list(&self, content: &str) -> Result<Vec<Stage3>, Error> {
        let mut stage3_images = Vec::new();
        let mut in_pgp_section = false;

        for line in content.lines() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') || line.starts_with("Hash:") {
                continue;
            }

            if line == "-----BEGIN PGP SIGNED MESSAGE-----" {
                continue;
            }

            if line == "-----BEGIN PGP SIGNATURE-----" {
                in_pgp_section = true;
                continue;
            }

            if line == "-----END PGP SIGNATURE-----" {
                in_pgp_section = false;
                continue;
            }

            if in_pgp_section {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let full_path = parts[0].to_string();
                let size = parts[1].parse::<u64>().map_err(|e| {
                    Error::ParseError(format!("Failed to parse size for {}: {}", full_path, e))
                })?;

                let name = full_path
                    .split('/')
                    .next_back()
                    .unwrap_or(&full_path)
                    .to_string();

                // Handle both standard stage3-*.tar.xz and alternative musl naming formats
                if name.starts_with("stage3-") {
                    let date = extract_date_from_filename(&name);
                    let variant = extract_variant_from_filename(&name);

                    stage3_images.push(Stage3::new(
                        name.clone(),
                        format!(
                            "{}/releases/{}/autobuilds/{}",
                            self.mirror_url.trim_end_matches('/'),
                            self.arch.as_keyword(),
                            full_path
                        ),
                        size,
                        date,
                        self.arch,
                        variant,
                        self.cache_dir.path(),
                    ));
                }
            }
        }

        if stage3_images.is_empty() {
            return Err(Error::ParseError(format!(
                "No stage3 images found for arch={}",
                self.arch
            )));
        }

        Ok(stage3_images)
    }

    /// Download a stage3 image
    fn download_stage3(&self, stage3: &Stage3) -> Result<(), Error> {
        std::fs::create_dir_all(self.cache_dir.path())?;

        let cache_path = stage3.cache_path();

        info!("Downloading stage3 image: {}", stage3.name);
        info!("URL: {}", stage3.url);
        info!("Size: {} bytes", stage3.size);

        // Use reqwest instead of curl for downloading
        let response = reqwest::blocking::get(&stage3.url)?;
        let bytes = response.bytes()?;

        // Write to file
        let mut file = std::fs::File::create(cache_path)?;
        copy(&mut bytes.as_ref(), &mut file)?;

        info!("Downloaded stage3 image to: {}", cache_path.display());

        Ok(())
    }
}

/// Extract timestamp from stage3 filename
fn extract_timestamp(filename: &str) -> u64 {
    let parts: Vec<&str> = filename.split('-').collect();
    if parts.len() >= 4 {
        let last_part = parts[parts.len() - 1];
        let timestamp_part = last_part
            .replace(".tar.xz", "")
            .replace("T", "")
            .replace("Z", "");

        if let Ok(ts) = timestamp_part.parse::<u64>() {
            return ts;
        }
    }
    0
}

/// Extract variant from stage3 filename
/// The variant is everything between "stage3-" and the final "-{timestamp}.tar.xz"
fn extract_variant_from_filename(filename: &str) -> String {
    // Remove the .tar.xz extension
    let without_ext = filename.strip_suffix(".tar.xz").unwrap_or(filename);

    // Remove the "stage3-" prefix
    let without_prefix = without_ext.strip_prefix("stage3-").unwrap_or(without_ext);

    // Find the last hyphen that separates variant from timestamp
    // We look for the pattern -YYYYMMDDTHHMMSSZ
    if let Some(last_hyphen_pos) = without_prefix.rfind('-') {
        // Check if the part after the last hyphen looks like a timestamp
        let potential_timestamp = &without_prefix[last_hyphen_pos + 1..];
        if potential_timestamp.contains('T') && potential_timestamp.ends_with('Z') {
            // This is a timestamp, so everything before it is the variant
            return without_prefix[..last_hyphen_pos].to_string();
        }
    }

    // Fallback: remove stage3- prefix if present
    without_prefix.to_string()
}

/// Extract date from stage3 filename
/// Returns the full datetime string (e.g., "20260216T163057Z")
/// or None if no valid timestamp can be extracted
fn extract_date_from_filename(filename: &str) -> Option<&str> {
    // Split from the right to handle complex arch names with hyphens
    let mut parts = filename.rsplit('-');

    // Get the last part (should be the timestamp)
    let last_part = parts.next()?;

    // Remove .tar.xz extension if present
    let timestamp_part = last_part.strip_suffix(".tar.xz").unwrap_or(last_part);

    // Check if it looks like a valid timestamp (contains T and ends with Z)
    if timestamp_part.contains('T') && timestamp_part.ends_with('Z') {
        Some(timestamp_part)
    } else {
        None
    }
}
