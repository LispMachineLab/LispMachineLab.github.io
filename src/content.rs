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

impl Media {
    /// Returns the still it shows first: a clip's poster, or the picture.
    pub const fn still(self) -> &'static str {
        match self {
            Media::Loop(clip) => clip.poster,
            Media::Picture(picture) => picture.image,
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
    "A 57-second film: from the Lisp Machines to LispM Desktop, an agent fixing a failing test \
     inside Emacs, Claude Code, Codex and Hermes side by side, and windows dragged off the desktop.",
    OPENED,
);

/// LispM's tagline.
pub const TAGLINE: &str = "A Lisp Machine for agents.";

/// The paragraph under the tagline.
pub const SUMMARY: &str = "LispM runs AI agents inside Emacs. Their conversations are org \
     documents, their tools are Lisp, and Emacs itself is the harness.";

/// The headline of the idea.
pub const IDEA_HEADLINE: &str = "The editor, the operating system and their agents, one world.";

/// The paragraph under it.
pub const IDEA: &str = "On a Lisp Machine everything was a live Lisp object, and the editor and the \
     system were one. Its inhabitants were programs. Emacs, born beside it at MIT in 1976, has kept \
     that way of working for fifty years, if not the machine. LispM makes room in it for agents: \
     the harness modern AI needs.";

/// What makes LispM's agents what they are.
pub const IDEA_POINTS: [Point; 4] = [
    Point {
        title: "Conversations are documents.",
        text: "In LispM, each agent's chat is an org document, and that document is its context.",
    },
    Point {
        title: "Tools are Lisp.",
        text: "A LispM agent's tools are Elisp functions. It writes Lisp to combine them, and drives Emacs itself.",
    },
    Point {
        title: "Approvals are TODOs.",
        text: "In LispM, a command or an edit waits as an org TODO entry until you allow it.",
    },
    Point {
        title: "Memory is git.",
        text: "What LispM's agents remember is org files under git.",
    },
];

/// The steps of LispM Agent's story, shown on `/agent/`.
pub const AGENTS_STAGES: [Stage; 5] = [
    Stage {
        headline: "It works in your project.",
        text: "LispM's agent reads the code, writes the fix and runs the tests, in a chat that is \
               an org document, with tools that are Lisp.",
        media: shown(
            "assets/video/agent.mp4",
            "assets/video/agent.jpg",
            "An agent is asked to fix a failing test: it reads the code, asks first, writes the fix, \
             the tests pass, and it writes Emacs Lisp to open two files side by side.",
            OPENED,
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
            OPENED,
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
    Stage {
        headline: "Characters with a soul of their own.",
        text: "In the Characters app, give an agent a name, a pixel-art portrait, a soul, a role and \
               a model of its own. Each character—like @Ada, @Milo and @Juno—becomes its own \
               application in the Applications folder that you, Claude Code and Codex can message by name.",
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
        text: "Press C-c O to open LispM Office: an animated pixel office where every agent session \
               has a desk. Working agents type at their laptops, idle agents hold coffee, waiting \
               agents raise a hand with an approval badge, and clicking any desk opens its inspector, \
               mailbox and pending tool calls.",
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
    text: "M-x ai, or the magnifying glass in the menu bar, drops one field under the menu bar for \
           the whole computer while your keyboard stays in the window you were in. As you type, \
           LispM finds apps, control panels, skills and tools; anything else goes to LispM, a \
           standing agent taught by the desktop and its apps that keeps one conversation across restarts.",
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
        text: "Press C-c O for an 8-room animated pixel office where every agent session has a desk: working agents type, idle agents hold coffee, waiting agents raise a hand, and clicking a desk inspects its work or approves a call.",
    },
    Point {
        title: "Characters (@Ada)",
        text: "Create persistent agents in the Characters app with a name, picture, soul and model; each becomes its own desktop app that you, Claude Code and Codex can message by name (@Ada).",
    },
    Point {
        title: "Skills on demand",
        text: "LispM's agents search tools, MCP servers and skills, the manuals apps bring, and load what a task needs.",
    },
    Point {
        title: "Memory that keeps itself",
        text: "In LispM, long sessions are summed up while you are idle, and a memory grown large consolidates itself, one git commit at a time.",
    },
    Point {
        title: "Checked against Emacs",
        text: "In LispM, code in a reply that names a function or a file this Emacs lacks is underlined, and says why.",
    },
    Point {
        title: "Woken by Emacs",
        text: "In LispM, hooks and timers start agents, and agents hand work to each other.",
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
        description: "A 1-bit monochrome Dynamic Windows workstation independent of Mac OS and \
                      Windows. Instead of desktop icons or a top menu bar, LispM opens into a \
                      full-screen Dynamic Lisp Listener with an interactive Command: prompt, \
                      subsystem table, Zmacs buffers, a white Who-Line and a 2-line inverse-video \
                      Mouse Documentation Line along the bottom.",
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
        description: "Mac OS 9's Platinum appearance in classic bevelled greys or dark Dracula \
                      colours. A top menu bar beside the rainbow λ changes with the frontmost \
                      application, pinstriped title bars carry close, zoom and collapse boxes, \
                      desktop icons sit along the right edge, and a collapsible Control Strip \
                      waits at the bottom left.",
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
        description: "A Windows 3.2 and 3.1 desktop in Windows Default VGA grey (#C0C0C0) with 3D \
                      bevels, navy (#000080) active title bars and white inactive captions. Windows \
                      carry a left control-menu box, right minimize and maximize arrow buttons, and \
                      4-pixel notched sizing borders, with desktop icons aligned down the left edge.",
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
        text: "LispM Desktop gives you a Finder for your files, an editor with a workspace for \
               each project—its files, agents, Alacritty terminals and LispM Office together—and \
               remote projects over SSH.",
        media: shown(
            "assets/video/desktop.mp4",
            "assets/video/desktop.jpg",
            "LispM Desktop starts on its classic-style desktop, opens a project in Emacs, and shows its workspaces.",
            OPENED,
        ),
    },
    Stage {
        headline: "Make it yours.",
        text: "Switch between Symbolics Open Genera, Mac OS 9 Platinum & Dracula, and Windows 3.2 \
               at any time, drag windows off LispM's desktop onto your screen in Coherence mode, \
               or change the look from Control Panels and the Control Strip.",
        media: shown(
            "assets/video/windows.mp4",
            "assets/video/windows.jpg",
            "A window is dragged off the desktop, the desktop hides and comes back, and the theme turns to Dracula.",
            OPENED,
        ),
    },
    Stage {
        headline: "Standalone Mac apps in your Dock.",
        text: "Hide the desktop (C-c h) and open Stickies, the App Store, Tetris or any LispM app \
               directly from the macOS Dock or Spotlight as a standalone Mac app with its own Dock \
               tile and menu bar, while LispM runs out of sight.",
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
        text: "Start Emacs with emacs -nw in Kitty or Ghostty and the entire LispM Desktop—its menu \
               bar, desktop icons rendered over the Kitty graphics protocol, floating windows and \
               games—runs inside your terminal.",
        media: shown(
            "assets/video/terminal-desktop.mp4",
            "assets/video/terminal-desktop.jpg",
            "emacs -nw starts inside a terminal and opens the full LispM Desktop with graphical icons, \
             menus and Conway's Game of Life running in a floating window.",
            GAINED,
        ),
    },
    Stage {
        headline: "Common Lisp beside Emacs, and GNU Emacs 31 & 32.",
        text: "In Control Panels → System → Kernel, turn on the LispM Kernel—a Common Lisp runtime \
               running beside Emacs that keeps your sessions—and switch live between GNU Emacs 31.1 \
               and experimental GNU Emacs 32.",
        media: Media::Picture(Picture {
            image: "assets/screenshots/kernel-panel.webp",
            width: 2000,
            height: 1094,
            description: "Control Panels open on the Kernel tab, showing the LispM Kernel Common Lisp \
                          runtime toggle and the Emacs Kernel selector between GNU Emacs 31.1 and GNU Emacs 32.",
        }),
    },
    Stage {
        headline: "The web, from the keyboard.",
        text: "LispM Browser is a WebKit browser with tabs, history and bookmarks. The location \
               field takes an address or a search; f labels every link in sight, and typing a label follows it.",
        media: shown(
            "assets/video/browser.mp4",
            "assets/video/browser.jpg",
            "LispM Browser opens a page, goes to an address from the location field, and labels the links to follow them from the keyboard.",
            OPENED,
        ),
    },
    Stage {
        headline: "Chrome extensions, inside Emacs.",
        text: "On macOS 15.4 and later, LispM Browser runs Chrome extensions from the Chrome Web \
               Store, managed at lispm://extensions, with toolbar popups, M-e keyboard actions and \
               full worker support for extensions like Rabby.",
        media: shown(
            "assets/video/extensions.mp4",
            "assets/video/extensions.jpg",
            "The puzzle piece on the browser's toolbar opens the Chrome Web Store in a tab, Add to \
             LispM on Dark Reader's page adds it once the question in the minibuffer is answered, \
             the Lisp machine article turns dark, and Dark Reader's own toolbar button opens its \
             popup under it.",
            OPENED,
        ),
    },
];

/// The features of LispM Desktop told at length on `/desktop/`.
pub const DESKTOP_SPOTLIGHTS: [Spotlight; 1] = [Spotlight {
    headline: "Focus Mode, a Lisp Machine's console.",
    text: "Press C-c T, or ask LispM, and the desktop gives way to LispM's terminal: the sessions on \
           the left, a listener on the right and a who line along the bottom, all from the keyboard, \
           as a Lisp Machine's console was. Leave it, and the desktop is exactly as it was; or start \
           LispM in it, with no desktop at all.",
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
        text: "Choose Symbolics Open Genera 2.0, Mac OS 9 Platinum & Dracula, or Windows 3.2 at first start or in Control Panels → Appearance, plus OS/2 Warp, phosphor and neon glass looks.",
    },
    Point {
        title: "Native Alacritty terminals",
        text: "Each terminal in LispM Desktop is emulated on its own thread by a native Rust module built on Alacritty, redrawing only visible panes so heavy output never stalls Emacs.",
    },
    Point {
        title: "Remote projects over SSH",
        text: "Add a remote host in the sidebar and LispM opens its directories over TRAMP, starting shells and coding agents on the remote machine in the same workspace.",
    },
    Point {
        title: "Control Panels & live kernels",
        text: "Configure Appearance, Language, Keyboard, Network proxy, Sandbox, Security, Apps, and System → Kernel, switching live between GNU Emacs 31 and Emacs 32.",
    },
    Point {
        title: "Seven languages",
        text: "LispM speaks 7 languages: English (US and UK), German, French, Japanese, and Simplified and Traditional Chinese. The agents follow.",
    },
    Point {
        title: "Apps of their own",
        text: "On macOS, each app in LispM also opens as a standalone Mac app in ~/Applications/LispM Apps, from Spotlight or the Dock, with its own Dock tile and menu bar.",
    },
];

/// The steps of the LispM App Store's story, shown on `/apps/`.
pub const STORE_STAGES: [Stage; 5] = [
    Stage {
        headline: "Every app, a page of its own.",
        text: "In the LispM App Store, each app shows what it does, who made it, its licence and \
               what it needs. Get installs it with whatever it needs, from Homebrew or Cargo, and \
               uninstalling takes those away when nothing else uses them.",
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
        headline: "No app for it yet? Ask anyway.",
        text: "Ask LispM in M-x ai for something no installed app does yet—like reading Lisp news—and \
               the agent searches the LispM App Store, asks before installing Elfeed, and opens the \
               headlines with its new skill.",
        media: shown(
            "assets/video/app-store-ai.mp4",
            "assets/video/app-store-ai.jpg",
            "Asked in the LispM bar to install a news reader and show what is new in Lisp, LispM \
             Agent finds Elfeed in the App Store, asks for approval, installs it, and opens the \
             headlines.",
            GAINED,
        ),
    },
    Stage {
        headline: "Three old AIs, running again.",
        text: "LispM's Classic AI holds Doctor from 1966, SHRDLU from 1970, moving its blocks as \
               you talk to it, and MYCIN from the 1970s, which shows the rule it is trying when \
               you ask why.",
        media: shown(
            "assets/video/classic-ai.mp4",
            "assets/video/classic-ai.jpg",
            "SHRDLU in LispM: told to find a block taller than the one it holds and put it into the \
             box, it plans the moves, and its hand carries the blocks across the table.",
            GAINED,
        ),
    },
    Stage {
        headline: "Doctor, SHRDLU and MYCIN, side by side.",
        text: "Joseph Weizenbaum's 1966 ELIZA Doctor, Terry Winograd's 1970 blocks-world SHRDLU and \
               Stanford's 1970s MYCIN expert system run side by side on LispM Desktop, each with a \
               skill so modern agents can converse with them.",
        media: Media::Picture(Picture {
            image: "assets/screenshots/classic-ai-all.webp",
            width: 2000,
            height: 1125,
            description: "SHRDLU, MYCIN and Doctor open side by side in three floating windows on \
                          LispM Desktop.",
        }),
    },
    Stage {
        headline: "Doom, drawn with OpenGL inside Emacs.",
        text: "On LispM Experimental (GNU Emacs 32), LispM Doom renders classic Doom at full speed \
               inside an Emacs window using LispM GL's OpenGL surface.",
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
        text: "Installed by default: an 8-room animated pixel office where every agent session has a desk, working agents type, waiting agents raise a hand, and clicking a desk approves calls.",
    },
    Point {
        title: "Skills come with apps",
        text: "Install an app in LispM and its manual comes as a skill: agents read feeds, play music, stick notes, manage mail, chat on IRC, Slack and Telegram, and check security.",
    },
    Point {
        title: "AI Harnesses",
        text: "From the LispM App Store, Hermes, Holon and OpenClaw, agents of their own, join Claude Code, Codex and @Ada in one sidebar.",
    },
    Point {
        title: "For every day",
        text: "The LispM App Store has LispM Mail, Stickies, an RSS feed reader, music and YouTube, a calendar, IRC, Slack and Telegram, and 1Password.",
    },
    Point {
        title: "Classic AI",
        text: "1960s and 1970s AI landmarks restored inside Emacs: Joseph Weizenbaum's 1966 Doctor, Terry Winograd's 1970 SHRDLU, and Stanford's MYCIN.",
    },
    Point {
        title: "And games",
        text: "Tetris, Solitaire, Conway's Game of Life and Dunnet, and Doom drawn with OpenGL on LispM Experimental.",
    },
];

/// The steps of LispM Remote's story, shown on `/lispm-remote/`.
pub const REMOTE_STAGES: [Stage; 3] = [
    Stage {
        headline: "Pair once.",
        text: "LispM shows a QR code, your phone scans it at lispm.ai/remote, and both screens show the \
               same code for you to approve. Nothing to install on the phone, no port to open.",
        media: shown(
            "assets/video/remote-pair.mp4",
            "assets/video/remote-pair.jpg",
            "LispM Remote shows a QR code on the desktop, a phone scans it, both screens show the \
             fingerprint 3981-CF90, and LispM approves the phone.",
            GAINED,
        ),
    },
    Stage {
        headline: "Approve from anywhere.",
        text: "In LispM Remote, each agent is a chat on the phone: watch its calls as they run, \
               answer the ones it waits on, then tell it what's next.",
        media: shown(
            "assets/video/remote-approve.mp4",
            "assets/video/remote-approve.jpg",
            "On the phone, an agent works through a fix and asks to run make test; Allow is tapped, \
             and all three tests pass.",
            GAINED,
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
            GAINED,
        ),
    },
];

/// What LispM Remote brings, point by point.
pub const REMOTE_POINTS: [Point; 3] = [
    Point {
        title: "No server in between",
        text: "LispM and your phone find each other on Rings Network, peer to peer, by their addresses.",
    },
    Point {
        title: "Back by itself",
        text: "When the network drops and comes back, LispM Remote on the phone reconnects, with no new scan.",
    },
    Point {
        title: "From the App Store",
        text: "One click in the LispM App Store installs LispM Remote, its Rings node downloaded for your computer and checked against the checksum it pins.",
    },
];

const TRUST_POINTS: [Point; 4] = [
    Point {
        title: "It asks first",
        text: "In LispM, a command, an edit or an install waits for your yes, in Emacs, in LispM Office or on your phone. Reading never needs one.",
    },
    Point {
        title: "LispM Auth vault",
        text: "Secrets are encrypted locally with Argon2id and XChaCha20-Poly1305 in a standalone Common Lisp service, with a lock screen, auto-lock and per-app password rules.",
    },
    Point {
        title: "Sandboxed and checked",
        text: "LispM runs commands and MCP servers in an OS sandbox, checks what Claude Code, Codex and its own agents run, and looks packages up in OSV.",
    },
    Point {
        title: "Free software",
        text: "Every part of LispM is open source under the GNU AGPL, and its agents use any OpenAI or Anthropic HTTPS endpoint with your own keys.",
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
        lead: "Talk to a LispM agent in a chat laid out like Claude Code's. Underneath, the chat is \
               an org document, its tools are Elisp, and it can evaluate Lisp in the Emacs you are using.",
        stages: &AGENTS_STAGES,
        spotlights: &AGENTS_SPOTLIGHTS,
        points: &AGENTS_POINTS,
        repository: "https://github.com/LispMachineLab/lispm-agent",
    },
    Chapter {
        anchor: Anchor::Desktop,
        eyebrow: "Desktop",
        headline: "Three classic desktops, one workspace for agents.",
        lead: "LispM Desktop comes with three classic workstation interfaces—Symbolics Open Genera \
               2.0, Mac OS 9 Platinum & Dracula, and Windows 3.2—built around a Finder, an editor, \
               LispM Browser, native Alacritty terminals and apps in windows of their own.",
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

/// Returns every clip and still beside the chapters, top to bottom, each
/// with the headline it illustrates: the Desktop chapter's three classic
/// desktop themes, a chapter's stages, then its spotlights' clips. The
/// film, above them all, is [`FILM`].
pub fn illustrations() -> impl Iterator<Item = Illustration> {
    CHAPTERS.iter().flat_map(|chapter| {
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
    })
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
pub const TITLE: &str = "AI agents inside Emacs, a Lisp Machine for agents — LispM";

/// What search engines show under the title, within the 160 characters
/// they show: what LispM is, and the agents it runs.
pub const DESCRIPTION: &str = "LispM is an AI agent harness built on GNU Emacs and the Lisp \
     Machine, fifty years in the making: Claude Code, Codex and its own agents, free on Mac and Linux.";

/// What LispM is, in two sentences: the answer search engines, answer
/// engines and language models are given everywhere they look, the page's
/// first question, its JSON-LD and llms.txt.
pub const DEFINITION: &str = "LispM is an AI agent harness built on GNU Emacs and the ideas of \
     the Lisp Machine, fifty years in the making. Its agents live inside Emacs: their \
     conversations are org documents, their tools are Lisp, and Emacs itself is the harness.";

/// A question people ask about LispM, and its answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Question {
    /// The question, as people ask it.
    pub question: &'static str,
    /// Its answer, the first sentence a whole answer on its own.
    pub answer: &'static str,
}

/// The headline over the questions.
pub const QUESTIONS_HEADLINE: &str = "Questions, answered.";

/// The paragraph under the questions' headline, and the description of the
/// Q&A page for search engines.
pub const QUESTIONS_LEAD: &str = "Answers to common questions about LispM: how AI agents run \
     inside GNU Emacs, which models and apps they use, how LispM Remote works, and how it stays safe.";

/// The Agent page's path on the site.
pub const AGENT: &str = "/agent/";

/// The description of the Agent page for search engines, within 160 characters.
pub const AGENT_DESCRIPTION: &str = "LispM Agent runs AI agents inside GNU Emacs: org-mode chats, \
     Elisp tools, TODO approvals, git memory, LispM Office, and Claude Code & Codex side by side.";

/// The Desktop page's path on the site.
pub const DESKTOP: &str = "/desktop/";

/// The description of the Desktop page for search engines, within 160 characters.
pub const DESKTOP_DESCRIPTION: &str = "LispM Desktop runs three classic desktops inside GNU Emacs: \
     Symbolics Open Genera 2.0, Mac OS 9 Platinum & Dracula, and Windows 3.2, with LispM Browser.";

/// The Apps page's path on the site.
pub const APPS: &str = "/apps/";

/// The description of the Apps page for search engines, within 160 characters.
pub const APPS_DESCRIPTION: &str = "The LispM App Store installs Emacs apps, skills, LispM Office, \
     AI harnesses (Hermes, Holon, OpenClaw), games and 1960s–70s classic AI (SHRDLU, Doctor, MYCIN).";

/// The Remote page's path on the site (distinct from `/remote/`, the QR pairing web app).
pub const REMOTE: &str = "/lispm-remote/";

/// The description of the Remote page for search engines, within 160 characters.
pub const REMOTE_DESCRIPTION: &str = "LispM Remote connects your phone and Emacs peer to peer over \
     Rings Network via QR code: follow agent chats, approve tool calls and view windows anywhere.";

/// The Q&A page's path on the site.
pub const QA: &str = "/qa/";

/// What people ask about LispM, the first what it is.
pub const QUESTIONS: [Question; 12] = [
    Question {
        question: "What is LispM?",
        answer: "LispM is an AI agent harness built on GNU Emacs and the ideas of the Lisp Machine, \
                 fifty years in the making. Its agents live inside Emacs: their conversations are \
                 org documents, their tools are Lisp, and Emacs itself is the harness. It comes free \
                 for macOS and GNU/Linux, with three classic desktops, a browser, an App Store and \
                 LispM Remote.",
    },
    Question {
        question: "What are the three desktops in LispM Desktop?",
        answer: "LispM Desktop includes three complete classic workstation interfaces that you can \
                 choose at first start or switch at any time in Control Panels → Appearance: \
                 Classic Lisp Machine (Symbolics Open Genera 2.0, a 1-bit monochrome Dynamic Lisp \
                 Listener with Command: prompt, hatched scrollbars and Who-Line), Classic Mac OS \
                 (Mac OS 9 Platinum and dark Dracula, with top menu bar, pinstriped windows and \
                 Control Strip), and Classic Win32 (Windows 3.2 on teal #008080 with navy captions \
                 and 4-pixel notched sizing borders), plus Focus Mode (C-c T).",
    },
    Question {
        question: "Why the Lisp Machine and Emacs?",
        answer: "Because an agent needs a world it can read and change while it runs. On the Lisp \
                 Machines of the 1970s, and in Emacs since 1976, everything is a live Lisp object: \
                 a program can look at any of it, call it and redefine it as it runs. Fifty years \
                 of that make Emacs the best harness architecture for modern AI, and LispM hands it \
                 to agents: their tools are Lisp functions, their conversations org documents, \
                 their approvals TODOs, and they write Lisp to drive Emacs itself.",
    },
    Question {
        question: "Which agents and models does LispM run?",
        answer: "LispM's own agents, persistent Characters (@Ada) with their own soul and model, \
                 and beside them Claude Code, Codex, Hermes, Holon and OpenClaw, all in one sidebar \
                 and able to message each other. LispM's agents use any model: OpenAI's or \
                 Anthropic's with a key, or any other server, local or hosted.",
    },
    Question {
        question: "How do agents talk to each other, and what is LispM Office?",
        answer: "In LispM, Claude Code, Codex, Hermes, Holon, OpenClaw, Characters (@Ada) and \
                 LispM's own agents share one sidebar and one message bus inside Emacs so any agent \
                 can ask another for a review or subtask. Pressing C-c O opens LispM Office, an \
                 8-room animated pixel office where every agent session has a desk, working agents \
                 type, waiting agents raise a hand, and clicking a desk inspects its work or \
                 approves pending calls.",
    },
    Question {
        question: "How does LispM keep agents safe on my computer?",
        answer: "LispM requires your approval before an agent runs a command, edits a file or \
                 installs a package, while reading files never needs one. Commands and MCP servers \
                 run inside an operating-system sandbox, and LispM Security checks agent actions, \
                 pins trusted tools and scans packages against the OSV vulnerability database.",
    },
    Question {
        question: "How does LispM Remote work without a cloud server?",
        answer: "LispM Remote pairs your phone and computer once via a QR code at lispm.ai/remote \
                 and connects them peer to peer over Rings Network, sealed end to end. Neither \
                 device needs an account or an open router port: you can read agent chats, approve \
                 waiting tool calls and view LispM's windows redrawn on your phone from anywhere.",
    },
    Question {
        question: "What is LispM Browser, and does it run Chrome extensions?",
        answer: "LispM Browser is a keyboard-driven WebKit browser inside Emacs with tabs, \
                 history, bookmarks and Vimium-style link hints. On macOS 15.4 and later, LispM \
                 Browser runs Chrome extensions from the Chrome Web Store, managed at \
                 lispm://extensions, with toolbar popups, M-e keyboard actions and service-worker \
                 support for extensions like Dark Reader and the Rabby wallet.",
    },
    Question {
        question: "What apps come in the LispM App Store?",
        answer: "The LispM App Store installs Emacs apps, agent harnesses and classic AI programs \
                 in one click, resolving dependencies through Homebrew or Cargo. Each app brings \
                 its manual as a skill so LispM's agents can drive it: LispM Office, LispM Mail, \
                 Stickies, a feed reader, music and YouTube, a calendar, IRC, Slack, Telegram, \
                 1Password, Doom, and 1960s and 1970s AI classics (Doctor, SHRDLU and MYCIN).",
    },
    Question {
        question: "Where does LispM store my data, memory and credentials?",
        answer: "LispM runs locally on your computer and keeps its installation in ~/.lispm, agent \
                 conversations and long-term memory as plain org documents under git, and fetched \
                 mail in your local Emacs directory. LispM Auth encrypts secrets locally using a \
                 256-bit key wrapped with Argon2id and XChaCha20-Poly1305 in a standalone Common \
                 Lisp service, with auto-lock and per-app password controls in Control Panels → \
                 Security.",
    },
    Question {
        question: "What is the difference between LispM on Emacs 31 and LispM Experimental on Emacs 32?",
        answer: "Standard LispM runs on GNU Emacs 31.1 on both macOS and GNU/Linux and receives \
                 automatic updates through Software Update. LispM Experimental runs the same \
                 desktop on development builds of GNU Emacs 32 on macOS to support capabilities \
                 like OpenGL drawn inside an Emacs buffer, and you can switch live between Emacs 31 \
                 and Emacs 32 from Control Panels → System → Kernel or the installer.",
    },
    Question {
        question: "Is LispM free, and can it run in my own Emacs?",
        answer: "Yes. LispM and every project it is made of are free software under the GNU Affero \
                 General Public License, version 3 or later, for macOS 11+ and Debian or Ubuntu \
                 GNU/Linux. GNU Emacs comes bundled with LispM, or you can install LispM Agent, \
                 LispM Desktop and LispM Browser directly inside your own Emacs 30.1 or later.",
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
        text: "The same line installs LispM on a Mac and on GNU/Linux, where apt asks for your password.",
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
mod tests;
