# serenity-utils

[![docs badge][]][docs link] [![crates.io badge][]][crates.io link] [![license badge][]][license link] [![rust 1.39.0+ badge]][rust 1.39.0+ link]

A library to provide conversions, prompts and menu functionality for
Discord bots created with [serenity].

This library provides implementations to easily:

- Convert a string to [serenity]'s guild-specific models.
- Get user response using message or reaction prompts.
- Display paginated reaction-based messages/menus.
- Format text in different ways before sending.
- Create embeds and messages with field access.

## Installation and Usage

To use this crate, add the following to your `Cargo.toml`:

```toml
[dependencies]
serenity_utils = "0.2.0"
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
    model::prelude::Message,
    prelude::Context,
};
use serenity_utils::{
    builder::message::MessageBuilder,
    menu::{Menu, MenuOptions},
    Error,
};

async fn use_menu(ctx: &Context, msg: &Message) -> Result<(), Error> {
    let mut page_one = MessageBuilder::default();
    page_one
        .set_content("Page number one!")
        .set_embed_with(|e| {
            e.set_description("The first page!");

            e
        });

    let mut page_two = MessageBuilder::default();
    page_two
        .set_content("Page number two!")
        .set_embed_with(|e| {
            e.set_description("The second page!");

            e
        });

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

The following serenity_utils features are optional:

- **cache**: It is required to get `Member` from user name, tag or nickname when using the `Conversion` trait.
- **rustls_backend**: Uses `Rustls` for all platforms, a pure Rust implementation.
- **native_tls_backend**: Uses `SChannel` on Windows, `Secure Transport` on macOS, and `OpenSSL` on other platforms.

They enable serenity's features with the same names.

**Note:** One of `rustls_backend` and `native_tls_backend` must be used.

**cache** and **rustls_backend** are enabled by default.

You can specify features by adding this to your `Cargo.toml`:

```toml
[dependencies.serenity_utils]
version = "0.2.0"

# To disable default features.
default-features = false

features = ["select", "features]
# Example: features = ["cache", "native_tls_backend"]
```

## License

serenity_utils is available under the ISC license. See [LICENSE](LICENSE.md) for more details.

[serenity]: https://github.com/serenity-rs/serenity
[serenity_utils]: https://github.com/AriusX7/serenity-utils
[`examples`]: https://github.com/AriusX7/serenity-utils/tree/master/examples
[`serenity_utils`]: https://github.com/AriusX7/serenity-utils
[license badge]: https://img.shields.io/badge/license-ISC-00D00D.svg?style=for-the-badge
[license link]: https://github.com/AriusX7/serenity-utils/blob/master/LICENSE.md
[docs badge]: https://img.shields.io/badge/docs-online-8E3FFF.svg?style=for-the-badge
[docs link]: https://docs.rs/serenity_utils/
[crates.io link]: https://crates.io/crates/serenity_utils
[crates.io badge]: https://img.shields.io/crates/v/serenity_utils?color=00A1D0&label=crates.io&style=for-the-badge
[rust 1.39.0+ badge]: https://img.shields.io/badge/rust-1.39.0+-93450a.svg?style=for-the-badge
[rust 1.39.0+ link]: https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html
