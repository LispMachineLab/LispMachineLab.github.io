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
    /// How to install them.
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
            Anchor::Start => "Get started",
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
    /// A shell's.
    Shell,
    /// Emacs Lisp, for an init file.
    EmacsLisp,
}

impl Language {
    /// Returns the language's name, as the snippet's caption shows it.
    pub const fn name(self) -> &'static str {
        match self {
            Language::Shell => "Shell",
            Language::EmacsLisp => "Emacs Lisp",
        }
    }

    /// Returns the language's name after a Markdown code fence.
    pub const fn fence(self) -> &'static str {
        match self {
            Language::Shell => "sh",
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

/// A feature told at length, after a project's story: a headline, a
/// paragraph, and its points.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spotlight {
    /// The headline.
    pub headline: &'static str,
    /// The paragraph under it.
    pub text: &'static str,
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
pub const IDEA_HEADLINE: &str = "The editor and the system, one again.";

/// The paragraph under it.
pub const IDEA: &str = "On a Lisp Machine everything was a live Lisp object, and the editor and the \
     system were one. Its inhabitants were programs. LispM makes room in it for agents.";

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
        text: "Open the Chrome Web Store in a tab and add an extension from its page: its signature is checked, and what it may do is asked first.",
    },
    Point {
        title: "Or from a folder",
        text: "Load an unpacked extension, as Chrome's developer mode does, or add one by its store link or id.",
    },
    Point {
        title: "Its button, from the keyboard",
        text: "M-e presses an extension's button for the page in front, and its popup hangs from the page's top right.",
    },
    Point {
        title: "Kept in Emacs",
        text: "List them to press, open their options, turn them on and off, reload or remove them; their questions are asked in the minibuffer.",
    },
];

const BROWSER_SPOTLIGHT: Spotlight = Spotlight {
    headline: "Chrome extensions, inside Emacs.",
    text: "On macOS 15.4 and later, WebKit runs Chrome extensions itself, as Safari runs its own. \
           The APIs WebKit lacks are filled in, so even heavy extensions work: the Rabby wallet runs \
           its worker, its page provider and its popup, and its approvals open in windows of their own.",
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
        audience: "Everything, ready from the first start: Emacs 30.1 or later with SVG, git and curl, on macOS or GNU/Linux.",
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

/// The licence the projects are free software under.
pub const LICENSE: &str = "https://www.gnu.org/licenses/gpl-3.0.html";

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
        std::iter::once(FILM).chain(
            PROJECTS
                .iter()
                .flat_map(|p| p.stages.iter().map(|s| s.clip)),
        )
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
    fn hrefs_point_at_ids() {
        assert_eq!(Anchor::Start.href(), "#start");
    }
}
