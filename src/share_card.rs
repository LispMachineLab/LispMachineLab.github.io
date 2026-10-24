//! Tests of the card shared links show: index.html names it in its Open
//! Graph and Twitter tags, it is in the assets, and it is the size the tags
//! say, 1200 by 630. So are the cards of the blog's pages.

use std::path::{Path, PathBuf};

use crate::content::SITE;

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn index() -> String {
    std::fs::read_to_string(root().join("index.html")).unwrap_or_default()
}

/// Returns the content of the meta tag whose `key` attribute is `name`.
fn meta(html: &str, key: &str, name: &str) -> Option<String> {
    let marker = format!("{key}=\"{name}\"");
    html.split("<meta")
        .find(|tag| tag.contains(&marker))
        .and_then(|tag| {
            let (_, rest) = tag.split_once("content=\"")?;
            let (content, _) = rest.split_once('"')?;
            Some(content.to_owned())
        })
}

/// Returns the file in the checkout served at `url`.
fn served(url: &str) -> Option<PathBuf> {
    url.strip_prefix(SITE).map(|path| root().join(path))
}

/// Returns a PNG's width and height, read from its header.
fn png_size(bytes: &[u8]) -> Option<(u32, u32)> {
    let signature: &[u8] = b"\x89PNG\r\n\x1a\n";
    let word = |at: usize| -> Option<u32> {
        let quad: [u8; 4] = bytes.get(at..at + 4)?.try_into().ok()?;
        Some(u32::from_be_bytes(quad))
    };
    if bytes.get(..8)? != signature || bytes.get(12..16)? != b"IHDR" {
        return None;
    }
    Some((word(16)?, word(20)?))
}

#[test]
fn both_tags_name_the_same_card() {
    let html = index();
    let open_graph = meta(&html, "property", "og:image");
    assert!(open_graph.is_some());
    assert_eq!(open_graph, meta(&html, "name", "twitter:image"));
    assert_eq!(
        meta(&html, "name", "twitter:card").as_deref(),
        Some("summary_large_image")
    );
}

#[test]
fn the_card_is_served_from_the_assets_at_the_size_the_tags_say() {
    let html = index();
    let card = meta(&html, "property", "og:image")
        .and_then(|url| served(&url))
        .and_then(|path| std::fs::read(path).ok())
        .unwrap_or_default();
    let width = meta(&html, "property", "og:image:width").and_then(|w| w.parse().ok());
    let height = meta(&html, "property", "og:image:height").and_then(|h| h.parse().ok());
    assert_eq!(png_size(&card), Some((1200, 630)));
    assert_eq!(width.zip(height), png_size(&card));
}

#[test]
fn the_card_is_described() {
    let html = index();
    for (key, name) in [("property", "og:image:alt"), ("name", "twitter:image:alt")] {
        assert!(
            meta(&html, key, name).is_some_and(|alt| !alt.is_empty()),
            "{name}"
        );
    }
}

#[test]
fn every_page_of_the_blog_has_its_card_at_that_size() {
    let heads = std::iter::once(crate::blog::publish::blog_head()).chain(
        crate::blog::POSTS
            .iter()
            .map(crate::blog::publish::post_head),
    );
    for head in heads {
        let (card, alt) = head.card;
        let path = root().join(card.trim_start_matches('/'));
        let bytes = std::fs::read(&path).unwrap_or_default();
        assert_eq!(png_size(&bytes), Some((1200, 630)), "{card}");
        assert!(!alt.is_empty(), "{card}");
    }
}

#[test]
fn png_size_rejects_what_is_not_a_png() {
    assert_eq!(png_size(b"GIF89a"), None);
    assert_eq!(png_size(&[]), None);
}
