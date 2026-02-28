//! Gentoo stage3 image fetching and management
//!
//! This crate provides functionality for fetching, parsing, and managing
//! Gentoo Linux stage3 images for cross-compilation purposes.

pub mod error;
pub mod models;
pub mod stage3;

pub use error::Stage3Error;
pub use models::{Stage3Info, Target};
pub use stage3::Stage3Fetcher;
