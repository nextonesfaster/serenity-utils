//! This example showcases how the `Conversion` trait can be used.
//!
//! You are expected to be familier with serenity's basics.

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::standard::{
        macros::{command, group},
        Args, CommandResult, StandardFramework,
    },
    model::{
        mention::Mentionable,
        prelude::{Member, Message, Ready},
    },
    prelude::GatewayIntents,
};

// Bring the `Conversion` trait into scope.
use serenity_utils::conversion::Conversion;

use std::env;

#[command]
async fn hello(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // We'll use the `from_guild_id_and_str` method as it works even if the
    // cache feature is not enabled.
    // Please note that a `Member` object cannot be created from user name,
    // nickname or user tag if the `cache` feature and the `GUILDS` and
    // `GUILD_PRESENCES` intents are not enabled. User mentions
    // and IDs work.
    if let Some(guild_id) = msg.guild_id {
        if let Some(member) = Member::from_guild_id_and_str(ctx, guild_id, args.message()).await {
            msg.channel_id
                .say(
                    &ctx.http,
                    format!("{} said hello, {}!", msg.author.name, member.mention()),
                )
                .await?;
        } else {
            msg.channel_id
                .say(&ctx.http, "No member found from the given input.")
                .await?;
        }
    } else {
        msg.channel_id
            .say(&ctx.http, "This command is only available in servers.")
            .await?;
    }

    // The `Conversion` trait can be used for `Role` and `GuildChannel` similarly.

    Ok(())
}

// The rest is basic serenity configuration.

#[group]
#[commands(hello)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(
        token,
        GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::GUILD_PRESENCES
            | GatewayIntents::GUILDS,
    )
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
