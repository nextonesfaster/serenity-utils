//! Functions to format text before sending them to the user.
//!
//! The functions defined in this module do not require any features to be enabled.

use serenity::model::channel::AttachmentType;
use std::{
    borrow::Cow,
    fmt::{Display, Write},
};

/// A struct to set [`pagify`]'s options.
///
/// The default options are:
/// - demils: &\["\n", " "\]
/// - escape_mass_mentions: true
/// - shorten_by: 0
/// - page_length: 2000
/// - priority: false
///
/// The easiest way to build [`PagifyOptions`] is to use the builder-pattern:
///
/// ```
/// # use serenity_utils::formatting::PagifyOptions;
/// #
/// let mut options = PagifyOptions::default();
///
/// // Change fields that you want to edit.
/// options.escape_mass_mentions(false).shorten_by(8);
/// ```
pub struct PagifyOptions<'a> {
    /// Characters at which pages are broken. If set to an empty slice, pages
    /// are broken at `page_length`.
    ///
    /// Defaults to `&["\n", " "]`.
    pub delims: &'a [&'a str],
    /// Whether to escape mass mentions ("@everyone" and "@here"). Set to `true`
    /// by default.
    pub escape_mass_mentions: bool,
    /// Length to shorten each page by. Defaults to `0`.
    pub shorten_by: usize,
    /// Maximum length of each page. Defaults to `2000`.
    pub page_length: usize,
    /// If set to `true`, pages are broken based on the the order of delimiters.
    /// Delimiters appearing before are given priority. Otherwise, pages are
    /// broken at the last possible delimiter.
    ///
    /// It defaults to `false`.
    pub priority: bool,
}

impl<'a> PagifyOptions<'a> {
    /// Creates a [`PagifyOptions`] object with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates the `delims` field.
    ///
    /// Pages are broken at these delimiters. The default delimiters
    /// are `"\n"` and `" "`.
    ///
    /// The order of delimiters matters if `priority` is set to `true`.
    ///
    /// It returns a mutable reference to the struct for easy chaining.
    pub fn delims(&mut self, delims: &'a [&'a str]) -> &mut Self {
        self.delims = delims;

        self
    }

    /// Updates the `escape_mass_mentions` field.
    ///
    /// If set `true`, "@everyone" and "@here" are escaped by adding a zero-width
    /// Unicode character between "@" and "everyone" or "here". It is set to
    /// `true` by default.
    ///
    /// It returns a mutable reference to the struct for easy chaining.
    pub fn escape_mass_mentions(&mut self, setting: bool) -> &mut Self {
        self.escape_mass_mentions = setting;

        self
    }

    /// Updates the `shorten_by` field.
    ///
    /// This is the length to shorten each page by. It defaults to `0`.
    ///
    /// It returns a mutable reference to the struct for easy chaining.
    pub fn shorten_by(&mut self, length: usize) -> &mut Self {
        self.shorten_by = length;

        self
    }

    /// Updates the `page_length` field.
    ///
    /// This is the maximum length a page can have. A page may be shorter
    /// depending on the delimiters. It defaults to `2000`.
    ///
    /// It returns a mutable reference to the struct for easy chaining.
    pub fn page_length(&mut self, length: usize) -> &mut Self {
        self.page_length = length;

        self
    }

    /// Updates the `priority` field.
    ///
    /// If `true`, pages are broken as per the order of the delimiters. Otherwise,
    /// pages are broken at the last possible delimiter. It defaults to `false`.
    ///
    /// It returns a mutable reference to the struct for easy chaining.
    pub fn priority(&mut self, priority: bool) -> &mut Self {
        self.priority = priority;

        self
    }
}

impl<'a> Default for PagifyOptions<'a> {
    fn default() -> Self {
        Self {
            delims: &["\n", " "],
            escape_mass_mentions: true,
            shorten_by: 8,
            page_length: 2000,
            priority: false,
        }
    }
}

/// Breaks a large chuck of text into smaller pages.
///
/// It can be tweaked by using appropriate [`PagifyOptions`].
///
/// ## Example
///
/// ```
/// # use serenity_utils::formatting::{pagify, PagifyOptions};
/// #
/// let mut options = PagifyOptions::default();
/// options.page_length(30).shorten_by(0).priority(true);
/// let pages = pagify(
///     "This is the first sentence.\
///     \nAnother sentence.\nThis is a long sentence and \
///     will be broken into two.",
///     options
/// );
/// assert_eq!(
///     vec![
///         "This is the first sentence.",
///         "\nAnother sentence.",
///         "\nThis is a long sentence and",
///         " will be broken into two."
///     ],
///     pages
/// );
/// ```
///
/// This is ported from [`Red-DiscordBot's pagify`] function.
///
/// [`Red-DiscordBot's pagify`]: https://github.com/Cog-Creators/Red-DiscordBot/blob/V3/develop/redbot/core/utils/chat_formatting.py#L212
pub fn pagify<S: ToString>(text: S, mut options: PagifyOptions<'_>) -> Vec<String> {
    let text = text.to_string();
    let mut in_text = text;

    let mut texts = Vec::new();

    options.page_length -= options.shorten_by;
    while in_text.len() > options.page_length {
        let mut this_page_len = options.page_length;

        if options.escape_mass_mentions {
            let sliced_text = match in_text.get(0..options.page_length) {
                Some(s) => s,
                None => continue,
            };
            this_page_len -=
                sliced_text.matches("@here").count() + sliced_text.matches("@everyone").count();
        }

        let mut possible_delims = options
            .delims
            .iter()
            .filter_map(|&d| in_text[1..this_page_len].rfind(d).map(|i| i + 1));

        let closest_delim = if options.priority {
            possible_delims.find(|&d| d > 1)
        } else {
            possible_delims.max()
        }
        .unwrap_or(this_page_len);

        let to_send = if options.escape_mass_mentions {
            escape_mass_mentions(&in_text[..closest_delim])
        } else {
            in_text[..closest_delim].to_string()
        };

        if !to_send.is_empty() {
            texts.push(to_send);
        }

        in_text = in_text[closest_delim..].to_string();
    }

    if !in_text.trim().is_empty() {
        if options.escape_mass_mentions {
            texts.push(escape_mass_mentions(in_text))
        } else {
            texts.push(in_text)
        }
    }

    texts
}

/// Returns text after escaping mass mentions (@everyone and @here).
///
/// A zero-width Unicode character (u200b) is added between `@` and `everyone` or `here`
/// to escape the mention.
///
/// Unlike serenity's [`content_safe`] function, this does not require the `cache`
/// feature to be enabled.
///
/// [`content_safe`]: serenity::utils::content_safe
pub fn escape_mass_mentions<S: ToString>(text: S) -> String {
    text.to_string()
        .replace("@everyone", "@\u{200b}everyone")
        .replace("@here", "@\u{200b}here")
}

/// Creates serenity's [`AttachmentType`] from the given text.
///
/// If `file_name` is not specified, `file.txt` is used as the default.
/// If `spoiler` is set to `true`, the file is marked as spoiler by appending
/// `spoiler_` in front of the file name.
pub fn text_to_file<'a, S: ToString, T: Display>(
    text: S,
    file_name: Option<T>,
    spoiler: bool,
) -> AttachmentType<'a> {
    let mut qualified_file_name = String::new();

    if spoiler {
        let _ = write!(qualified_file_name, "spoiler_");
    }

    if let Some(name) = file_name {
        let _ = write!(qualified_file_name, "{}", name);
    } else {
        let _ = write!(qualified_file_name, "file.txt");
    }

    AttachmentType::Bytes {
        data: Cow::from(text.to_string().into_bytes()),
        filename: qualified_file_name,
    }
}
