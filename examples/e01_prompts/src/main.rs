//! This example showcases how prompts can be used.
//!
//! You are expected to be familier with serenity's basics.

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::prelude::{Message, ReactionType, Ready},
};

// Bring prompt functions into scope.
use serenity_utils::prompt::{message_prompt_content, reaction_prompt};

use std::env;

#[group]
#[commands(colour, pet)]
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
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn colour(ctx: &Context, msg: &Message) -> CommandResult {
    let prompt_msg = msg
        .channel_id
        .say(&ctx.http, "What is your favourite colour?")
        .await?;

    // We want to get the content of the user's response. The prompt will wait for
    // the first message user sends in the channel where this message was used
    // and then send the `content` of the message. If that user doesn't send any
    // message in 30 seconds, the prompt will end.
    // You can use `message_prompt` to get the `Message` instead of the content.
    if let Some(colour) = message_prompt_content(ctx, &prompt_msg, &msg.author, 30.0).await {
        msg.reply(&ctx.http, format!("{} is my favourite too!", colour))
            .await?;
    } else {
        // No response.
        msg.reply(&ctx.http, "I like red!").await?;
    }

    Ok(())
}

#[command]
async fn pet(ctx: &Context, msg: &Message) -> CommandResult {
    // Emojis for the prompt.
    let emojis = [ReactionType::from('🐶'), ReactionType::from('🐱')];

    // Send a message to the user and ask them to react.
    let prompt_msg = msg
        .channel_id
        .say(&ctx.http, "Do you like dogs or cats more? React below!")
        .await?;

    // The prompt will wait for the first reaction user adds to the `prompt_msg`
    // and then return the index of the emoji and the emoji itself. If that user
    // doesn't react in 30 seconds, the prompt will end.
    let (index, _emoji) = reaction_prompt(ctx, &prompt_msg, &msg.author, &emojis, 30.0).await?;

    if index == 0 {
        // The user reacted with `🐶`.
        msg.reply(&ctx.http, format!("I like {} more!", emojis[1]))
            .await?;
    } else {
        // The user reacted with `🐱`.
        msg.reply(&ctx.http, format!("I like {} more!", emojis[0]))
            .await?;
    }

    Ok(())
}
