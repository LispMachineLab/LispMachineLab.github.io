//! The blog's feed, in Atom: each post whole, for feed readers and for the
//! search engines and language models that follow feeds to what is new.

use std::fmt::Write as _;

use super::markdown::{self, escape};
use super::{DESCRIPTION, POSTS, Post, absolute, size};
use crate::content::{BLOG, SITE};

/// Where the feed is served.
pub const FEED: &str = "/blog/feed.xml";

/// Returns `html` with the addresses it names from the root named whole,
/// as a feed reader, away from the site, needs them.
fn absolute_addresses(html: &str) -> String {
    html.replace("=\"/", &format!("=\"{SITE}"))
}

/// Returns a post's day as Atom writes a moment: its start, in UTC.
fn moment(post: &Post) -> String {
    format!("{}T00:00:00Z", post.date.iso())
}

/// Returns `post` as an Atom entry: its cover over its words, as HTML.
fn entry(post: &Post) -> String {
    let url = escape(&absolute(&post.path()));
    let cover = format!(
        "<p><img src=\"{}\" alt=\"{}\" /></p>\n",
        escape(post.cover.src),
        escape(post.cover.alt)
    );
    let content = absolute_addresses(&format!(
        "{cover}{}",
        markdown::to_html(post.body, size::of)
    ));
    format!(
        "  <entry>\n    <title>{}</title>\n    <link href=\"{url}\" />\n    <id>{url}</id>\n    \
         <published>{moment}</published>\n    <updated>{moment}</updated>\n    \
         <author><name>{}</name><uri>{}</uri></author>\n    <summary>{}</summary>\n    \
         <content type=\"html\">{}</content>\n  </entry>\n",
        escape(post.title),
        escape(post.author.name),
        escape(&post.author.profile()),
        escape(post.description),
        escape(&content),
        moment = moment(post),
    )
}

/// Returns the feed: the blog, as new as its latest post, and every post,
/// the latest first.
pub fn feed() -> String {
    let blog = escape(&absolute(BLOG));
    let updated = POSTS
        .iter()
        .max_by_key(|post| post.date)
        .map(moment)
        .unwrap_or_default();
    let mut out = format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
         <feed xmlns=\"http://www.w3.org/2005/Atom\" xml:lang=\"en\">\n  \
         <title>LispM Blog</title>\n  <subtitle>{}</subtitle>\n  \
         <link href=\"{}\" rel=\"self\" />\n  <link href=\"{blog}\" />\n  <id>{blog}</id>\n  \
         <updated>{updated}</updated>\n  <author><name>LispM</name><uri>{SITE}</uri></author>\n  \
         <icon>{SITE}favicon.ico</icon>\n",
        escape(DESCRIPTION),
        escape(&absolute(FEED)),
    );
    for post in &POSTS {
        out.push_str(&entry(post));
    }
    let _ = writeln!(out, "</feed>");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_feed_has_every_post_whole_at_its_address() {
        let feed = feed();
        assert!(feed.starts_with("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<feed "));
        assert!(feed.ends_with("</feed>\n"));
        assert!(feed.contains("<link href=\"https://lispm.ai/blog/feed.xml\" rel=\"self\" />"));
        assert_eq!(feed.matches("<entry>").count(), POSTS.len());
        for post in &POSTS {
            assert!(feed.contains(&format!("<id>{}</id>", absolute(&post.path()))));
            assert!(feed.contains(&format!("<published>{}T00:00:00Z", post.date.iso())));
        }
        let latest = POSTS
            .iter()
            .map(|post| post.date)
            .max()
            .map(|date| date.iso());
        assert!(latest.is_some_and(|date| feed.contains(&format!("<updated>{date}T00:00:00Z"))));
    }

    #[test]
    fn the_feed_names_every_address_whole() {
        let feed = feed();
        // In the escaped content, an attribute's quote is `&quot;`.
        assert!(!feed.contains("=&quot;/"));
        assert!(feed.contains("src=&quot;https://lispm.ai/assets/blog/"));
        assert!(
            !feed.contains("<content type=\"html\"><"),
            "the content is escaped"
        );
    }
}
