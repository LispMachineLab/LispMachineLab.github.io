//! What the page says, as data: its sections, the clips from the films and
//! the pictures beside them, the points each project makes, and the code to
//! install them. The words follow the projects' READMEs and release notes.

/// A section of the page, the target of a link.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anchor {
    /// The film, under the headline.
    Film,
    /// What LispM is.
    Idea,
    /// LispM Agent.
    Agent,
    /// LispM Desktop.
    Desktop,
    /// LispM Browser.
    Browser,
    /// LispM's App Store.
    Store,
    /// LispM Remote.
    Remote,
    /// How to install LispM, and each project on its own.
    Start,
}

impl Anchor {
    /// The sections the navigation bar links to, left to right.
    pub const NAVIGATION: [Anchor; 6] = [
        Anchor::Agent,
        Anchor::Desktop,
        Anchor::Browser,
        Anchor::Store,
        Anchor::Remote,
        Anchor::Start,
    ];

    /// Returns the section's element id.
    pub const fn id(self) -> &'static str {
        match self {
            Anchor::Film => "film",
            Anchor::Idea => "idea",
            Anchor::Agent => "agent",
            Anchor::Desktop => "desktop",
            Anchor::Browser => "browser",
            Anchor::Store => "store",
            Anchor::Remote => "remote",
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
            Anchor::Agent => "Agent",
            Anchor::Desktop => "Desktop",
            Anchor::Browser => "Browser",
            Anchor::Store => "App Store",
            Anchor::Remote => "Remote",
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
    /// A line for macOS's Terminal, named after it for those who know no shell.
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

/// A feature told at length, after a project's story: a headline, a
/// paragraph, a clip if it has one, and its points.
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

/// One of LispM's projects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Project {
    /// Its section.
    pub anchor: Anchor,
    /// Its name, as the eyebrow over the headline.
    pub name: &'static str,
    /// The section's headline.
    pub headline: &'static str,
    /// The paragraph under it.
    pub lead: &'static str,
    /// Its story, each step with a clip.
    pub stages: &'static [Stage],
    /// A feature told at length after the story, if it has one.
    pub spotlight: Option<Spotlight>,
    /// The points under the story.
    pub points: &'static [Point],
    /// Who installs it, and why, in the Get started section; or, for a
    /// project that comes with LispM, where it is found.
    pub audience: &'static str,
    /// How to install it in your own Emacs, if it stands on its own there.
    pub install: Option<Snippet>,
    /// Its repository.
    pub repository: &'static str,
    /// The languages it is written in, the most used first.
    pub languages: &'static [&'static str],
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

const AGENT_STAGES: [Stage; 2] = [
    Stage {
        headline: "Ask, and it works in your project.",
        text: "It reads the code, asks before it runs a command or changes a file, writes the fix and runs the tests.",
        media: shown(
            "assets/video/agent.mp4",
            "assets/video/agent.jpg",
            "An agent is asked to fix a failing test: it reads the code, asks first, writes the fix, \
             the tests pass, and it writes Emacs Lisp to open two files side by side.",
        ),
    },
    Stage {
        headline: "Agents that talk to each other.",
        text: "Claude Code, Codex and Hermes are peers of LispM's agents: any of them asks another for a \
               review and gets the answer as its next message. They use LispM's skills and tools too, \
               which ask you before they write or run anything.",
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

const AGENT_SPOTLIGHT_POINTS: [Point; 4] = [
    Point {
        title: "Out of your way",
        text: "The field drops under the menu bar, and your keyboard never leaves the window you were in.",
    },
    Point {
        title: "It knows the desktop",
        text: "The desktop and its apps teach it, as skills: change the look or the language, open a window, find and install an app.",
    },
    Point {
        title: "It asks first",
        text: "Installing, uninstalling and lowering the sandbox always wait for your yes.",
    },
    Point {
        title: "Memory that keeps itself",
        text: "Long sessions are summed up while you are idle, and a memory grown large consolidates itself, one git commit at a time.",
    },
];

const AGENT_SPOTLIGHT: Spotlight = Spotlight {
    headline: "LispM itself, one field away.",
    text: "M-x ai, or the magnifying glass in the desktop's menu bar, opens one field for the whole \
           computer. As you type, it finds apps, control panels, skills and tools; anything else goes \
           to LispM, a standing agent of the whole computer that keeps one conversation across restarts.",
    clip: None,
    points: &AGENT_SPOTLIGHT_POINTS,
};

const AGENT_POINTS: [Point; 4] = [
    Point {
        title: "Any model",
        text: "Any endpoint speaking the OpenAI Chat Completions API or the Anthropic Messages API, local or hosted.",
    },
    Point {
        title: "Skills on demand",
        text: "Agents search tools, MCP servers and skills, the manuals apps bring, and load what a task needs.",
    },
    Point {
        title: "A sandbox",
        text: "Commands and MCP servers run in a sandbox the operating system enforces, at the level you choose or by rules of your own.",
    },
    Point {
        title: "Checked against Emacs",
        text: "Code in a reply that names a function or a file this Emacs lacks is underlined, and says why.",
    },
];

const DESKTOP_STAGES: [Stage; 3] = [
    Stage {
        headline: "Every project, a workspace.",
        text: "A desktop, a Finder for your files, and an editor with a workspace for each project, its files, agents and shells together.",
        media: shown(
            "assets/video/desktop.mp4",
            "assets/video/desktop.jpg",
            "LispM Desktop starts on its classic-style desktop, opens a project in Emacs, and shows its workspaces.",
        ),
    },
    Stage {
        headline: "Every agent, one sidebar.",
        text: "LispM's agents, Claude Code, Codex and Hermes work side by side, every session listed in the sidebar to resume where it ran.",
        media: shown(
            "assets/video/agents.mp4",
            "assets/video/agents.jpg",
            "Claude Code, Codex and Hermes open beside a LispM agent, all running at once, listed in one sidebar.",
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
];

const FOCUS_POINTS: [Point; 4] = [
    Point {
        title: "Sessions",
        text: "LispM itself, then every agent alive and every recorded session, each marked at work or waiting for you.",
    },
    Point {
        title: "The who line",
        text: "The time, the project and agent, what it is doing, its model and sandbox, and which other agent waits for you.",
    },
    Point {
        title: "No function keys",
        text: "Every action has a key, and none of them is a function key.",
    },
    Point {
        title: "Start in it",
        text: "LispM can start in Focus Mode, with no desktop built at all, and the console runs in a text terminal too.",
    },
];

const DESKTOP_SPOTLIGHT: Spotlight = Spotlight {
    headline: "Focus Mode, a Lisp Machine's console.",
    text: "Press C-c T, or ask LispM, and the desktop gives way to LispM's terminal: sessions on the \
           left, a listener on the right and a who line along the bottom, all from the keyboard, as a \
           Lisp Machine's console was. Leave it, and the desktop is exactly as it was.",
    clip: None,
    points: &FOCUS_POINTS,
};

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

const BROWSER_STAGES: [Stage; 1] = [Stage {
    headline: "Browse from the keyboard.",
    text: "The location field takes an address or a search; f labels every link in sight, and typing a label follows it.",
    media: shown(
        "assets/video/browser.mp4",
        "assets/video/browser.jpg",
        "LispM Browser opens a page, goes to an address from the location field, and labels the links to follow them from the keyboard.",
    ),
}];

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

const BROWSER_SPOTLIGHT: Spotlight = Spotlight {
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
};

const BROWSER_POINTS: [Point; 4] = [
    Point {
        title: "Tabs",
        text: "Every page a tab, with its icon and a close button; history and bookmarks kept as Emacs keeps them.",
    },
    Point {
        title: "Through a proxy",
        text: "HTTP or SOCKS5, handed to WebKit itself, and set in the Network control panel.",
    },
    Point {
        title: "Steady",
        text: "Closing pages and running their scripts no longer crashes Emacs on macOS.",
    },
    Point {
        title: "On its own",
        text: "Any Emacs built with xwidgets runs it; LispM Desktop makes it one of its applications.",
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
        text: "Each agent is a chat on the phone: watch its calls as they run, Allow, Always or Deny the \
               ones it waits on, then tell it what's next.",
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

const REMOTE_POINTS: [Point; 4] = [
    Point {
        title: "No server in between",
        text: "LispM and your phone find each other on Rings Network, peer to peer, by their addresses.",
    },
    Point {
        title: "Sealed end to end",
        text: "Noise keys every session: the nodes relaying a message see who talks to whom, and nothing of what is said.",
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

/// LispM's projects, in the order the page shows them.
pub const PROJECTS: [Project; 5] = [
    Project {
        anchor: Anchor::Agent,
        name: "LispM Agent",
        headline: "Emacs is the harness.",
        lead: "Talk to an agent in a chat laid out like Claude Code's. Underneath, the chat is an org \
               document, its tools are Elisp, and it can evaluate Lisp in the Emacs you are using.",
        stages: &AGENT_STAGES,
        spotlight: Some(AGENT_SPOTLIGHT),
        points: &AGENT_POINTS,
        audience: "The agents, in your own Emacs 30.1 or later. Then M-x lispm in any project.",
        install: Some(Snippet {
            language: Language::EmacsLisp,
            code: "(use-package lispm\n  :vc (:url \"https://github.com/LispMachineLab/lispm-agent\" :rev :newest)\n  :config (lispm-mode-line-mode 1))",
        }),
        repository: "https://github.com/LispMachineLab/lispm-agent",
        languages: &["Emacs Lisp"],
    },
    Project {
        anchor: Anchor::Desktop,
        name: "LispM Desktop",
        headline: "A desktop for your agents.",
        lead: "An Emacs distribution built around LispM Agent, laid out as a classic-style desktop and \
               ready from the first start.",
        stages: &DESKTOP_STAGES,
        spotlight: Some(DESKTOP_SPOTLIGHT),
        points: &DESKTOP_POINTS,
        audience: "What LispM.app runs, in your own Emacs 30.1 or later with SVG, git and curl, on macOS or GNU/Linux.",
        install: Some(Snippet {
            language: Language::Shell,
            code: "git clone https://github.com/LispMachineLab/lispm-desktop ~/lispm-desktop\nemacs --init-directory ~/lispm-desktop",
        }),
        repository: "https://github.com/LispMachineLab/lispm-desktop",
        languages: &["Emacs Lisp", "Rust"],
    },
    Project {
        anchor: Anchor::Browser,
        name: "LispM Browser",
        headline: "The web, inside Emacs.",
        lead: "A WebKit browser in Emacs: tabs, history and bookmarks, the keyboard as Vimium uses \
               pages, and Chrome extensions.",
        stages: &BROWSER_STAGES,
        spotlight: Some(BROWSER_SPOTLIGHT),
        points: &BROWSER_POINTS,
        audience: "The browser, in an Emacs 30.1 or later built with xwidgets.",
        install: Some(Snippet {
            language: Language::EmacsLisp,
            code: "(package-vc-install \"https://github.com/LispMachineLab/lispm-browser\")\n(require 'lispm-browser)\n(lispm-browser-setup)",
        }),
        repository: "https://github.com/LispMachineLab/lispm-browser",
        languages: &["Swift", "Emacs Lisp"],
    },
    Project {
        anchor: Anchor::Store,
        name: "LispM App Store",
        headline: "Apps your agents can drive.",
        lead: "Apps, games and other projects' agents, each a click away. An app brings its manual, a \
               skill, so LispM's agents can work it for you; ask for something no app does yet, and \
               LispM finds one and asks before it installs it.",
        stages: &STORE_STAGES,
        spotlight: None,
        points: &STORE_POINTS,
        audience: "Comes with LispM. Its catalog is plain data, in a repository of its own.",
        install: None,
        repository: "https://github.com/LispMachineLab/lispm-store",
        languages: &["Emacs Lisp"],
    },
    Project {
        anchor: Anchor::Remote,
        name: "LispM Remote",
        headline: "Your agents, in your pocket.",
        lead: "Scan a QR code once. Then, from your phone, follow every agent, approve what it waits on, \
               send it the next task and open LispM's windows, wherever you are. Peer to peer over \
               Rings, sealed end to end.",
        stages: &REMOTE_STAGES,
        spotlight: None,
        points: &REMOTE_POINTS,
        audience: "In LispM's App Store; the phone needs only its browser.",
        install: None,
        repository: "https://github.com/LispMachineLab/lispm-remote",
        languages: &["Rust", "Emacs Lisp", "TypeScript"],
    },
];

/// Where the site is served.
pub const SITE: &str = "https://lispm.ai/";

/// The blog's path on the site, its posts under it.
pub const BLOG: &str = "/blog/";

/// The headline of the download.
pub const DOWNLOAD_HEADLINE: &str = "Get LispM.";

/// The paragraph under it, for those who have never heard of Emacs.
pub const DOWNLOAD_LEAD: &str = "LispM is a Mac app. Emacs, the editor it is made of, comes inside it: \
     you need not know Emacs to begin.";

/// LispM.app's icon.
pub const APP_ICON: &str = "assets/lispm-app.png";

/// What LispM.app runs on.
pub const REQUIREMENTS: &str =
    "macOS 11 or later, on Apple silicon or Intel. On GNU Emacs 31.1. Free.";

/// The installer, served beside the page; it runs lispm-desktop's.
pub const INSTALLER: &str = "install.sh";

/// The installer the served one runs, to read before running it: the
/// served one runs it as of lispm-desktop's latest release, main's before
/// the first.
pub const INSTALLER_SOURCE: &str =
    "https://github.com/LispMachineLab/lispm-desktop/blob/main/install.sh";

/// The line that installs LispM.app, or updates it.
pub const INSTALL: Snippet = Snippet {
    language: Language::Terminal,
    code: "curl -fsSL https://lispm.ai/install.sh | sh",
};

/// Installing LispM.app, step by step.
pub const DOWNLOAD_STEPS: [Step; 3] = [
    Step {
        title: "Open Terminal.",
        text: "It is in Applications, in the Utilities folder; or press Command-Space and type Terminal.",
        snippet: None,
    },
    Step {
        title: "Paste this line, and press Return.",
        text: "",
        snippet: Some(INSTALL),
    },
    Step {
        title: "LispM opens.",
        text: "A few minutes later, once Emacs and LispM are in. From then on it is in Applications \
               with your other apps.",
        snippet: None,
    },
];

/// The headline of the experimental LispM.app.
pub const EXPERIMENTAL_HEADLINE: &str = "LispM Experimental";

/// What it is, and who it is for.
pub const EXPERIMENTAL: &str = "The same LispM on GNU Emacs 32, still in development, for what \
     only Emacs 32 can do, as OpenGL drawn in a buffer. Expect rough edges.";

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

/// What the line does, after the link to the installer.
pub const INSTALLER_DOES: &str = " installs Apple's command line tools if they are missing, puts \
     LispM.app in Applications and LispM's files in ~/.lispm. From then on Software Update brings \
     each release by itself, and the line run again does too; delete both to uninstall.";

/// The headline over the projects' own installs.
pub const OWN_EMACS_HEADLINE: &str = "Already use Emacs?";

/// The paragraph under it.
pub const OWN_EMACS: &str = "LispM Agent, LispM Desktop and LispM Browser also stand on their own, \
     in your own Emacs.";

/// The licence the projects are free software under.
pub const LICENSE: &str = "https://www.gnu.org/licenses/agpl-3.0.html";

/// The organization on GitHub.
pub const ORGANIZATION: &str = "https://github.com/LispMachineLab";

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
        Anchor::Agent,
        Anchor::Desktop,
        Anchor::Browser,
        Anchor::Store,
        Anchor::Remote,
        Anchor::Start,
    ];

    fn media() -> impl Iterator<Item = Media> {
        PROJECTS
            .iter()
            .flat_map(|p| p.stages.iter().map(|s| s.media))
    }

    fn clips() -> impl Iterator<Item = Clip> {
        std::iter::once(FILM)
            .chain(media().filter_map(|m| match m {
                Media::Loop(clip) => Some(clip),
                Media::Picture(_) => None,
            }))
            .chain(PROJECTS.iter().filter_map(|p| p.spotlight?.clip))
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
    fn every_project_has_its_own_section() {
        for (i, a) in PROJECTS.iter().enumerate() {
            assert!(ALL.contains(&a.anchor), "{}", a.name);
            for b in PROJECTS.iter().skip(i + 1) {
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
    fn links_stay_in_the_organization() {
        for link in PROJECTS
            .iter()
            .map(|p| p.repository)
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
                    install.code.contains(project.repository),
                    "{}",
                    project.name
                );
                assert!(!install.code.ends_with('\n'), "{}", project.name);
            }
        }
    }

    #[test]
    fn every_project_has_a_story() {
        for project in PROJECTS {
            assert!(!project.stages.is_empty(), "{}", project.name);
            assert!(!project.points.is_empty(), "{}", project.name);
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
    }
}
