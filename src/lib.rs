//! A library to provide conversions, prompts and menu functionality for
//! Discord bots created with [`serenity`].
//!
//! ## Features
//!
//! This library provides the implementations to easily:
//! - Convert a string to [`serenity`]'s guild-specific models.
//! - Get user response using message or reaction prompts.
//! - Display paginated reaction-based messages/menus.
//!
//! See module level documentation for in-depth info about the utilities
//! provided by this crate.
//!
//! **Note:** This crate only supports [`serenity`]'s await versions.
//!
//! [`serenity`]: https://github.com/serenity-rs/serenity

pub mod conversion;
mod error;
pub mod menu;
pub mod prompt;

#[doc(inline)]
pub use error::Error;
