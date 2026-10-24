//! The page, top to bottom: the navigation bar, the headline and film, the
//! idea, the three projects, how to start, and the footer. The blog's pages
//! share its navigation bar and footer.

use yew::prelude::*;

use crate::content::{
    APP_ICON, Anchor, BACKENDS, BLOG, CREDITS, DOWNLOAD_HEADLINE, DOWNLOAD_LEAD, DOWNLOAD_STEPS,
    EXPERIMENTAL, EXPERIMENTAL_BACK, EXPERIMENTAL_HEADLINE, FILM, IDEA, IDEA_HEADLINE, IDEA_POINTS,
    INSTALL_BACK, INSTALL_EXPERIMENTAL, INSTALLER_DOES, INSTALLER_SOURCE, ORGANIZATION, OWN_EMACS,
    OWN_EMACS_HEADLINE, PROJECTS, Point, Project, REQUIREMENTS, SUMMARY, Spotlight, Step, TAGLINE,
};

use crate::effects;

use super::code::Code;
use super::media::{Film, Loop};

/// The rainbow λ, LispM's mark, from the root, as the blog's pages are deeper.
const LOGO: &str = "/assets/lispm-lambda.png";

/// The page a navigation bar is on: the home page reaches its sections by
/// their ids, the blog's pages through the home page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Place {
    /// The home page.
    Home,
    /// The blog, or a post in it.
    Blog,
}

impl Place {
    /// Returns the link to the home page's `anchor` from here.
    pub(crate) fn href(self, anchor: Anchor) -> String {
        match self {
            Place::Home => anchor.href(),
            Place::Blog => format!("/{}", anchor.href()),
        }
    }
}

/// The site.
#[function_component(App)]
pub fn app() -> Html {
    use_effect_with((), |_| effects::reveal_addressed_section());
    html! {
        <>
            { navigation(Place::Home) }
            <main>
                { hero() }
                { idea() }
                { for PROJECTS.iter().map(project) }
                { start() }
            </main>
            { footer(Place::Home) }
        </>
    }
}

/// Returns a link opening `href` in a new tab.
pub(crate) fn outward(href: &'static str, class: &'static str, label: &'static str) -> Html {
    html! { <a {href} {class} target="_blank" rel="noopener">{ label }</a> }
}

/// Returns the navigation bar of a page at `place`.
pub(crate) fn navigation(place: Place) -> Html {
    let home = match place {
        Place::Home => "#top",
        Place::Blog => "/",
    };
    html! {
        <header class="navigation">
            <nav aria-label="LispM">
                <a class="brand" href={home}>
                    <img src={LOGO} alt="" width="22" height="22" />
                    <span>{ "LispM" }</span>
                </a>
                <ul>
                    { for Anchor::NAVIGATION.iter().map(|anchor| html! {
                        <li><a href={place.href(*anchor)}>{ anchor.name() }</a></li>
                    }) }
                    <li>
                        <a href={BLOG} aria-current={(place == Place::Blog).then_some("true")}>
                            { "Blog" }
                        </a>
                    </li>
                </ul>
                { outward(ORGANIZATION, "github", "GitHub") }
            </nav>
        </header>
    }
}

fn hero() -> Html {
    html! {
        <section class="hero" id="top">
            <img class="logo" src={LOGO} alt="" width="112" height="112" />
            <p class="eyebrow">{ "LispM" }</p>
            <h1>{ TAGLINE }</h1>
            <p class="summary">{ SUMMARY }</p>
            <div class="actions">
                <a class="button primary" href={Anchor::Start.href()}>{ "Download for Mac" }</a>
                { outward(ORGANIZATION, "button", "View on GitHub") }
            </div>
            <div class="stage" id={Anchor::Film.id()}>
                <Film clip={FILM} />
            </div>
        </section>
    }
}

/// Returns `points` as a grid of tiles.
fn points(points: &[Point]) -> Html {
    html! {
        <ul class="points">
            { for points.iter().map(|point| html! {
                <li>
                    <h3>{ point.title }</h3>
                    <p>{ point.text }</p>
                </li>
            }) }
        </ul>
    }
}

fn idea() -> Html {
    html! {
        <section class="section idea" id={Anchor::Idea.id()}>
            <h2>{ IDEA_HEADLINE }</h2>
            <p class="lead">{ IDEA }</p>
            { points(&IDEA_POINTS) }
        </section>
    }
}

/// Returns a feature told at length: its headline and paragraph over its
/// clip, if it has one, and its points.
fn spotlight(spotlight: &Spotlight) -> Html {
    html! {
        <div class="spotlight">
            <h3>{ spotlight.headline }</h3>
            <p>{ spotlight.text }</p>
            { for spotlight.clip.map(|clip| html! {
                <div class="screen"><Loop {clip} /></div>
            }) }
            { points(spotlight.points) }
        </div>
    }
}

fn project(project: &Project) -> Html {
    html! {
        <section class="section project" id={project.anchor.id()}>
            <p class="eyebrow">{ project.name }</p>
            <h2>{ project.headline }</h2>
            <p class="lead">{ project.lead }</p>
            { for project.stages.iter().map(|stage| html! {
                <div class="story">
                    <div class="words">
                        <h3>{ stage.headline }</h3>
                        <p>{ stage.text }</p>
                    </div>
                    <div class="screen"><Loop clip={stage.clip} /></div>
                </div>
            }) }
            { for project.spotlight.iter().map(spotlight) }
            { points(project.points) }
            <div class="actions">
                <a class="button primary" href={Anchor::Start.href()}>{ "Install" }</a>
                { outward(project.repository, "button", "View on GitHub") }
            </div>
        </section>
    }
}

/// Returns a step of installing LispM.app.
fn step(step: &Step) -> Html {
    html! {
        <li>
            <h4>{ step.title }</h4>
            if !step.text.is_empty() {
                <p>{ step.text }</p>
            }
            { for step.snippet.map(|snippet| html! { <Code {snippet} /> }) }
        </li>
    }
}

/// LispM.app, for everyone: its icon and what it runs on beside the steps
/// that install it.
fn download() -> Html {
    html! {
        <div class="download">
            <div class="app">
                <img src={APP_ICON} alt="LispM's icon, the rainbow λ" width="128" height="128" />
                <h3>{ "LispM for Mac" }</h3>
                <p>{ REQUIREMENTS }</p>
            </div>
            <ol class="steps">
                { for DOWNLOAD_STEPS.iter().map(step) }
            </ol>
            <p class="installer">
                { "The line runs " }{ outward(INSTALLER_SOURCE, "", "install.sh") }{ ", which" }
                { INSTALLER_DOES }
            </p>
        </div>
    }
}

/// The experimental LispM.app, on Emacs 32: who it is for beside its line,
/// as the projects' own installs are laid out.
fn experimental() -> Html {
    html! {
        <article class="install experimental">
            <div>
                <p class="eyebrow">{ "Experimental" }</p>
                <h3>{ EXPERIMENTAL_HEADLINE }</h3>
                <p>{ EXPERIMENTAL }</p>
            </div>
            <div>
                <Code snippet={INSTALL_EXPERIMENTAL} />
                <p class="back">{ EXPERIMENTAL_BACK }{ " " }<code>{ INSTALL_BACK }</code></p>
            </div>
        </article>
    }
}

fn start() -> Html {
    html! {
        <section class="section start" id={Anchor::Start.id()}>
            <h2>{ DOWNLOAD_HEADLINE }</h2>
            <p class="lead">{ DOWNLOAD_LEAD }</p>
            { download() }
            { experimental() }
            <h3 class="own-emacs">{ OWN_EMACS_HEADLINE }</h3>
            <p class="lead">{ OWN_EMACS }</p>
            <div class="installs">
                { for PROJECTS.iter().map(|project| html! {
                    <article class="install">
                        <div>
                            <h3>{ project.name }</h3>
                            <p>{ project.audience }</p>
                        </div>
                        <Code snippet={project.install} />
                    </article>
                }) }
            </div>
            <p class="model">
                { "Then give the agents a model. Claude Code, Codex and Hermes, once installed, \
                   show beside LispM's own agents, which need only " }
                <code>{ "OPENAI_API_KEY" }</code>{ " or " }<code>{ "ANTHROPIC_API_KEY" }</code>
                { " set in your shell's profile, where LispM.app reads it, or in your own Emacs's \
                   environment. For any other server, local or hosted, see " }
                { outward(BACKENDS, "", "Backends") }{ "." }
            </p>
        </section>
    }
}

/// Returns the footer of a page at `place`; the home page's credits its film.
pub(crate) fn footer(place: Place) -> Html {
    html! {
        <footer class="footer">
            <div class="brand">
                <img src={LOGO} alt="" width="28" height="28" />
                <span>{ "LispM" }</span>
            </div>
            <ul>
                { for PROJECTS.iter().map(|project| html! {
                    <li>{ outward(project.repository, "", project.name) }</li>
                }) }
                <li>{ outward(ORGANIZATION, "", "LispMachineLab") }</li>
                <li><a href={BLOG}>{ "Blog" }</a></li>
            </ul>
            <p>{ "Free software, under the GNU Affero General Public License, version 3 or later." }</p>
            if place == Place::Home {
                <p class="credits">{ CREDITS }</p>
            }
        </footer>
    }
}
