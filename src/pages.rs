//! The pages that stand alone, beside the home page and the blog: LispM
//! Mail's own page and the privacy policy, as data. Each page's words are a
//! Markdown file in `pages/`. Prerender draws each as a plain page at its
//! path, under the home page's navigation bar and over its footer, its
//! shared links showing the home page's card; no WebAssembly runs on them.

use std::fmt::Write as _;

use crate::blog::{Picture, absolute};
use crate::content::PRIVACY;

/// A page that stands alone: its words under a headline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Page {
    /// Its path on the site, from the root, ending in `/`.
    pub path: &'static str,
    /// The words over its headline: what it is about.
    pub eyebrow: &'static str,
    /// Its headline.
    pub title: &'static str,
    /// Its title as the window, search engines and shared links show it:
    /// at most [`TITLE_MOST`] characters.
    pub meta_title: &'static str,
    /// What search engines and shared links show of it: at most 160
    /// characters.
    pub description: &'static str,
    /// Its words, in Markdown, under the headline.
    pub body: &'static str,
}

/// The most characters a title may have before search engines cut it.
pub const TITLE_MOST: usize = 60;

/// The card links to these pages show, 1200 by 630: the home page's.
pub const CARD: Picture = Picture {
    src: "/assets/social/lispm-card.png",
    alt: "LispM: a Lisp Machine for agents. Beside the words, an agent's chat in Emacs next to \
          the file it is fixing.",
};

/// The pages, in the order the site's map and llms.txt list them.
pub static PAGES: [Page; 2] = [
    Page {
        path: "/mail/",
        eyebrow: "LispM Mail",
        title: "Your mail, in one window.",
        meta_title: "LispM Mail — your mail, in one classic-style window",
        description: "LispM Mail puts your mail accounts in one classic-style window: a folder \
             list, a table of messages you can sort, and the message you choose under it.",
        body: include_str!("../pages/mail.md"),
    },
    Page {
        path: PRIVACY,
        eyebrow: "LispM",
        title: "Privacy Policy",
        meta_title: "Privacy Policy — LispM",
        description: "What LispM Mail and the lispm.ai website read, keep and send. Your mail \
             stays on your computer and goes only to your mail provider's servers.",
        body: include_str!("../pages/privacy.md"),
    },
];

/// Returns the pages as a section of llms.txt: a link to each, with what
/// it is about.
pub fn llms_section() -> String {
    let mut out = String::from("## Pages\n\n");
    for page in &PAGES {
        let _ = writeln!(
            out,
            "- [{}]({}): {}",
            page.meta_title,
            absolute(page.path),
            page.description,
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blog::{DESCRIPTION_MOST, POSTS};
    use crate::content::BLOG;

    /// Returns the paths from the root a page's Markdown names, in `(` `)`.
    fn named_paths(body: &str) -> Vec<&str> {
        body.split("](")
            .skip(1)
            .filter_map(|rest| rest.split([')', ' ']).next())
            .filter(|path| path.starts_with('/'))
            .collect()
    }

    #[test]
    fn titles_and_descriptions_fit_where_search_engines_show_them() {
        for page in &PAGES {
            for (text, most) in [
                (page.meta_title, TITLE_MOST),
                (page.description, DESCRIPTION_MOST),
            ] {
                assert!(!text.is_empty(), "{}", page.path);
                assert!(
                    text.chars().count() <= most,
                    "{} characters: {text}",
                    text.chars().count()
                );
            }
        }
    }

    #[test]
    fn every_page_has_its_own_address_off_the_blog() {
        for (i, a) in PAGES.iter().enumerate() {
            for b in PAGES.iter().skip(i + 1) {
                assert_ne!(a.path, b.path);
            }
            let name = a.path.trim_matches('/');
            assert!(
                a.path.starts_with('/') && a.path.ends_with('/') && !name.is_empty(),
                "{}",
                a.path
            );
            assert!(
                name.chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "{}",
                a.path
            );
            assert!(!a.path.starts_with(BLOG), "{}", a.path);
        }
    }

    #[test]
    fn every_page_has_its_words_under_its_own_headline() {
        for page in &PAGES {
            assert!(!page.body.trim().is_empty(), "{}", page.path);
            assert!(!page.eyebrow.is_empty() && !page.title.is_empty());
            assert!(
                !page.body.lines().any(|line| line.starts_with("# ")),
                "the headline is the page's: {}",
                page.path
            );
        }
    }

    #[test]
    fn the_footers_privacy_link_and_every_link_in_the_pages_land_on_a_page() {
        let on_site = |path: &str| {
            path == "/"
                || path == BLOG
                || PAGES.iter().any(|page| page.path == path)
                || POSTS.iter().any(|post| post.path() == path)
        };
        assert!(on_site(PRIVACY));
        let mut named = 0;
        for page in &PAGES {
            for path in named_paths(page.body) {
                assert!(on_site(path), "{path} in {}", page.path);
                named += 1;
            }
        }
        assert!(named > 0, "LispM Mail's page links the privacy policy");
    }

    #[test]
    fn llms_txt_links_every_page() {
        let section = llms_section();
        assert!(section.starts_with("## Pages\n\n"));
        for page in &PAGES {
            assert!(section.contains(&format!(
                "- [{}](https://lispm.ai{}): {}\n",
                page.meta_title, page.path, page.description
            )));
        }
    }
}
