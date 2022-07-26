# serenity-utils

[![docs badge][]][docs link] [![crates.io badge][]][crates.io link] [![license badge][]][license link] [![rust 1.53.0+ badge]][rust 1.53.0+ link]

A library to provide conversions, prompts and menu functionality for
Discord bots created with [serenity].

This library provides implementations to easily:

- Get user response using message or reaction prompts.
- Display paginated reaction-based messages/menus.
- Format text in different ways before sending.
- ~~Convert a string to [serenity]'s guild-specific models.~~ (deprecated; use serenity's `ArgumentConvert` trait instead)
- ~~Create embeds and messages with field access.~~ (deprecated; use serenity's builder directly)

## Installation and Usage

To use this crate, add the following to your `Cargo.toml`:

```toml
[dependencies]
serenity_utils = "0.7.0"
```

**Note:** This crate only supports [serenity]'s async versions and a minimum of Rust 1.53 (consistent with the latest serenity version).

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
    builder::CreateMessage,
    model::prelude::Message,
    prelude::Context,
};
use serenity_utils::{
    menu::{Menu, MenuOptions},
    Error,
};

async fn use_menu(ctx: &Context, msg: &Message) -> Result<(), Error> {
    let mut page_one = CreateMessage::default();
    page_one
        .content("Page number one!")
        .embed(|e| {
            e.description("The first page!");

            e
        });

    let mut page_two = CreateMessage::default();
    page_two
        .content("Page number two!")
        .embed(|e| {
            e.description("The second page!");

            e
        });

    let pages = [page_one, page_two];

    // Creates a new menu.
    let menu = Menu::new(ctx, msg, &pages, MenuOptions::default());

    // Runs the menu and returns optional `Message` used to display the menu.
    let opt_message = menu.run().await?;

    Ok(())
}
```

### More Examples

More examples detailing the crate's functionality can be found in the [`examples`] directory.

## Features

Some functionality of this crate is dependent on [serenity]'s features.

The following [serenity] features are required when using [`serenity_utils`]:

- **client**
- **collector**
- **gateway**
- **model**

The following [`serenity_utils`] features are optional:

- **cache**: Enables [serenity]'s `cache` feature. It is required to get `Member` from user name, tag or nickname when using the `Conversion` trait.
- **rustls_backend**: Uses `Rustls` for all platforms, a pure Rust implementation.
- **native_tls_backend**: Uses `SChannel` on Windows, `Secure Transport` on macOS, and `OpenSSL` on other platforms.

**cache** and **rustls_backend** are enabled by default.

- **default_native_tls**: Enables default [`serenity_utils`] features with `native_tls_backend`.

They enable [serenity]'s features with the same names.

**Note:** One of `rustls_backend` and `native_tls_backend` must be used.

You can specify features by adding this to your `Cargo.toml`:

```toml
[dependencies.serenity_utils]
version = "0.7.0"

# To disable default features.
default-features = false

# Choose features you need.
features = ["cache", "native_tls_backend"]
```

## License

[`serenity_utils`] is available under the ISC license. See [LICENSE](LICENSE.md) for more details.

[serenity]: https://github.com/serenity-rs/serenity
[serenity_utils]: https://github.com/AriusX7/serenity-utils
[`examples`]: https://github.com/AriusX7/serenity-utils/tree/current/examples
[`serenity_utils`]: https://github.com/AriusX7/serenity-utils
[license badge]: https://img.shields.io/badge/license-ISC-00D00D.svg?style=for-the-badge
[license link]: https://github.com/AriusX7/serenity-utils/blob/master/LICENSE.md
[docs badge]: https://img.shields.io/badge/docs-online-8E3FFF.svg?style=for-the-badge
[docs link]: https://docs.rs/serenity_utils/
[crates.io link]: https://crates.io/crates/serenity_utils
[crates.io badge]: https://img.shields.io/crates/v/serenity_utils?color=00A1D0&label=crates.io&style=for-the-badge
[rust 1.53.0+ badge]: https://img.shields.io/badge/rust-1.53.0+-93450a.svg?style=for-the-badge
[rust 1.53.0+ link]: https://blog.rust-lang.org/2021/06/17/Rust-1.53.0.html
