//! This example showcases how reaction-based menu can be used.
//!
//! You are expected to be familier with serenity's basics.

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::prelude::{Message, Reaction, ReactionType, Ready},
};

// Bring menu items into scope along with `MessageBuilder`.
use serenity_utils::{builder::message::MessageBuilder, menu::*};

use std::{env, sync::Arc};

// A custom function to be used as a control function for the menu.
async fn first_page<'a>(menu: &mut Menu<'a>, reaction: Reaction) {
    // Remove the reaction used to change the menu.
    let _ = &reaction.delete(&menu.ctx.http).await;

    // Set page number to `0`.
    menu.options.page = 0;
}

// A custom function to be used as a control function for the menu.
async fn last_page<'a>(menu: &mut Menu<'a>, reaction: Reaction) {
    // Remove the reaction used to change the menu.
    let _ = &reaction.delete(&menu.ctx.http).await;

    // Set page number to total - 1.
    menu.options.page = menu.pages.len() - 1;
}

#[command]
async fn scoreboard(ctx: &Context, msg: &Message) -> CommandResult {
    // We'll use a reaction-based menu to display the scoreboard.

    // First, let's create controls for the menu.
    let controls = vec![
        Control::new(
            ReactionType::from('⏪'),
            Arc::new(|m, r| Box::pin(first_page(m, r))),
        ),
        Control::new(
            ReactionType::from('◀'),
            Arc::new(|m, r| Box::pin(prev_page(m, r))),
        ),
        Control::new(
            ReactionType::from('❌'),
            Arc::new(|m, r| Box::pin(close_menu(m, r))),
        ),
        Control::new(
            ReactionType::from('▶'),
            Arc::new(|m, r| Box::pin(next_page(m, r))),
        ),
        Control::new(
            ReactionType::from('⏩'),
            Arc::new(|m, r| Box::pin(last_page(m, r))),
        ),
    ];

    // Let's create options for the menu.
    let options = MenuOptions {
        controls,
        ..Default::default()
    };

    // Now, we need pages to display the scoreboard.
    let mut page_one = MessageBuilder::default();
    page_one.set_content("Player A!").set_embed_with(|e| {
        e.set_description("Player A scored 10 points!");

        e
    });

    let mut page_two = MessageBuilder::default();
    page_two.set_content("Player B!").set_embed_with(|e| {
        e.set_description("Player B scored 5 points!");

        e
    });

    let mut page_three = MessageBuilder::default();
    page_three.set_content("Player C!").set_embed_with(|e| {
        e.set_description("Player C scored 8 points!");

        e
    });

    let pages = &[page_one, page_two, page_three];

    // Finally, we'll create a menu and run it.
    let mut menu = Menu::new(ctx, msg, pages, options);
    let _ = menu.run().await?;

    Ok(())
}

// The rest is basic serenity configuration.

#[group]
#[commands(scoreboard)]
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
    let mut client = Client::new(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
