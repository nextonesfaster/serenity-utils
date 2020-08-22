//! Provides alternative to serenity's message builder.
//!
//! Unlike serenity's builder, the builder here uses separate fields for all
//! values instead of a `HashMap`. This provides an easy way to access the
//! builder's fields.
//!
//! Due to the user-friendliness of this builder, it is slightly less efficient
//! than serenity's builders. You should only use these when you need access to
//! the message's values which are set somewhere else.
//!
//! The builder provides trait implementations to convert it into serenity's
//! builder.
//!
//! ## Example
//!
//! ```
//! # use serenity_utils::builder::message::MessageBuilder;
//! #
//! let mut message = MessageBuilder::new();
//!
//! // Fields can set using setters.
//! message.set_content("content").set_tts(true);
//!
//! // Or by directly mutating the struct.
//! message.content = Some("content".to_string());
//! message.tts = true;
//! ```
//!
//! Other builders can be used in a similar fashion.

use super::embed::EmbedBuilder;
use serenity::{
    builder::{CreateMessage, EditMessage},
    http::AttachmentType,
    model::channel::ReactionType,
};

/// A struct to build a message.
///
/// It is meant to serve as an alternative to serenity's `CreateMessage`.
/// Unlike serenity's builder, this builder uses separate fields for all values
/// instead of a `HashMap`. This provides an easy way to access the builder's
/// fields.
///
/// All fields have setter methods like serenity's builder to allow you to pass
/// in a wide range of parameters/arguments.
///
/// ## Example
///
/// ```
/// # use serenity_utils::builder::message::MessageBuilder;
/// #
/// let mut message = MessageBuilder::new();
///
/// // Fields can set using setters.
/// message.set_content("content").set_tts(true);
///
/// // Or by directly mutating the struct.
/// message.content = Some("content".to_string());
/// message.tts = true;
/// ```
#[derive(Clone, Debug, Default)]
pub struct MessageBuilder<'a> {
    /// The content of the message.
    pub content: Option<String>,
    /// The embed of the message.
    pub embed: Option<EmbedBuilder>,
    /// The files attached with the message.
    pub files: Vec<AttachmentType<'a>>,
    /// The reactions to add after the message is sent.
    pub reactions: Vec<ReactionType>,
    /// Indicator whether to set this message as text-to-speech.
    ///
    /// Defaults to `false`.
    pub tts: bool,
}

impl<'a> MessageBuilder<'a> {
    /// Creates an empty [`MessageBuilder`] object.
    ///
    /// All fields are set to `None` or empty vectors. They can be changed by
    /// mutating the struct directly or by using the setter methods.
    ///
    /// [`MessageBuilder`]: struct.MessageBuilder.html
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the message's content.
    pub fn set_content<S: ToString>(&mut self, content: S) -> &mut Self {
        self.content = Some(content.to_string());

        self
    }

    /// Sets the message's embed.
    pub fn set_embed(&mut self, embed: EmbedBuilder) -> &mut Self {
        self.embed = Some(embed);

        self
    }

    /// Sets the message's embed using the specified closure.
    ///
    /// It allows you to set the embed without importing [`EmbedBuilder`].
    ///
    /// ## Example
    ///
    /// ```
    /// # use serenity_utils::builder::message::MessageBuilder;
    /// #
    /// let mut message = MessageBuilder::new();
    /// message.set_embed_with(|e| {
    ///     e.set_description("description");
    ///     e.set_author_with(|a| {
    ///         a.set_name("name");
    ///
    ///         a
    ///     });
    ///
    ///     e
    /// });
    /// ```
    ///
    /// [`EmbedBuilder`]: ../embed/struct.EmbedBuilder.html
    pub fn set_embed_with<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedBuilder) -> &mut EmbedBuilder,
    {
        let mut embed = EmbedBuilder::default();
        f(&mut embed);

        self.set_embed(embed)
    }

    /// Adds a file to include in the message.
    ///
    /// It does not overwrite previously set files.
    pub fn add_file<F>(&mut self, file: F) -> &mut Self
    where
        F: Into<AttachmentType<'a>>,
    {
        self.files.push(file.into());

        self
    }

    /// Adds multiple files to include in the message.
    ///
    /// It does not overwrite previously set files.
    pub fn add_files<T, It>(&mut self, files: It) -> &mut Self
    where
        T: Into<AttachmentType<'a>>,
        It: IntoIterator<Item = T>,
    {
        self.files
            .extend(files.into_iter().map(|f| f.into()).collect::<Vec<_>>());

        self
    }

    /// Sets files to include in the message.
    ///
    /// It overwrites previously set files.
    pub fn set_files<T, It>(&mut self, files: It) -> &mut Self
    where
        T: Into<AttachmentType<'a>>,
        It: IntoIterator<Item = T>,
    {
        self.files = files.into_iter().map(|f| f.into()).collect();

        self
    }

    /// Adds a reaction which will be added after message is sent.
    ///
    /// It does not overwrite previously set reactions.
    pub fn add_reaction<R>(&mut self, reaction: R) -> &mut Self
    where
        R: Into<ReactionType>,
    {
        self.reactions.push(reaction.into());

        self
    }

    /// Adds list of reactions which will be added after message is sent.
    ///
    /// It does not overwrite previously set reactions.
    pub fn add_reactions<R, It>(&mut self, reactions: It) -> &mut Self
    where
        R: Into<ReactionType>,
        It: IntoIterator<Item = R>,
    {
        self.reactions
            .extend(reactions.into_iter().map(|r| r.into()).collect::<Vec<_>>());

        self
    }

    /// Sets list of reactions which will be added after message is sent.
    ///
    /// It overwrites previously set reactions.
    pub fn set_reactions<R, It>(&mut self, reactions: It) -> &mut Self
    where
        R: Into<ReactionType>,
        It: IntoIterator<Item = R>,
    {
        self.reactions = reactions.into_iter().map(|r| r.into()).collect();

        self
    }

    /// Sets whether the message is text-to-speech.
    ///
    /// Defaults to `false`.
    pub fn set_tts(&mut self, tts: bool) -> &mut Self {
        self.tts = tts;

        self
    }

    /// Converts [`MessageBuilder`] into serenity's `CreateMessage`.
    ///
    /// [`MessageBuilder`]: struct.MessageBuilder.html
    pub fn to_create_message(&self) -> CreateMessage {
        self.into()
    }

    /// Converts [`MessageBuilder`] into serenity's `EditMessage`.
    ///
    /// The resultant `EditMessage` only has content and embed — all other
    /// fields are ignored.
    ///
    /// [`MessageBuilder`]: struct.MessageBuilder.html
    pub fn to_edit_message(&self) -> EditMessage {
        self.into()
    }
}

impl<'a> From<MessageBuilder<'a>> for CreateMessage<'a> {
    fn from(message_builder: MessageBuilder<'a>) -> Self {
        let mut message = CreateMessage::default();

        if let Some(content) = message_builder.content {
            message.content(content);
        }

        if let Some(embed) = message_builder.embed {
            message.embed(|e| {
                e.0 = embed.to_create_embed().0;

                e
            });
        }

        message.files(message_builder.files);

        message.reactions(message_builder.reactions);

        message.tts(message_builder.tts);

        message
    }
}

impl<'a> From<&MessageBuilder<'a>> for CreateMessage<'a> {
    fn from(message_builder: &MessageBuilder<'a>) -> Self {
        let mut message = CreateMessage::default();

        if let Some(content) = &message_builder.content {
            message.content(content);
        }

        if let Some(embed) = &message_builder.embed {
            message.embed(|e| {
                e.0 = embed.to_create_embed().0;

                e
            });
        }

        message.files(message_builder.files.clone());

        message.reactions(message_builder.reactions.clone());

        message.tts(message_builder.tts);

        message
    }
}

impl<'a> From<MessageBuilder<'a>> for EditMessage {
    fn from(message_builder: MessageBuilder<'a>) -> Self {
        let mut message = EditMessage::default();

        if let Some(content) = message_builder.content {
            message.content(content);
        }

        if let Some(embed) = message_builder.embed {
            message.embed(|e| {
                e.0 = embed.to_create_embed().0;

                e
            });
        }

        message
    }
}

impl<'a> From<&MessageBuilder<'a>> for EditMessage {
    fn from(message_builder: &MessageBuilder<'a>) -> Self {
        let mut message = EditMessage::default();

        if let Some(content) = &message_builder.content {
            message.content(content);
        }

        if let Some(embed) = &message_builder.embed {
            message.embed(|e| {
                e.0 = embed.to_create_embed().0;

                e
            });
        }

        message
    }
}
