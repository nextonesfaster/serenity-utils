//! Prompts to get a user's response via a reaction.
//!
//! ## Example
//!
//! ```
//! # use serenity::{
//! #    model::prelude::{ChannelId, Message},
//! #    prelude::Context,
//! # };
//! # use serenity_utils::{prompt::yes_or_no_prompt, Error};
//! #
//!
//! async fn prompt(ctx: &Context, msg: &Message) -> Result<(), Error> {
//!     let prompt_msg = ChannelId(7).say(&ctx.http, "What is your favourite colour?").await?;
//!
//!     // Result of user's reaction to the prompt.
//!     let result = yes_or_no_prompt(ctx, &prompt_msg, &msg.author, 30.0).await?;
//!
//!     Ok(())
//! }
//! ```

use crate::{error::Error, misc::add_reactions};
use serenity::{
    collector::ReactionAction,
    futures::StreamExt,
    model::prelude::{Message, ReactionType, User},
    prelude::Context,
};
use std::time::Duration;

/// Creates a reaction prompt to get user's reaction.
///
/// Reactions are collected on the specified message. Only messages sent by `user`
/// are considered. Reactions are only considered for `timeout` seconds.
///
/// ## Example
///
/// ```
/// # use serenity::{
/// #    model::prelude::{ChannelId, Message, ReactionType},
/// #    prelude::Context,
/// # };
/// # use serenity_utils::{prompt::reaction_prompt, Error};
/// #
/// async fn prompt(ctx: &Context, msg: &Message) -> Result<(), Error> {
///     // Emojis for the prompt.
///     let emojis = [
///         ReactionType::from('🐶'),
///         ReactionType::from('🐱'),
///     ];
///
///     let prompt_msg = ChannelId(7).say(&ctx.http, "Dogs or cats?").await?;
///
///     // Creates the prompt and returns the result. Because of `reaction_prompt`'s
///     // return type, you can use the `?` operator to get the result.
///     // The `Ok()` value is the selected emoji's index (wrt the `emojis` slice)
///     // and the emoji itself. We don't require the emoji here, so we ignore it.
///     let (idx, _) = reaction_prompt(
///         ctx,
///         &prompt_msg,
///         &msg.author,
///         &emojis,
///         30.0
///     )
///     .await?;
///
///     if idx == 0 {
///         // Dogs!
///     } else {
///         // Cats!
///     }
///
///     Ok(())
/// }
/// ```
///
/// ## Errors
///
/// Returns [`Error::SerenityError`] if cache is enabled and the current
/// user does not have the required permissions to add reactions.
///
/// Returns [`Error::TimeoutError`] if user does not react at all.
///
/// [`Error::SerenityError`]: ../error/enum.Error.html#variant.SerenityError
/// [`Error::TimeoutError`]: ../error/enum.Error.html#variant.TimeoutError
pub async fn reaction_prompt(
    ctx: &Context,
    msg: &Message,
    user: &User,
    emojis: &[ReactionType],
    timeout: f32,
) -> Result<(usize, ReactionType), Error> {
    add_reactions(ctx, msg, emojis.to_vec()).await?;

    let mut collector = user
        .await_reactions(&ctx)
        .message_id(msg.id)
        .timeout(Duration::from_secs_f32(timeout))
        .await;

    while let Some(action) = collector.next().await {
        if let ReactionAction::Added(reaction) = action.as_ref() {
            if emojis.contains(&reaction.emoji) {
                return Ok((
                    emojis.iter().position(|p| p == &reaction.emoji).unwrap(),
                    reaction.emoji.clone(),
                ));
            }
        }
    }

    Err(Error::TimeoutError)
}

/// A special reaction prompt to check if user reacts with yes or no.
///
/// ✅ is used for yes and ❌ is used for no.
///
/// This function behaves in same way as [`reaction_prompt`] except for the
/// return type. If the user reacts with the yes emoji, the Ok value is `true`.
/// It user reacts with no emoji, the value is `false`.
///
/// ## Example
///
/// ```
/// # use serenity::{
/// #    model::prelude::{ChannelId, Message, ReactionType},
/// #    prelude::Context,
/// # };
/// # use serenity_utils::{prompt::yes_or_no_prompt, Error};
/// #
/// async fn prompt(ctx: &Context, msg: &Message) -> Result<(), Error> {
///     let prompt_msg = ChannelId(7).say(&ctx.http, "Are you a bot?").await?;
///
///     // Creates a yes/no prompt and returns the result.
///     let result = yes_or_no_prompt(
///         ctx,
///         &prompt_msg,
///         &msg.author,
///         30.0
///     )
///     .await?;
///
///     if result {
///         // Is a bot!
///     } else {
///         // Not a bot!
///     }
///
///     Ok(())
/// }
/// ```
///
/// ## Errors
///
/// It can return the same errors as [`reaction_prompt`].
///
/// [`reaction_prompt`]: fn.reaction_prompt.html
pub async fn yes_or_no_prompt(
    ctx: &Context,
    msg: &Message,
    user: &User,
    timeout: f32,
) -> Result<bool, Error> {
    let emojis = [ReactionType::from('✅'), ReactionType::from('❌')];

    reaction_prompt(ctx, msg, user, &emojis, timeout)
        .await
        .map(|(i, _)| i == 0)
}
