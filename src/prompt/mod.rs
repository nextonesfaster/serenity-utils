//! Prompts to get user's response interactively.
//!
//! ## Examples
//!
//! This library provides two types of prompts: message-based and reaction-based.
//! An example for both is given below.
//!
//! ### Message Prompt
//!
//! ```
//! # use serenity::{
//! #    model::prelude::{ChannelId, Message},
//! #    prelude::Context,
//! # };
//! # use serenity_utils::{prompt::message_prompt_content, Error};
//! #
//! async fn mprompt(ctx: &Context, msg: &Message) -> Result<(), Error> {
//!     let prompt_msg = ChannelId(7).say(&ctx.http, "What is your favourite colour?").await?;
//!
//!     // User's optional response to the message.
//!     let optional_content = message_prompt_content(ctx, &prompt_msg, &msg.author, 30.0).await;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Reaction Prompt
//!
//! ```
//! # use serenity::{
//! #    model::prelude::{ChannelId, Message},
//! #    prelude::Context,
//! # };
//! # use serenity_utils::{prompt::yes_or_no_prompt, Error};
//! #
//! async fn rprompt(ctx: &Context, msg: &Message) -> Result<(), Error> {
//!     let prompt_msg = ChannelId(7).say(&ctx.http, "Is red your favourite colour?").await?;
//!
//!     // Result of user's reaction to the prompt.
//!     let result = yes_or_no_prompt(ctx, &prompt_msg, &msg.author, 30.0).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! For more in-depth usage and examples, see individual functions.

mod message;
mod reaction;

#[doc(inline)]
pub use message::*;
#[doc(inline)]
pub use reaction::*;
