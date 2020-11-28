//! Provides a trait to convert strings into serenity's guild-specific models.
//!
//! The trait provides two methods:
//! - [`from_guild_and_str`]
//! - [`from_guild_id_and_str`]
//!
//! The first method is available only when `cache` feature is enabled. The
//! second method is always available.
//!
//! ## Limitation
//!
//! If the `cache` feature is not enabled, an argument is only treated as an ID
//! or mention when trying to convert to `Member`. It is not treated as user
//! name, nickname or user tag.
//!
//! ## Example
//!
//! Bring [`Conversion`] trait into scope.
//!
//! ```
//! use serenity_utils::conversion::Conversion;
//! ```
//!
//! If cache is enabled and you require `Guild` for other purposes too,
//! use [`from_guild_and_str`] method so the guild is not fetched multiple times.
//!
//! ```
//! # use serenity::{model::prelude::{Guild, Member, Message}, prelude::Context};
//! # use serenity_utils::conversion::Conversion;
//! #
//! async fn foo(ctx: &Context, msg: &Message, arg: &str) {
//!     let guild = match msg.guild(&ctx.cache).await {
//!         Some(g) => g,
//!         None => return
//!     };
//!
//!     // Tries to get member from guild and the argument.
//!     let opt_member = Member::from_guild_and_str(&guild, arg).await;
//! }
//! ```
//!
//! If cache is disabled or if cache is enabled but you don't require `Guild`
//! for other purposes, use [`from_guild_id_and_str`] method.
//!
//! ```
//! # use serenity::{model::prelude::{Guild, Message, Role}, prelude::Context};
//! # use serenity_utils::conversion::Conversion;
//! #
//! async fn bar(ctx: &Context, msg: &Message, arg: &str) {
//!     // Tries to get role from guild id and the argument.
//!     if let Some(guild_id) = msg.guild_id {
//!         let opt_role = Role::from_guild_id_and_str(ctx, guild_id, arg).await;
//!     }
//! }
//! ```
//!
//! [`from_guild_and_str`]: Conversion::from_guild_and_str
//! [`from_guild_id_and_str`]: Conversion::from_guild_id_and_str

use serenity::{async_trait, model::prelude::*, prelude::Context, utils::parse_mention};
use std::collections::HashMap;

/// A trait to convert a string into serenity's models.
///
/// It provides two methods to convert a string into a guild-specific model.
/// The first method, [`from_guild_and_str`], is available only if cache is enabled.
/// The second method, [`from_guild_id_and_str`], is always available.
///
/// The second method tries to use the cache if it is enabled. Otherwise, it
/// gets data over the REST API.
///
/// ## Conversion Strategy
///
/// Conversion follows this strategy:
/// - Converting argument into a ID and then fetching model using the ID.
/// - Converting argument into a mention and then fetching model using the
///     extracted ID.
/// - Treating argument as model's name.
///
/// **Note:** For [`Member`], nickname and user tag are considered along
/// with the user name.
///
/// ## Limitation
///
/// If the `cache` feature is not enabled, an argument is only treated as an ID
/// or mention when trying to convert to [`Member`]. It is not treated as user
/// name, nickname or tag.
///
/// ## Implementation
///
/// To implement this trait for a custom type, you have to implement both
/// [`from_guild_and_str`] and [`from_guild_id_and_str`] methods.
/// The strategy you use may depend on your model.
///
/// [`from_guild_and_str`]: Conversion::from_guild_and_str
/// [`from_guild_id_and_str`]: Conversion::from_guild_id_and_str
#[async_trait]
pub trait Conversion {
    /// The type of the model to convert to.
    type Item;

    /// Converts `arg` into the specified type, if possible.
    #[cfg(feature = "cache")]
    async fn from_guild_and_str(guild: &Guild, arg: &str) -> Option<Self::Item>
    where
        Self: Sized;

    async fn from_guild_id_and_str(
        ctx: &Context,
        guild_id: GuildId,
        arg: &str,
    ) -> Option<Self::Item>
    where
        Self: Sized;
}

#[async_trait]
impl Conversion for Role {
    type Item = Self;

    /// Converts `arg` into a [`Role`] object.
    #[cfg(feature = "cache")]
    async fn from_guild_and_str(guild: &Guild, arg: &str) -> Option<Self>
    where
        Self: Sized,
    {
        let roles = &guild.roles;

        role_from_mapping(arg, roles).await
    }

    async fn from_guild_id_and_str(
        ctx: &Context,
        guild_id: GuildId,
        arg: &str,
    ) -> Option<Self::Item>
    where
        Self: Sized,
    {
        #[cfg(feature = "cache")]
        {
            if let Some(roles) = ctx.cache.guild_roles(guild_id).await {
                return role_from_mapping(arg, &roles).await;
            }
        }

        // Get guild's roles using http requests.
        let roles = ctx.http.get_guild_roles(guild_id.0).await.ok()?;
        match arg.parse::<u64>() {
            // `arg` is role ID.
            Ok(id) => roles.iter().find(|r| r.id.0 == id).cloned(),
            Err(_) => match parse_mention(arg) {
                // `arg` is role mention.
                Some(id) => roles.iter().find(|r| r.id.0 == id).cloned(),
                // `arg` is role name.
                None => roles.iter().find(|r| r.name == arg).cloned(),
            },
        }
    }
}

#[async_trait]
impl Conversion for Member {
    type Item = Self;

    /// Converts `arg` into a [`Member`] object.
    #[cfg(feature = "cache")]
    async fn from_guild_and_str(guild: &Guild, arg: &str) -> Option<Self>
    where
        Self: Sized,
    {
        let members = &guild.members;

        member_from_mapping(arg, members).await
    }

    async fn from_guild_id_and_str(
        ctx: &Context,
        guild_id: GuildId,
        arg: &str,
    ) -> Option<Self::Item>
    where
        Self: Sized,
    {
        #[cfg(feature = "cache")]
        {
            if let Some(members) = ctx.cache.guild_field(guild_id, |g| g.members.clone()).await {
                return member_from_mapping(arg, &members).await;
            }
        }

        let id = match arg.parse::<u64>() {
            // `arg` is a user ID.
            Ok(id) => id,
            Err(_) => match parse_mention(arg) {
                Some(id) => id,
                None => return None,
            },
        };

        ctx.http.get_member(guild_id.0, id).await.ok()
    }
}

#[async_trait]
impl Conversion for GuildChannel {
    type Item = Self;

    /// Converts `arg` into a [`GuildChannel`] object.
    #[cfg(feature = "cache")]
    async fn from_guild_and_str(guild: &Guild, arg: &str) -> Option<Self>
    where
        Self: Sized,
    {
        let channels = &guild.channels;

        channel_from_mapping(arg, channels).await
    }

    async fn from_guild_id_and_str(
        ctx: &Context,
        guild_id: GuildId,
        arg: &str,
    ) -> Option<Self::Item>
    where
        Self: Sized,
    {
        #[cfg(feature = "cache")]
        {
            if let Some(channels) = ctx.cache.guild_channels(guild_id).await {
                return channel_from_mapping(arg, &channels).await;
            }
        }

        // Get guild's roles using http requests.
        let channels = ctx.http.get_channels(guild_id.0).await.ok()?;
        match arg.parse::<u64>() {
            // `arg` is channel ID.
            Ok(id) => channels.iter().find(|c| c.id.0 == id).cloned(),
            Err(_) => match parse_mention(arg) {
                // `arg` is channel mention.
                Some(id) => channels.iter().find(|c| c.id.0 == id).cloned(),
                // `arg` is channel name.
                None => channels.iter().find(|c| c.name == arg).cloned(),
            },
        }
    }
}

async fn role_from_mapping(arg: &str, roles: &HashMap<RoleId, Role>) -> Option<Role> {
    match arg.parse::<u64>() {
        // `arg` is a role ID.
        Ok(id) => roles.get(&RoleId(id)).cloned(),
        Err(_) => match parse_mention(arg) {
            // `arg` is a role mention.
            Some(id) => roles.get(&RoleId(id)).cloned(),
            // `arg` is a role name.
            None => roles.values().find(|r| r.name == arg).cloned(),
        },
    }
}

async fn member_from_mapping(arg: &str, members: &HashMap<UserId, Member>) -> Option<Member> {
    match arg.parse::<u64>() {
        // `arg` is a user ID.
        Ok(id) => members.get(&UserId(id)).cloned(),
        Err(_) => match parse_mention(arg) {
            // `arg` is a member mention.
            Some(id) => members.get(&UserId(id)).cloned(),
            // `arg` is a member's name or nickname.
            None => members
                .values()
                .find(|m| {
                    m.display_name().as_str() == arg || m.user.name == arg || m.user.tag() == arg
                })
                .cloned(),
        },
    }
}

async fn channel_from_mapping(
    arg: &str,
    channels: &HashMap<ChannelId, GuildChannel>,
) -> Option<GuildChannel> {
    match arg.parse::<u64>() {
        // `arg` is a channel ID.
        Ok(id) => channels.get(&ChannelId(id)).cloned(),
        Err(_) => match parse_mention(arg) {
            // `arg` is a channel mention.
            Some(id) => channels.get(&ChannelId(id)).cloned(),
            // `arg` is a channel name.
            None => channels.values().find(|c| c.name == arg).cloned(),
        },
    }
}
