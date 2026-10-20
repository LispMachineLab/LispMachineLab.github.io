//! What the page says, as data: its sections, the clips from the films, the
//! points each project makes, and the code to install them. The words
//! follow the three projects' READMEs.

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
    /// How to install LispM, and each project on its own.
    Start,
}

impl Anchor {
    /// The sections the navigation bar links to, left to right.
    pub const NAVIGATION: [Anchor; 4] = [
        Anchor::Agent,
        Anchor::Desktop,
        Anchor::Browser,
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

/// A step of a project's story: a headline, a sentence, and a clip.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stage {
    /// The headline.
    pub headline: &'static str,
    /// The sentence under it.
    pub text: &'static str,
    /// What plays beside them.
    pub clip: Clip,
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

/// One of the three projects.
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
    /// Who installs it, and why, in the Get started section.
    pub audience: &'static str,
    /// How to install it.
    pub install: Snippet,
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

const AGENT_STAGES: [Stage; 1] = [Stage {
    headline: "Ask, and it works in your project.",
    text: "It reads the code, asks before it runs a command or changes a file, writes the fix and runs the tests.",
    clip: clip(
        "assets/video/agent.mp4",
        "assets/video/agent.jpg",
        "An agent is asked to fix a failing test: it reads the code, asks first, writes the fix, \
         the tests pass, and it writes Emacs Lisp to open two files side by side.",
    ),
}];

const AGENT_POINTS: [Point; 4] = [
    Point {
        title: "Any model",
        text: "Any endpoint speaking the OpenAI Chat Completions API or the Anthropic Messages API, local or hosted.",
    },
    Point {
        title: "Many agents",
        text: "Run several in a project, each in a session you can come back to.",
    },
    Point {
        title: "Tools on demand",
        text: "A core set of tools, and the rest, MCP servers among them, found when a session needs them.",
    },
    Point {
        title: "Woken by Emacs",
        text: "Hooks and timers start agents, and agents delegate work to each other.",
    },
];

const DESKTOP_STAGES: [Stage; 3] = [
    Stage {
        headline: "Every project, a workspace.",
        text: "A desktop, a Finder for your files, and an editor with a workspace for each project, its files, agents and shells together.",
        clip: clip(
            "assets/video/desktop.mp4",
            "assets/video/desktop.jpg",
            "LispM Desktop starts on a Mac OS 9 desktop, opens a project in Emacs, and shows its workspaces.",
        ),
    },
    Stage {
        headline: "Every agent, one sidebar.",
        text: "LispM's agents, Claude Code, Codex and Hermes work side by side, every session listed in the sidebar to resume where it ran.",
        clip: clip(
            "assets/video/agents.mp4",
            "assets/video/agents.jpg",
            "Claude Code, Codex and Hermes open beside a LispM agent, all running at once, listed in one sidebar.",
        ),
    },
    Stage {
        headline: "Make it yours.",
        text: "Drag windows off the desktop, hide it and bring it back in a click, and change the look from the Control Strip.",
        clip: clip(
            "assets/video/windows.mp4",
            "assets/video/windows.jpg",
            "A window is dragged off the desktop, the desktop hides and comes back, and the theme turns to Dracula.",
        ),
    },
];

const DESKTOP_POINTS: [Point; 3] = [
    Point {
        title: "Ready at once",
        text: "The first start asks for vim or Emacs keys and an appearance, then installs the rest.",
    },
    Point {
        title: "Two appearances",
        text: "Platinum, Mac OS 9's own greys, and Dracula.",
    },
    Point {
        title: "Leaves yours alone",
        text: "Try it with its own init directory; your configuration is not touched.",
    },
];

const BROWSER_STAGES: [Stage; 1] = [Stage {
    headline: "Browse from the keyboard.",
    text: "The location field takes an address or a search; f labels every link in sight, and typing a label follows it.",
    clip: clip(
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

const BROWSER_POINTS: [Point; 3] = [
    Point {
        title: "Tabs",
        text: "Every page a tab, with its icon, history and bookmarks kept as Emacs keeps them.",
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

/// The three projects, in the order the page shows them.
pub const PROJECTS: [Project; 3] = [
    Project {
        anchor: Anchor::Agent,
        name: "LispM Agent",
        headline: "Emacs is the harness.",
        lead: "Talk to an agent in a chat laid out like Claude Code's. Underneath, the chat is an org \
               document, its tools are Elisp, and it can evaluate Lisp in the Emacs you are using.",
        stages: &AGENT_STAGES,
        spotlight: None,
        points: &AGENT_POINTS,
        audience: "The agents, in your own Emacs 30.1 or later. Then M-x lispm in any project.",
        install: Snippet {
            language: Language::EmacsLisp,
            code: "(use-package lispm\n  :vc (:url \"https://github.com/LispMachineLab/lispm-agent\" :rev :newest)\n  :config (lispm-mode-line-mode 1))",
        },
        repository: "https://github.com/LispMachineLab/lispm-agent",
        languages: &["Emacs Lisp"],
    },
    Project {
        anchor: Anchor::Desktop,
        name: "LispM Desktop",
        headline: "A desktop for your agents.",
        lead: "An Emacs distribution built around LispM Agent, laid out as Mac OS 9's desktop and \
               ready from the first start.",
        stages: &DESKTOP_STAGES,
        spotlight: None,
        points: &DESKTOP_POINTS,
        audience: "What LispM.app runs, in your own Emacs 30.1 or later with SVG, git and curl, on macOS or GNU/Linux.",
        install: Snippet {
            language: Language::Shell,
            code: "git clone https://github.com/LispMachineLab/lispm-desktop ~/lispm-desktop\nemacs --init-directory ~/lispm-desktop",
        },
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
        install: Snippet {
            language: Language::EmacsLisp,
            code: "(package-vc-install \"https://github.com/LispMachineLab/lispm-browser\")\n(require 'lispm-browser)\n(lispm-browser-setup)",
        },
        repository: "https://github.com/LispMachineLab/lispm-browser",
        languages: &["Swift", "Emacs Lisp"],
    },
];

/// Where the site is served.
pub const SITE: &str = "https://lispmachinelab.github.io/";

/// The headline of the download.
pub const DOWNLOAD_HEADLINE: &str = "Get LispM.";

/// The paragraph under it, for those who have never heard of Emacs.
pub const DOWNLOAD_LEAD: &str = "LispM is a Mac app. Emacs, the editor it is made of, comes inside it: \
     you need not know Emacs to begin.";

/// LispM.app's icon.
pub const APP_ICON: &str = "assets/lispm-app.png";

/// What LispM.app runs on.
pub const REQUIREMENTS: &str = "macOS 11 or later, on Apple silicon or Intel. Free.";

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
    code: "curl -fsSL https://lispmachinelab.github.io/install.sh | sh",
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

/// What the line does, after the link to the installer.
pub const INSTALLER_DOES: &str = " installs Apple's command line tools if they are missing, puts \
     LispM.app in Applications and LispM's files in ~/.lispm. Run the line again to update; delete \
     both to uninstall.";

/// The headline over the projects' own installs.
pub const OWN_EMACS_HEADLINE: &str = "Already use Emacs?";

/// The paragraph under it.
pub const OWN_EMACS: &str = "Each project also stands on its own, in your own Emacs.";

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
    const ALL: [Anchor; 6] = [
        Anchor::Film,
        Anchor::Idea,
        Anchor::Agent,
        Anchor::Desktop,
        Anchor::Browser,
        Anchor::Start,
    ];

    fn clips() -> impl Iterator<Item = Clip> {
        std::iter::once(FILM)
            .chain(
                PROJECTS
                    .iter()
                    .flat_map(|p| p.stages.iter().map(|s| s.clip)),
            )
            .chain(PROJECTS.iter().filter_map(|p| p.spotlight?.clip))
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
            assert!(
                project.install.code.contains(project.repository),
                "{}",
                project.name
            );
            assert!(!project.install.code.ends_with('\n'), "{}", project.name);
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
