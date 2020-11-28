//! Provides alternatives to serenity's embed builders.
//!
//! Unlike serenity's builders, the builders here use separate fields for all
//! values instead of a [`HashMap`]. This provides an easy way to access the
//! builder's fields.
//!
//! Due to the user-friendliness of these builders, they are slightly less
//! efficient than serenity's builders. You should only use these when you need
//! access to the embed's values which are set somewhere else.
//!
//! All builders provide trait implementations to convert them into serenity's
//! builders.
//!
//! ## Example
//!
//! ```
//! # use serenity_utils::builder::embed::EmbedBuilder;
//! #
//! let mut embed = EmbedBuilder::new();
//!
//! // Fields can set using setters.
//! embed.set_description("description").set_title("title");
//!
//! // Or by directly mutating the struct.
//! embed.description = Some("description".to_string());
//! embed.title = Some("title".to_string());
//! ```
//!
//! Other builders can be used in a similar fashion.
//!
//! [`HashMap`]: std::collections::HashMap

use serenity::{
    builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, Timestamp},
    model::channel::EmbedField,
    utils::Colour,
};

/// A struct to build the author portion of an embed.
///
/// It is meant to serve as an alternative to serenity's [`CreateEmbedAuthor`].
/// Unlike serenity's builder, this builder uses separate fields for all values
/// instead of a [`HashMap`]. This provides an easy way to access the builder's
/// fields.
///
/// All fields have setter methods like serenity's builder to allow you to pass
/// in a wide range of parameters/arguments.
///
/// The `name` field cannot be empty. All other fields are optional.
///
/// ## Example
///
/// ```
/// # use serenity_utils::builder::embed::EmbedAuthorBuilder;
/// #
/// let mut author = EmbedAuthorBuilder::new("name");
///
/// // Fields can set using the setter.
/// author.set_icon_url("icon_url");
///
/// // Or by directly mutating the struct.
/// author.icon_url = Some("icon_url".to_string());
/// ```
///
/// [`HashMap`]: std::collections::HashMap
#[derive(Clone, Debug)]
pub struct EmbedAuthorBuilder {
    /// The icon URL of the author. This only supports HTTP(S).
    pub icon_url: Option<String>,
    /// The name of the author.
    pub name: String,
    /// The URL of the author.
    pub url: Option<String>,
}

impl EmbedAuthorBuilder {
    /// Creates a new [`EmbedAuthorBuilder`] object.
    ///
    /// `name` must be specified when creating as it cannot be empty. Other
    /// fields are optional and be specified by directly mutating the struct
    /// or using one of the setters.
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            icon_url: None,
            name: name.to_string(),
            url: None,
        }
    }

    /// Sets the author's icon URL. This only supports HTTP(S).
    pub fn set_icon_url<S: ToString>(&mut self, icon_url: S) -> &mut Self {
        self.icon_url = Some(icon_url.to_string());

        self
    }

    /// Sets the author's name.
    pub fn set_name<S: ToString>(&mut self, name: S) -> &mut Self {
        self.name = name.to_string();

        self
    }

    /// Sets the author's URL.
    pub fn set_url<S: ToString>(&mut self, url: S) -> &mut Self {
        self.url = Some(url.to_string());

        self
    }

    /// Converts [`EmbedAuthorBuilder`] into serenity's [`CreateEmbedAuthor`].
    pub fn to_create_embed_author(&self) -> CreateEmbedAuthor {
        self.into()
    }
}

impl From<EmbedAuthorBuilder> for CreateEmbedAuthor {
    fn from(author_builder: EmbedAuthorBuilder) -> Self {
        let mut author = CreateEmbedAuthor::default();

        author.name(author_builder.name);

        if let Some(value) = author_builder.icon_url {
            author.icon_url(value);
        }

        if let Some(value) = author_builder.url {
            author.url(value);
        }

        author
    }
}

impl From<&EmbedAuthorBuilder> for CreateEmbedAuthor {
    fn from(author_builder: &EmbedAuthorBuilder) -> Self {
        let mut author = CreateEmbedAuthor::default();

        author.name(&author_builder.name);

        if let Some(value) = &author_builder.icon_url {
            author.icon_url(value);
        }

        if let Some(value) = &author_builder.url {
            author.url(value);
        }

        author
    }
}

/// A struct to build the footer portion of an embed.
///
/// It is meant to serve as an alternative to serenity's [`CreateEmbedFooter`].
/// Unlike serenity's builder, this builder uses separate fields for all values
/// instead of a [`HashMap`]. This provides an easy way to access the builder's
/// fields.
///
/// All fields have setter methods like serenity's builder to allow you to pass
/// in a wide range of parameters/arguments.
///
/// The `text` field cannot be empty. All other fields are optional.
///
/// ## Example
///
/// ```
/// # use serenity_utils::builder::embed::EmbedFooterBuilder;
/// #
/// let mut footer = EmbedFooterBuilder::new("text");
///
/// // Fields can set using the setter.
/// footer.set_icon_url("icon_url");
///
/// // Or by directly mutating the struct.
/// footer.icon_url = Some("icon_url".to_string());
/// ```
/// 
/// [`HashMap`]: std::collections::HashMap
#[derive(Clone, Debug)]
pub struct EmbedFooterBuilder {
    /// The icon url of the footer. This only supports HTTP(S).
    pub icon_url: Option<String>,
    /// The text of the footer.
    pub text: String,
}

impl EmbedFooterBuilder {
    /// Creates a new [`EmbedFooterBuilder`] object.
    ///
    /// `text` must be specified when creating as it cannot be empty. Other
    /// fields are optional and be specified by directly mutating the struct
    /// or using one of the setters.
    pub fn new<S: ToString>(text: S) -> Self {
        Self {
            icon_url: None,
            text: text.to_string(),
        }
    }

    /// Sets the footer's icon url. This only supports HTTP(S).
    pub fn set_icon_url<S: ToString>(&mut self, icon_url: S) -> &mut Self {
        self.icon_url = Some(icon_url.to_string());

        self
    }

    // Sets the footer's text.
    pub fn set_text<S: ToString>(&mut self, text: S) -> &mut Self {
        self.text = text.to_string();

        self
    }

    /// Converts [`EmbedFooterBuilder`] into serenity's [`CreateEmbedFooter`].
    pub fn to_create_embed_footer(&self) -> CreateEmbedFooter {
        self.into()
    }
}

impl From<EmbedFooterBuilder> for CreateEmbedFooter {
    fn from(footer_builder: EmbedFooterBuilder) -> Self {
        let mut footer = CreateEmbedFooter::default();

        footer.text(footer_builder.text);

        if let Some(value) = footer_builder.icon_url {
            footer.icon_url(value);
        }

        footer
    }
}

impl From<&EmbedFooterBuilder> for CreateEmbedFooter {
    fn from(footer_builder: &EmbedFooterBuilder) -> Self {
        let mut footer = CreateEmbedFooter::default();

        footer.text(&footer_builder.text);

        if let Some(value) = &footer_builder.icon_url {
            footer.icon_url(value);
        }

        footer
    }
}

/// A struct to build an embed field.
///
/// All fields have setter methods like serenity's builders to allow you to pass
/// in a wide range of parameters/arguments.
///
/// None of the fields are optional.
///
/// ## Example
///
/// ```
/// # use serenity_utils::builder::embed::EmbedFieldBuilder;
/// #
/// let mut field = EmbedFieldBuilder::new("name", "value", true);
///
/// # let inline = true;
/// // Fields can set using the setter.
/// field.set_inline(inline);
///
/// // Or by directly mutating the struct.
/// field.inline = inline;
/// ```
#[derive(Clone, Debug)]
pub struct EmbedFieldBuilder {
    /// Indicator of whether the field should display as inline.
    pub inline: bool,
    /// The name of the field.
    ///
    /// The maxiumum length of this field is 512 unicode codepoints.
    pub name: String,
    /// The value of the field.
    ///
    /// The maxiumum length of this field is 1024 unicode codepoints.
    pub value: String,
}

impl EmbedFieldBuilder {
    /// Creates a new [`EmbedFieldBuilder`] object.
    pub fn new<T: ToString, U: ToString>(name: T, value: U, inline: bool) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            inline,
        }
    }

    /// Sets the field's name.
    pub fn set_name<S: ToString>(&mut self, name: S) -> &mut Self {
        self.name = name.to_string();

        self
    }

    /// Sets the field's value.
    pub fn set_value<S: ToString>(&mut self, value: S) -> &mut Self {
        self.value = value.to_string();

        self
    }

    /// Sets the field's inline field.
    pub fn set_inline(&mut self, inline: bool) -> &mut Self {
        self.inline = inline;

        self
    }

    /// Adds the field to the given `CreateEmbed`.
    ///
    /// See [`EmbedBuilder::add_field`] method to add a field to [`EmbedBuilder`].
    ///
    /// [`EmbedBuilder::add_field`]: EmbedBuilder::add_field()
    pub fn insert_to(self, embed: &mut CreateEmbed) -> &mut CreateEmbed {
        embed.field(self.name, self.value, self.inline)
    }
}

impl From<EmbedFieldBuilder> for EmbedField {
    fn from(field: EmbedFieldBuilder) -> Self {
        EmbedField::new(field.name, field.value, field.inline)
    }
}

impl From<&EmbedFieldBuilder> for EmbedField {
    fn from(field: &EmbedFieldBuilder) -> Self {
        EmbedField::new(&field.name, &field.value, field.inline)
    }
}

/// A struct to build an embed.
///
/// It is meant to serve as an alternative to serenity's [`CreateEmbed`].
/// Unlike serenity's builder, this builder uses separate fields for all values
/// instead of a [`HashMap`]. This provides an easy way to access the builder's
/// fields.
///
/// All fields have setter methods like serenity's builder to allow you to pass
/// in a wide range of parameters/arguments.
///
/// ## Example
///
/// ```
/// # use serenity_utils::builder::embed::EmbedBuilder;
/// #
/// let mut embed = EmbedBuilder::new();
///
/// // Fields can set using setters.
/// embed.set_description("description").set_title("title");
///
/// // Or by directly mutating the struct.
/// embed.description = Some("description".to_string());
/// embed.title = Some("title".to_string());
/// ```
///
/// [`HashMap`]: std::collections::HashMap
#[derive(Clone, Debug, Default)]
pub struct EmbedBuilder {
    /// The author of the embed.
    pub author: Option<EmbedAuthorBuilder>,
    /// The colour of the embed.
    pub colour: Option<Colour>,
    /// The description of the embed.
    ///
    /// It can't be longer than 2048 characters.
    pub description: Option<String>,
    /// The fields of the embed.
    pub fields: Vec<EmbedFieldBuilder>,
    /// The footer of the embed.
    pub footer: Option<EmbedFooterBuilder>,
    /// The image of the embed.
    ///
    /// This only supports HTTP(S).
    pub image: Option<String>,
    /// The thumbnail of the embed.
    ///
    /// This only supports HTTP(S).
    pub thumbnail: Option<String>,
    /// The timestamp of the embed.
    pub timestamp: Option<Timestamp>,
    /// The title of the embed.
    pub title: Option<String>,
    /// The title url of the embed.
    pub url: Option<String>,
    /// The attachment of the embed.
    pub attachment: Option<String>,
}

impl EmbedBuilder {
    /// Creates an empty [`EmbedBuilder`] object.
    ///
    /// All fields are set to [`None`] or empty vectors. They can be changed by
    /// mutating the struct directly or by using the setter methods.
    pub fn new() -> Self {
        Self::default()
    }

    /// Same as calling [`set_image`] with "attachment://filename.(jpg, png)".
    ///
    /// Note however, you have to be sure you set an attachment (with serenity's
    /// `ChannelId::send_files`) with the provided filename or else this won't
    /// work.
    ///
    /// [`set_image`]: EmbedBuilder::set_image()
    pub fn set_attachment<S: ToString>(&mut self, filename: S) -> &mut Self {
        let mut filename = filename.to_string();
        filename.insert_str(0, "attachment://");

        self.attachment = Some(filename);

        self
    }

    /// Sets the embed's author.
    pub fn set_author(&mut self, author: EmbedAuthorBuilder) -> &mut Self {
        self.author = Some(author);

        self
    }

    /// Sets the embed's author using the specified closure.
    ///
    /// It allows you to set the author without unnecessarily importing
    /// [`EmbedAuthorBuilder`]. Note that it sets the `name` field of the
    /// author to "name", so you need to, at least, update that in the closure.
    ///
    /// ## Example
    ///
    /// ```
    /// # use serenity_utils::builder::embed::EmbedBuilder;
    /// #
    /// let mut embed = EmbedBuilder::new();
    /// embed.set_author_with(|a| {
    ///     a.set_name("name");
    ///     a.set_url("url");
    ///
    ///     a
    /// });
    /// ```
    ///
    pub fn set_author_with<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedAuthorBuilder) -> &mut EmbedAuthorBuilder,
    {
        let mut author = EmbedAuthorBuilder::new("name");
        f(&mut author);

        self.set_author(author)
    }

    /// Sets the embed's colour.
    pub fn set_colour<C: Into<Colour>>(&mut self, colour: C) -> &mut Self {
        self.colour = Some(colour.into());

        self
    }

    /// Sets the embed's description.
    ///
    /// It can't be longer than 2048 characters.
    pub fn set_description<S: ToString>(&mut self, description: S) -> &mut Self {
        self.description = Some(description.to_string());

        self
    }

    /// Adds a field to the embed.
    ///
    /// The name of a field can contain 256 characters at most. The value can
    /// contain 1024 characters at most.
    pub fn add_field<T, U>(&mut self, field: (T, U, bool)) -> &mut Self
    where
        T: ToString,
        U: ToString,
    {
        self.fields
            .push(EmbedFieldBuilder::new(field.0, field.1, field.2));

        self
    }

    /// Adds multiple fields to the embed.
    ///
    /// The name of a field can contain 256 characters at most. The value can
    /// contain 1024 characters at most.
    pub fn add_fields<T, U, It>(&mut self, fields: It) -> &mut Self
    where
        It: IntoIterator<Item = (T, U, bool)>,
        T: ToString,
        U: ToString,
    {
        let mut fields = fields
            .into_iter()
            .map(|(n, v, b)| EmbedFieldBuilder::new(n, v, b))
            .collect::<Vec<_>>();

        self.fields.append(&mut fields);

        self
    }

    /// Sets field at position `index`, if it is within bounds.
    pub fn set_field_at(&mut self, index: usize, field: EmbedFieldBuilder) -> &mut Self {
        if self.fields.len() - 1 > index {
            self.fields[index] = field;
        }

        self
    }

    /// Sets the embed's footer.
    pub fn set_footer(&mut self, footer: EmbedFooterBuilder) -> &mut Self {
        self.footer = Some(footer);

        self
    }

    /// Sets the embed's footer using the specified closure.
    ///
    /// It allows you to set the footer without unnecessarily importing
    /// [`EmbedFooterBuilder`]. Note that it sets the `text` field of the
    /// footer to "text", so you need to, at least, update that in the closure.
    ///
    /// ## Example
    ///
    /// ```
    /// # use serenity_utils::builder::embed::EmbedBuilder;
    /// #
    /// let mut embed = EmbedBuilder::new();
    /// embed.set_footer_with(|f| {
    ///     f.set_text("text");
    ///
    ///     f
    /// });
    /// ```
    ///
    pub fn set_footer_with<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedFooterBuilder) -> &mut EmbedFooterBuilder,
    {
        let mut footer = EmbedFooterBuilder::new("text");
        f(&mut footer);

        self.set_footer(footer)
    }

    /// Sets the embed's image. This only supports HTTP(S).
    pub fn set_image<S: ToString>(&mut self, url: S) -> &mut Self {
        self.image = Some(url.to_string());

        self
    }

    /// Sets the embed's thumbnail. This only supports HTTP(S).
    pub fn set_thumbnail<S: ToString>(&mut self, url: S) -> &mut Self {
        self.thumbnail = Some(url.to_string());

        self
    }

    /// Sets the embed's timestamp.
    pub fn set_timestamp<T: Into<Timestamp>>(&mut self, timestamp: T) -> &mut Self {
        self.timestamp = Some(timestamp.into());

        self
    }

    /// Sets the embed's title.
    pub fn set_title<S: ToString>(&mut self, title: S) -> &mut Self {
        self.title = Some(title.to_string());

        self
    }

    /// Sets the embed's title url.
    pub fn set_url<S: ToString>(&mut self, url: S) -> &mut Self {
        self.url = Some(url.to_string());

        self
    }

    /// Converts [`EmbedBuilder`] into serenity's [`CreateEmbed`].
    pub fn to_create_embed(&self) -> CreateEmbed {
        self.into()
    }
}

impl From<EmbedBuilder> for CreateEmbed {
    fn from(embed_builder: EmbedBuilder) -> Self {
        let mut embed = CreateEmbed::default();

        if let Some(author) = embed_builder.author {
            embed.author(|f| {
                f.0 = author.to_create_embed_author().0;

                f
            });
        }

        if let Some(colour) = embed_builder.colour {
            embed.colour(colour);
        }

        if let Some(description) = embed_builder.description {
            embed.description(description);
        }

        for field in embed_builder.fields {
            embed.field(field.name, field.value, field.inline);
        }

        if let Some(footer) = embed_builder.footer {
            embed.footer(|f| {
                f.0 = footer.to_create_embed_footer().0;

                f
            });
        }

        if let Some(image) = embed_builder.image {
            embed.image(image);
        }

        if let Some(thumbnail) = embed_builder.thumbnail {
            embed.thumbnail(thumbnail);
        }

        if let Some(timestamp) = embed_builder.timestamp {
            embed.timestamp(timestamp);
        }

        if let Some(title) = embed_builder.title {
            embed.title(title);
        }

        if let Some(url) = embed_builder.url {
            embed.url(url);
        }

        if let Some(attachment) = embed_builder.attachment {
            embed.attachment(attachment);
        }

        embed
    }
}

impl From<&EmbedBuilder> for CreateEmbed {
    fn from(embed_builder: &EmbedBuilder) -> Self {
        let mut embed = CreateEmbed::default();

        if let Some(author) = &embed_builder.author {
            embed.author(|f| {
                f.0 = author.to_create_embed_author().0;

                f
            });
        }

        if let Some(colour) = embed_builder.colour {
            embed.colour(colour);
        }

        if let Some(description) = &embed_builder.description {
            embed.description(description);
        }

        for field in &embed_builder.fields {
            embed.field(&field.name, &field.value, field.inline);
        }

        if let Some(footer) = &embed_builder.footer {
            embed.footer(|f| {
                f.0 = footer.to_create_embed_footer().0;

                f
            });
        }

        if let Some(image) = &embed_builder.image {
            embed.image(image);
        }

        if let Some(thumbnail) = &embed_builder.thumbnail {
            embed.thumbnail(thumbnail);
        }

        if let Some(timestamp) = &embed_builder.timestamp {
            embed.timestamp(timestamp.clone());
        }

        if let Some(title) = &embed_builder.title {
            embed.title(title);
        }

        if let Some(url) = &embed_builder.url {
            embed.url(url);
        }

        if let Some(attachment) = &embed_builder.attachment {
            embed.attachment(attachment);
        }

        embed
    }
}
