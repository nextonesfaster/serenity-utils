#![allow(deprecated)]

use serenity::builder::*;
use serenity::model::prelude::ReactionType;
use serenity_utils::builder::prelude::*;

#[test]
fn test_to_create_embed_author() {
    let mut builder = EmbedAuthorBuilder::new("Arius");
    builder.set_url("https://github.com/AriusX7/serenity-utils");

    let mut create_embed_author = CreateEmbedAuthor::default();
    create_embed_author.name("Arius").url("https://github.com/AriusX7/serenity-utils");

    assert_eq!(builder.to_create_embed_author().0, create_embed_author.0);
}

#[test]
fn test_to_create_embed_footer() {
    let mut builder = EmbedFooterBuilder::new("text");
    builder.set_icon_url("https://github.com/AriusX7/serenity-utils");

    let mut create_embed_footer = CreateEmbedFooter::default();
    create_embed_footer.text("text").icon_url("https://github.com/AriusX7/serenity-utils");

    assert_eq!(builder.to_create_embed_footer().0, create_embed_footer.0);
}

#[test]
fn test_to_create_embed() {
    let mut builder = EmbedBuilder::new();
    builder
        .set_description("This is the embed description.")
        .set_author_with(|a| a.set_name("The embed author name!"));

    let mut create_embed = CreateEmbed::default();
    create_embed
        .description("This is the embed description.")
        .author(|a| a.name("The embed author name!"));

    assert_eq!(builder.to_create_embed().0, create_embed.0);
}

#[test]
fn test_to_create_message() {
    let mut builder = MessageBuilder::new();
    builder
        .set_content("This is the message content.")
        .set_embed_with(|e| {
            e.set_description("This is the embed description.");
            e.set_author_with(|a| {
                a.set_name("The embed author name!");

                a
            });

            e
        })
        .add_reactions(vec![ReactionType::from('🐶'), ReactionType::from('🐱')]);

    let transformed_create_message = builder.to_create_message();

    let mut create_message = CreateMessage::default();
    create_message
        .content("This is the message content.")
        .embed(|e| {
            e.description("This is the embed description.");
            e.author(|a| {
                a.name("The embed author name!");

                a
            });

            e
        })
        .reactions(vec![ReactionType::from('🐶'), ReactionType::from('🐱')]);

    assert_eq!(transformed_create_message.0, create_message.0);
    assert_eq!(transformed_create_message.1, create_message.1);
}

#[test]
fn test_to_edit_message() {
    let mut builder = MessageBuilder::new();
    builder
        .set_content("This is the message content.")
        .set_embed_with(|e| {
            e.set_description("This is the embed description.");
            e.set_author_with(|a| {
                a.set_name("The embed author name!");

                a
            });

            e
        })
        .add_reactions(vec![ReactionType::from('🐶'), ReactionType::from('🐱')]);

    let mut edit_message = EditMessage::default();
    edit_message.content("This is the message content.").embed(|e| {
        e.description("This is the embed description.");
        e.author(|a| {
            a.name("The embed author name!");

            a
        });

        e
    });

    assert_eq!(builder.to_edit_message().0, edit_message.0);
}
