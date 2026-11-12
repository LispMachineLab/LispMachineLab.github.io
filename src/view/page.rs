//! The page, top to bottom: the navigation bar, the headline and film, the
//! idea, the chapters, how to start, the questions people ask, and the
//! footer. The blog's pages, and the pages that stand alone, share its
//! navigation bar and footer.

use yew::prelude::*;

use crate::content::{
    AGENT, APP_ICON, APPS, Anchor, BLOG, CHAPTERS, CREDITS, Chapter, DESKTOP, DOWNLOAD,
    DOWNLOAD_HEADLINE, DOWNLOAD_LEAD, DOWNLOAD_STEPS, FILM, IDEA, IDEA_HEADLINE, IDEA_POINTS,
    INSTALLATION, INSTALLER_DOES, INSTALLER_SOURCE, INSTALLER_UPDATES, Media, ORGANIZATION,
    PLATFORMS, PRIVACY, PROJECTS, Platform, Point, QA, REMOTE, REQUIREMENTS, SUMMARY, Spotlight,
    Step, TAGLINE, X,
};

use crate::effects;

use super::code::Code;
use super::media::{Film, Loop, Still};

/// The rainbow λ, LispM's mark, from the root, as the blog's pages are deeper.
const LOGO: &str = "/assets/lispm-lambda.png";

/// The page a navigation bar is on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub(crate) enum Place {
    /// The home page.
    Home,
    /// The LispM Agent page.
    Agent,
    /// The LispM Desktop page.
    Desktop,
    /// The LispM App Store page.
    Apps,
    /// The LispM Remote page.
    Remote,
    /// The download and Emacs installation guide.
    Download,
    /// The questions page.
    Questions,
    /// The blog, or a post in it.
    Blog,
    /// Any other page: one that stands alone, as LispM Mail's and the
    /// privacy policy do, or the one shown at addresses not on the site.
    Elsewhere,
}

/// The site.
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            { navigation(Place::Home) }
            <main>
                { hero() }
                { idea() }
                { for CHAPTERS
                    .iter()
                    .filter(|c| c.anchor == Anchor::Trust)
                    .map(chapter) }
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
    html! {
        <header class="navigation">
            <nav aria-label="LispM">
                <a class="brand" href="/">
                    <img src={LOGO} alt="" width="22" height="22" />
                    <span>{ "LispM" }</span>
                </a>
                <ul>
                    <li>
                        <a href="/" aria-current={(place == Place::Home).then_some("true")}>
                            { "Home" }
                        </a>
                    </li>
                    <li>
                        <a href={AGENT} aria-current={(place == Place::Agent).then_some("true")}>
                            { "Agent" }
                        </a>
                    </li>
                    <li>
                        <a href={DESKTOP} aria-current={(place == Place::Desktop).then_some("true")}>
                            { "Desktop" }
                        </a>
                    </li>
                    <li>
                        <a href={APPS} aria-current={(place == Place::Apps).then_some("true")}>
                            { "Apps" }
                        </a>
                    </li>
                    <li>
                        <a href={REMOTE} aria-current={(place == Place::Remote).then_some("true")}>
                            { "Remote" }
                        </a>
                    </li>
                    <li>
                        <a href={DOWNLOAD} aria-current={(place == Place::Download).then_some("true")}>
                            { "Download" }
                        </a>
                    </li>
                    <li>
                        <a href={QA} aria-current={(place == Place::Questions).then_some("true")}>
                            { "Q&A" }
                        </a>
                    </li>
                    <li>
                        <a href={BLOG} aria-current={(place == Place::Blog).then_some("true")}>
                            { "Blog" }
                        </a>
                    </li>
                </ul>
                { outward(ORGANIZATION, "github", "GitHub") }
                <a href={X} class="x" target="_blank" rel="noopener" aria-label="LispM on X">
                    { "X" }
                </a>
            </nav>
        </header>
    }
}

/// The hero's download button. It is drawn for the Mac, as the prerender
/// knows no visitor, and names GNU/Linux once the browser says it runs there.
#[function_component(Download)]
fn download_button() -> Html {
    let platform = use_state_eq(|| Platform::Mac);
    {
        let platform = platform.setter();
        use_effect_with((), move |_| platform.set(effects::platform()));
    }
    html! {
        <a class="button primary" href={DOWNLOAD}>{ platform.download() }</a>
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
                <Download />
                { outward(ORGANIZATION, "button", "View on GitHub") }
            </div>
            <p class="platforms">{ PLATFORMS }</p>
            <div class="stage" id={Anchor::Film.id()}>
                <Film clip={FILM} />
            </div>
        </section>
    }
}

/// Returns `points` as a grid of tiles: in rows of three when they fill
/// rows of three but not of four.
pub(crate) fn points(points: &[Point]) -> Html {
    let thirds = points.len().is_multiple_of(3) && !points.len().is_multiple_of(4);
    html! {
        <ul class={classes!("points", thirds.then_some("thirds"))}>
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

/// Returns what shows beside a step of a project's story.
fn media(media: Media) -> Html {
    match media {
        Media::Loop(clip) => html! { <Loop {clip} /> },
        Media::Picture(picture) => html! { <Still {picture} /> },
    }
}

/// Returns a feature told at length: its headline and paragraph over its
/// clip, if it has one, and its points, if it has any.
fn spotlight(spotlight: &Spotlight) -> Html {
    html! {
        <div class="spotlight">
            <h3>{ spotlight.headline }</h3>
            <p>{ spotlight.text }</p>
            { for spotlight.clip.map(|clip| html! {
                <div class="screen"><Loop {clip} /></div>
            }) }
            if !spotlight.points.is_empty() {
                { points(spotlight.points) }
            }
        </div>
    }
}

/// Returns a chapter: its eyebrow, headline and lead, its story, the
/// features it tells at length, its points, and where to go from it.
fn chapter(chapter: &Chapter) -> Html {
    html! {
        <section class="section chapter" id={chapter.anchor.id()}>
            <p class="eyebrow">{ chapter.eyebrow }</p>
            <h2>{ chapter.headline }</h2>
            <p class="lead">{ chapter.lead }</p>
            { for chapter.stages.iter().map(|stage| html! {
                <div class="story">
                    <div class="words">
                        <h3>{ stage.headline }</h3>
                        <p>{ stage.text }</p>
                    </div>
                    <div class="screen">{ media(stage.media) }</div>
                </div>
            }) }
            { for chapter.spotlights.iter().map(spotlight) }
            { points(chapter.points) }
            <div class="actions">
                { outward(chapter.repository, "button", "View on GitHub") }
            </div>
        </section>
    }
}

/// Returns a step of installing LispM.
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

/// LispM, for everyone: its icon and the systems it runs on beside the
/// steps that install it, the same on each.
fn download() -> Html {
    html! {
        <div class="download">
            <div class="app">
                <img src={APP_ICON} alt="LispM's icon, the rainbow λ" width="128" height="128" />
                <h3>{ "LispM" }</h3>
                <dl class="systems">
                    { for Platform::ALL.iter().map(|platform| html! {
                        <div>
                            <dt>{ platform.name() }</dt>
                            <dd>{ platform.requirements() }</dd>
                        </div>
                    }) }
                </dl>
                <p>{ REQUIREMENTS }</p>
            </div>
            <ol class="steps">
                { for DOWNLOAD_STEPS.iter().map(step) }
            </ol>
            <p class="installer">
                { "The line runs " }{ outward(INSTALLER_SOURCE, "", "install.sh") }{ ", which" }
                { INSTALLER_DOES }
                { for Platform::ALL.iter().map(|platform| html! { <>{ " " }{ platform.installer() }</> }) }
                { " " }{ INSTALLER_UPDATES }{ " To uninstall, see " }
                { outward(INSTALLATION, "", "LispM Desktop's installation guide") }{ "." }
            </p>
        </div>
    }
}

fn start() -> Html {
    html! {
        <section class="section start" id={Anchor::Start.id()}>
            <h2>{ DOWNLOAD_HEADLINE }</h2>
            <p class="lead">{ DOWNLOAD_LEAD }</p>
            { download() }
            <div class="actions">
                <a class="button" href={DOWNLOAD}>
                    { "LispM Experimental, your own Emacs & model setup" }
                </a>
            </div>
        </section>
    }
}

/// Returns the footer of a page at `place`, linking the privacy policy as
/// every page does; the home page's credits its film.
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
                <li>{ outward(X, "", "@lispm_ai on X") }</li>
                <li><a href="/">{ "Home" }</a></li>
                <li><a href={AGENT}>{ "Agent" }</a></li>
                <li><a href={DESKTOP}>{ "Desktop" }</a></li>
                <li><a href={APPS}>{ "Apps" }</a></li>
                <li><a href={REMOTE}>{ "Remote" }</a></li>
                <li><a href={DOWNLOAD}>{ "Download" }</a></li>
                <li><a href={QA}>{ "Q&A" }</a></li>
                <li><a href={BLOG}>{ "Blog" }</a></li>
                <li><a href={PRIVACY}>{ "Privacy" }</a></li>
            </ul>
            <p>{ "Free software, under the GNU Affero General Public License, version 3 or later." }</p>
            if place == Place::Home {
                <p class="credits">{ CREDITS }</p>
            }
        </footer>
    }
}
