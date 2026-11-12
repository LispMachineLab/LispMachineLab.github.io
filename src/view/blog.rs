//! The blog's pages: the list of posts, and each post; and the page shown
//! at addresses not on the site. Prerender draws them at build time, under
//! the home page's navigation bar and over its footer; they hold no state,
//! and no WebAssembly runs on them.

use yew::prelude::*;

use crate::blog::{HEADLINE, LEAD, POSTS, Picture, Post, markdown, size};
use crate::content::{
    AGENTS_POINTS, AGENTS_SPOTLIGHTS, AGENTS_STAGES, BLOG, DESKTOP_POINTS, DESKTOP_SPOTLIGHTS,
    DESKTOP_STAGES, DESKTOP_THEMES, DOWNLOAD, IDEA_POINTS, Media, QUESTIONS, QUESTIONS_HEADLINE,
    QUESTIONS_LEAD, REMOTE_POINTS, REMOTE_STAGES, STORE_POINTS, STORE_STAGES, Stage,
};

use super::page::{Place, footer, navigation, outward, points};

/// The properties of a [`PostPage`].
#[derive(Properties, PartialEq)]
pub struct PostProps {
    /// The post.
    pub post: &'static Post,
}

/// Returns what shows beside a step of LispM Desktop's story on `/desktop/`,
/// with root-relative asset paths so it plays without WebAssembly.
fn desktop_media(media: Media) -> Html {
    match media {
        Media::Loop(clip) => html! {
            <video
                class="clip"
                src={format!("/{}", clip.video)}
                poster={format!("/{}", clip.poster)}
                autoplay=true
                muted=true
                loop=true
                playsinline=true
                controls=true
                preload="metadata"
                aria-label={clip.description}
            />
        },
        Media::Picture(picture) => html! {
            <img
                class="still"
                src={format!("/{}", picture.image)}
                alt={picture.description}
                width={picture.width.to_string()}
                height={picture.height.to_string()}
                loading="lazy"
                decoding="async"
            />
        },
    }
}

/// Returns a sequence of story steps with root-relative media paths.
fn subpage_stages(stages: &'static [Stage]) -> Html {
    html! {
        { for stages.iter().map(|stage| html! {
            <div class="story">
                <div class="words">
                    <h3>{ stage.headline }</h3>
                    <p>{ stage.text }</p>
                </div>
                <div class="screen">{ desktop_media(stage.media) }</div>
            </div>
        }) }
    }
}

/// LispM Agent's own page: org-document conversations, Elisp tools, TODO
/// approvals, git-backed memory, multi-agent collaboration with Claude Code
/// and Codex, LispM Office and Characters (@Ada).
#[function_component(Agent)]
pub fn agent() -> Html {
    html! {
        <>
            { navigation(Place::Agent) }
            <main class="desktop-page">
                <header class="blog-head">
                    <p class="eyebrow">{ "LispM Agent" }</p>
                    <h1>{ "Ask, and it gets done." }</h1>
                    <p class="lead">
                        { "Talk to a LispM agent in a chat laid out like Claude Code's. \
                           Underneath, the conversation is an org document that is also the \
                           model's context, its tools are Elisp functions, its approvals are TODO \
                           entries, its memory is org files under git, and it evaluates Lisp in \
                           the Emacs you are using." }
                    </p>
                </header>
                <section class="section chapter" id="agent">
                    { points(&IDEA_POINTS) }
                    { subpage_stages(&AGENTS_STAGES) }
                    { for AGENTS_SPOTLIGHTS.iter().map(|spotlight| html! {
                        <div class="spotlight">
                            <h3>{ spotlight.headline }</h3>
                            <p>{ spotlight.text }</p>
                            { spotlight.clip.map(|clip| html! {
                                <div class="screen">{ desktop_media(Media::Loop(clip)) }</div>
                            }).unwrap_or_default() }
                        </div>
                    }) }
                    { points(&AGENTS_POINTS) }
                    <div class="actions">
                        <a class="button primary" href={DOWNLOAD}>
                            { "Download LispM" }
                        </a>
                        { outward("https://github.com/LispMachineLab/lispm-agent", "button", "View on GitHub") }
                    </div>
                </section>
            </main>
            { footer(Place::Agent) }
        </>
    }
}

/// LispM Desktop's own page: the three classic workstation desktops, followed
/// by its workspaces, LispM Browser, Chrome extensions, Focus Mode and Control Panels.
#[function_component(Desktop)]
pub fn desktop() -> Html {
    html! {
        <>
            { navigation(Place::Desktop) }
            <main class="desktop-page">
                <header class="blog-head">
                    <p class="eyebrow">{ "LispM Desktop" }</p>
                    <h1>{ "Three classic desktops, one workspace for agents." }</h1>
                    <p class="lead">
                        { "LispM Desktop turns GNU Emacs into a complete workstation ready from \
                           the first start. Pick the classic system you know—Symbolics Open \
                           Genera 2.0, Mac OS 9 Platinum & Dracula, or Windows 3.2—and work with \
                           your files, coding agents, LispM Browser and native Alacritty terminals \
                           in one place." }
                    </p>
                </header>
                <section class="section desktop-themes" id="themes">
                    <p class="eyebrow">{ "Three Desktops" }</p>
                    <h2>{ "Which system do you know?" }</h2>
                    <p class="lead">
                        { "At first start, or at any time in Control Panels → Appearance, switch \
                           the entire workstation between three faithful classic interfaces." }
                    </p>
                    <ul class="theme-cards">
                        { for DESKTOP_THEMES.iter().map(|theme| html! {
                            <li class="theme-card" id={theme.key}>
                                <img
                                    class="theme-preview"
                                    src={format!("/{}", theme.picture.image)}
                                    alt={theme.picture.description}
                                    width={theme.picture.width.to_string()}
                                    height={theme.picture.height.to_string()}
                                    loading="lazy"
                                    decoding="async"
                                />
                                <div class="theme-body">
                                    <p class="eyebrow">{ theme.eyebrow }</p>
                                    <h3>{ theme.title }</h3>
                                    <p>{ theme.description }</p>
                                    <ul class="theme-highlights">
                                        { for theme.highlights.iter().map(|item| html! {
                                            <li>{ *item }</li>
                                        }) }
                                    </ul>
                                    <code class="theme-setting">{ theme.setting }</code>
                                </div>
                            </li>
                        }) }
                    </ul>
                </section>
                <section class="section chapter" id="workspaces">
                    <p class="eyebrow">{ "Workspaces & Browser" }</p>
                    <h2>{ "A Finder, an editor, a WebKit browser and Chrome extensions." }</h2>
                    <p class="lead">
                        { "Every project gets its own workspace with its files, agents, LispM \
                           Office pixel desks and shells together, alongside LispM Browser." }
                    </p>
                    { subpage_stages(&DESKTOP_STAGES) }
                    { for DESKTOP_SPOTLIGHTS.iter().map(|spotlight| html! {
                        <div class="spotlight">
                            <h3>{ spotlight.headline }</h3>
                            <p>{ spotlight.text }</p>
                            { spotlight.clip.map(|clip| html! {
                                <div class="screen">{ desktop_media(Media::Loop(clip)) }</div>
                            }).unwrap_or_default() }
                        </div>
                    }) }
                    { points(&DESKTOP_POINTS) }
                    <div class="actions">
                        <a class="button primary" href={DOWNLOAD}>
                            { "Download LispM" }
                        </a>
                        { outward("https://github.com/LispMachineLab/lispm-desktop", "button", "View on GitHub") }
                    </div>
                </section>
            </main>
            { footer(Place::Desktop) }
        </>
    }
}

/// The LispM App Store's own page: apps, skills, LispM Office, AI harnesses,
/// classic AI programs and games.
#[function_component(Apps)]
pub fn apps() -> Html {
    html! {
        <>
            { navigation(Place::Apps) }
            <main class="desktop-page">
                <header class="blog-head">
                    <p class="eyebrow">{ "LispM App Store" }</p>
                    <h1>{ "Apps your agents can drive." }</h1>
                    <p class="lead">
                        { "Apps, games and other projects' agents, each a click away. Every app \
                           brings its manual as a skill so LispM's agents can work it for you; \
                           ask for something no app does yet, and LispM finds one in the catalog \
                           and asks before it installs it." }
                    </p>
                </header>
                <section class="section chapter" id="store">
                    { subpage_stages(&STORE_STAGES) }
                    { points(&STORE_POINTS) }
                    <div class="actions">
                        <a class="button primary" href={DOWNLOAD}>
                            { "Download LispM" }
                        </a>
                        <a class="button" href="/mail/">
                            { "LispM Mail" }
                        </a>
                        { outward("https://github.com/LispMachineLab/lispm-store", "button", "View on GitHub") }
                    </div>
                </section>
            </main>
            { footer(Place::Apps) }
        </>
    }
}

/// LispM Remote's own page: QR pairing, peer-to-peer Rings Network transport,
/// phone approvals and redrawn Emacs windows.
#[function_component(Remote)]
pub fn remote() -> Html {
    html! {
        <>
            { navigation(Place::Remote) }
            <main class="desktop-page">
                <header class="blog-head">
                    <p class="eyebrow">{ "LispM Remote" }</p>
                    <h1>{ "Your agents, in your pocket." }</h1>
                    <p class="lead">
                        { "Scan a QR code once. Then, from your phone, follow every agent, \
                           approve what it waits on, send it the next task and open LispM's \
                           windows redrawn on your screen, peer to peer over Rings Network with \
                           no server in between." }
                    </p>
                </header>
                <section class="section chapter" id="remote">
                    { subpage_stages(&REMOTE_STAGES) }
                    { points(&REMOTE_POINTS) }
                    <div class="actions">
                        <a class="button primary" href={DOWNLOAD}>
                            { "Download LispM" }
                        </a>
                        { outward("https://github.com/LispMachineLab/lispm-remote", "button", "View on GitHub") }
                    </div>
                </section>
            </main>
            { footer(Place::Remote) }
        </>
    }
}

/// The questions people ask about LispM, each answered in full on its own
/// page.
#[function_component(Questions)]
pub fn questions() -> Html {
    html! {
        <>
            { navigation(Place::Questions) }
            <main class="blog">
                <header class="blog-head">
                    <p class="eyebrow">{ "Q&A" }</p>
                    <h1>{ QUESTIONS_HEADLINE }</h1>
                    <p class="lead">{ QUESTIONS_LEAD }</p>
                </header>
                <ul class="answers">
                    { for QUESTIONS.iter().map(|question| html! {
                        <li>
                            <h2>{ question.question }</h2>
                            <p>{ question.answer }</p>
                        </li>
                    }) }
                </ul>
                <div class="actions">
                    <a class="button primary" href={DOWNLOAD}>
                        { "Download LispM" }
                    </a>
                </div>
            </main>
            { footer(Place::Questions) }
        </>
    }
}

/// The list of posts, the latest first.
#[function_component(Blog)]
pub fn blog() -> Html {
    html! {
        <>
            { navigation(Place::Blog) }
            <main class="blog">
                <header class="blog-head">
                    <p class="eyebrow">{ "LispM Blog" }</p>
                    <h1>{ HEADLINE }</h1>
                    <p class="lead">{ LEAD }</p>
                </header>
                <ol class="posts">
                    { for POSTS.iter().map(teaser) }
                </ol>
            </main>
            { footer(Place::Blog) }
        </>
    }
}

/// Returns the width and height of `picture`, as attributes, if they are known.
fn dimensions(picture: Picture) -> (Option<AttrValue>, Option<AttrValue>) {
    let size = size::of(picture.src);
    (
        size.map(|(width, _)| width.to_string().into()),
        size.map(|(_, height)| height.to_string().into()),
    )
}

/// Returns when a post was published, as a `<time>`.
fn date(post: &Post) -> Html {
    html! { <time datetime={post.date.iso()}>{ post.date.long() }</time> }
}

/// Returns a post in the list: its cover, date, author, title and what it
/// is about, all one link to it.
fn teaser(post: &'static Post) -> Html {
    let (width, height) = dimensions(post.cover);
    html! {
        <li>
            <a href={post.path()}>
                <img src={post.cover.src} alt="" {width} {height} loading="lazy" decoding="async" />
                <div>
                    <p class="byline">{ date(post) }{ " · " }{ post.author.name }</p>
                    <h2>{ post.title }</h2>
                    <p>{ post.description }</p>
                </div>
            </a>
        </li>
    }
}

/// A post: its title, author and date, its cover, its words, and where it
/// was first published.
#[function_component(PostPage)]
pub fn post_page(props: &PostProps) -> Html {
    let post = props.post;
    let author = post.author;
    let body = Html::from_html_unchecked(markdown::to_html(post.body, size::of).into());
    let (width, height) = dimensions(post.cover);
    html! {
        <>
            { navigation(Place::Blog) }
            <main class="post">
                <article>
                    <header class="post-head">
                        <a class="eyebrow" href={BLOG}>{ "LispM Blog" }</a>
                        <h1>{ post.title }</h1>
                        <p class="byline">
                            { "By " }
                            <a href={author.profile()} target="_blank" rel="noopener">{ author.name }</a>
                            { " · " }{ date(post) }
                        </p>
                    </header>
                    <img class="cover" src={post.cover.src} alt={post.cover.alt} {width} {height} />
                    <div class="prose">{ body }</div>
                    <footer class="post-end">
                        if let Some(on_x) = post.on_x {
                            <p>
                                { "First published on X by " }
                                <a href={author.profile()} target="_blank" rel="noopener">
                                    { format!("@{}", author.handle) }
                                </a>
                                { ". " }
                                { outward(on_x, "", "Read it there") }
                                { "." }
                            </p>
                        }
                        <div class="actions">
                            <a class="button primary" href={DOWNLOAD}>
                                { "Download LispM" }
                            </a>
                            <a class="button" href={BLOG}>{ "More from the blog" }</a>
                        </div>
                    </footer>
                </article>
            </main>
            { footer(Place::Blog) }
        </>
    }
}

/// The page shown at addresses not on the site: where to go instead.
#[function_component(Missing)]
pub fn missing() -> Html {
    html! {
        <>
            { navigation(Place::Elsewhere) }
            <main class="blog">
                <header class="blog-head">
                    <p class="eyebrow">{ "Not here" }</p>
                    <h1>{ "No page at this address." }</h1>
                    <p class="lead">
                        { "It may have moved, or a letter may be off. LispM, and what we write \
                           about it, are a click away." }
                    </p>
                    <div class="actions">
                        <a class="button primary" href="/">{ "LispM" }</a>
                        <a class="button" href={BLOG}>{ "The blog" }</a>
                    </div>
                </header>
            </main>
            { footer(Place::Elsewhere) }
        </>
    }
}
