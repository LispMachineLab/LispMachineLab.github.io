//! The page, top to bottom: the navigation bar, the headline and film, the
//! idea, the three projects, how to start, and the footer.

use yew::prelude::*;

use crate::content::{
    Anchor, BACKENDS, CREDITS, FILM, IDEA, IDEA_HEADLINE, IDEA_POINTS, ORGANIZATION, PROJECTS,
    Point, Project, SUMMARY, Spotlight, TAGLINE,
};

use crate::effects;

use super::code::Code;
use super::media::{Film, Loop};

/// The rainbow λ, LispM's mark.
const LOGO: &str = "assets/lispm-lambda.png";

/// The site.
#[function_component(App)]
pub fn app() -> Html {
    use_effect_with((), |_| effects::reveal_addressed_section());
    html! {
        <>
            { navigation() }
            <main>
                { hero() }
                { idea() }
                { for PROJECTS.iter().map(project) }
                { start() }
            </main>
            { footer() }
        </>
    }
}

/// Returns a link opening `href` in a new tab.
fn outward(href: &'static str, class: &'static str, label: &'static str) -> Html {
    html! { <a {href} {class} target="_blank" rel="noopener">{ label }</a> }
}

fn navigation() -> Html {
    html! {
        <header class="navigation">
            <nav aria-label="LispM">
                <a class="brand" href="#top">
                    <img src={LOGO} alt="" width="22" height="22" />
                    <span>{ "LispM" }</span>
                </a>
                <ul>
                    { for Anchor::NAVIGATION.iter().map(|anchor| html! {
                        <li><a href={anchor.href()}>{ anchor.name() }</a></li>
                    }) }
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
                <a class="button primary" href={Anchor::Start.href()}>{ "Get started" }</a>
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

/// Returns a feature told at length: its headline and paragraph over its points.
fn spotlight(spotlight: &Spotlight) -> Html {
    html! {
        <div class="spotlight">
            <h3>{ spotlight.headline }</h3>
            <p>{ spotlight.text }</p>
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

fn start() -> Html {
    html! {
        <section class="section start" id={Anchor::Start.id()}>
            <h2>{ "Get started." }</h2>
            <p class="lead">{ "Each project stands on its own. Start with the one you want." }</p>
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
                { "Then give the agents a model. With " }<code>{ "OPENAI_API_KEY" }</code>{ " or " }
                <code>{ "ANTHROPIC_API_KEY" }</code>
                { " set in Emacs's environment nothing more is needed; for any other server, local or hosted, see " }
                { outward(BACKENDS, "", "Backends") }{ "." }
            </p>
        </section>
    }
}

fn footer() -> Html {
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
            </ul>
            <p>{ "Free software, under the GNU General Public License, version 3 or later." }</p>
            <p class="credits">{ CREDITS }</p>
        </footer>
    }
}
