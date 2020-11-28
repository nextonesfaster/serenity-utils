//! Provides alternatives to serenity's message and embed builders.
//!
//! Unlike serenity's builders, the builders here use separate fields for all
//! values instead of a [`HashMap`]. This provides an easy way to access the
//! builder's fields.
//!
//! Due to the user-friendliness of these builders, they are slightly less
//! efficient than serenity's builders. You should only use these when you need
//! access to the builder's values which are set somewhere else.
//!
//! All builders provide trait implementations to convert them into serenity's
//! builders.
//!
//! [`HashMap`]: std::collections::HashMap

pub mod embed;
pub mod message;
pub mod prelude;
