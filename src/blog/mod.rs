//! The blog: LispM's Articles from X, as data. Each post's words are a
//! Markdown file in `blog/`, its cover, share card, pictures and films in
//! `assets/blog/<slug>/`. Prerender draws the list and each post as plain
//! pages at `/blog/` and `/blog/<slug>/`; no WebAssembly runs on them.

pub mod markdown;
pub mod publish;

use crate::content::{BLOG, SITE};

/// The day a post was first published.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    /// The year.
    pub year: u16,
    /// The month, 1 to 12.
    pub month: u8,
    /// The day of the month.
    pub day: u8,
}

impl Date {
    /// Returns the date as ISO 8601 writes it, `2026-10-22`.
    pub fn iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Returns the date as a reader writes it, `22 October 2026`.
    pub fn long(self) -> String {
        format!("{} {} {}", self.day, month_name(self.month), self.year)
    }
}

/// Returns the name of the month numbered `month`, from 1, or nothing past 12.
const fn month_name(month: u8) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    }
}

/// Who wrote a post, as X knows them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Author {
    /// The name shown.
    pub name: &'static str,
    /// The handle on X, without the `@`.
    pub handle: &'static str,
}

impl Author {
    /// Returns the author's profile on X.
    pub fn profile(self) -> String {
        format!("https://x.com/{}", self.handle)
    }
}

/// LispM's own account.
pub const LISPM: Author = Author {
    name: "LispM",
    handle: "lispm_ai",
};

/// The account of LispM's author.
pub const RYAN_KUNG: Author = Author {
    name: "Ryan Kung",
    handle: "Elder24601",
};

/// A picture and what it shows, for those who cannot see it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Picture {
    /// Its path on the site, from the root.
    pub src: &'static str,
    /// What it shows.
    pub alt: &'static str,
}

/// A post.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Post {
    /// Its name in its address, `/blog/<slug>/`, and its assets' folder.
    pub slug: &'static str,
    /// Its title.
    pub title: &'static str,
    /// What it is about, in a sentence or two: under its title in the list,
    /// and in its description for search engines and shared links.
    pub description: &'static str,
    /// The day it was first published.
    pub date: Date,
    /// Who wrote it.
    pub author: Author,
    /// The picture over it, 5 by 2.
    pub cover: Picture,
    /// The card its shared links show, 1200 by 630; it shows the cover.
    pub card: &'static str,
    /// The post on X, where it was first published, if it is known.
    pub on_x: Option<&'static str>,
    /// Its words, in Markdown, under the title.
    pub body: &'static str,
}

impl Post {
    /// Returns the post's path on the site, `/blog/<slug>/`.
    pub fn path(&self) -> String {
        format!("{BLOG}{}/", self.slug)
    }
}

/// Returns the address on the site of `path`, a path from the root.
pub fn absolute(path: &str) -> String {
    format!("{}{path}", SITE.trim_end_matches('/'))
}

/// The blog's headline.
pub const HEADLINE: &str = "Notes from the lab.";

/// The paragraph under it.
pub const LEAD: &str =
    "What we build into LispM, and why. Each post here was first an Article on X.";

/// The card links to the blog show, 1200 by 630.
pub const CARD: Picture = Picture {
    src: "/assets/blog/card.png",
    alt: "LispM Blog: notes from the lab. Beside the words, the covers of its posts.",
};

/// The posts, the latest first.
pub static POSTS: [Post; 3] = [
    Post {
        slug: "your-agents-can-talk-to-each-other",
        title: "Your agents can talk to each other now, in LispM",
        description: "Claude Code, Codex, Hermes, Holon and OpenClaw can now message each other in \
             LispM, ask each other for things and answer, under one policy. How it works, and why \
             it takes a Lisp Machine.",
        date: Date {
            year: 2026,
            month: 10,
            day: 24,
        },
        author: LISPM,
        cover: Picture {
            src: "/assets/blog/your-agents-can-talk-to-each-other/cover.webp",
            alt: "Blueprint drawing titled \"Your agents can talk to each other\". A λ for LispM \
                  sits in the middle, joined to claude, codex, hermes, holon and openclaw, with \
                  \"Request R1 from claude\" and \"Message from holon\" on the arrows between them.",
        },
        card: "/assets/blog/your-agents-can-talk-to-each-other/card.png",
        on_x: Some("https://x.com/lispm_ai/status/2113998019578273891"),
        body: include_str!("../../blog/your-agents-can-talk-to-each-other.md"),
    },
    Post {
        slug: "three-old-ais-running-again",
        title: "Three old AIs, running again in LispM",
        description: "We've recreated three classic AI applications from the 1970s on LispM, and \
             they're now available in the LispM App Store: Doctor, SHRDLU and MYCIN. Come \
             experience what AI was like on Lisp Machines half a century ago.",
        date: Date {
            year: 2026,
            month: 10,
            day: 22,
        },
        author: LISPM,
        cover: Picture {
            src: "/assets/blog/three-old-ais-running-again/cover.webp",
            alt: "Classic AI, from the LispM App Store: three programs from 1966 to the 1970s. \
                  Doctor, ELIZA's psychotherapist, MIT, 1966; SHRDLU, Winograd's blocks world, \
                  MIT, 1970; MYCIN, the expert system, Stanford, 1970s.",
        },
        card: "/assets/blog/three-old-ais-running-again/card.png",
        on_x: Some("https://x.com/lispm_ai/status/2113303074261659862"),
        body: include_str!("../../blog/three-old-ais-running-again.md"),
    },
    Post {
        slug: "when-agents-meet-the-lisp-machine",
        title: "When Agents Meet the Lisp Machine",
        description: "While connecting various agents to Emacs, I suddenly wondered: why not build \
             an Emacs-native agent? With Emacs's introspective capabilities, it could become an \
             almost perfect realization of the Lisp Machines of the 1970s.",
        date: Date {
            year: 2026,
            month: 10,
            day: 12,
        },
        author: RYAN_KUNG,
        cover: Picture {
            src: "/assets/blog/when-agents-meet-the-lisp-machine/cover.webp",
            alt: "LispM, a Lisp Machine for agents: AI agents, inside Emacs. Beside the words, a \
                  beige computer of the 1980s shows the rainbow λ on its screen.",
        },
        card: "/assets/blog/when-agents-meet-the-lisp-machine/card.png",
        on_x: Some("https://x.com/Elder24601/status/2109688119213867522"),
        body: include_str!("../../blog/when-agents-meet-the-lisp-machine.md"),
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// Returns the file in the checkout served at `path`, a path from the root.
    fn served(path: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(path.trim_start_matches('/'))
    }

    /// Returns the paths from the root a post's Markdown names, in `(` `)`.
    fn named_paths(body: &str) -> Vec<&str> {
        body.split("](")
            .skip(1)
            .filter_map(|rest| rest.split([')', ' ']).next())
            .filter(|path| path.starts_with('/'))
            .collect()
    }

    #[test]
    fn dates_read_both_ways() {
        let date = Date {
            year: 2026,
            month: 10,
            day: 2,
        };
        assert_eq!(date.iso(), "2026-10-02");
        assert_eq!(date.long(), "2 October 2026");
        assert_eq!(month_name(13), "");
    }

    #[test]
    fn posts_come_latest_first_each_at_its_own_address() {
        for pair in POSTS.windows(2) {
            if let [later, earlier] = pair {
                assert!(later.date > earlier.date, "{}", later.slug);
            }
        }
        for (i, a) in POSTS.iter().enumerate() {
            for b in POSTS.iter().skip(i + 1) {
                assert_ne!(a.slug, b.slug);
            }
            assert!(
                a.slug
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "{}",
                a.slug
            );
            assert_eq!(a.path(), format!("/blog/{}/", a.slug));
        }
    }

    #[test]
    fn every_post_has_its_words_and_its_pictures() {
        for post in POSTS {
            let folder = format!("/assets/blog/{}/", post.slug);
            assert!(!post.body.trim().is_empty(), "{}", post.slug);
            assert!(
                !post.body.starts_with("# "),
                "the title is the page's: {}",
                post.slug
            );
            assert!(!post.description.is_empty() && !post.cover.alt.is_empty());
            for path in [post.cover.src, post.card]
                .into_iter()
                .chain(named_paths(post.body))
            {
                assert!(path.starts_with(&folder), "{path}");
                assert!(served(path).is_file(), "{path}");
            }
            for film in named_paths(post.body)
                .into_iter()
                .filter_map(|path| path.strip_suffix(".mp4"))
            {
                assert!(served(&format!("{film}.jpg")).is_file(), "{film}");
            }
        }
        assert!(served(CARD.src).is_file());
    }

    #[test]
    fn posts_on_x_are_their_authors() {
        for post in POSTS {
            if let Some(on_x) = post.on_x {
                assert!(
                    on_x.starts_with(&format!("{}/status/", post.author.profile())),
                    "{on_x}"
                );
            }
        }
    }

    #[test]
    fn addresses_are_on_the_site() {
        assert_eq!(absolute("/blog/"), "https://lispm.ai/blog/");
    }
}
