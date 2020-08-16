//! Prompts to get a user's response via a message.
//!
//! ## Example
//!
//! ```
//! # use serenity::{
//! #    model::prelude::{ChannelId, Message},
//! #    prelude::Context,
//! # };
//! # use serenity_utils::{prompt::message_prompt_content, Error};
//! #
//! async fn prompt(ctx: &Context, msg: &Message) -> Result<(), Error> {
//!     // Assuming `channel_id` is bound.
//!     let prompt_msg = ChannelId(7).say(&ctx.http, "What is your favourite colour?").await?;
//!
//!     // User's optional response to the message.
//!     let optional_content = message_prompt_content(ctx, &prompt_msg, &msg.author, 30.0).await;
//!
//!     Ok(())
//! }
//! ```

use serenity::{
    model::prelude::{Message, User},
    prelude::Context,
};
use std::time::Duration;

/// Creates a message prompt to get the next message a user sends.
///
/// Only messages sent in the channel of the original message are considered.
/// The bot waits for a message for `timeout` seconds only. `None` is returned
/// if the user does not send another message.
///
/// ## Example
///
/// ```
/// # use serenity::{
/// #    model::prelude::{ChannelId, Message},
/// #    prelude::Context,
/// # };
/// # use serenity_utils::{prompt::message_prompt, Error};
/// #
/// async fn prompt(ctx: &Context, msg: &Message) -> Result<(), Error> {
///     // Assuming `channel_id` is bound.
///     let prompt_msg = ChannelId(7).say(&ctx.http, "What is your favourite colour?").await?;
///
///     // Optional `Message` object of user's response to the message.
///     let optional_msg = message_prompt(ctx, &prompt_msg, &msg.author, 30.0).await;
///
///     Ok(())
/// }
/// ```
///
/// See [`message_prompt_content`] if you only need the message's content.
///
/// [`message_prompt_content`]: fn.message_prompt_content.html
pub async fn message_prompt(
    ctx: &Context,
    msg: &Message,
    user: &User,
    timeout: f32,
) -> Option<Message> {
    user.await_reply(&ctx)
        .channel_id(msg.channel_id)
        .timeout(Duration::from_secs_f32(timeout))
        .await
        .map(|m| m.as_ref().clone())
}

/// Creates a message prompt to get the content of the next message a user sends.
///
/// Only messages sent in the channel of the original message are considered.
/// The bot waits for a message for `timeout` seconds only. `None` is returned
/// if the user does not send another message.
///
/// ## Example
///
/// ```
/// # use serenity::{
/// #    model::prelude::{ChannelId, Message},
/// #    prelude::Context,
/// # };
/// # use serenity_utils::{prompt::message_prompt_content, Error};
/// #
/// async fn prompt(ctx: &Context, msg: &Message) -> Result<(), Error> {
///     // Assuming `channel_id` is bound.
///     let prompt_msg = ChannelId(7).say(&ctx.http, "What is your favourite colour?").await?;
///
///     // User's optional response to the message.
///     let optional_content = message_prompt_content(ctx, &prompt_msg, &msg.author, 30.0).await;
///
///     Ok(())
/// }
/// ```
///
/// See [`message_prompt`] if you need the whole message object.
///
/// [`message_prompt`]: fn.message_prompt.html
pub async fn message_prompt_content(
    ctx: &Context,
    msg: &Message,
    user: &User,
    timeout: f32,
) -> Option<String> {
    user.await_reply(&ctx)
        .channel_id(msg.channel_id)
        .timeout(Duration::from_secs_f32(timeout))
        .await
        .map(|m| m.content.clone())
}
