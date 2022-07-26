//! Miscellaneous utility functions to aid with performing common tasks.

use serenity::model::prelude::{Message, ReactionType};
use serenity::prelude::Context;
use serenity::Error;

/// Adds reactions in a non-blocking fashion.
///
/// This allows you to perform other tasks while reactions are being added. This
/// works by creating a separate task for adding emojis in the background. The
/// order of `emojis` is preserved.
///
/// See [`add_reactions_blocking`] to add reactions in a blocking fashion. This
/// function is slightly less efficient than the blocking counterpart.
pub async fn add_reactions(
    ctx: &Context,
    msg: &Message,
    emojis: Vec<ReactionType>,
) -> Result<(), Error> {
    let channel_id = msg.channel_id;
    let msg_id = msg.id;
    let http = ctx.http.clone();

    tokio::spawn(async move {
        for emoji in emojis {
            http.create_reaction(channel_id.0, msg_id.0, &emoji).await?;
        }

        Result::<_, Error>::Ok(())
    });

    Ok(())
}

/// Adds reactions in a blocking fashion.
///
/// This blocks the execution of code until all reactions are added. The order
/// of `emojis` is preserved.
///
/// See [`add_reactions`] to add reactions in a non-blocking fashion.
pub async fn add_reactions_blocking(
    ctx: &Context,
    msg: &Message,
    emojis: &[ReactionType],
) -> Result<(), Error> {
    for emoji in emojis {
        ctx.http.create_reaction(msg.channel_id.0, msg.id.0, emoji).await?;
    }

    Ok(())
}
