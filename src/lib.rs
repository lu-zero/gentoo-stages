//! Gentoo stage3 image support
//!
//! This crate provides functionality for fetching, parsing, and managing
//! Gentoo Linux stage3 images.

mod client;
mod error;
mod models;

pub use client::Client;
pub use error::Error;
pub use models::{Cache, Stage3};
