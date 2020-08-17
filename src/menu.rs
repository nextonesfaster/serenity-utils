//! Reaction-based menu functionality.
//!
//! It provides three default menu control functions that:
//! - move to previous page
//! - move to next page
//! - close menu
//!
//! These functions are exposed publicly to allow customisation.
//! Custom control functions can also be used with a menu.
//! For more information, see [`Menu`].
//!
//! **Note:** This functionality has been ported from [`Red-DiscordBot`]'s
//! [`menu`] function.
//!
//! [`Menu`]: struct.Menu.html
//! [`Red-DiscordBot`]: https://github.com/Cog-Creators/Red-DiscordBot/
//! [`menu`]: https://github.com/Cog-Creators/Red-DiscordBot/blob/46eb9ce7a0bcded991af02665fec39fcb542c76d/redbot/core/utils/menus.py#L17

use crate::Error;
use serenity::{
    builder::CreateEmbed,
    collector::ReactionAction,
    futures::StreamExt,
    model::prelude::{Message, Reaction, ReactionType},
    prelude::Context,
};
use std::{future::Future, pin::Pin, sync::Arc, time::Duration};

/// Result variant for menu methods.
pub type MenuResult = Result<(), Error>;

/// A fully functioning reaction-based menu.
///
/// A reaction menu is a paginated message where the user can use reactions to
/// change the page/content of the message. Currently, only embeds are supported.
///
/// ## Example
///
/// ```
/// # use serenity::{
/// #     builder::CreateEmbed,
/// #     model::prelude::Message,
/// #     prelude::Context,
/// # };
/// use serenity_utils::{menu::{Menu, MenuOptions}, Error};
///
/// async fn use_menu(ctx: &Context, msg: &Message) -> Result<(), Error> {
///     let mut page_one = CreateEmbed::default();
///     page_one.description("Page number one!");
///
///     let mut page_two = CreateEmbed::default();
///     page_two.description("Page number two!");
///
///     let pages = [page_one, page_two];
///
///     // Creates a new menu.
///     let mut menu = Menu::new(ctx, msg, &pages, MenuOptions::default());
///
///     // Runs the menu and returns optional `Message` used to display the menu.
///     let opt_message = menu.run().await?;
///
///     Ok(())
/// }
/// ```
///
/// A reaction menu can be configured by changing its options. See
/// [`MenuOptions`] for more details.
///
/// [`MenuOptions`]: struct.MenuOptions.html
pub struct Menu<'a> {
    /// The Discord/serenity context.
    pub ctx: &'a Context,
    /// The invocation message.
    pub msg: &'a Message,
    /// The embeds of the menu.
    pub pages: &'a [CreateEmbed],
    /// The menu options.
    pub options: MenuOptions<'a>,
}

impl<'a> Menu<'a> {
    /// Creates a new [`Menu`](struct.Menu.html) object.
    pub fn new(
        ctx: &'a Context,
        msg: &'a Message,
        pages: &'a [CreateEmbed],
        options: MenuOptions<'a>,
    ) -> Self {
        Self {
            ctx,
            msg,
            pages,
            options,
        }
    }

    /// Runs the reaction menu.
    ///
    /// It returns the message used to display the reaction menu after running.
    ///
    /// ## Errors
    /// Returns [`Error`] in these scenarios:
    /// - Current user/bot doesn't have the permissions to add reactions.
    /// - `msg` is specified in [`MenuOptions`] but the current user/bot isn't
    ///     the author of the message.
    /// - The embed content lengths are over Discord's limit.
    /// - Current user/bot doesn't have the permissions to send an embed.
    /// - If `pages` is empty.
    /// - If the page number specified in [`MenuOptions`] is out of bounds.
    ///
    /// [`Error`]: .../struct.Error.html
    /// [`MenuOptions`]: struct.MenuOptions.html
    pub async fn run(&mut self) -> Result<Option<Message>, Error> {
        loop {
            match self.work().await {
                Ok((index, reaction)) => match &self.options.controls.get(index) {
                    Some(control) => {
                        Arc::clone(&control.function)(self, reaction).await;
                    }
                    None => {
                        // We don't have to return an error for this as bot won't
                        // have permission to remove reactions in all cases. This
                        // is simply an inconvenience for the user.
                        let _ = self.clean_reactions().await;
                    }
                },
                Err(_) => {
                    self.clean_reactions().await?;
                    break;
                }
            }
        }

        Ok(self.options.message.clone())
    }

    async fn work(&mut self) -> Result<(usize, Reaction), Error> {
        if self.pages.is_empty() {
            return Err(Error::from("`pages` is empty."));
        }

        if self.options.page > self.pages.len() - 1 {
            return Err(Error::from("`page` is out of bounds."));
        }

        let page = &self.pages[self.options.page];
        match &self.options.message {
            Some(m) => {
                m.clone()
                    .edit(&self.ctx.http, |m| {
                        m.embed(|e| {
                            e.clone_from(page);

                            e
                        })
                    })
                    .await?;
            }
            None => {
                let msg = self
                    .msg
                    .channel_id
                    .send_message(&self.ctx.http, |m| {
                        m.embed(|e| {
                            e.clone_from(page);

                            e
                        })
                    })
                    .await?;

                self.add_reactions(&msg).await?;

                self.options.message = Some(msg);
            }
        }

        let message = self.options.message.as_ref().unwrap();
        let mut reaction_collector = message
            .await_reactions(&self.ctx)
            .timeout(Duration::from_secs_f64(self.options.timeout))
            .author_id(self.msg.author.id)
            .await;

        let (choice, reaction) = {
            let mut choice = None;
            let mut reaction = None;

            while let Some(item) = reaction_collector.next().await {
                if let ReactionAction::Added(r) = item.as_ref() {
                    let r = r.as_ref().clone();
                    if let Some(i) = self.process_reaction(&r) {
                        choice = Some(i);
                        reaction = Some(r);
                        break;
                    }
                }
            }
            (choice, reaction)
        };

        match choice {
            Some(c) => Ok((c, reaction.unwrap())),
            None => Err(Error::from("Invalid choice, closing menu.")),
        }
    }

    async fn add_reactions(&self, msg: &Message) -> MenuResult {
        for control in &self.options.controls {
            msg.react(&self.ctx.http, control.emoji.clone()).await?;
        }

        Ok(())
    }

    fn process_reaction(&self, reaction: &Reaction) -> Option<usize> {
        let emoji = &reaction.emoji;

        for (idx, control) in self.options.controls.iter().enumerate() {
            if &control.emoji == emoji {
                return Some(idx);
            }
        }

        None
    }

    async fn clean_reactions(&self) -> MenuResult {
        if let Some(msg) = &self.options.message {
            msg.delete_reactions(&self.ctx.http).await?;
        }

        Ok(())
    }
}

/// Options to tweak a menu.
///
/// Default menu options are as follows:
/// - page: 0
/// - timeout: 30.0,
/// - message: None,
/// - controls: The default controls are:
///     - ◀️ -> [`prev_page`]
///     - ❌ -> [`close_menu`]
///     - ▶️ -> [`next_page`]
///
/// See [`Control`] for details to implement your own controls.
///
/// [`prev_page`]: fn.prev_page.html
/// [`close_menu`]: fn.close_menu.html
/// [`next_page`]: fn.next_page.html
/// [`Control`]: struct.Control.html
pub struct MenuOptions<'a> {
    /// The 0-indexed page number to start at.
    pub page: usize,
    /// Number of seconds to keep the menu active.
    pub timeout: f64,
    /// Optional message to edit.
    ///
    /// If supplied, this message is edited instead of the bot creating a new
    /// message to display the menu. This message must be sent by the bot.
    pub message: Option<Message>,
    /// The controls for the menu.
    pub controls: Vec<Control<'a>>,
}

impl<'a> MenuOptions<'a> {
    /// Creates a new [`MenuOptions`](struct.MenuOptions.html) object.
    pub fn new(
        page: usize,
        timeout: f64,
        message: Option<Message>,
        controls: Vec<Control<'a>>,
    ) -> Self {
        Self {
            page,
            timeout,
            message,
            controls,
        }
    }
}

impl<'a> Default for MenuOptions<'a> {
    fn default() -> Self {
        let controls = vec![
            Control::new('◀'.into(), Arc::new(|m, r| Box::pin(prev_page(m, r)))),
            Control::new('❌'.into(), Arc::new(|m, r| Box::pin(close_menu(m, r)))),
            Control::new('▶'.into(), Arc::new(|m, r| Box::pin(next_page(m, r)))),
        ];

        Self {
            page: 0,
            timeout: 30.0,
            message: None,
            controls,
        }
    }
}

/// A struct representing a control for reaction menus.
///
/// Each control must have a unique emoji and a function to control it's
/// behaviour. See [`ControlFunction`]'s documentation to learn more about how
/// they are implemented.
///
/// [`ControlFunction`]: type.ControlFunction.html
pub struct Control<'a> {
    /// The emoji for the control.
    pub emoji: ReactionType,
    /// The [`ControlFunction`](type.ControlFunction.html) to control the behaviour.
    pub function: ControlFunction<'a>,
}

impl<'a> Control<'a> {
    /// Creates a new [`Control`](struct.Control.html) object.
    pub fn new(emoji: ReactionType, function: ControlFunction<'a>) -> Self {
        Self { emoji, function }
    }
}

/// A function used to control the behaviour of a reaction menu's reaction.
///
/// An example implementation is provided here:
///
/// ```
/// use serenity::model::channel::Reaction;
/// use serenity_utils::menu::Menu;
///
/// async fn first_page<'a>(menu: &mut Menu<'a>, reaction: Reaction) {
///     // Remove the reaction used to change the menu.
///     let _ = &reaction.delete(&menu.ctx.http).await;
///
///     // Set page number to `0`.
///     menu.options.page = 0;
/// }
/// ```
///
/// Please note that the above function is not a [`ControlFunction`]. To make it
/// a control function, you need to pin it and then create an `Arc` of it.
///
/// ```
/// # use serenity::model::channel::Reaction;
/// # use serenity_utils::menu::Menu;
/// #
/// # async fn first_page<'a>(menu: &mut Menu<'a>, reaction: Reaction) {}
/// #
/// use std::sync::Arc;
///
/// let control_function = Arc::new(|m, r| Box::pin(first_page(m, r)));
/// ```
///
/// Now, `control_function` can be used to control a menu.
///
/// [`ControlFunction`]: type.ControlFunction.html
pub type ControlFunction<'a> = Arc<
    dyn for<'b> Fn(&'b mut Menu<'a>, Reaction) -> Pin<Box<dyn Future<Output = ()> + 'b + Send>>
        + Sync
        + Send,
>;

/// Moves a reaction menu forward.
///
/// **Note:** This function is not a [`ControlFunction`]. To turn it into a
/// control function, it must pin it and then create an `Arc` of it.
///
/// ```
/// # use serenity_utils::menu::next_page;
/// # use std::sync::Arc;
/// #
/// let next_page_cfn = Arc::new(|m, r| Box::pin(next_page(m, r)));
/// ```
///
/// `next_page_cfn` is a [`ControlFunction`] and can be used to control a menu.
///
/// [`ControlFunction`]: type.ControlFunction.html
pub async fn next_page<'a>(menu: &mut Menu<'a>, reaction: Reaction) {
    let _ = &reaction.delete(&menu.ctx.http).await;

    if menu.options.page == menu.pages.len() - 1 {
        menu.options.page = 0;
    } else {
        menu.options.page += 1;
    }
}

/// Moves a reaction menu backward.
///
/// **Note:** This function is not a [`ControlFunction`]. To turn it into a
/// control function, it must pin it and then create an `Arc` of it.
///
/// ```
/// # use serenity_utils::menu::prev_page;
/// # use std::sync::Arc;
/// #
/// let prev_page_cfn = Arc::new(|m, r| Box::pin(prev_page(m, r)));
/// ```
///
/// `prev_page_cfn` is a [`ControlFunction`] and can be used to control a menu.
///
/// [`ControlFunction`]: type.ControlFunction.html
pub async fn prev_page<'a>(menu: &mut Menu<'a>, reaction: Reaction) {
    let _ = reaction.delete(&menu.ctx.http).await;

    if menu.options.page == 0 {
        menu.options.page = menu.pages.len() - 1;
    } else {
        menu.options.page -= 1;
    }
}

/// Closes a reaction menu by deleting the menu's message.
///
/// **Note:** This function is not a [`ControlFunction`]. To turn it into a
/// control function, it must pin it and then create an `Arc` of it.
///
/// ```
/// # use serenity_utils::menu::close_menu;
/// # use std::sync::Arc;
/// #
/// let close_menu_cfn = Arc::new(|m, r| Box::pin(close_menu(m, r)));
/// ```
///
/// `close_menu_cfn` is a [`ControlFunction`] and can be used to control a menu.
///
/// [`ControlFunction`]: type.ControlFunction.html
pub async fn close_menu<'a>(menu: &mut Menu<'a>, _reaction: Reaction) {
    let _ = menu
        .options
        .message
        .as_ref()
        .unwrap()
        .delete(&menu.ctx.http)
        .await;
}
