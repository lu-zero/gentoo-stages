use crate::{error::Stage3Error, models::{Stage3Info, Target}};
use log::info;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Stage3 image fetcher and manager
pub struct Stage3Fetcher {
    target: Target,
    cache_dir: PathBuf,
    mirror_url: String,
}

impl Stage3Fetcher {
    /// Create a new Stage3Fetcher
    pub fn new(target: Target, cache_dir: impl AsRef<Path>, mirror_url: &str) -> Self {
        Self {
            target,
            cache_dir: cache_dir.as_ref().to_path_buf(),
            mirror_url: mirror_url.to_string(),
        }
    }

    /// List available stage3 flavors for the configured architecture
    pub fn list_available_flavors(&self) -> Result<Vec<String>, Stage3Error> {
        let stage3_list = self.fetch_all_stage3_flavors()?;
        Ok(self.list_available_flavors_from_list(&stage3_list))
    }

    /// Helper method to extract unique flavors from a stage3 list
    fn list_available_flavors_from_list(&self, stage3_list: &[Stage3Info]) -> Vec<String> {
        let mut flavors = Vec::new();

        for stage3 in stage3_list {
            if !flavors.contains(&stage3.flavor) {
                flavors.push(stage3.flavor.clone());
            }
        }

        flavors.sort();
        flavors
    }

    /// Fetch the latest stage3 image for the configured architecture
    pub fn fetch_latest(&self) -> Result<Stage3Info, Stage3Error> {
        let stage3_list = self.fetch_stage3_list()?;
        let latest = self.find_latest_stage3(&stage3_list)?;

        info!("Found latest stage3 image: {}", latest.name);

        if self.is_cached(&latest) {
            info!("Stage3 image already cached: {}", latest.name);
            return Ok(latest);
        }

        self.download_stage3(&latest)?;
        self.verify_stage3(&latest)?;

        Ok(latest)
    }

    /// Fetch the list of available stage3 images from Gentoo mirrors
    fn fetch_stage3_list(&self) -> Result<Vec<Stage3Info>, Stage3Error> {
        let latest_url = format!(
            "{}/releases/{}/autobuilds/latest-stage3-{}.txt",
            self.mirror_url.trim_end_matches('/'),
            self.target.arch.as_keyword(),
            self.target.flavor
        );

        info!("Fetching stage3 list from: {}", latest_url);

        let output = Command::new("curl")
            .arg("-s")
            .arg("-f")
            .arg(&latest_url)
            .output()
            .map_err(|e| Stage3Error::IoError(e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Stage3Error::FetchError(format!(
                "Failed to fetch stage3 list: {}",
                stderr
            )));
        }

        let content = String::from_utf8_lossy(&output.stdout);
        self.parse_stage3_list(&content)
    }

    /// Fetch all available stage3 images for the architecture (not flavor-specific)
    fn fetch_all_stage3_flavors(&self) -> Result<Vec<Stage3Info>, Stage3Error> {
        let latest_url = format!(
            "{}/releases/{}/autobuilds/latest-stage3.txt",
            self.mirror_url.trim_end_matches('/'),
            self.target.arch.as_keyword()
        );

        info!("Fetching all stage3 flavors from: {}", latest_url);

        let output = Command::new("curl")
            .arg("-s")
            .arg("-f")
            .arg(&latest_url)
            .output()
            .map_err(|e| Stage3Error::IoError(e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Stage3Error::FetchError(format!(
                "Failed to fetch all stage3 flavors: {}",
                stderr
            )));
        }

        let content = String::from_utf8_lossy(&output.stdout);
        self.parse_all_flavors_list(&content)
    }

    /// Parse stage3 list content into Stage3Info structures (for all flavors)
    fn parse_all_flavors_list(&self, content: &str) -> Result<Vec<Stage3Info>, Stage3Error> {
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
                    Stage3Error::ParseError(format!(
                        "Failed to parse size for {}: {}",
                        full_path, e
                    ))
                })?;

                let name = full_path
                    .split('/')
                    .last()
                    .unwrap_or(&full_path)
                    .to_string();

                if name.starts_with("stage3-") {
                    let date = extract_date_from_filename(&name);
                    let actual_flavor = extract_flavor_from_filename(&name);

                    stage3_images.push(Stage3Info {
                        name: name.clone(),
                        url: format!(
                            "{}/releases/{}/autobuilds/{}",
                            self.mirror_url.trim_end_matches('/'),
                            self.target.arch.as_keyword(),
                            full_path
                        ),
                        size,
                        date,
                        arch: self.target.arch,
                        flavor: actual_flavor,
                    });
                }
            }
        }

        if stage3_images.is_empty() {
            return Err(Stage3Error::ParseError(format!(
                "No stage3 images found for arch={}",
                self.target.arch
            )));
        }

        Ok(stage3_images)
    }

    /// Parse stage3 list content into Stage3Info structures
    fn parse_stage3_list(&self, content: &str) -> Result<Vec<Stage3Info>, Stage3Error> {
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
                    Stage3Error::ParseError(format!(
                        "Failed to parse size for {}: {}",
                        full_path, e
                    ))
                })?;

                let name = full_path
                    .split('/')
                    .last()
                    .unwrap_or(&full_path)
                    .to_string();

                if name.starts_with("stage3-") {
                    let date = extract_date_from_filename(&name);
                    let actual_flavor = extract_flavor_from_filename(&name);

                    stage3_images.push(Stage3Info {
                        name: name.clone(),
                        url: format!(
                            "{}/releases/{}/autobuilds/{}",
                            self.mirror_url.trim_end_matches('/'),
                            self.target.arch.as_keyword(),
                            full_path
                        ),
                        size,
                        date,
                        arch: self.target.arch,
                        flavor: actual_flavor,
                    });
                }
            }
        }

        if stage3_images.is_empty() {
            return Err(Stage3Error::ParseError(format!(
                "No stage3 images found for arch={}, flavor={}",
                self.target.arch, self.target.flavor
            )));
        }

        Ok(stage3_images)
    }

    /// Find the latest stage3 image from a list
    fn find_latest_stage3(&self, images: &[Stage3Info]) -> Result<Stage3Info, Stage3Error> {
        images
            .iter()
            .max_by(|a, b| {
                let a_ts = extract_timestamp(&a.name);
                let b_ts = extract_timestamp(&b.name);
                a_ts.cmp(&b_ts)
            })
            .cloned()
            .ok_or_else(|| Stage3Error::ParseError("No stage3 images available".to_string()))
    }

    /// Check if a stage3 image is already cached
    fn is_cached(&self, stage3: &Stage3Info) -> bool {
        let cache_path = self.cache_dir.join(&stage3.name);
        cache_path.exists()
    }

    /// Download a stage3 image
    fn download_stage3(&self, stage3: &Stage3Info) -> Result<(), Stage3Error> {
        std::fs::create_dir_all(&self.cache_dir).map_err(|e| Stage3Error::IoError(e))?;

        let cache_path = self.cache_dir.join(&stage3.name);

        info!("Downloading stage3 image: {}", stage3.name);
        info!("URL: {}", stage3.url);
        info!("Size: {} bytes", stage3.size);

        let output = Command::new("curl")
            .arg("-L")
            .arg("-o")
            .arg(&cache_path)
            .arg(&stage3.url)
            .output()
            .map_err(|e| Stage3Error::IoError(e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Stage3Error::DownloadError(format!(
                "Failed to download {}: {}",
                stage3.name, stderr
            )));
        }

        info!("Downloaded stage3 image to: {}", cache_path.display());

        Ok(())
    }

    /// Verify a downloaded stage3 image
    fn verify_stage3(&self, stage3: &Stage3Info) -> Result<(), Stage3Error> {
        let cache_path = self.cache_dir.join(&stage3.name);

        if !cache_path.exists() {
            return Err(Stage3Error::VerifyError(format!(
                "Stage3 image not found: {}",
                cache_path.display()
            )));
        }

        let metadata = std::fs::metadata(&cache_path).map_err(|e| Stage3Error::IoError(e))?;

        if metadata.len() != stage3.size {
            return Err(Stage3Error::VerifyError(format!(
                "Size mismatch for {}: expected {}, got {}",
                stage3.name,
                stage3.size,
                metadata.len()
            )));
        }

        info!("Stage3 image verified successfully: {}", stage3.name);

        Ok(())
    }

    /// Extract stage3 image to target directory
    pub fn extract_stage3(
        &self,
        stage3: &Stage3Info,
        target_dir: impl AsRef<Path>,
    ) -> Result<(), Stage3Error> {
        let cache_path = self.cache_dir.join(&stage3.name);
        let target_dir = target_dir.as_ref();

        if !cache_path.exists() {
            return Err(Stage3Error::ExtractError(format!(
                "Stage3 image not found in cache: {}",
                cache_path.display()
            )));
        }

        info!("Extracting stage3 image: {}", stage3.name);
        info!("Target directory: {}", target_dir.display());

        std::fs::create_dir_all(target_dir).map_err(|e| Stage3Error::IoError(e))?;

        let output = Command::new("tar")
            .arg("--exclude")
            .arg("dev/*")
            .arg("-xJpf")
            .arg(&cache_path)
            .arg("-C")
            .arg(target_dir)
            .output()
            .map_err(|e| Stage3Error::IoError(e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Stage3Error::ExtractError(format!(
                "Failed to extract {}: {}",
                stage3.name, stderr
            )));
        }

        info!(
            "Stage3 image extracted successfully to: {}",
            target_dir.display()
        );

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

/// Extract flavor from stage3 filename
fn extract_flavor_from_filename(filename: &str) -> String {
    let without_ext = filename.replace(".tar.xz", "");
    let parts: Vec<&str> = without_ext.split('-').collect();

    if parts.len() >= 3 {
        return format!("{}-{}", parts[1], parts[2]);
    }

    without_ext.replace("stage3-", "")
}

/// Extract date from stage3 filename
fn extract_date_from_filename(filename: &str) -> String {
    let parts: Vec<&str> = filename.split('-').collect();
    if parts.len() >= 4 {
        let last_part = parts[parts.len() - 1];
        let timestamp_part = last_part.replace(".tar.xz", "");
        if timestamp_part.len() >= 8 {
            return timestamp_part[..8].to_string();
        }
    }
    "unknown".to_string()
}
