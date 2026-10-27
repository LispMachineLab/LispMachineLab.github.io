//! What the page says, as data: its sections, the chapters that tell what
//! LispM does with the clips from the films and the pictures beside them,
//! the projects LispM is made of, and the code to install them. The words
//! follow the projects' READMEs and release notes.
//!
//! The chapters follow what a visitor can do, not how the code is split:
//! a feature is told once, in the chapter it matters to.

/// A section of the page, the target of a link.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anchor {
    /// The film, under the headline.
    Film,
    /// What LispM is.
    Idea,
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
}

impl Anchor {
    /// The sections the navigation bar links to, left to right.
    pub const NAVIGATION: [Anchor; 6] = [
        Anchor::Agents,
        Anchor::Desktop,
        Anchor::Store,
        Anchor::Remote,
        Anchor::Trust,
        Anchor::Start,
    ];

    /// Returns the section's element id. The agents' keeps the id it had
    /// when the section was LispM Agent's, so links to it still land there.
    pub const fn id(self) -> &'static str {
        match self {
            Anchor::Film => "film",
            Anchor::Idea => "idea",
            Anchor::Agents => "agent",
            Anchor::Desktop => "desktop",
            Anchor::Store => "store",
            Anchor::Remote => "remote",
            Anchor::Trust => "trust",
            Anchor::Start => "start",
        }
    }

    /// Returns the link to the section, `#` and its id.
    pub fn href(self) -> String {
        format!("#{}", self.id())
    }

    /// Returns the section's name in the navigation bar.
    pub const fn name(self) -> &'static str {
        match self {
            Anchor::Film => "Film",
            Anchor::Idea => "Idea",
            Anchor::Agents => "Agents",
            Anchor::Desktop => "Desktop",
            Anchor::Store => "App Store",
            Anchor::Remote => "Remote",
            Anchor::Trust => "Trust",
            Anchor::Start => "Download",
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

/// Returns the clip playing `video`, `poster` before it, described by `description`.
const fn clip(video: &'static str, poster: &'static str, description: &'static str) -> Clip {
    Clip {
        video,
        poster,
        description,
    }
}

/// Returns a looping clip playing `video`, `poster` before it, described by
/// `description`.
const fn shown(video: &'static str, poster: &'static str, description: &'static str) -> Media {
    Media::Loop(clip(video, poster, description))
}

/// The film, with sound, under the headline.
pub const FILM: Clip = clip(
    "assets/video/lispm-film.mp4",
    "assets/video/lispm-film.jpg",
    "A 57-second film: from the Lisp Machines to LispM Desktop, an agent fixing a failing test \
     inside Emacs, Claude Code, Codex and Hermes side by side, and windows dragged off the desktop.",
);

/// LispM's tagline.
pub const TAGLINE: &str = "A Lisp Machine for agents.";

/// The paragraph under the tagline.
pub const SUMMARY: &str = "AI agents that run inside Emacs. Their conversations are org documents, \
     their tools are Lisp, and Emacs itself is the harness.";

/// The headline of the idea.
pub const IDEA_HEADLINE: &str = "The editor, the operating system and their agents, one world.";

/// The paragraph under it.
pub const IDEA: &str = "On a Lisp Machine everything was a live Lisp object, and the editor and the \
     system were one. Its inhabitants were programs. Emacs kept that way of working, if not the \
     machine; LispM makes room in it for agents.";

/// What makes LispM's agents what they are.
pub const IDEA_POINTS: [Point; 4] = [
    Point {
        title: "Conversations are documents.",
        text: "Each agent's chat is an org document, and that document is its context.",
    },
    Point {
        title: "Tools are Lisp.",
        text: "An agent's tools are Elisp functions. It writes Lisp to combine them, and drives Emacs itself.",
    },
    Point {
        title: "Approvals are TODOs.",
        text: "A command or an edit waits as an org TODO entry until you allow it.",
    },
    Point {
        title: "Memory is git.",
        text: "What agents remember is org files under git.",
    },
];

const AGENTS_STAGES: [Stage; 3] = [
    Stage {
        headline: "It works in your project.",
        text: "It reads the code, writes the fix and runs the tests, in a chat that is an org \
               document, with tools that are Lisp.",
        media: shown(
            "assets/video/agent.mp4",
            "assets/video/agent.jpg",
            "An agent is asked to fix a failing test: it reads the code, asks first, writes the fix, \
             the tests pass, and it writes Emacs Lisp to open two files side by side.",
        ),
    },
    Stage {
        headline: "Every agent, one sidebar.",
        text: "Claude Code, Codex and Hermes work beside LispM's agents, every session listed in the \
               sidebar, to resume where it ran.",
        media: shown(
            "assets/video/agents.mp4",
            "assets/video/agents.jpg",
            "Claude Code, Codex and Hermes open beside a LispM agent, all running at once, listed in one sidebar.",
        ),
    },
    Stage {
        headline: "And they talk to each other.",
        text: "Any of them asks another for a review and gets the answer as its next message, and \
               uses LispM's skills and tools as LispM's own agents do.",
        media: Media::Picture(Picture {
            image: "assets/blog/your-agents-can-talk-to-each-other/claude-asks-codex.webp",
            width: 2000,
            height: 1121,
            description: "LispM's classic-style desktop with Claude Code and Codex side by side, both \
                          running. Claude Code asks Codex to review a function; Codex gets the request \
                          as its next message, and its reply comes back to Claude Code as one.",
        }),
    },
];

const BAR_POINTS: [Point; 2] = [
    Point {
        title: "Out of your way",
        text: "The field drops under the menu bar, and your keyboard never leaves the window you were in.",
    },
    Point {
        title: "It knows the desktop",
        text: "The desktop and its apps teach it, as skills: change the look or the language, open a window, find and install an app.",
    },
];

const AGENTS_SPOTLIGHTS: [Spotlight; 1] = [Spotlight {
    headline: "LispM itself, one field away.",
    text: "M-x ai, or the magnifying glass in the menu bar, opens one field for the whole computer. \
           As you type, it finds apps, control panels, skills and tools; anything else goes to LispM, \
           a standing agent of the whole computer that keeps one conversation across restarts.",
    clip: None,
    points: &BAR_POINTS,
}];

const AGENTS_POINTS: [Point; 4] = [
    Point {
        title: "Skills on demand",
        text: "Agents search tools, MCP servers and skills, the manuals apps bring, and load what a task needs.",
    },
    Point {
        title: "Memory that keeps itself",
        text: "Long sessions are summed up while you are idle, and a memory grown large consolidates itself, one git commit at a time.",
    },
    Point {
        title: "Checked against Emacs",
        text: "Code in a reply that names a function or a file this Emacs lacks is underlined, and says why.",
    },
    Point {
        title: "Woken by Emacs",
        text: "Hooks and timers start agents, and agents hand work to each other.",
    },
];

const DESKTOP_STAGES: [Stage; 3] = [
    Stage {
        headline: "Every project, a workspace.",
        text: "A Finder for your files, and an editor with a workspace for each project, its files, \
               agents and shells together.",
        media: shown(
            "assets/video/desktop.mp4",
            "assets/video/desktop.jpg",
            "LispM Desktop starts on its classic-style desktop, opens a project in Emacs, and shows its workspaces.",
        ),
    },
    Stage {
        headline: "Make it yours.",
        text: "Drag windows off the desktop, hide it and bring it back in a click, and change the look from the Control Strip.",
        media: shown(
            "assets/video/windows.mp4",
            "assets/video/windows.jpg",
            "A window is dragged off the desktop, the desktop hides and comes back, and the theme turns to Dracula.",
        ),
    },
    Stage {
        headline: "The web, from the keyboard.",
        text: "A WebKit browser with tabs, history and bookmarks. The location field takes an address \
               or a search; f labels every link in sight, and typing a label follows it.",
        media: shown(
            "assets/video/browser.mp4",
            "assets/video/browser.jpg",
            "LispM Browser opens a page, goes to an address from the location field, and labels the links to follow them from the keyboard.",
        ),
    },
];

const EXTENSION_POINTS: [Point; 4] = [
    Point {
        title: "From the Web Store",
        text: "The puzzle piece opens the Chrome Web Store; Add to LispM on an extension's page adds it: its signature is checked, and what it may do is asked first.",
    },
    Point {
        title: "On the toolbar",
        text: "Each pinned extension has its button there, badge and all: a click opens its popup under it, a right click its menu.",
    },
    Point {
        title: "A page of their own",
        text: "lispm://extensions, like chrome://extensions: turn them on and off, see what they may do, reload, remove, or load an unpacked one.",
    },
    Point {
        title: "And from the keyboard",
        text: "M-e presses an extension's button for the page in front, and every action is a command, its questions asked in the minibuffer.",
    },
];

const DESKTOP_SPOTLIGHTS: [Spotlight; 2] = [
    Spotlight {
        headline: "Chrome extensions, inside Emacs.",
        text: "On macOS 15.4 and later, WebKit runs Chrome extensions itself, as Safari runs its own. \
               The APIs WebKit lacks are filled in, so even heavy extensions work: the Rabby wallet runs \
               its worker, its page provider and its popup, and its approvals open in windows of their own.",
        clip: Some(clip(
            "assets/video/extensions.mp4",
            "assets/video/extensions.jpg",
            "The puzzle piece on the browser's toolbar opens the Chrome Web Store in a tab, Add to \
             LispM on Dark Reader's page adds it once the question in the minibuffer is answered, \
             the Lisp machine article turns dark, and Dark Reader's own toolbar button opens its \
             popup under it.",
        )),
        points: &EXTENSION_POINTS,
    },
    Spotlight {
        headline: "Focus Mode, a Lisp Machine's console.",
        text: "Press C-c T, or ask LispM, and the desktop gives way to LispM's terminal: the sessions on \
               the left, a listener on the right and a who line along the bottom, all from the keyboard, \
               as a Lisp Machine's console was. Leave it, and the desktop is exactly as it was; or start \
               LispM in it, with no desktop at all.",
        clip: None,
        points: &[],
    },
];

const DESKTOP_POINTS: [Point; 4] = [
    Point {
        title: "Ready at once",
        text: "The first start asks for vim or Emacs keys and an appearance, then installs the rest.",
    },
    Point {
        title: "Five looks",
        text: "From Platinum greys to glowing phosphor and neon behind glass, each with colours and icons of its own.",
    },
    Point {
        title: "Seven languages",
        text: "English (US and UK), German, French, Japanese, and Simplified and Traditional Chinese. The agents follow.",
    },
    Point {
        title: "Apps of their own",
        text: "Each app opens as a Mac app of its own, from Launchpad, Spotlight or the Dock, with its own Dock tile and menu bar.",
    },
];

const STORE_STAGES: [Stage; 2] = [
    Stage {
        headline: "Every app, a page of its own.",
        text: "What it does, who made it, its licence and what it needs. Get installs it with whatever it \
               needs, from Homebrew or Cargo, and uninstalling takes those away when nothing else uses them.",
        media: Media::Picture(Picture {
            image: "assets/blog/three-old-ais-running-again/app-store-shrdlu.webp",
            width: 2000,
            height: 1125,
            description: "The App Store on LispM's classic-style desktop: its categories, from News to \
                          Classic AI, and SHRDLU's page, with its banner, what it does, and its \
                          Uninstall and Open buttons.",
        }),
    },
    Stage {
        headline: "Three old AIs, running again.",
        text: "Classic AI holds Doctor from 1966, SHRDLU from 1970, moving its blocks as you talk to it, \
               and MYCIN from the 1970s, which shows the rule it is trying when you ask why.",
        media: shown(
            "assets/video/classic-ai.mp4",
            "assets/video/classic-ai.jpg",
            "SHRDLU in LispM: told to find a block taller than the one it holds and put it into the \
             box, it plans the moves, and its hand carries the blocks across the table.",
        ),
    },
];

const STORE_POINTS: [Point; 4] = [
    Point {
        title: "Skills come with apps",
        text: "Install an app and its skill comes too: agents read feeds, play music, stick notes, chat on IRC, Slack and Telegram, and check security.",
    },
    Point {
        title: "AI Harness",
        text: "Hermes, Holon and OpenClaw, agents of their own, join Claude Code and Codex in the editor.",
    },
    Point {
        title: "For every day",
        text: "Stickies, a feed reader, music and YouTube, a calendar, IRC, Slack and Telegram, and 1Password.",
    },
    Point {
        title: "And games",
        text: "Tetris, Solitaire, Life and Dunnet, and Doom, drawn with OpenGL on the experimental LispM.",
    },
];

const REMOTE_STAGES: [Stage; 3] = [
    Stage {
        headline: "Pair once.",
        text: "LispM shows a QR code, your phone scans it at lispm.ai/remote, and both screens show the \
               same code for you to approve. Nothing to install on the phone, no port to open.",
        media: shown(
            "assets/video/remote-pair.mp4",
            "assets/video/remote-pair.jpg",
            "LispM Remote shows a QR code on the desktop, a phone scans it, both screens show the \
             fingerprint 3981-CF90, and LispM approves the phone.",
        ),
    },
    Stage {
        headline: "Approve from anywhere.",
        text: "Each agent is a chat on the phone: watch its calls as they run, answer the ones it waits \
               on, then tell it what's next.",
        media: shown(
            "assets/video/remote-approve.mp4",
            "assets/video/remote-approve.jpg",
            "On the phone, an agent works through a fix and asks to run make test; Allow is tapped, \
             and all three tests pass.",
        ),
    },
    Stage {
        headline: "Every window, too.",
        text: "LispM's windows come along, redrawn on the phone, not streamed as video.",
        media: shown(
            "assets/video/remote-windows.mp4",
            "assets/video/remote-windows.jpg",
            "The phone opens slug.el from LispM's windows, redrawn there, and the film says: no \
             server in between, peer to peer over Rings, sealed end to end.",
        ),
    },
];

const REMOTE_POINTS: [Point; 3] = [
    Point {
        title: "No server in between",
        text: "LispM and your phone find each other on Rings Network, peer to peer, by their addresses.",
    },
    Point {
        title: "Back by itself",
        text: "When the network drops and comes back, the phone reconnects, with no new scan.",
    },
    Point {
        title: "From the App Store",
        text: "One click installs it, its Rings node downloaded for your computer and checked against the checksum it pins.",
    },
];

const TRUST_POINTS: [Point; 6] = [
    Point {
        title: "It asks first",
        text: "A command, an edit or an install waits for your yes, in Emacs or on your phone. Reading never needs one.",
    },
    Point {
        title: "A sandbox",
        text: "Commands, MCP servers and services run in a sandbox the operating system enforces, at the level you choose. If it cannot start, they do not run.",
    },
    Point {
        title: "Checked",
        text: "LispM Security checks what every agent is given to run, Claude Code's and Codex's too, pins trusted tools, and looks packages up in OSV.",
    },
    Point {
        title: "Any model",
        text: "Any endpoint speaking the OpenAI Chat Completions API or the Anthropic Messages API, local or hosted, with your own keys.",
    },
    Point {
        title: "Sealed end to end",
        text: "Between your phone and LispM, the nodes that relay a message see who talks to whom, and nothing of what is said.",
    },
    Point {
        title: "Free software",
        text: "Every part is open source, under the GNU AGPL: read it, change it, run it yourself.",
    },
];

/// The chapters, in the order the page tells them: what the agents do, the
/// desktop they live on, the apps they use, LispM from a phone, and why you
/// stay in charge.
pub const CHAPTERS: [Chapter; 5] = [
    Chapter {
        anchor: Anchor::Agents,
        eyebrow: "Agents",
        headline: "Ask, and it gets done.",
        lead: "Talk to an agent in a chat laid out like Claude Code's. Underneath, the chat is an org \
               document, its tools are Elisp, and it can evaluate Lisp in the Emacs you are using.",
        stages: &AGENTS_STAGES,
        spotlights: &AGENTS_SPOTLIGHTS,
        points: &AGENTS_POINTS,
        repository: "https://github.com/LispMachineLab/lispm-agent",
    },
    Chapter {
        anchor: Anchor::Desktop,
        eyebrow: "Desktop",
        headline: "A desktop for your agents.",
        lead: "LispM is laid out as a classic-style desktop, ready from the first start: a Finder, an \
               editor, a browser and apps, each in windows of its own.",
        stages: &DESKTOP_STAGES,
        spotlights: &DESKTOP_SPOTLIGHTS,
        points: &DESKTOP_POINTS,
        repository: "https://github.com/LispMachineLab/lispm-desktop",
    },
    Chapter {
        anchor: Anchor::Store,
        eyebrow: "App Store",
        headline: "Apps your agents can drive.",
        lead: "Apps, games and other projects' agents, each a click away. An app brings its manual, a \
               skill, so LispM's agents can work it for you; ask for something no app does yet, and \
               LispM finds one and asks before it installs it.",
        stages: &STORE_STAGES,
        spotlights: &[],
        points: &STORE_POINTS,
        repository: "https://github.com/LispMachineLab/lispm-store",
    },
    Chapter {
        anchor: Anchor::Remote,
        eyebrow: "LispM Remote",
        headline: "Your agents, in your pocket.",
        lead: "Scan a QR code once. Then, from your phone, follow every agent, approve what it waits on, \
               send it the next task and open LispM's windows, wherever you are.",
        stages: &REMOTE_STAGES,
        spotlights: &[],
        points: &REMOTE_POINTS,
        repository: "https://github.com/LispMachineLab/lispm-remote",
    },
    Chapter {
        anchor: Anchor::Trust,
        eyebrow: "Trust",
        headline: "You stay in charge.",
        lead: "Agents act on your computer, so LispM shows you every step they take and leaves the risky \
               ones to you.",
        stages: &[],
        spotlights: &[],
        points: &TRUST_POINTS,
        repository: "https://github.com/LispMachineLab/lispm-security",
    },
];

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
            audience: "The browser, in an Emacs 30.1 or later built with xwidgets.",
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

/// The blog's path on the site, its posts under it.
pub const BLOG: &str = "/blog/";

/// The headline of the download.
pub const DOWNLOAD_HEADLINE: &str = "Get LispM.";

/// The paragraph under it, for those who have never heard of Emacs.
pub const DOWNLOAD_LEAD: &str = "One line installs LispM, on a Mac or on GNU/Linux. Emacs, the \
     editor it is made of, comes with it: you need not know Emacs to begin.";

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
        text: "On a Mac, Terminal is in Applications, in the Utilities folder; or press Command-Space \
               and type Terminal.",
        snippet: None,
    },
    Step {
        title: "Paste this line, and press Return.",
        text: "The same line on a Mac and on GNU/Linux, where apt asks for your password.",
        snippet: Some(INSTALL),
    },
    Step {
        title: "LispM opens.",
        text: "A few minutes later, once Emacs and LispM are in. From then on it is with your other \
               apps: in Applications on a Mac, in the applications menu on GNU/Linux, where the \
               lispm command opens it too.",
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
pub const INSTALLER_DOES: &str = " keeps LispM's files in ~/.lispm.";

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
mod tests {
    use super::*;
    use std::path::Path;

    /// Every section, top to bottom.
    const ALL: [Anchor; 8] = [
        Anchor::Film,
        Anchor::Idea,
        Anchor::Agents,
        Anchor::Desktop,
        Anchor::Store,
        Anchor::Remote,
        Anchor::Trust,
        Anchor::Start,
    ];

    fn media() -> impl Iterator<Item = Media> {
        CHAPTERS
            .iter()
            .flat_map(|c| c.stages.iter().map(|s| s.media))
    }

    fn spotlights() -> impl Iterator<Item = &'static Spotlight> {
        CHAPTERS.iter().flat_map(|c| c.spotlights.iter())
    }

    fn clips() -> impl Iterator<Item = Clip> {
        std::iter::once(FILM)
            .chain(media().filter_map(|m| match m {
                Media::Loop(clip) => Some(clip),
                Media::Picture(_) => None,
            }))
            .chain(spotlights().filter_map(|s| s.clip))
    }

    fn pictures() -> impl Iterator<Item = Picture> {
        media().filter_map(|m| match m {
            Media::Picture(picture) => Some(picture),
            Media::Loop(_) => None,
        })
    }

    /// Returns the canvas of an extended WebP, the kind the pictures are:
    /// its width and height, each stored less one in three bytes.
    fn webp_size(bytes: &[u8]) -> Option<(u32, u32)> {
        if bytes.get(..4)? != b"RIFF" || bytes.get(8..16)? != b"WEBPVP8X" {
            return None;
        }
        let side = |at: usize| match bytes.get(at..at + 3)? {
            [low, middle, high] => {
                Some(1 + (u32::from(*high) << 16 | u32::from(*middle) << 8 | u32::from(*low)))
            }
            _ => None,
        };
        Some((side(24)?, side(27)?))
    }

    #[test]
    fn anchors_have_distinct_ids() {
        for (i, a) in ALL.iter().enumerate() {
            for b in ALL.iter().skip(i + 1) {
                assert_ne!(a.id(), b.id());
            }
        }
    }

    #[test]
    fn every_chapter_has_its_own_section() {
        for (i, a) in CHAPTERS.iter().enumerate() {
            assert!(ALL.contains(&a.anchor), "{}", a.eyebrow);
            for b in CHAPTERS.iter().skip(i + 1) {
                assert_ne!(a.anchor, b.anchor);
            }
        }
    }

    #[test]
    fn every_clip_and_poster_is_in_the_assets() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        for clip in clips() {
            assert!(
                clip.video.ends_with(".mp4") && root.join(clip.video).is_file(),
                "{}",
                clip.video
            );
            assert!(
                clip.poster.ends_with(".jpg") && root.join(clip.poster).is_file(),
                "{}",
                clip.poster
            );
            assert!(!clip.description.is_empty());
        }
    }

    #[test]
    fn every_picture_is_in_the_assets_at_its_size() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut seen = 0;
        for picture in pictures() {
            let bytes = std::fs::read(root.join(picture.image)).unwrap_or_default();
            assert_eq!(
                webp_size(&bytes),
                Some((picture.width, picture.height)),
                "{}",
                picture.image
            );
            assert!(!picture.description.is_empty());
            seen += 1;
        }
        assert!(seen > 0);
    }

    #[test]
    fn every_clip_is_told_once() {
        let all = clips().map(|clip| clip.video).collect::<Vec<_>>();
        for (i, a) in all.iter().enumerate() {
            assert!(!all.iter().skip(i + 1).any(|b| b == a), "{a}");
        }
    }

    #[test]
    fn links_stay_in_the_organization() {
        for link in PROJECTS
            .iter()
            .map(|p| p.repository)
            .chain(CHAPTERS.iter().map(|c| c.repository))
            .chain([ORGANIZATION, BACKENDS])
        {
            assert!(link.starts_with(ORGANIZATION), "{link}");
        }
    }

    #[test]
    fn snippets_install_from_their_own_repository() {
        for project in PROJECTS {
            if let Some(install) = project.install {
                assert!(
                    install.snippet.code.contains(project.repository),
                    "{}",
                    project.name
                );
                assert!(!install.snippet.code.ends_with('\n'), "{}", project.name);
            }
        }
    }

    #[test]
    fn the_projects_on_their_own_come_first() {
        let first_without = PROJECTS
            .iter()
            .position(|p| p.install.is_none())
            .unwrap_or(PROJECTS.len());
        assert!(
            PROJECTS
                .iter()
                .skip(first_without)
                .all(|p| p.install.is_none())
        );
        assert!(first_without > 0);
    }

    #[test]
    fn every_chapter_makes_its_points() {
        for chapter in CHAPTERS {
            assert!(!chapter.points.is_empty(), "{}", chapter.eyebrow);
        }
    }

    #[test]
    fn the_experimental_lines_are_the_install_line_choosing_an_emacs() {
        for (emacs, line) in [("32", INSTALL_EXPERIMENTAL.code), ("31", INSTALL_BACK)] {
            assert_eq!(
                line,
                INSTALL
                    .code
                    .replace("| sh", &format!("| LISPM_EMACS={emacs} sh"))
            );
        }
    }

    #[test]
    fn the_install_line_runs_the_installer_served_here() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert_eq!(INSTALL.code, format!("curl -fsSL {SITE}{INSTALLER} | sh"));
        let served =
            std::fs::read_to_string(root.join("static").join(INSTALLER)).unwrap_or_default();
        assert!(served.starts_with("#!/bin/sh\n"));
        let source = INSTALLER_SOURCE
            .replace("github.com", "raw.githubusercontent.com")
            .replace("/blob/main/", "/${release:-main}/");
        assert!(served.contains(&source), "{source}");
        assert!(
            served.contains(
                "https://api.github.com/repos/LispMachineLab/lispm-desktop/releases/latest"
            )
        );
        let index = std::fs::read_to_string(root.join("index.html")).unwrap_or_default();
        assert!(index.contains(&format!("rel=\"copy-file\" href=\"static/{INSTALLER}\"")));
        assert!(INSTALLER_SOURCE.starts_with(ORGANIZATION));
    }

    #[test]
    fn the_steps_hold_the_install_line_once() {
        let lines = DOWNLOAD_STEPS
            .iter()
            .filter_map(|step| step.snippet)
            .collect::<Vec<_>>();
        assert_eq!(lines, [INSTALL]);
        assert!(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(APP_ICON)
                .is_file()
        );
    }

    #[test]
    fn hrefs_point_at_ids() {
        assert_eq!(Anchor::Start.href(), "#start");
        assert_eq!(Anchor::Agents.href(), "#agent");
    }

    #[test]
    fn a_desktop_on_gnu_linux_is_offered_linux_and_every_other_browser_the_mac() {
        for (agent, platform) in [
            (
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) \
                 Chrome/141.0.0.0 Safari/537.36",
                Platform::Linux,
            ),
            (
                "Mozilla/5.0 (X11; Ubuntu; Linux aarch64; rv:144.0) Gecko/20100101 Firefox/144.0",
                Platform::Linux,
            ),
            (
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 \
                 (KHTML, like Gecko) Version/26.0 Safari/605.1.15",
                Platform::Mac,
            ),
            (
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
                 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36",
                Platform::Mac,
            ),
            (
                "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) \
                 Chrome/141.0.0.0 Mobile Safari/537.36",
                Platform::Mac,
            ),
            (
                "Mozilla/5.0 (X11; CrOS x86_64 14541.0.0) AppleWebKit/537.36 \
                 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36",
                Platform::Mac,
            ),
            (
                "Mozilla/5.0 (iPhone; CPU iPhone OS 18_6 like Mac OS X) AppleWebKit/605.1.15 \
                 (KHTML, like Gecko) Version/18.6 Mobile/15E148 Safari/604.1",
                Platform::Mac,
            ),
            ("", Platform::Mac),
        ] {
            assert_eq!(Platform::of_user_agent(agent), platform, "{agent}");
        }
    }

    #[test]
    fn every_platform_is_told_what_it_needs_and_what_the_line_does_there() {
        for platform in Platform::ALL {
            assert!(
                platform
                    .download()
                    .ends_with(platform.name().trim_start_matches("GNU/"))
            );
            assert!(
                platform.requirements().ends_with('.'),
                "{}",
                platform.name()
            );
            assert!(
                platform.installer().starts_with("On "),
                "{}",
                platform.name()
            );
        }
        assert!(Platform::Linux.installer().contains("/opt/lispm/emacs"));
        assert!(DOWNLOAD_LEAD.contains(Platform::Linux.name()));
        assert!(INSTALLATION.starts_with(ORGANIZATION));
    }
}
