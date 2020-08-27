use serenity_utils::formatting::{escape_mass_mentions, pagify, PagifyOptions};

#[test]
fn test_pagify() {
    let mut options = PagifyOptions::default();
    options.page_length(30).shorten_by(0).priority(true);

    let pages = pagify(
        "This is the first sentence.\
        \nAnother sentence.\nThis is a long sentence and \
        will be broken into two.",
        options,
    );

    assert_eq!(
        vec![
            "This is the first sentence.",
            "\nAnother sentence.",
            "\nThis is a long sentence and",
            " will be broken into two."
        ],
        pages
    );
}

#[test]
fn test_escape_mass_mentions() {
    let text = "Hello, @everyone! I can filter both @everyone and @here pings!";

    assert_eq!(
        escape_mass_mentions(text),
        String::from(
            "Hello, @\u{200b}everyone! I can filter both @\u{200b}everyone \
            and @\u{200b}here pings!"
        )
    )
}
