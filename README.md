# serenity-utils

A library to provide conversions, prompts and menu functionality for
Discord bots created with [serenity].

This library provides implementations to easily:

- Convert a string to [serenity]'s guild-specific models.
- Get user response using message or reaction prompts.
- Display paginated reaction-based messages/menus.
- Format text in different ways before sending.

## Installation and Usage

To use this crate, add the following to your `Cargo.toml`:

```toml
[dependencies]
serenity_utils = "0.1.0"
```

**Note:** This crate only supports [serenity]'s async versions and a minimum of Rust 1.39.

## Examples

Here are a few examples to use some of [serenity_utils]'s features.

### Reaction Prompt

```rust
use serenity::{
   model::prelude::{ChannelId, Message, ReactionType},
   prelude::Context,
};
use serenity_utils::{prompt::reaction_prompt, Error};

async fn prompt(ctx: &Context, msg: &Message) -> Result<(), Error> {
    // Emojis for the prompt.
    let emojis = [
        ReactionType::from('🐶'),
        ReactionType::from('🐱'),
    ];

    let prompt_msg = ChannelId(7).say(&ctx.http, "Dogs or cats?").await?;

    // Creates the prompt and returns the result. Because of `reaction_prompt`'s
    // return type, you can use the `?` operator to get the result.
    // The `Ok()` value is the selected emoji's index (wrt the `emojis` slice)
    // and the emoji itself. We don't require the emoji here, so we ignore it.
    let (idx, _) = reaction_prompt(
        ctx,
        &prompt_msg,
        &msg.author,
        &emojis,
        30.0
    )
    .await?;

    if idx == 0 {
        // Dogs!
    } else {
        // Cats!
    }

    Ok(())
}
```

### Menu

```rust
use serenity::{
    builder::CreateEmbed,
    model::prelude::Message,
    prelude::Context,
};
use serenity_utils::{menu::{Menu, MenuOptions}, Error};

async fn use_menu(ctx: &Context, msg: &Message) -> Result<(), Error> {
    let mut page_one = CreateEmbed::default();
    page_one.description("Page number one!");

    let mut page_two = CreateEmbed::default();
    page_two.description("Page number two!");

    let pages = [page_one, page_two];

    // Creates a new menu.
    let mut menu = Menu::new(ctx, msg, &pages, MenuOptions::default());

    // Runs the menu and returns optional `Message` used to display the menu.
    let opt_message = menu.run().await?;

    Ok(())
}
```

### More Examples

More examples detailing the crate's functionality can be found in the [`examples`] directory.

## Features

Some functionality of this crate is dependent on [serenity]'s features.

The following serenity features are required when using [`serenity_utils`]:

- **client**
- **collector**
- **gateway**
- **model**

The following features are optional:

- **cache**: It is required to get `Member` from user name, tag or nickname when using the `Conversion` trait.
- **rustls_backend**: Uses `Rustls` for all platforms, a pure Rust implementation.
- **native_tls_backend**: Uses `SChannel` on Windows, `Secure Transport` on macOS, and `OpenSSL` on other platforms.

**Note:** One of `rustls_backend` and `native_tls_backend` must be used.

The following optional features are enabled by default:

- **cache**
- **rustls_backend**

[serenity]: https://github.com/serenity-rs/serenity
[serenity_utils]: https://github.com/AriusX7/serenity-utils
[`examples`]: https://github.com/AriusX7/serenity-utils/tree/master/examples
[`serenity_utils`]: https://github.com/AriusX7/serenity-utils
