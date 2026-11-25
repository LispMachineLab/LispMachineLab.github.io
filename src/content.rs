//! What the page says, as data: its sections, the chapters that tell what
//! LispM does with the clips from the films and the pictures beside them,
//! the projects LispM is made of, and the code to install them. The words
//! follow the projects' READMEs and release notes.
//!
//! The chapters follow what a visitor can do, not how the code is split:
//! a feature is told once, in the chapter it matters to.

use crate::date::Date;

/// A section of the page, the target of a link.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anchor {
    /// The film, under the headline.
    Film,
    /// What LispM is.
    Idea,
    /// FrontierHarness 1.0 benchmark results.
    Benchmark,
    /// Dedicated feature pages of the Emacs AI agent harness.
    Explore,
    /// What its agents do.
    Agents,
    /// The desktop they live on.
    Desktop,
    /// The App Store's apps.
    Store,
    /// LispM from a phone.
    Remote,
    /// Why you stay in charge.
    Trust,
    /// How to install LispM, and its projects on their own.
    Start,
    /// What people ask about LispM, answered.
    Questions,
}

impl Anchor {
    /// Returns the section's element id.
    pub const fn id(self) -> &'static str {
        match self {
            Anchor::Film => "film",
            Anchor::Idea => "idea",
            Anchor::Benchmark => "benchmark",
            Anchor::Explore => "explore",
            Anchor::Agents => "agent",
            Anchor::Desktop => "desktop",
            Anchor::Store => "store",
            Anchor::Remote => "remote",
            Anchor::Trust => "trust",
            Anchor::Start => "start",
            Anchor::Questions => "questions",
        }
    }
}

/// A clip from the films, played muted and looped while in sight.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Clip {
    /// The video's address.
    pub video: &'static str,
    /// The still shown before it plays.
    pub poster: &'static str,
    /// What it shows, for those who cannot see it.
    pub description: &'static str,
    /// The day it was first published on the page.
    pub published: Date,
}

/// A still, where a moment says it better than a clip, or where there is no
/// film of it yet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Picture {
    /// The image's address.
    pub image: &'static str,
    /// Its width in pixels.
    pub width: u32,
    /// Its height in pixels.
    pub height: u32,
    /// What it shows, for those who cannot see it.
    pub description: &'static str,
}

/// What shows beside a step of a project's story.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Media {
    /// A clip, looping.
    Loop(Clip),
    /// A still.
    Picture(Picture),
}

impl Clip {
    /// Returns the width and height in pixels of the clip's poster image.
    pub const fn poster_size(self) -> (u32, u32) {
        if matches!(self.poster.as_bytes(), b"assets/video/lispm-film.jpg") {
            (1920, 1080)
        } else {
            (1280, 720)
        }
    }
}

impl Media {
    /// Returns the still it shows first: a clip's poster, or the picture.
    pub const fn still(self) -> &'static str {
        match self {
            Media::Loop(clip) => clip.poster,
            Media::Picture(picture) => picture.image,
        }
    }

    /// Returns the width and height in pixels of the still it shows first.
    pub const fn size(self) -> (u32, u32) {
        match self {
            Media::Loop(clip) => clip.poster_size(),
            Media::Picture(picture) => (picture.width, picture.height),
        }
    }

    /// Returns what it shows, for those who cannot see it.
    pub const fn description(self) -> &'static str {
        match self {
            Media::Loop(clip) => clip.description,
            Media::Picture(picture) => picture.description,
        }
    }

    /// Returns the clip, if it is one.
    pub const fn clip(self) -> Option<Clip> {
        match self {
            Media::Loop(clip) => Some(clip),
            Media::Picture(_) => None,
        }
    }
}

/// A clip or a still with the headline it illustrates, the stage's or the
/// spotlight's it shows beside.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Illustration {
    /// The headline.
    pub headline: &'static str,
    /// What shows beside it.
    pub media: Media,
}

/// A step of a project's story: a headline, a sentence, and a clip or a
/// picture.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stage {
    /// The headline.
    pub headline: &'static str,
    /// The sentence under it.
    pub text: &'static str,
    /// What shows beside them.
    pub media: Media,
}

/// A short point: a title and a sentence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    /// The title.
    pub title: &'static str,
    /// The sentence.
    pub text: &'static str,
}

/// The language a snippet is written in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    /// A line for a terminal, named after the Mac's Terminal for those who
    /// know no shell.
    Terminal,
    /// A shell's.
    Shell,
    /// Emacs Lisp, for an init file.
    EmacsLisp,
}

impl Language {
    /// Returns the language's name, as the snippet's caption shows it.
    pub const fn name(self) -> &'static str {
        match self {
            Language::Terminal => "Terminal",
            Language::Shell => "Shell",
            Language::EmacsLisp => "Emacs Lisp",
        }
    }

    /// Returns the language's name after a Markdown code fence.
    pub const fn fence(self) -> &'static str {
        match self {
            Language::Terminal | Language::Shell => "sh",
            Language::EmacsLisp => "elisp",
        }
    }
}

/// Code to paste.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Snippet {
    /// What it is written in.
    pub language: Language,
    /// The code, its lines joined by newlines.
    pub code: &'static str,
}

/// A step of installing LispM: what to do, and the line to paste, if any.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Step {
    /// What to do.
    pub title: &'static str,
    /// How, when it needs saying.
    pub text: &'static str,
    /// The line to paste.
    pub snippet: Option<Snippet>,
}

/// A feature told at length, after a chapter's story: a headline, a
/// paragraph, a clip if it has one, and its points, if it has any.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spotlight {
    /// The headline.
    pub headline: &'static str,
    /// The paragraph under it.
    pub text: &'static str,
    /// What plays under the paragraph, if anything does.
    pub clip: Option<Clip>,
    /// What it comes to, point by point.
    pub points: &'static [Point],
}

/// A chapter of the page: what LispM does for one part of a visitor's day.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chapter {
    /// Its section.
    pub anchor: Anchor,
    /// The eyebrow over the headline.
    pub eyebrow: &'static str,
    /// The headline.
    pub headline: &'static str,
    /// The paragraph under it.
    pub lead: &'static str,
    /// Its story, each step with a clip or a picture; none for a chapter
    /// told in points alone.
    pub stages: &'static [Stage],
    /// Features told at length after the story.
    pub spotlights: &'static [Spotlight],
    /// The points under the story.
    pub points: &'static [Point],
    /// Where to read its code.
    pub repository: &'static str,
}

/// How to install a project in your own Emacs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Install {
    /// Who installs it this way, and what it needs.
    pub audience: &'static str,
    /// The code to paste.
    pub snippet: Snippet,
}

/// One of the projects LispM is made of, each a repository of its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Project {
    /// Its name.
    pub name: &'static str,
    /// What it is, in a sentence.
    pub description: &'static str,
    /// Its repository.
    pub repository: &'static str,
    /// The languages it is written in, the most used first.
    pub languages: &'static [&'static str],
    /// How to install it in your own Emacs, if it stands on its own there.
    pub install: Option<Install>,
}

/// Returns the clip playing `video`, `poster` before it, described by
/// `description`, first published on `published`.
const fn clip(
    video: &'static str,
    poster: &'static str,
    description: &'static str,
    published: Date,
) -> Clip {
    Clip {
        video,
        poster,
        description,
        published,
    }
}

/// Returns a looping clip playing `video`, `poster` before it, described by
/// `description`, first published on `published`.
const fn shown(
    video: &'static str,
    poster: &'static str,
    description: &'static str,
    published: Date,
) -> Media {
    Media::Loop(clip(video, poster, description, published))
}

/// The day the page opened, with the film and the first clips.
const OPENED: Date = Date {
    year: 2026,
    month: 10,
    day: 20,
};

/// The day the page showed what LispM had gained since: the classic AIs
/// and LispM Remote.
const GAINED: Date = Date {
    year: 2026,
    month: 10,
    day: 25,
};

/// The film, with sound, under the headline.
pub const FILM: Clip = clip(
    "assets/video/lispm-film.mp4",
    "assets/video/lispm-film.jpg",
    "LispM 57-second keynote film: from 1970s Lisp Machines to LispM Desktop, a LispM Agent fixing \
     a failing test inside GNU Emacs, Claude Code, Codex and Hermes side by side, and windows \
     dragged off the desktop.",
    OPENED,
);

/// The eyebrow over the hero's headline.
pub const HERO_EYEBROW: &str = "LispM · Emacs AI Agent Harness";

/// LispM's tagline.
pub const TAGLINE: &str = "A Lisp Machine for agents.";

/// The paragraph under the tagline.
pub const SUMMARY: &str = "LispM is an open-source Emacs AI agent harness. Its agents run inside \
     GNU Emacs: conversations are org documents, tools are Elisp, and Emacs itself is the harness.";

/// The headline of the idea.
pub const IDEA_HEADLINE: &str =
    "The editor, the operating system and your Emacs AI agent harness, one world.";

/// The paragraph under it.
pub const IDEA: &str = "On a 1970s Lisp Machine, the editor and the operating system lived together \
     as live Lisp objects. Born beside it at MIT in 1976, GNU Emacs has kept that live \
     architecture for fifty years—and LispM opens it up as an Emacs AI agent harness.";

/// What makes LispM's agents what they are.
pub const IDEA_POINTS: [Point; 4] = [
    Point {
        title: "Conversations are documents.",
        text: "In LispM, each agent's chat is a live org-mode document in Emacs, and that document is its model context.",
    },
    Point {
        title: "Tools are Lisp.",
        text: "A LispM agent's tools are native Elisp functions and MCP tools; it writes Lisp to combine them and drives GNU Emacs itself.",
    },
    Point {
        title: "Approvals are TODOs.",
        text: "In LispM, a shell command or file edit waits as an org-mode TODO entry until you approve it.",
    },
    Point {
        title: "Memory is git.",
        text: "What LispM's agents remember is stored locally as plain-text org files under git.",
    },
];

/// The steps of the Classic Lisp Machine (Symbolics Open Genera 2.0) story,
/// shown on the home page under [`IDEA`] and on `/desktop/`.
pub const IDEA_STAGES: [Stage; 4] = [
    Stage {
        headline: "Symbolics Open Genera, in 1-bit monochrome.",
        text: "Switch LispM Desktop to Symbolics Open Genera 2.0 and the top menu bar, desktop \
               icons and Control Strip give way to a 1-bit monochrome Dynamic Windows workstation: \
               paper-white headers, double-rule borders, 45° hatched scrollbars, a Who-Line and a \
               2-line inverse-video Mouse Documentation Line.",
        media: shown(
            "assets/video/genera-desktop.mp4",
            "assets/video/genera-desktop.jpg",
            "LispM Desktop switching from Mac OS 9 Platinum to Symbolics Open Genera 2.0 monochrome \
             Dynamic Windows, then closing floating panes to reveal Dynamic Lisp Listener 1.",
            GAINED,
        ),
    },
    Stage {
        headline: "Lisp and AI at one Command: prompt.",
        text: "In LispM's Dynamic Lisp Listener 1, type a Lisp form at Command: to evaluate it \
               inline, or type a coding task in plain English at the very same prompt—the Who-Line \
               switches from User Input to Agent Run as LispM edits the code and runs the tests.",
        media: shown(
            "assets/video/genera-listener.mp4",
            "assets/video/genera-listener.jpg",
            "LispM Dynamic Lisp Listener 1 evaluating a Lisp form at the Command: prompt, followed \
             by an English request for LispM Agent to fix a failing test as the Who-Line switches \
             to Agent Run.",
            GAINED,
        ),
    },
    Stage {
        headline: "Create ▸ Emacs, sweep to size.",
        text: "Right-click anywhere in LispM to open the 1-bit Dynamic Windows System Menu, choose \
               Create ▸ Emacs, and drag a dotted rubber-band outline across the screen to size and \
               place the window—then click [Bury] or [Kill] on any frame header to return to the \
               Listener.",
        media: shown(
            "assets/video/genera-windows.mp4",
            "assets/video/genera-windows.jpg",
            "LispM Desktop in Symbolics Open Genera 2.0: right-clicking opens the 1-bit System Menu \
             to choose Create and Emacs, sweeping a dotted rubber-band outline across Dynamic Lisp \
             Listener 1 to size and place the window.",
            GAINED,
        ),
    },
    Stage {
        headline: "The Lisp Machine in a terminal, too.",
        text: "Start LispM with emacs -nw in any terminal or over SSH and the same Dynamic Lisp \
               Listener 1, the block-character λ, Command: prompt and Who-Line run in text mode.",
        media: shown(
            "assets/video/genera-terminal.mp4",
            "assets/video/genera-terminal.jpg",
            "LispM running in a terminal with emacs -nw: Dynamic Lisp Listener 1 with the rainbow \
             block-character lambda, Command: prompt and Who-Line evaluating Lisp in text mode.",
            GAINED,
        ),
    },
];

/// A headline metric card in the FrontierHarness 1.0 benchmark section.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BenchmarkStat {
    /// The metric's name.
    pub label: &'static str,
    /// The primary value.
    pub value: &'static str,
    /// Context note under the value.
    pub detail: &'static str,
}

/// One row of the 13-harness FrontierHarness 1.0 evaluation table (30 tasks).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BenchmarkRow {
    /// Harness name.
    pub harness: &'static str,
    /// Pass rate percentage (e.g. `66.7%`).
    pub pass_rate: &'static str,
    /// Tasks passed out of 30 (e.g. `20/30`).
    pub passed: &'static str,
    /// Pass rate width in percent (0..=100) for visual bar rendering.
    pub pass_bar_pct: u8,
    /// Median cost per successful task under standardized Kimi K3 pricing.
    pub median_cost: &'static str,
    /// Effective cost per pass (failures included).
    pub effective_cost: &'static str,
    /// Typical warm prompt-cache hit rate.
    pub cache_hit: &'static str,
    /// Median wall-clock time per successful task.
    pub median_time: &'static str,
    /// Median time in seconds, for visual speed bar scaling.
    pub median_seconds: u32,
    /// Whether this row is LispM.
    pub is_lispm: bool,
}

/// Eyebrow over the FrontierHarness 1.0 benchmark section.
pub const BENCHMARK_EYEBROW: &str = "FrontierHarness 1.0";

/// Headline of the FrontierHarness 1.0 benchmark section.
pub const BENCHMARK_HEADLINE: &str = "Measured on FrontierHarness 1.0.";

/// A pillar card on the home page linking to one of LispM's dedicated pages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExploreCard {
    /// Short label over the card's title.
    pub eyebrow: &'static str,
    /// Link title naming the product and capability.
    pub title: &'static str,
    /// Root-relative path to the dedicated page.
    pub path: &'static str,
    /// Self-contained summary with concrete technical bounds.
    pub text: &'static str,
    /// Real screenshot of LispM shown at the top of the card.
    pub picture: Picture,
}

/// Eyebrow over the home page's pillar hub section.
pub const EXPLORE_EYEBROW: &str = "Inside LispM";

/// Headline of the home page's pillar hub section.
pub const EXPLORE_HEADLINE: &str = "Every layer of the Emacs AI agent harness.";

/// Lead paragraph of the home page's pillar hub section.
pub const EXPLORE_LEAD: &str = "LispM combines an open-source GNU Emacs AI agent harness with \
     three classic workstation desktops, an App Store of agent-driven skills and peer-to-peer \
     mobile control.";

/// Pillar hub cards linking the home page to LispM's dedicated feature pages.
pub const EXPLORE_CARDS: [ExploreCard; 4] = [
    ExploreCard {
        eyebrow: "LispM Agent",
        title: "Native GNU Emacs AI Agent Harness",
        path: AGENT,
        text: "LispM Agent runs AI coding agents inside GNU Emacs—org-mode chats, Elisp tools and \
               TODO approvals—alongside Claude Code, Codex and Antigravity over a local message \
               bus and MCP.",
        picture: Picture {
            image: "assets/blog/your-agents-can-talk-to-each-other/claude-asks-codex.webp",
            width: 2000,
            height: 1121,
            description: "LispM Agent on LispM Desktop in GNU Emacs with Claude Code and Codex \
                          running side by side and reviewing code over LispM's JSON message bus.",
        },
    },
    ExploreCard {
        eyebrow: "LispM Desktop",
        title: "Three Classic Workstations in Emacs",
        path: DESKTOP,
        text: "LispM Desktop turns GNU Emacs into a complete AI agent workstation in 7 languages \
               with Symbolics Open Genera 2.0, Mac OS 9 Platinum and Windows 3.2 looks plus a \
               WebKit browser.",
        picture: Picture {
            image: "assets/screenshots/theme-genera.webp",
            width: 2000,
            height: 1050,
            description: "LispM Desktop in the Symbolics Open Genera 2.0 Lisp Machine look: \
                          Dynamic Lisp Listener 1, 1-bit Dynamic Windows, Who-Line and Mouse \
                          Documentation Line.",
        },
    },
    ExploreCard {
        eyebrow: "LispM App Store",
        title: "Emacs Apps, Skills & AI Harnesses",
        path: APPS,
        text: "LispM App Store installs open-source Emacs apps, skills, LispM Office, external AI \
               harnesses (Antigravity, Hermes, Holon, OpenClaw) and 1970s Classic AI in one click.",
        picture: Picture {
            image: "assets/screenshots/lispm-office.webp",
            width: 1720,
            height: 1448,
            description: "LispM Office in LispM Desktop showing the 8-room animated pixel office \
                          and LispM Office Inspector tracking live AI coding agents in GNU Emacs.",
        },
    },
    ExploreCard {
        eyebrow: "LispM Remote",
        title: "Peer-to-Peer Mobile Control via WebRTC",
        path: REMOTE,
        text: "LispM Remote pairs your phone and GNU Emacs by QR code over encrypted peer-to-peer \
               WebRTC on Rings Network, with no cloud server in between.",
        picture: Picture {
            image: "assets/screenshots/theme-platinum.webp",
            width: 2000,
            height: 1068,
            description: "LispM Desktop in the Mac OS 9 Platinum look with the rainbow lambda top \
                          menu bar, pinstriped windows and desktop icons inside GNU Emacs.",
        },
    },
];

/// Lead paragraph of the FrontierHarness 1.0 benchmark section.
pub const BENCHMARK_LEAD: &str = "Evaluation results from running LispM alongside twelve other \
     agent harnesses on the 30 containerized tasks of FrontierHarness 1.0 (Terminal-Bench 2.0 and \
     DeepSWE) under standardized Kimi K3 pricing. Across these 30 tasks, LispM passed 20/30 \
     (66.7%) with a median time of 3m 24s (203.9s) per passing task.";

/// Headline metrics from the FrontierHarness 1.0 30-task evaluation.
pub const BENCHMARK_STATS: [BenchmarkStat; 4] = [
    BenchmarkStat {
        label: "Pass Rate (30 Tasks)",
        value: "66.7%",
        detail: "20 / 30 containerized tasks passed",
    },
    BenchmarkStat {
        label: "Median Time per Pass",
        value: "3m 24s",
        detail: "203.9s median wall-clock time",
    },
    BenchmarkStat {
        label: "Cost per Pass (Standardized)",
        value: "$0.08",
        detail: "$0.0809 median · $1.98 effective",
    },
    BenchmarkStat {
        label: "Warm Prompt-Cache Hit Rate",
        value: "83.2%",
        detail: "94.2% normalized across 13.5 median turns",
    },
];

/// Full 13-harness evaluation table on the 30 FrontierHarness 1.0 tasks.
pub const BENCHMARK_ROWS: [BenchmarkRow; 13] = [
    BenchmarkRow {
        harness: "LispM",
        pass_rate: "66.7%",
        passed: "20/30",
        pass_bar_pct: 67,
        median_cost: "$0.0809",
        effective_cost: "$1.98",
        cache_hit: "83.2%",
        median_time: "3m 24s (203.9s)",
        median_seconds: 204,
        is_lispm: true,
    },
    BenchmarkRow {
        harness: "Codex",
        pass_rate: "66.7%",
        passed: "20/30",
        pass_bar_pct: 67,
        median_cost: "$0.1243",
        effective_cost: "$3.47",
        cache_hit: "88.0%",
        median_time: "6m 43s (403.3s)",
        median_seconds: 403,
        is_lispm: false,
    },
    BenchmarkRow {
        harness: "DSH (Creator)",
        pass_rate: "63.3%",
        passed: "19/30",
        pass_bar_pct: 63,
        median_cost: "$0.1194",
        effective_cost: "$3.28",
        cache_hit: "84.3%",
        median_time: "6m 44s (403.9s)",
        median_seconds: 404,
        is_lispm: false,
    },
    BenchmarkRow {
        harness: "Claude Code",
        pass_rate: "63.3%",
        passed: "19/30",
        pass_bar_pct: 63,
        median_cost: "$0.2880",
        effective_cost: "$18.34",
        cache_hit: "67.8%",
        median_time: "9m 38s (578.0s)",
        median_seconds: 578,
        is_lispm: false,
    },
    BenchmarkRow {
        harness: "Pi",
        pass_rate: "60.0%",
        passed: "18/30",
        pass_bar_pct: 60,
        median_cost: "$0.0709",
        effective_cost: "$2.43",
        cache_hit: "79.4%",
        median_time: "7m 33s (453.3s)",
        median_seconds: 453,
        is_lispm: false,
    },
    BenchmarkRow {
        harness: "DSH (Standard)",
        pass_rate: "60.0%",
        passed: "18/30",
        pass_bar_pct: 60,
        median_cost: "$0.1201",
        effective_cost: "$3.46",
        cache_hit: "86.5%",
        median_time: "6m 17s (376.5s)",
        median_seconds: 377,
        is_lispm: false,
    },
    BenchmarkRow {
        harness: "DSH (PTC)",
        pass_rate: "60.0%",
        passed: "18/30",
        pass_bar_pct: 60,
        median_cost: "$0.1370",
        effective_cost: "$4.58",
        cache_hit: "87.2%",
        median_time: "7m 44s (463.8s)",
        median_seconds: 464,
        is_lispm: false,
    },
    BenchmarkRow {
        harness: "Kimi Code",
        pass_rate: "56.7%",
        passed: "17/30",
        pass_bar_pct: 57,
        median_cost: "$0.1818",
        effective_cost: "$3.65",
        cache_hit: "88.0%",
        median_time: "7m 56s (475.9s)",
        median_seconds: 476,
        is_lispm: false,
    },
    BenchmarkRow {
        harness: "DSH (Minimal)",
        pass_rate: "56.7%",
        passed: "17/30",
        pass_bar_pct: 57,
        median_cost: "$0.1214",
        effective_cost: "$4.72",
        cache_hit: "84.6%",
        median_time: "5m 41s (341.2s)",
        median_seconds: 341,
        is_lispm: false,
    },
    BenchmarkRow {
        harness: "Oh My Pi",
        pass_rate: "56.7%",
        passed: "17/30",
        pass_bar_pct: 57,
        median_cost: "$0.1354",
        effective_cost: "$4.75",
        cache_hit: "82.2%",
        median_time: "6m 46s (406.4s)",
        median_seconds: 406,
        is_lispm: false,
    },
    BenchmarkRow {
        harness: "Exo Harness",
        pass_rate: "53.3%",
        passed: "16/30",
        pass_bar_pct: 53,
        median_cost: "$0.0748",
        effective_cost: "$1.05",
        cache_hit: "70.3%",
        median_time: "6m 17s (377.2s)",
        median_seconds: 377,
        is_lispm: false,
    },
    BenchmarkRow {
        harness: "Hermes",
        pass_rate: "50.0%",
        passed: "15/30",
        pass_bar_pct: 50,
        median_cost: "$0.1746",
        effective_cost: "$2.90",
        cache_hit: "85.9%",
        median_time: "6m 58s (417.8s)",
        median_seconds: 418,
        is_lispm: false,
    },
    BenchmarkRow {
        harness: "OpenCode",
        pass_rate: "50.0%",
        passed: "15/30",
        pass_bar_pct: 50,
        median_cost: "$0.0615",
        effective_cost: "$3.24",
        cache_hit: "78.4%",
        median_time: "6m 27s (387.3s)",
        median_seconds: 387,
        is_lispm: false,
    },
];

/// The steps of LispM Agent's story, shown on `/agent/`.
pub const AGENTS_STAGES: [Stage; 5] = [
    Stage {
        headline: "It works in your project.",
        text: "LispM Agent reads your repository, writes the fix and runs tests inside an org-mode \
               chat document, using native Elisp tools.",
        media: shown(
            "assets/video/agent.mp4",
            "assets/video/agent.jpg",
            "LispM Agent fixing a failing test inside GNU Emacs: it reads the code, asks for approval, \
             writes the fix, passes the tests, and evaluates Emacs Lisp to open two files side by side.",
            OPENED,
        ),
    },
    Stage {
        headline: "Every agent, one sidebar.",
        text: "Claude Code, Codex, Antigravity and Hermes run beside LispM's own Elisp agents, \
               with every session listed in one Emacs sidebar to resume where it left off.",
        media: shown(
            "assets/video/agents.mp4",
            "assets/video/agents.jpg",
            "LispM Desktop running Claude Code, Codex and Hermes beside a native LispM Agent, all \
             active at once and listed in one Emacs sidebar.",
            OPENED,
        ),
    },
    Stage {
        headline: "And they talk to each other.",
        text: "In LispM, any agent can ask another for a code review over a local message bus \
               and receive the reply as its next message, sharing LispM's skills and MCP tools.",
        media: Media::Picture(Picture {
            image: "assets/blog/your-agents-can-talk-to-each-other/claude-asks-codex.webp",
            width: 2000,
            height: 1121,
            description: "LispM's classic-style desktop with Claude Code and Codex side by side, both \
                          running. Claude Code asks Codex to review a function; Codex gets the request \
                          as its next message, and its reply comes back to Claude Code as one.",
        }),
    },
    Stage {
        headline: "Characters with a soul of their own.",
        text: "In LispM's Characters app, give an agent a name, pixel-art portrait, soul, \
               role and model of its own—each character (@Ada, @Milo, @Juno) becomes an app \
               in Applications that you, Claude Code and Codex can message by name.",
        media: Media::Picture(Picture {
            image: "assets/screenshots/characters-ada.webp",
            width: 2000,
            height: 1119,
            description: "LispM's Applications folder showing Ada, Milo and Juno as applications with \
                          pixel-art portraits alongside Ada's chat window reviewing a diff.",
        }),
    },
    Stage {
        headline: "An 8-room pixel office for every agent.",
        text: "Press C-c O to open LispM Office, an 8-room animated pixel office where every \
               live agent session has a desk: working agents type at their laptops, idle agents \
               hold coffee, waiting agents raise a hand, and clicking any desk opens its inspector \
               and pending calls.",
        media: Media::Picture(Picture {
            image: "assets/screenshots/lispm-office.webp",
            width: 1720,
            height: 1448,
            description: "LispM Office in Focus Mode showing the QA Office pixel room in the middle \
                          and the LispM Office Inspector on the right with Work, Mailbox and Character tabs.",
        }),
    },
];

/// The features of LispM Agent told at length on `/agent/`.
pub const AGENTS_SPOTLIGHTS: [Spotlight; 1] = [Spotlight {
    headline: "LispM itself, one field away.",
    text: "M-x ai, or the magnifying glass in the menu bar, drops one launcher field under the \
           menu bar while your keyboard stays in the window you were in. As you type, LispM \
           searches installed apps, control panels, skills and tools; anything else goes to LispM, \
           a standing agent that keeps one conversation across restarts.",
    clip: Some(clip(
        "assets/video/lispm-bar.mp4",
        "assets/video/lispm-bar.jpg",
        "The LispM bar drops from the top menu bar when clicked, takes the request to play calm \
         music and stick a note on the desktop, and drives Stickies and EMMS at once.",
        GAINED,
    )),
    points: &[],
}];

/// What LispM Agent brings, point by point.
pub const AGENTS_POINTS: [Point; 6] = [
    Point {
        title: "LispM Office",
        text: "Press C-c O for LispM Office, an 8-room pixel office where every agent session has a desk: working agents type, idle agents hold coffee, waiting agents raise a hand, and clicking a desk inspects its work or approves a call.",
    },
    Point {
        title: "Characters (@Ada)",
        text: "Create persistent agents in LispM's Characters app with a name, portrait, soul and model; each becomes its own desktop app that you, Claude Code and Codex can message by name (@Ada).",
    },
    Point {
        title: "Skills on demand",
        text: "LispM's agents search Elisp tools, MCP servers and app manuals on demand, loading only what a task needs to keep context focused.",
    },
    Point {
        title: "Memory that keeps itself",
        text: "In LispM, long sessions are summed up while you are idle, and growing memory consolidates locally, one git commit at a time.",
    },
    Point {
        title: "Checked against Emacs",
        text: "In LispM, Elisp code in a reply that names a function or file this Emacs lacks is underlined and explains why.",
    },
    Point {
        title: "Woken by Emacs",
        text: "In LispM, Emacs hooks and timers wake agents automatically, and agents hand subtasks to each other.",
    },
];

/// One of the three classic workstation desktops built into LispM Desktop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DesktopTheme {
    /// Its Lisp symbol (`genera`, `platinum`, or `win3`).
    pub key: &'static str,
    /// The era and workstation family it recreates.
    pub eyebrow: &'static str,
    /// Its name on the page.
    pub title: &'static str,
    /// The paragraph describing how it works and looks.
    pub description: &'static str,
    /// Its signature interface elements.
    pub highlights: &'static [&'static str],
    /// How to select it in Emacs Lisp or Control Panels.
    pub setting: &'static str,
    /// A real screenshot of LispM Desktop running in this look.
    pub picture: Picture,
}

/// The three classic desktops LispM Desktop offers from the first start and
/// in Control Panels → Appearance.
pub const DESKTOP_THEMES: [DesktopTheme; 3] = [
    DesktopTheme {
        key: "genera",
        eyebrow: "Classic Lisp Machine",
        title: "Symbolics Open Genera 2.0",
        description: "A 1-bit monochrome Dynamic Windows workstation in LispM Desktop, \
                      independent of Mac OS and Windows. Instead of desktop icons or a top menu \
                      bar, LispM opens into a full-screen Dynamic Lisp Listener with a Command: \
                      prompt, Zmacs buffers, a white Who-Line and a 2-line inverse-video Mouse \
                      Documentation Line.",
        highlights: &[
            "Full-screen Dynamic Lisp Listener 1 root screen with Command: prompt and quick-action buttons",
            "Paper-white pane headers, 1-bit double-rule borders (3px outer, 2px gap, 1px inner) and 45° hatched scrollbars",
            "1-bit Dynamic Windows Command Menus (Select, Create with mouse-sweep outline placement, Bury, Kill, Move, Reshape)",
            "Bottom Who-Line and 2-line inverse-video Mouse Documentation Line showing live L / M / R mouse bindings",
        ],
        setting: "(setopt lispm-desktop-look 'genera lispm-desktop-theme 'genera)",
        picture: Picture {
            image: "assets/screenshots/theme-genera.webp",
            width: 2000,
            height: 1050,
            description: "LispM Desktop in the Symbolics Open Genera 2.0 look: Dynamic Lisp Listener 1 \
                          on the left, the Appearance control panel in a 1-bit Dynamic Windows frame on \
                          the right, and the Who-Line and inverse-video Mouse Documentation Line along \
                          the bottom.",
        },
    },
    DesktopTheme {
        key: "platinum",
        eyebrow: "Classic Mac OS",
        title: "Mac OS 9 Platinum & Dracula",
        description: "Mac OS 9's Platinum appearance in LispM Desktop in classic bevelled \
                      greys or dark Dracula colours. A top menu bar beside the rainbow λ changes \
                      with the frontmost application, pinstriped title bars carry close, zoom and \
                      collapse boxes, and a collapsible Control Strip waits at the bottom left.",
        highlights: &[
            "Top menu bar with the rainbow λ, frontmost app menus, M-x ai search field and live clock",
            "Pinstriped title bars with close, zoom and collapse (windowshade) boxes, Charcoal and Geneva type, and window-bottom placards",
            "Desktop icons down the right edge, Finder windows, bottom-left Control Strip (C-c b), and Balloon Help",
            "Interactive Apple Guide tour (C-c ?) with step-by-step coach marks and Show Me highlights",
        ],
        setting: "(setopt lispm-desktop-look 'platinum lispm-desktop-theme 'platinum) ; or 'dracula",
        picture: Picture {
            image: "assets/screenshots/theme-platinum.webp",
            width: 2000,
            height: 1068,
            description: "LispM Desktop in the Mac OS 9 Platinum look: top menu bar with the rainbow \
                          lambda, pinstriped Appearance window over the periwinkle desktop, and desktop \
                          icons along the right edge.",
        },
    },
    DesktopTheme {
        key: "win3",
        eyebrow: "Classic Win32",
        title: "Windows 3.2 Workstation",
        description: "A Windows 3.2 and 3.1 workstation in LispM Desktop in classic VGA grey \
                      (#C0C0C0) with 3D bevels, navy (#000080) active title bars and white \
                      inactive captions, left control-menu boxes, minimize and maximize arrows, \
                      and 4-pixel notched sizing borders.",
        highlights: &[
            "Windows Default VGA grey (#C0C0C0) desktop with icons aligned down the left edge and double-click Task List",
            "Navy (#000080) active captions, left control-menu slot, right ▲ / ▼ minimize and maximize buttons, and 4px notched sizing borders",
            "Cut-corner Win3.1 push buttons with dotted focus rings, crossed [×] check boxes, and sunken entry fields",
            "In-window menu bars under the title bar and classic Windows 3.2 system message boxes",
        ],
        setting: "(setopt lispm-desktop-look 'win3 lispm-desktop-theme 'win3)",
        picture: Picture {
            image: "assets/screenshots/theme-win3.webp",
            width: 2000,
            height: 1050,
            description: "LispM Desktop in the Windows 3.2 look: navy active caption bar, control-menu \
                          box, minimize arrow button, 4-pixel notched sizing borders and desktop icons \
                          down the left edge.",
        },
    },
];

/// The steps of LispM Desktop's story, shown on `/desktop/`.
pub const DESKTOP_STAGES: [Stage; 7] = [
    Stage {
        headline: "Every project, a workspace.",
        text: "LispM Desktop gives each project its own workspace—Finder, editor, AI coding \
               agents, Alacritty terminals and LispM Office together—plus remote projects over SSH.",
        media: shown(
            "assets/video/desktop.mp4",
            "assets/video/desktop.jpg",
            "LispM Desktop starting on its classic-style desktop, opening a project in GNU Emacs, and showing its workspaces.",
            OPENED,
        ),
    },
    Stage {
        headline: "Make it yours.",
        text: "Switch between Symbolics Open Genera, Mac OS 9 Platinum & Dracula, and Windows 3.2 \
               at any time, drag windows onto your screen in Coherence mode, or adjust settings \
               from Control Panels and the Control Strip.",
        media: shown(
            "assets/video/windows.mp4",
            "assets/video/windows.jpg",
            "LispM Desktop in Coherence mode: a window is dragged off the desktop, the desktop hides and comes back, and the theme switches to Dracula.",
            OPENED,
        ),
    },
    Stage {
        headline: "Standalone Mac apps in your Dock.",
        text: "Hide LispM's desktop (C-c h) and open Stickies, the App Store, Tetris or any \
               LispM app directly from the macOS Dock or Spotlight with its own Dock tile and \
               menu bar.",
        media: shown(
            "assets/video/standalone-apps.mp4",
            "assets/video/standalone-apps.jpg",
            "With LispM's desktop hidden on macOS, Stickies, the App Store and Tetris are clicked in \
             the Dock and open as standalone Mac applications with their own menu bars.",
            GAINED,
        ),
    },
    Stage {
        headline: "The full desktop, inside a terminal.",
        text: "Start Emacs with emacs -nw in Kitty or Ghostty and the full LispM Desktop—its menu \
               bar, desktop icons over the Kitty graphics protocol, floating windows and \
               games—runs inside your terminal.",
        media: shown(
            "assets/video/terminal-desktop.mp4",
            "assets/video/terminal-desktop.jpg",
            "LispM Desktop running inside a terminal via emacs -nw with graphical icons, menus and \
             Conway's Game of Life in a floating window.",
            GAINED,
        ),
    },
    Stage {
        headline: "Common Lisp beside Emacs, and GNU Emacs 31 & 32.",
        text: "In Control Panels → System → Kernel, turn on the LispM Kernel—a Common Lisp runtime \
               running beside Emacs that keeps your sessions—and switch live between GNU Emacs \
               31.1 and experimental GNU Emacs 32.",
        media: Media::Picture(Picture {
            image: "assets/screenshots/kernel-panel.webp",
            width: 2000,
            height: 1094,
            description: "LispM Control Panels open on the Kernel tab, showing the LispM Kernel Common \
                          Lisp runtime toggle and the Emacs Kernel selector between GNU Emacs 31.1 and GNU Emacs 32.",
        }),
    },
    Stage {
        headline: "The web, from the keyboard.",
        text: "LispM Browser is a WebKit browser inside Emacs with tabs, history and bookmarks: \
               the location field takes an address or search, f labels every link in sight, and \
               typing a label follows it.",
        media: shown(
            "assets/video/browser.mp4",
            "assets/video/browser.jpg",
            "LispM Browser opening a page, navigating from the location field, and labeling links to follow them from the keyboard.",
            OPENED,
        ),
    },
    Stage {
        headline: "Chrome extensions, inside Emacs.",
        text: "On macOS 15.4 and later, LispM Browser runs Chrome Web Store extensions managed at \
               lispm://extensions, with service-worker support, toolbar popups and M-e keyboard \
               actions for extensions like Dark Reader and Rabby.",
        media: shown(
            "assets/video/extensions.mp4",
            "assets/video/extensions.jpg",
            "LispM Browser installing Dark Reader from the Chrome Web Store, turning the Lisp machine \
             article dark, and opening Dark Reader's toolbar popup inside GNU Emacs.",
            OPENED,
        ),
    },
];

/// The features of LispM Desktop told at length on `/desktop/`.
pub const DESKTOP_SPOTLIGHTS: [Spotlight; 1] = [Spotlight {
    headline: "Focus Mode, a Lisp Machine's console.",
    text: "Press C-c T, or ask LispM, and the desktop gives way to LispM's terminal console: \
           agent sessions on the left, a Lisp listener on the right and a Who-Line along the \
           bottom, all from the keyboard. Leave Focus Mode and your desktop returns intact.",
    clip: Some(clip(
        "assets/video/focus-mode.mp4",
        "assets/video/focus-mode.jpg",
        "LispM switches from the desktop into Focus Mode, runs a coding task from the keyboard, and \
         returns to the desktop when told 'bring desktop back'.",
        GAINED,
    )),
    points: &[],
}];

/// What LispM Desktop brings, point by point.
pub const DESKTOP_POINTS: [Point; 6] = [
    Point {
        title: "Three classic desktops",
        text: "Switch LispM Desktop at any time between Symbolics Open Genera 2.0, Mac OS 9 Platinum & Dracula, and Windows 3.2, plus OS/2 Warp, phosphor and neon glass looks.",
    },
    Point {
        title: "Native Alacritty terminals",
        text: "Each terminal in LispM Desktop runs on its own thread via a native Rust Alacritty module, redrawing only visible panes so heavy output does not stall Emacs.",
    },
    Point {
        title: "Remote projects over SSH",
        text: "Add a remote host in LispM's sidebar and open directories over TRAMP, launching remote shells and AI coding agents in one workspace.",
    },
    Point {
        title: "Control Panels & live kernels",
        text: "Configure LispM's Appearance, Language, Keyboard, Network proxy, Sandbox, Security, Apps and Kernel, switching live between GNU Emacs 31 and Emacs 32.",
    },
    Point {
        title: "Seven languages",
        text: "LispM speaks 7 languages: English (US and UK), German, French, Japanese, and Simplified and Traditional Chinese. The agents follow.",
    },
    Point {
        title: "Apps of their own",
        text: "On macOS, each app in LispM also opens from ~/Applications/LispM Apps via Spotlight or the Dock, with its own Dock tile and menu bar.",
    },
];

/// The steps of the LispM App Store's story, shown on `/apps/`.
pub const STORE_STAGES: [Stage; 5] = [
    Stage {
        headline: "Every app, a page of its own.",
        text: "In the LispM App Store, each app shows its author, free licence and dependencies. \
               Clicking Get installs what it needs via Homebrew or Cargo, and uninstalling removes \
               unused dependencies cleanly.",
        media: Media::Picture(Picture {
            image: "assets/blog/three-old-ais-running-again/app-store-shrdlu.webp",
            width: 2000,
            height: 1125,
            description: "LispM App Store on LispM's classic-style desktop: its categories, from News to \
                          Classic AI, and SHRDLU's page, with its banner, what it does, and its \
                          Uninstall and Open buttons.",
        }),
    },
    Stage {
        headline: "No app for it yet? Ask anyway.",
        text: "Ask LispM in M-x ai for something no installed app provides yet—like reading Lisp \
               news—and the agent searches the LispM App Store, asks before installing Elfeed, \
               and opens the headlines.",
        media: shown(
            "assets/video/app-store-ai.mp4",
            "assets/video/app-store-ai.jpg",
            "LispM Agent asked in the LispM bar to install a news reader and show what is new in Lisp: \
             it finds Elfeed in the LispM App Store, asks for approval, installs it, and opens the \
             headlines.",
            GAINED,
        ),
    },
    Stage {
        headline: "Three old AIs, running again.",
        text: "LispM's Classic AI catalog restores interactive Lisp landmarks: 1966 ELIZA \
               Doctor, 1970 blocks-world SHRDLU, and 1970s MYCIN showing every rule it evaluates.",
        media: shown(
            "assets/video/classic-ai.mp4",
            "assets/video/classic-ai.jpg",
            "SHRDLU running in LispM Desktop: told to find a block taller than the one it holds and put \
             it into the box, it plans the moves and carries the blocks across the table.",
            GAINED,
        ),
    },
    Stage {
        headline: "Doctor, SHRDLU and MYCIN, side by side.",
        text: "Weizenbaum's 1966 Doctor, Winograd's 1970 SHRDLU and Stanford's 1970s MYCIN run \
               side by side on LispM Desktop, each bundling an Elisp skill so modern AI agents can \
               converse with them.",
        media: Media::Picture(Picture {
            image: "assets/screenshots/classic-ai-all.webp",
            width: 2000,
            height: 1125,
            description: "LispM Desktop running SHRDLU, MYCIN and Doctor side by side in three floating \
                          windows inside GNU Emacs.",
        }),
    },
    Stage {
        headline: "Doom, drawn with OpenGL inside Emacs.",
        text: "On LispM Experimental (GNU Emacs 32), LispM Doom renders classic Doom at full speed \
               inside an Emacs buffer using LispM GL's OpenGL surface.",
        media: shown(
            "assets/video/doom.mp4",
            "assets/video/doom.jpg",
            "LispM Doom running inside a floating window on LispM Desktop, rendered with OpenGL in Emacs.",
            GAINED,
        ),
    },
];

/// What the LispM App Store brings, point by point.
pub const STORE_POINTS: [Point; 6] = [
    Point {
        title: "LispM Office (C-c O)",
        text: "Installed by default in LispM: an 8-room animated pixel office where every agent session has a desk, working agents type, waiting agents raise a hand, and clicking a desk approves calls.",
    },
    Point {
        title: "Skills come with apps",
        text: "Install an app in LispM and its manual arrives as an agent skill for RSS feeds, music, Stickies, mail, IRC, Slack, Telegram and security checks.",
    },
    Point {
        title: "AI Harnesses",
        text: "From the LispM App Store, external AI harnesses—Antigravity, Hermes, Holon and OpenClaw—join Claude Code, Codex and @Ada in one sidebar over LispM's local message bus.",
    },
    Point {
        title: "For every day",
        text: "The LispM App Store includes local daily apps: LispM Mail, Stickies, RSS feeds, music, MP4 and YouTube playback, calendar, IRC, Slack, Telegram and 1Password.",
    },
    Point {
        title: "Classic AI",
        text: "LispM restores playable 1960s and 1970s AI landmarks inside GNU Emacs: Joseph Weizenbaum's 1966 Doctor, Terry Winograd's 1970 SHRDLU and Stanford's MYCIN.",
    },
    Point {
        title: "And games",
        text: "Play native Emacs games in LispM—Tetris, Solitaire, Conway's Game of Life and Dunnet—plus Doom rendered with OpenGL on LispM Experimental.",
    },
];

/// The steps of LispM Remote's story, shown on `/lispm-remote/`.
pub const REMOTE_STAGES: [Stage; 3] = [
    Stage {
        headline: "Pair once.",
        text: "LispM displays a QR code, your phone scans it at lispm.ai/remote, and both screens \
               verify the same fingerprint over WebRTC—nothing to install on the phone and no \
               router port to open.",
        media: shown(
            "assets/video/remote-pair.mp4",
            "assets/video/remote-pair.jpg",
            "LispM Remote showing a QR code on LispM Desktop, a phone scanning it, both screens showing \
             the fingerprint 3981-CF90, and LispM approving the phone.",
            GAINED,
        ),
    },
    Stage {
        headline: "Approve from anywhere.",
        text: "In LispM Remote, each Emacs AI agent appears as a live phone chat: watch tool \
               calls as they run, approve pending calls, and send the next prompt.",
        media: shown(
            "assets/video/remote-approve.mp4",
            "assets/video/remote-approve.jpg",
            "LispM Remote on a phone: a LispM Agent works through a fix and asks to run make test; \
             Allow is tapped on the phone, and all three tests pass.",
            GAINED,
        ),
    },
    Stage {
        headline: "Every window, too.",
        text: "LispM's Emacs windows redraw natively on your phone over encrypted WebRTC rather \
               than streaming video.",
        media: shown(
            "assets/video/remote-windows.mp4",
            "assets/video/remote-windows.jpg",
            "LispM Remote on a phone opening slug.el from LispM's Emacs windows, redrawn peer to peer \
             over Rings Network WebRTC with no server in between.",
            GAINED,
        ),
    },
];

/// What LispM Remote brings, point by point.
pub const REMOTE_POINTS: [Point; 3] = [
    Point {
        title: "No server in between",
        text: "LispM and your phone connect peer to peer over Rings Network WebRTC, with no central cloud server relaying your session.",
    },
    Point {
        title: "Back by itself",
        text: "When mobile connectivity drops and returns, LispM Remote reconnects over WebRTC automatically without scanning again.",
    },
    Point {
        title: "From the App Store",
        text: "One click in the LispM App Store installs LispM Remote, with its Rings node binary verified against a pinned SHA-256 checksum.",
    },
];

const TRUST_POINTS: [Point; 4] = [
    Point {
        title: "It asks first",
        text: "In LispM, shell commands, file edits and package installs wait for your approval in Emacs, LispM Office or on your phone. Read-only inspection runs without pausing.",
    },
    Point {
        title: "LispM Auth vault",
        text: "LispM Auth encrypts secrets locally with Argon2id and XChaCha20-Poly1305 in a standalone Common Lisp daemon.",
    },
    Point {
        title: "Sandboxed and checked",
        text: "LispM isolates shell commands and MCP servers inside an OS sandbox, checks what Claude Code, Codex and its own agents run, and scans packages against OSV.",
    },
    Point {
        title: "Free software",
        text: "Every part of LispM is open source under the GNU AGPL, connecting directly to any OpenAI or Anthropic HTTPS endpoint with your own keys.",
    },
];

/// The chapters, in the order the page tells them: what the agents do, the
/// desktop they live on, the apps they use, LispM from a phone, and why you
/// stay in charge.
pub const CHAPTERS: [Chapter; 5] = [
    Chapter {
        anchor: Anchor::Agents,
        eyebrow: "Agents",
        headline: "An AI agent harness inside GNU Emacs.",
        lead: "Talk to a LispM agent in a chat laid out like Claude Code's. Underneath, the chat \
               is an org document, its tools are Elisp and MCP functions, and it evaluates live \
               Lisp inside GNU Emacs.",
        stages: &AGENTS_STAGES,
        spotlights: &AGENTS_SPOTLIGHTS,
        points: &AGENTS_POINTS,
        repository: "https://github.com/LispMachineLab/lispm-agent",
    },
    Chapter {
        anchor: Anchor::Desktop,
        eyebrow: "Desktop",
        headline: "Three classic desktops, one workspace for agents.",
        lead: "LispM Desktop ships three classic workstation interfaces—Symbolics Open Genera \
               2.0, Mac OS 9 Platinum & Dracula, and Windows 3.2—in seven languages with a Finder, \
               WebKit browser and Alacritty terminals.",
        stages: &DESKTOP_STAGES,
        spotlights: &DESKTOP_SPOTLIGHTS,
        points: &DESKTOP_POINTS,
        repository: "https://github.com/LispMachineLab/lispm-desktop",
    },
    Chapter {
        anchor: Anchor::Store,
        eyebrow: "App Store",
        headline: "Apps your agents can drive.",
        lead: "Install open-source Emacs apps, games and external AI harnesses in one click. Every \
               app brings its manual as a skill so LispM's agents can drive it for you.",
        stages: &STORE_STAGES,
        spotlights: &[],
        points: &STORE_POINTS,
        repository: "https://github.com/LispMachineLab/lispm-store",
    },
    Chapter {
        anchor: Anchor::Remote,
        eyebrow: "LispM Remote",
        headline: "Your agents, in your pocket.",
        lead: "Scan a QR code once. From your phone over encrypted WebRTC, follow your Emacs \
               agents, approve waiting tool calls and inspect LispM's windows anywhere.",
        stages: &REMOTE_STAGES,
        spotlights: &[],
        points: &REMOTE_POINTS,
        repository: "https://github.com/LispMachineLab/lispm-remote",
    },
    Chapter {
        anchor: Anchor::Trust,
        eyebrow: "Trust",
        headline: "You stay in charge.",
        lead: "AI agents act on your machine, so LispM shows every step they take and gates risky \
               commands or file edits on your explicit approval.",
        stages: &[],
        spotlights: &[],
        points: &TRUST_POINTS,
        repository: "https://github.com/LispMachineLab/lispm-security",
    },
];

/// Returns every clip and still beside the chapters, top to bottom, each
/// with the headline it illustrates: the home page's Classic Lisp Machine
/// stages, the Desktop chapter's three classic desktop themes, a chapter's
/// stages, then its spotlights' clips. The film, above them all, is [`FILM`].
pub fn illustrations() -> impl Iterator<Item = Illustration> {
    let idea = IDEA_STAGES.iter().map(|stage| Illustration {
        headline: stage.headline,
        media: stage.media,
    });
    idea.chain(CHAPTERS.iter().flat_map(|chapter| {
        let themes = if chapter.anchor == Anchor::Desktop {
            DESKTOP_THEMES.as_slice()
        } else {
            &[]
        };
        let themes = themes.iter().map(|theme| Illustration {
            headline: theme.title,
            media: Media::Picture(theme.picture),
        });
        let stages = chapter.stages.iter().map(|stage| Illustration {
            headline: stage.headline,
            media: stage.media,
        });
        let spotlights = chapter.spotlights.iter().filter_map(|spotlight| {
            spotlight.clip.map(|clip| Illustration {
                headline: spotlight.headline,
                media: Media::Loop(clip),
            })
        });
        themes.chain(stages).chain(spotlights)
    }))
}

/// The projects LispM is made of, those that stand on their own in your
/// own Emacs first.
pub const PROJECTS: [Project; 5] = [
    Project {
        name: "LispM Agent",
        description: "AI agents that run inside Emacs: the chat is an org document, the tools are \
                      Elisp, and an agent can evaluate Lisp in the Emacs you are using.",
        repository: "https://github.com/LispMachineLab/lispm-agent",
        languages: &["Emacs Lisp"],
        install: Some(Install {
            audience: "The agents, in your own Emacs 30.1 or later. Then M-x lispm in any project.",
            snippet: Snippet {
                language: Language::EmacsLisp,
                code: "(use-package lispm\n  :vc (:url \"https://github.com/LispMachineLab/lispm-agent\" :rev :newest)\n  :config (lispm-mode-line-mode 1))",
            },
        }),
    },
    Project {
        name: "LispM Desktop",
        description: "An Emacs distribution built around LispM Agent, laid out as a classic-style \
                      desktop and ready from the first start.",
        repository: "https://github.com/LispMachineLab/lispm-desktop",
        languages: &["Emacs Lisp", "Rust"],
        install: Some(Install {
            audience: "What LispM.app runs, in your own Emacs 30.1 or later with SVG, git and curl, on macOS or GNU/Linux.",
            snippet: Snippet {
                language: Language::Shell,
                code: "git clone https://github.com/LispMachineLab/lispm-desktop ~/lispm-desktop\nemacs --init-directory ~/lispm-desktop",
            },
        }),
    },
    Project {
        name: "LispM Browser",
        description: "A WebKit browser in Emacs: tabs, history and bookmarks, the keyboard as Vimium \
                      uses pages, and Chrome extensions.",
        repository: "https://github.com/LispMachineLab/lispm-browser",
        languages: &["Swift", "Emacs Lisp"],
        install: Some(Install {
            audience: "LispM Browser, in an Emacs 30.1 or later built with xwidgets.",
            snippet: Snippet {
                language: Language::EmacsLisp,
                code: "(package-vc-install \"https://github.com/LispMachineLab/lispm-browser\")\n(require 'lispm-browser)\n(lispm-browser-setup)",
            },
        }),
    },
    Project {
        name: "LispM App Store",
        description: "The App Store's catalog: apps, agent roles and skills for LispM, as plain data.",
        repository: "https://github.com/LispMachineLab/lispm-store",
        languages: &["Emacs Lisp"],
        install: None,
    },
    Project {
        name: "LispM Remote",
        description: "LispM from your phone or another computer: pairing by QR code, and sessions \
                      sealed end to end, peer to peer over Rings Network.",
        repository: "https://github.com/LispMachineLab/lispm-remote",
        languages: &["Rust", "Emacs Lisp", "TypeScript"],
        install: None,
    },
];

/// Where the site is served.
pub const SITE: &str = "https://lispm.ai/";

/// The home page's title, as search engines show it: what LispM is, in the
/// words it is searched for, within the 60 characters they show.
pub const TITLE: &str = "Emacs AI Agent Harness: A Lisp Machine for Agents — LispM";

/// What search engines show under the title, within the 160 characters
/// they show: what LispM is, and the agents it runs.
pub const DESCRIPTION: &str = "LispM is an open-source Emacs AI agent harness built on GNU Emacs \
     and the Lisp Machine: org-mode chats, Elisp tools, Claude Code and Codex on Mac and Linux.";

/// What LispM is, in two sentences: the answer search engines, answer
/// engines and language models are given everywhere they look, the page's
/// first question, its JSON-LD and llms.txt.
pub const DEFINITION: &str = "LispM is an open-source Emacs AI agent harness built on GNU Emacs \
     and the ideas of the Lisp Machine, fifty years in the making. In LispM, AI coding agents \
     live inside Emacs: conversations are org documents, tools are Elisp functions, and GNU Emacs \
     itself is the harness.";

/// A question people ask about LispM, and its answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Question {
    /// The question, as people ask it.
    pub question: &'static str,
    /// Its answer, the first sentence a whole answer on its own.
    pub answer: &'static str,
}

/// The headline over the questions.
pub const QUESTIONS_HEADLINE: &str = "Emacs AI agent harness: questions, answered.";

/// The paragraph under the questions' headline, and the description of the
/// Q&A page for search engines.
pub const QUESTIONS_LEAD: &str = "Answers to common questions about LispM, the Emacs AI agent \
     harness: how native coding agents run in GNU Emacs with org-mode and Elisp, models and Remote.";

/// Number of core harness questions shown directly on the home page (matching home FAQPage JSON-LD).
pub const HOME_QUESTION_COUNT: usize = 5;

/// The Agent page's path on the site.
pub const AGENT: &str = "/agent/";

/// The description of the Agent page for search engines, within 160 characters.
pub const AGENT_DESCRIPTION: &str = "LispM Agent is an open-source Emacs AI agent harness in GNU \
     Emacs: org-mode chats, Elisp tools, TODO approvals, MCP, Claude Code and Codex in one sidebar.";

/// The Desktop page's path on the site.
pub const DESKTOP: &str = "/desktop/";

/// The description of the Desktop page for search engines, within 160 characters.
pub const DESKTOP_DESCRIPTION: &str = "LispM Desktop turns GNU Emacs into an AI agent workstation \
     with three classic desktops: Symbolics Open Genera 2.0, Mac OS 9 Platinum, Windows 3.2 and WebKit.";

/// The Apps page's path on the site.
pub const APPS: &str = "/apps/";

/// The description of the Apps page for search engines, within 160 characters.
pub const APPS_DESCRIPTION: &str = "LispM App Store installs Emacs apps, skills, LispM Office, \
     AI harnesses (Antigravity, Hermes, Holon, OpenClaw), games and 1960s–70s AI (SHRDLU, Doctor, MYCIN).";

/// The Remote page's path on the site (distinct from `/remote/`, the QR pairing web app).
pub const REMOTE: &str = "/lispm-remote/";

/// The description of the Remote page for search engines, within 160 characters.
pub const REMOTE_DESCRIPTION: &str = "LispM Remote connects your phone and GNU Emacs peer to peer \
     over Rings WebRTC via QR code: follow AI agent chats, approve calls and view windows anywhere.";

/// The Q&A page's path on the site.
pub const QA: &str = "/qa/";

/// What people ask about LispM, the first what it is.
pub const QUESTIONS: [Question; 14] = [
    Question {
        question: "What is LispM?",
        answer: "LispM is an open-source Emacs AI agent harness built on GNU Emacs and the ideas \
                 of the Lisp Machine, fifty years in the making. In LispM, AI coding agents live \
                 inside Emacs: conversations are org documents, tools are Elisp functions, and GNU \
                 Emacs itself is the harness. LispM runs free on macOS and GNU/Linux with three \
                 classic desktops, a WebKit browser, an App Store and LispM Remote.",
    },
    Question {
        question: "How does LispM perform on the FrontierHarness 1.0 benchmark?",
        answer: "On the 30 containerized tasks of FrontierHarness 1.0 (Terminal-Bench 2.0 and \
                 DeepSWE) under standardized Kimi K3 pricing, LispM passed 20/30 tasks (66.7%) \
                 with a median time per pass of 3m 24s (203.9s), a median cost of $0.0809 per \
                 pass ($1.98 effective cost per pass), and an 83.2% warm prompt-cache hit rate \
                 (94.2% normalized). The full 13-harness table is published on the home page and \
                 /agent/.",
    },
    Question {
        question: "How does LispM compare with CLI harnesses and Emacs packages like gptel or agent-shell?",
        answer: "Unlike terminal-only CLI wrappers or standalone chat buffers, LispM turns GNU \
                 Emacs itself into a live Lisp AI agent harness while hosting external CLI agents \
                 side by side. Alongside workflows like gptel or agent-shell, LispM runs Claude \
                 Code, Codex, Antigravity, Hermes, Holon and OpenClaw in one sidebar over a local \
                 MCP message bus, while LispM's native agents evaluate live Elisp, keep chats as \
                 org documents and gate edits as org-mode TODOs.",
    },
    Question {
        question: "Why build an AI agent harness on GNU Emacs and the Lisp Machine?",
        answer: "An AI coding agent benefits from a live environment it can inspect and extend \
                 while running. On 1970s Lisp Machines, and in GNU Emacs since 1976, buffers, \
                 windows and commands are live Lisp objects with built-in documentation. LispM \
                 exposes that introspectable architecture to agents: tools are Elisp functions, \
                 conversations are org documents, approvals are TODOs, and agents write Lisp to \
                 drive Emacs itself.",
    },
    Question {
        question: "Which AI coding agents and LLM models does LispM run?",
        answer: "LispM runs its own native Elisp agents, persistent Characters (@Ada) with custom \
                 souls and models, and external harnesses—Claude Code, Codex, Antigravity, Hermes, \
                 Holon and OpenClaw—in one sidebar over a shared local message bus. LispM's agents \
                 connect to any OpenAI or Anthropic HTTPS endpoint with your own API key, or any \
                 local or hosted LLM server.",
    },
    Question {
        question: "What are the three desktops in LispM Desktop?",
        answer: "LispM Desktop includes three classic workstation interfaces in seven languages \
                 that you can switch at any time in Control Panels → Appearance: Classic Lisp \
                 Machine (Symbolics Open Genera 2.0, a 1-bit monochrome Dynamic Lisp Listener with \
                 Command: prompt and Who-Line), Classic Mac OS (Mac OS 9 Platinum and Dracula \
                 with top menu bar and Control Strip), and Classic Win32 (Windows 3.2 in VGA grey \
                 #C0C0C0 with navy captions), plus keyboard-driven Focus Mode (C-c T).",
    },
    Question {
        question: "How do agents talk to each other, and what is LispM Office?",
        answer: "In LispM, Claude Code, Codex, Antigravity, Hermes, Holon, OpenClaw, Characters \
                 (@Ada) and LispM's own agents share one sidebar and a local message bus inside \
                 Emacs so any agent can request a review or subtask. Pressing C-c O opens LispM \
                 Office, an 8-room pixel office where every agent session has a desk, waiting \
                 agents raise a hand, and clicking a desk inspects its work or approves calls.",
    },
    Question {
        question: "How does LispM keep agents safe on my computer?",
        answer: "LispM requires your approval before an agent runs a shell command, edits a file \
                 or installs a package, while read-only inspection runs without pausing. Commands \
                 and MCP servers execute inside an OS sandbox, and LispM Security checks agent \
                 actions, pins trusted tools and scans packages against OSV.",
    },
    Question {
        question: "How does LispM Remote work without a cloud server?",
        answer: "LispM Remote pairs your phone and computer once via a QR code at lispm.ai/remote \
                 and connects them peer to peer over Rings Network WebRTC, with no cloud server in \
                 between. With no account or open router port, you can read agent chats, approve \
                 waiting tool calls and view Emacs windows anywhere.",
    },
    Question {
        question: "What is LispM Browser, and does it run Chrome extensions?",
        answer: "LispM Browser is a keyboard-driven WebKit browser inside Emacs with tabs, \
                 history, bookmarks and Vimium-style link hints. On macOS 15.4 and later, LispM \
                 Browser runs Chrome Web Store extensions at lispm://extensions with \
                 service-worker support, toolbar popups and M-e actions for extensions like Dark \
                 Reader and Rabby.",
    },
    Question {
        question: "What apps come in the LispM App Store?",
        answer: "The LispM App Store installs open-source Emacs apps, AI harnesses and classic AI \
                 programs in one click via Homebrew or Cargo. Every app bundles its manual as an \
                 agent skill: LispM Office, LispM Mail, Stickies, RSS feeds, MP4 and YouTube \
                 playback, calendar, IRC, Slack, Telegram, 1Password, Doom, Doctor, SHRDLU and \
                 MYCIN.",
    },
    Question {
        question: "Where does LispM store my data, memory and credentials?",
        answer: "LispM runs locally on your computer, keeping its files in ~/.lispm, agent chats \
                 and memory as plain org documents under git, and mail in your local Emacs \
                 directory. LispM Auth encrypts secrets locally with Argon2id and \
                 XChaCha20-Poly1305 in a standalone Common Lisp daemon.",
    },
    Question {
        question: "What is the difference between LispM on Emacs 31 and LispM Experimental on Emacs 32?",
        answer: "Standard LispM runs on GNU Emacs 31.1 on macOS and GNU/Linux with automatic \
                 Software Update. LispM Experimental runs on development builds of GNU Emacs 32 on \
                 macOS for capabilities like OpenGL buffers for Doom, and you can switch live \
                 between Emacs 31 and Emacs 32 in Control Panels → System → Kernel.",
    },
    Question {
        question: "Is LispM free, and can it run in my own Emacs?",
        answer: "Yes. LispM and its subprojects are free software under the GNU Affero General \
                 Public License v3+ for macOS 11+ and Debian or Ubuntu GNU/Linux. GNU Emacs 31.1 \
                 comes bundled, or you can install LispM Agent, LispM Desktop and LispM Browser \
                 directly inside your own Emacs 30.1 or later.",
    },
];

/// The blog's path on the site, its posts under it.
pub const BLOG: &str = "/blog/";

/// The download and Emacs installation guide's path on the site.
pub const DOWNLOAD: &str = "/download/";

/// The privacy policy's path on the site, which every page's footer links.
pub const PRIVACY: &str = "/privacy/";

/// The headline of the download.
pub const DOWNLOAD_HEADLINE: &str = "Get LispM.";

/// The paragraph under it, for those who have never heard of Emacs.
pub const DOWNLOAD_LEAD: &str = "One line installs LispM on a Mac or on GNU/Linux. GNU Emacs, the \
     live Lisp editor LispM is built on, comes bundled: you need not know Emacs to begin.";

/// LispM's icon.
pub const APP_ICON: &str = "assets/lispm-app.png";

/// A system LispM installs on, by the same line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Platform {
    /// macOS, where LispM is LispM.app.
    Mac,
    /// Debian and Ubuntu, and the systems made from them.
    Linux,
}

impl Platform {
    /// Every system LispM installs on, the Mac first.
    pub const ALL: [Platform; 2] = [Platform::Mac, Platform::Linux];

    /// Returns its name.
    pub const fn name(self) -> &'static str {
        match self {
            Platform::Mac => "Mac",
            Platform::Linux => "GNU/Linux",
        }
    }

    /// Returns the button that offers LispM to a visitor on it.
    pub const fn download(self) -> &'static str {
        match self {
            Platform::Mac => "Download for Mac",
            Platform::Linux => "Download for Linux",
        }
    }

    /// Returns the releases and processors LispM runs on there.
    pub const fn requirements(self) -> &'static str {
        match self {
            Platform::Mac => "macOS 11 or later, on Apple silicon or Intel.",
            Platform::Linux => "Debian 12 or 13, or Ubuntu 22.04 or 24.04, on amd64 or arm64.",
        }
    }

    /// Returns what the install line does there.
    pub const fn installer(self) -> &'static str {
        match self {
            Platform::Mac => {
                "On a Mac it installs Apple's command line tools if they are missing \
                 and puts LispM.app in Applications."
            }
            Platform::Linux => {
                "On GNU/Linux it installs LispM's own GNU Emacs 31.1, with \
                 xwidgets, in /opt/lispm/emacs with apt, beside any Emacs of the system, and adds \
                 the lispm command and LispM to the applications menu."
            }
        }
    }

    /// Returns the system a browser runs on, as its user agent tells: GNU/Linux
    /// on a desktop running it, else the Mac, whose download is offered
    /// wherever LispM does not run too. Android and ChromeOS are made from
    /// Linux but are no GNU/Linux the installer knows.
    pub fn of_user_agent(user_agent: &str) -> Platform {
        let linux = user_agent.contains("X11")
            && user_agent.contains("Linux")
            && !user_agent.contains("Android");
        if linux {
            Platform::Linux
        } else {
            Platform::Mac
        }
    }
}

/// The systems LispM runs on, under the hero's buttons.
pub const PLATFORMS: &str = "Free, for macOS and GNU/Linux.";

/// What LispM runs on everywhere, and what it costs.
pub const REQUIREMENTS: &str = "On GNU Emacs 31.1. Free.";

/// The installer, served beside the page; it runs lispm-desktop's.
pub const INSTALLER: &str = "install.sh";

/// The installer the served one runs, to read before running it: the
/// served one runs it as of lispm-desktop's latest release, main's before
/// the first.
pub const INSTALLER_SOURCE: &str =
    "https://github.com/LispMachineLab/lispm-desktop/blob/main/install.sh";

/// LispM Desktop's guide to installing LispM, uninstalling it too.
pub const INSTALLATION: &str = "https://github.com/LispMachineLab/lispm-desktop#installation";

/// The line that installs LispM, or updates it, on every system.
pub const INSTALL: Snippet = Snippet {
    language: Language::Terminal,
    code: "curl -fsSL https://lispm.ai/install.sh | sh",
};

/// Installing LispM, step by step.
pub const DOWNLOAD_STEPS: [Step; 3] = [
    Step {
        title: "Open a terminal.",
        text: "On a Mac, Terminal is in Applications → Utilities (or press Command-Space and type Terminal).",
        snippet: None,
    },
    Step {
        title: "Paste this line, and press Return.",
        text: "The same line installs LispM on a Mac and on GNU/Linux, where apt asks for your password.",
        snippet: Some(INSTALL),
    },
    Step {
        title: "LispM opens.",
        text: "A few minutes later, once Emacs and LispM are in, LispM sits with your apps: in \
               Applications on a Mac and in the menu on GNU/Linux, where the lispm command opens it.",
        snippet: None,
    },
];

/// The headline of the experimental LispM.app.
pub const EXPERIMENTAL_HEADLINE: &str = "LispM Experimental";

/// What it is, and who it is for.
pub const EXPERIMENTAL: &str = "The same LispM on GNU Emacs 32, still in development, for what \
     only Emacs 32 can do, as OpenGL drawn in a buffer. On a Mac only; expect rough edges.";

/// The line that installs the experimental LispM.app, updates it, or moves
/// LispM.app onto it.
pub const INSTALL_EXPERIMENTAL: Snippet = Snippet {
    language: Language::Terminal,
    code: "curl -fsSL https://lispm.ai/install.sh | LISPM_EMACS=32 sh",
};

/// How to leave it, before the line that does.
pub const EXPERIMENTAL_BACK: &str = "Run it again to update. Back to LispM on Emacs 31:";

/// The line that moves LispM.app back onto Emacs 31.
pub const INSTALL_BACK: &str = "curl -fsSL https://lispm.ai/install.sh | LISPM_EMACS=31 sh";

/// What the line does on every system, after the link to the installer.
pub const INSTALLER_DOES: &str = " keeps LispM's files locally in ~/.lispm.";

/// How LispM stays up to date, after what the line does on each system.
pub const INSTALLER_UPDATES: &str = "From then on Software Update brings each release by itself, \
     and the line run again does too.";

/// The headline over the projects' own installs.
pub const OWN_EMACS_HEADLINE: &str = "Already use Emacs?";

/// The paragraph under it.
pub const OWN_EMACS: &str = "LispM Agent, LispM Desktop and LispM Browser also stand on their own, \
     in your own Emacs.";

/// The licence the projects are free software under.
pub const LICENSE: &str = "https://www.gnu.org/licenses/agpl-3.0.html";

/// The organization on GitHub.
pub const ORGANIZATION: &str = "https://github.com/LispMachineLab";

/// LispM's account on X.
pub const X: &str = "https://x.com/lispm_ai";

/// Where to read about other model servers.
pub const BACKENDS: &str = "https://github.com/LispMachineLab/lispm-agent#backends";

/// Where the film's photographs come from.
pub const CREDITS: &str = "The film's photographs are from Wikimedia Commons; their authors and \
     licenses are credited at its end.";

#[cfg(test)]
mod tests;
