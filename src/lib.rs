//! Gentoo stage3 image fetching and management
//!
//! This crate provides functionality for fetching, parsing, and managing
//! Gentoo Linux stage3 images for cross-compilation purposes.

pub mod client;
pub mod error;
pub mod models;

pub use client::Client;
pub use error::Error;
pub use models::{Cache, Stage3};
