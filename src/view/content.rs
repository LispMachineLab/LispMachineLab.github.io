//! What each window holds. The words follow the three projects' READMEs.

use yew::prelude::*;

use crate::model::command::Link;
use crate::model::window::WindowId;

use super::controls::{Opens, code, group, push_link};

/// Returns what `window` holds.
pub fn content(window: WindowId) -> Html {
    match window {
        WindowId::About => about(),
        WindowId::Agent => agent(),
        WindowId::Desktop => desktop(),
        WindowId::Browser => browser(),
        WindowId::ReadMe => read_me(),
        WindowId::Trash => trash(),
    }
}

/// Returns a project's line in About LispM: its icon and name opening its window, then `what`.
fn project(window: WindowId, icon: &'static str, what: &'static str) -> Html {
    html! {
        <li>
            <Opens {window}>
                <img src={icon} alt="" width="32" height="32" />
                <span>{ window.title() }</span>
            </Opens>
            <span class="what">{ what }</span>
        </li>
    }
}

/// Returns a row of push buttons, the last the default, as Mac OS 9 laid them out.
fn buttons(children: Html) -> Html {
    html! { <div class="buttons">{ children }</div> }
}

fn about() -> Html {
    html! {
        <>
            <header class="masthead">
                <img class="logo" src="assets/lispm-lambda.png" alt="" width="88" height="88" />
                <h1>{ "LispM" }</h1>
                <p class="tagline">{ "A Lisp Machine for agents" }</p>
            </header>
            <p>
                { "The Lisp Machine was an environment where everything was a live Lisp object, and \
                   the editor and the system were one. Its inhabitants were programs. LispM makes room \
                   in it for agents: they run inside Emacs rather than beside it, their conversations \
                   are org documents that are also their context, their tools are Elisp functions, \
                   their approvals org TODO states, and their memory org files under git." }
            </p>
            { group("Three projects", html! {
                <ul class="projects">
                    { project(WindowId::Agent, "assets/icons/robot.svg",
                              "AI agents that run inside Emacs, their conversations org documents, their tools Elisp.") }
                    { project(WindowId::Desktop, "assets/icons/computer.svg",
                              "Emacs laid out as Mac OS 9's desktop, a workspace for agents, ready from the first start.") }
                    { project(WindowId::Browser, "assets/icons/netscape.svg",
                              "A WebKit browser in Emacs, with tabs, keyboard hints and Chrome extensions.") }
                </ul>
            }) }
            <p class="note">
                { "Each stands on its own; LispM Desktop brings the other two together. This page is \
                   drawn as LispM Desktop draws Emacs, in Mac OS 9's Platinum." }
                <span class="desktop-only">
                    { " Double-click an icon to open it, and drag a window by its title bar." }
                </span>
            </p>
            { buttons(html! {
                <>
                    { push_link("GitHub", Link::Organization, false) }
                    <Opens window={WindowId::ReadMe} push=true default=true>{ "Read Me" }</Opens>
                </>
            }) }
        </>
    }
}

fn agent() -> Html {
    html! {
        <>
            <p class="lead">{ "AI agents that run inside Emacs rather than beside it." }</p>
            <p>
                { "You talk to an agent in a chat laid out like Claude Code's. It asks before it runs \
                   a command or changes a file, and each change streams into its file as the model \
                   writes it. Underneath, the chat is an org document, and that document is the \
                   agent's context." }
            </p>
            <figure class="demo">
                <video src="assets/lispm-demo.mp4" poster="assets/lispm-demo.jpg" controls=true
                       muted=true playsinline=true preload="none" />
                <figcaption>
                    { "An agent runs a project's failing tests, fixes the bug and writes a README, \
                       asking before each step; then the chat turns out to be an org document whose \
                       tool calls are TODO entries." }
                </figcaption>
            </figure>
            { group("What it is made of", html! {
                <dl class="facts">
                    <dt>{ "Conversations" }</dt>
                    <dd>{ "org documents, one per session; run several agents in a project and resume any of them." }</dd>
                    <dt>{ "Tools" }</dt>
                    <dd>{ "Elisp functions. Agents carry a core set and search for the rest, MCP servers among them, when a session needs them." }</dd>
                    <dt>{ "Approvals" }</dt>
                    <dd>{ "org TODO states: a command or an edit waits as a TODO entry until you allow it." }</dd>
                    <dt>{ "Memory" }</dt>
                    <dd>{ "org files under git." }</dd>
                    <dt>{ "Triggers" }</dt>
                    <dd>{ "Emacs hooks and timers wake agents, and agents delegate work to each other." }</dd>
                    <dt>{ "Models" }</dt>
                    <dd>{ "any endpoint speaking the OpenAI Chat Completions API or the Anthropic Messages API, local or hosted." }</dd>
                </dl>
            }) }
            { group("Install", html! {
                <>
                    <p>{ "Emacs 30.1 or later, and curl:" }</p>
                    { code("(package-vc-install '(lispm :url \"https://github.com/LispMachineLab/lispm-agent\"))\n(require 'lispm)") }
                    <p>{ "Then " }<kbd>{ "M-x lispm" }</kbd>{ " in any project starts an agent there." }</p>
                </>
            }) }
            { buttons(push_link("View on GitHub", Link::AgentRepository, true)) }
        </>
    }
}

fn desktop() -> Html {
    html! {
        <>
            <p class="lead">{ "Emacs laid out as Mac OS 9's desktop, a workspace for AI agents, ready from the first start." }</p>
            <p>
                { "LispM Desktop is an Emacs distribution built around LispM Agent. It gives the agents \
                   a place to work, puts Claude Code, Codex and Hermes beside them when you have those \
                   installed, and wears Mac OS 9's Platinum, as this page does." }
            </p>
            { group("What you get", html! {
                <ul>
                    <li>{ "A desktop with its icons down the right edge, a Finder window for your files, and the Trash." }</li>
                    <li>{ "Applications one in front at a time, the desktop, the editor and the browser, each filling the menu bar with its menus under the rainbow λ." }</li>
                    <li>{ "An editor with a sidebar of files, agents and shells, and a workspace for each project." }</li>
                    <li>{ "Every agent session in the sidebar, LispM's and those of Claude Code, Codex and Hermes, to resume where it ran." }</li>
                    <li>{ "Windows moved by their title bars and rolled up by their collapse boxes, Control Panels, and the Control Strip." }</li>
                    <li>{ "Two appearances: Platinum, Mac OS 9's own greys, and Dracula." }</li>
                </ul>
            }) }
            { group("Try it", html! {
                <>
                    <p>{ "Emacs 30.1 or later built with SVG images, git and curl, on macOS or GNU/Linux. Your own configuration is left alone:" }</p>
                    { code("git clone https://github.com/LispMachineLab/lispm-desktop ~/lispm-desktop\nemacs --init-directory ~/lispm-desktop") }
                    <p>{ "The first start asks whether you edit with vim or Emacs keys and which appearance you want, then installs LispM Agent and LispM Browser with the rest." }</p>
                </>
            }) }
            { buttons(push_link("View on GitHub", Link::DesktopRepository, true)) }
        </>
    }
}

fn browser() -> Html {
    html! {
        <>
            <p class="lead">{ "A WebKit browser in Emacs, with tabs, keyboard hints and Chrome extensions." }</p>
            <p>
                { "An Emacs built with xwidgets shows web pages in its buffers; LispM Browser makes them \
                   a browser. In LispM Desktop it is drawn as Netscape Communicator's window was." }
            </p>
            { group("What you get", html! {
                <ul>
                    <li>{ "Tabs, each with its page's icon." }</li>
                    <li>{ "History and bookmarks, the bookmarks Emacs's own, so " }<code>{ "bookmark-jump" }</code>{ " opens them too." }</li>
                    <li>{ "The keyboard, as Vimium uses pages: " }<kbd>{ "f" }</kbd>{ " labels the links, buttons and fields in sight, and typing a label follows it." }</li>
                    <li>{ "Chrome extensions on macOS 15.4 and later, run by WebKit itself, wallets such as Rabby among them." }</li>
                    <li>{ "Pages that do not take Emacs down: closing them and running their scripts no longer crashes Emacs on macOS." }</li>
                </ul>
            }) }
            { group("Install", html! {
                <>
                    <p>{ "Emacs 30.1 or later built with xwidgets: on macOS the NS build, elsewhere GTK with WebKitGTK." }</p>
                    { code("(package-vc-install \"https://github.com/LispMachineLab/lispm-browser\")\n(require 'lispm-browser)\n(lispm-browser-setup)") }
                </>
            }) }
            { buttons(push_link("View on GitHub", Link::BrowserRepository, true)) }
        </>
    }
}

fn read_me() -> Html {
    html! {
        <article class="read-me">
            <p>{ "LispM is three projects, and each stands on its own. Start with the one you want." }</p>
            <h3>{ "Everything: LispM Desktop" }</h3>
            { code("git clone https://github.com/LispMachineLab/lispm-desktop ~/lispm-desktop\nemacs --init-directory ~/lispm-desktop") }
            <p>{ "The first start installs LispM Agent and LispM Browser with the rest of the packages, a few minutes, once." }</p>
            <h3>{ "The agents, in your own Emacs" }</h3>
            { code("(use-package lispm\n  :vc (:url \"https://github.com/LispMachineLab/lispm-agent\" :rev :newest)\n  :config (lispm-mode-line-mode 1))") }
            <h3>{ "The browser, in your own Emacs" }</h3>
            { code("(package-vc-install \"https://github.com/LispMachineLab/lispm-browser\")\n(require 'lispm-browser)\n(lispm-browser-setup)") }
            <h3>{ "A model" }</h3>
            <p>
                { "With " }<code>{ "OPENAI_API_KEY" }</code>{ " or " }<code>{ "ANTHROPIC_API_KEY" }</code>
                { " set in Emacs's environment, the agents need nothing more. For any other server, \
                   local or hosted, see LispM Agent's " }
                <a href="https://github.com/LispMachineLab/lispm-agent#backends" target="_blank" rel="noopener">{ "Backends" }</a>
                { "." }
            </p>
            <h3>{ "License" }</h3>
            <p>{ "All three are free software, under the GNU General Public License, version 3 or later." }</p>
        </article>
    }
}

fn trash() -> Html {
    html! {
        <>
            <div class="header-strip">{ "0 items" }</div>
            <p class="empty">{ "The Trash is empty." }</p>
        </>
    }
}
