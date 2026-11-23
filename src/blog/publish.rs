//! What makes the blog's pages whole documents, all from [`super`]: each
//! page's head, its title, description, share card and JSON-LD, with the
//! stylesheet Trunk built for the home page; the same for the pages that
//! stand alone, from [`crate::pages`], and for the page shown at addresses
//! not on the site; and, with the posts, the site's map, llms.txt's list of
//! them and llms-full.txt's words of them.

use std::fmt::{self, Write as _};

use super::feed::FEED;
use super::markdown::escape;
use super::{Author, CARD, DESCRIPTION, HEADLINE, LEAD, LISPM, POSTS, Post, absolute};
use crate::content::{
    AGENT, AGENT_DESCRIPTION, AGENTS_SPOTLIGHTS, AGENTS_STAGES, APPS, APPS_DESCRIPTION, BLOG, Clip,
    DESKTOP, DESKTOP_DESCRIPTION, DESKTOP_SPOTLIGHTS, DESKTOP_STAGES, DESKTOP_THEMES, DesktopTheme,
    FILM, IDEA_STAGES, Illustration, Media, QA, QUESTIONS_HEADLINE, QUESTIONS_LEAD, REMOTE,
    REMOTE_DESCRIPTION, REMOTE_STAGES, SITE, STORE_STAGES, Spotlight, Stage,
};
use crate::pages::{self, PAGES, Page};
use crate::publish::{
    KEYWORDS, Node, application, breadcrumb, breadcrumb_id, graph, home_illustrations, home_videos,
    json_array, json_string, organization, questions, screenshot, speakable, stage_videos, video,
    video_id, website,
};

/// Why the blog's pages could not be made.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlogError {
    /// The built index.html links no stylesheet for the pages to share.
    NoStylesheet,
}

impl fmt::Display for BlogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlogError::NoStylesheet => write!(f, "the built index.html links no stylesheet"),
        }
    }
}

impl std::error::Error for BlogError {}

/// Returns the `<link>`s for the stylesheet and the icon that Trunk wrote
/// into the built `index`, for the blog's pages to share: Trunk names them
/// by their content's hash, so they are only known after each build.
///
/// Pre: `index` is index.html as Trunk built it.
/// Post: the links are as they are in `index`, the stylesheet's among them.
pub fn shared_links(index: &str) -> Result<String, BlogError> {
    let links: Vec<String> = index
        .split("<link")
        .skip(1)
        .filter_map(|rest| rest.split_once('>').map(|(tag, _)| tag))
        .filter(|tag| tag.contains("rel=\"stylesheet\"") || tag.contains("rel=\"icon\""))
        .map(|tag| format!("<link{tag}>"))
        .collect();
    if links.iter().any(|link| link.contains("rel=\"stylesheet\"")) {
        Ok(links.join("\n    "))
    } else {
        Err(BlogError::NoStylesheet)
    }
}

/// What a page is, to those it is shared with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// LispM Agent's own page: AI agents inside GNU Emacs.
    Agent,
    /// LispM Desktop's own page: the three classic desktops and workspaces.
    Desktop,
    /// The LispM App Store's own page: apps, skills, classic AI and games.
    Apps,
    /// LispM Remote's own page: QR pairing and peer-to-peer phone access.
    Remote,
    /// The questions people ask about LispM, answered.
    Questions,
    /// The list of posts.
    Blog,
    /// A post.
    Article(&'static Post),
    /// A page that stands alone: the download guide, LispM Mail's, or the
    /// privacy policy.
    Page(&'static Page),
    /// The page at addresses not on the site: kept out of search engines.
    Missing,
}

/// A page's head: what search engines and shared links show of it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Head {
    /// The window's title.
    pub title: String,
    /// The title shared links show.
    pub headline: &'static str,
    /// What it is about.
    pub description: &'static str,
    /// Its path on the site, from the root.
    pub path: String,
    /// What it is.
    pub kind: Kind,
    /// Its share card, 1200 by 630, and what it shows.
    pub card: (&'static str, &'static str),
}

/// The blog's name.
const NAME: &str = "LispM Blog";

/// Returns the head of the Agent page.
pub fn agent_head() -> Head {
    let headline = "LispM Agent: Native GNU Emacs AI Agent Harness — LispM";
    Head {
        title: headline.to_owned(),
        headline,
        description: AGENT_DESCRIPTION,
        path: AGENT.to_owned(),
        kind: Kind::Agent,
        card: (pages::CARD.src, pages::CARD.alt),
    }
}

/// Returns the head of the Desktop page.
pub fn desktop_head() -> Head {
    let headline = "LispM Desktop: Genera, Mac OS 9 & Win 3.2 in Emacs — LispM";
    Head {
        title: headline.to_owned(),
        headline,
        description: DESKTOP_DESCRIPTION,
        path: DESKTOP.to_owned(),
        kind: Kind::Desktop,
        card: (pages::CARD.src, pages::CARD.alt),
    }
}

/// Returns the head of the Apps page.
pub fn apps_head() -> Head {
    let headline = "LispM App Store: Emacs Apps & AI Agent Harnesses — LispM";
    Head {
        title: headline.to_owned(),
        headline,
        description: APPS_DESCRIPTION,
        path: APPS.to_owned(),
        kind: Kind::Apps,
        card: (pages::CARD.src, pages::CARD.alt),
    }
}

/// Returns the head of the Remote page.
pub fn remote_head() -> Head {
    let headline = "LispM Remote: Emacs AI Agents on Your Phone — LispM";
    Head {
        title: headline.to_owned(),
        headline,
        description: REMOTE_DESCRIPTION,
        path: REMOTE.to_owned(),
        kind: Kind::Remote,
        card: (pages::CARD.src, pages::CARD.alt),
    }
}

/// Returns the head of the questions page.
pub fn questions_head() -> Head {
    let headline = "Emacs AI Agent Harness FAQ: Questions, Answered — LispM";
    Head {
        title: headline.to_owned(),
        headline,
        description: QUESTIONS_LEAD,
        path: QA.to_owned(),
        kind: Kind::Questions,
        card: (pages::CARD.src, pages::CARD.alt),
    }
}

/// Returns the head of the list of posts.
pub fn blog_head() -> Head {
    let headline = "LispM Blog: notes from the lab";
    Head {
        title: format!("{headline} — LispM"),
        headline,
        description: DESCRIPTION,
        path: BLOG.to_owned(),
        kind: Kind::Blog,
        card: (CARD.src, CARD.alt),
    }
}

/// Returns the head of `post`'s page.
pub fn post_head(post: &'static Post) -> Head {
    Head {
        title: format!("{} — {NAME}", post.title),
        headline: post.title,
        description: post.summary,
        path: post.path(),
        kind: Kind::Article(post),
        card: (post.card, post.cover.alt),
    }
}

/// Returns the head of `page`, a page that stands alone: its shared links
/// show the home page's card.
pub fn page_head(page: &'static Page) -> Head {
    Head {
        title: page.meta_title.to_owned(),
        headline: page.meta_title,
        description: page.description,
        path: page.path.to_owned(),
        kind: Kind::Page(page),
        card: (pages::CARD.src, pages::CARD.alt),
    }
}

/// Returns the head of the page at addresses not on the site.
pub fn missing_head() -> Head {
    Head {
        title: "Not here — LispM".to_owned(),
        headline: "Not here",
        description: "There is no page at this address on lispm.ai.",
        path: "/404.html".to_owned(),
        kind: Kind::Missing,
        card: (CARD.src, CARD.alt),
    }
}

/// Returns the id of the blog's JSON-LD node.
fn blog_id() -> String {
    format!("{}#blog", absolute(BLOG))
}

/// Returns `author` as JSON-LD: LispM, the organization every page names
/// by its id, or a person.
fn author(author: Author) -> String {
    if author == LISPM {
        Node::Organization.reference()
    } else {
        format!(
            "{{\"@type\":\"Person\",\"name\":{},\"url\":{}}}",
            json_string(author.name),
            json_string(&author.profile()),
        )
    }
}

/// Returns `post` as a schema.org BlogPosting, its page the one it is the
/// main thing on.
fn posting(post: &Post) -> String {
    let url = absolute(&post.path());
    format!(
        "{{\"@type\":\"BlogPosting\",\"@id\":{},\"headline\":{},\"description\":{},\
         \"keywords\":{},\"url\":{url},\"mainEntityOfPage\":{url},\"datePublished\":{},\
         \"image\":{},\"author\":{},\"publisher\":{},\"about\":{},\"isPartOf\":{{\"@id\":{}}},\
         \"inLanguage\":\"en\"{}}}",
        json_string(&format!("{url}#article")),
        json_string(post.title),
        json_string(post.description),
        json_string(post.keywords),
        json_string(&post.date.iso()),
        json_string(&absolute(post.card)),
        author(post.author),
        Node::Organization.reference(),
        Node::Application.reference(),
        json_string(&blog_id()),
        post.on_x
            .map(|on_x| format!(",\"sameAs\":{}", json_string(on_x)))
            .unwrap_or_default(),
        url = json_string(&url),
    )
}

/// Returns the blog as a schema.org Blog, its posts in it.
fn blog() -> String {
    format!(
        "{{\"@type\":\"Blog\",\"@id\":{},\"name\":{},\"url\":{},\"description\":{},\
         \"inLanguage\":\"en\",\"publisher\":{},\"isPartOf\":{},\"blogPost\":{}}}",
        json_string(&blog_id()),
        json_string(NAME),
        json_string(&absolute(BLOG)),
        json_string(DESCRIPTION),
        Node::Organization.reference(),
        Node::WebSite.reference(),
        json_array(POSTS.iter().map(posting)),
    )
}

/// Returns the illustrations visibly rendered on a feature page (`themes`, `stages`,
/// and `spotlights`).
fn page_illustrations(
    themes: &'static [DesktopTheme],
    stages: &'static [Stage],
    spotlights: &'static [Spotlight],
) -> Vec<Illustration> {
    themes
        .iter()
        .map(|theme| Illustration {
            headline: theme.title,
            media: Media::Picture(theme.picture),
        })
        .chain(stages.iter().map(|stage| Illustration {
            headline: stage.headline,
            media: stage.media,
        }))
        .chain(spotlights.iter().filter_map(|spotlight| {
            spotlight.clip.map(|clip| Illustration {
                headline: spotlight.headline,
                media: Media::Loop(clip),
            })
        }))
        .collect()
}

/// Returns a feature page at `path` as a schema.org WebPage of the site,
/// linking the `images` and `clips` visibly rendered on it.
fn feature_page(
    path: &str,
    name: &str,
    description: &str,
    selectors: &[&str],
    images: &[Illustration],
    clips: &[(&str, Clip)],
) -> String {
    let url = absolute(path);
    let image_field = if images.is_empty() {
        String::new()
    } else {
        format!(
            ",\"image\":{}",
            json_array(images.iter().copied().map(screenshot))
        )
    };
    let video_field = if clips.is_empty() {
        String::new()
    } else {
        format!(
            ",\"video\":{}",
            json_array(
                clips
                    .iter()
                    .map(|(_, clip)| format!("{{\"@id\":{}}}", json_string(&video_id(*clip))))
            )
        )
    };
    format!(
        "{{\"@type\":\"WebPage\",\"@id\":{quoted},\"url\":{quoted},\"name\":{},\
         \"description\":{},\"keywords\":{},\"inLanguage\":\"en\",\"isPartOf\":{},\"about\":{},\
         \"publisher\":{},\"breadcrumb\":{{\"@id\":{}}},\"speakable\":{}{image_field}{video_field},\
         \"mainEntity\":[{}]}}",
        json_string(name),
        json_string(description),
        json_string(KEYWORDS),
        Node::WebSite.reference(),
        Node::Application.reference(),
        Node::Organization.reference(),
        json_string(&breadcrumb_id(&url)),
        speakable(selectors),
        Node::Application.reference(),
        quoted = json_string(&url),
    )
}

/// Returns the Q&A page as a schema.org WebPage and FAQPage of the site.
fn questions_page() -> String {
    let url = absolute(QA);
    format!(
        "{{\"@type\":[\"WebPage\",\"FAQPage\"],\"@id\":{quoted},\"url\":{quoted},\"name\":{},\
         \"description\":{},\"keywords\":{},\"inLanguage\":\"en\",\"isPartOf\":{},\"about\":{},\
         \"publisher\":{},\"breadcrumb\":{{\"@id\":{}}},\"speakable\":{},\
         \"mainEntity\":{}}}",
        json_string(QUESTIONS_HEADLINE),
        json_string(QUESTIONS_LEAD),
        json_string(KEYWORDS),
        Node::WebSite.reference(),
        Node::Application.reference(),
        Node::Organization.reference(),
        json_string(&breadcrumb_id(&url)),
        speakable(&["h1", ".lead", ".answers"]),
        questions(),
        quoted = json_string(&url),
    )
}

/// Returns the blog's page as a schema.org CollectionPage of the site.
fn blog_page() -> String {
    let url = absolute(BLOG);
    format!(
        "{{\"@type\":\"CollectionPage\",\"@id\":{quoted},\"url\":{quoted},\"name\":{},\
         \"description\":{},\"inLanguage\":\"en\",\"isPartOf\":{},\"about\":{},\
         \"publisher\":{},\"breadcrumb\":{{\"@id\":{}}},\"speakable\":{},\
         \"mainEntity\":[{{\"@id\":{}}}]}}",
        json_string(NAME),
        json_string(DESCRIPTION),
        Node::WebSite.reference(),
        Node::Application.reference(),
        Node::Organization.reference(),
        json_string(&breadcrumb_id(&url)),
        speakable(&["h1", ".lead", ".posts"]),
        json_string(&blog_id()),
        quoted = json_string(&url),
    )
}

/// Returns `post`'s page as a schema.org WebPage of the site, the post its
/// main entity.
fn post_page(post: &Post) -> String {
    let url = absolute(&post.path());
    format!(
        "{{\"@type\":\"WebPage\",\"@id\":{quoted},\"url\":{quoted},\"name\":{},\
         \"description\":{},\"keywords\":{},\"inLanguage\":\"en\",\"isPartOf\":{},\"about\":{},\
         \"publisher\":{},\"breadcrumb\":{{\"@id\":{}}},\"speakable\":{},\
         \"mainEntity\":[{{\"@id\":{}}}]}}",
        json_string(post.title),
        json_string(post.summary),
        json_string(post.keywords),
        Node::WebSite.reference(),
        Node::Application.reference(),
        Node::Organization.reference(),
        json_string(&breadcrumb_id(&url)),
        speakable(&["h1", ".prose"]),
        json_string(&format!("{url}#article")),
        quoted = json_string(&url),
    )
}

/// Returns `page`, a page that stands alone, as a schema.org WebPage of
/// the site.
fn web_page(page: &Page) -> String {
    let url = absolute(page.path);
    format!(
        "{{\"@type\":\"WebPage\",\"@id\":{quoted},\"url\":{quoted},\"name\":{},\"description\":{},\
         \"inLanguage\":\"en\",\"isPartOf\":{},\"about\":{},\"publisher\":{},\
         \"breadcrumb\":{{\"@id\":{}}},\"speakable\":{},\"mainEntity\":[{}]}}",
        json_string(page.meta_title),
        json_string(page.description),
        Node::WebSite.reference(),
        Node::Application.reference(),
        Node::Organization.reference(),
        json_string(&breadcrumb_id(&url)),
        speakable(&["h1", ".prose"]),
        Node::Application.reference(),
        quoted = json_string(&url),
    )
}

/// Returns the JSON-LD of a page, LispM the organization in it, or nothing
/// for a page kept out of search engines.
pub fn json_ld(kind: Kind) -> Option<String> {
    let blog_url = absolute(BLOG);
    let nodes: Vec<String> = match kind {
        Kind::Agent => {
            let url = absolute(AGENT);
            let images = page_illustrations(&[], &AGENTS_STAGES, &AGENTS_SPOTLIGHTS);
            let clips = stage_videos(&AGENTS_STAGES, &AGENTS_SPOTLIGHTS);
            let mut list = vec![
                organization(),
                website(),
                application(),
                breadcrumb(&url, &[("LispM", SITE), ("Agent", &url)]),
                feature_page(
                    AGENT,
                    "LispM Agent: Native GNU Emacs AI Agent Harness",
                    AGENT_DESCRIPTION,
                    &["h1", ".lead", "#benchmark .lead", ".points"],
                    &images,
                    &clips,
                ),
            ];
            list.extend(clips.into_iter().map(video));
            list
        }
        Kind::Desktop => {
            let url = absolute(DESKTOP);
            let mut images = page_illustrations(&DESKTOP_THEMES, &IDEA_STAGES, &[]);
            images.extend(page_illustrations(
                &[],
                &DESKTOP_STAGES,
                &DESKTOP_SPOTLIGHTS,
            ));
            let mut clips = stage_videos(&IDEA_STAGES, &[]);
            clips.extend(stage_videos(&DESKTOP_STAGES, &DESKTOP_SPOTLIGHTS));
            let mut list = vec![
                organization(),
                website(),
                application(),
                breadcrumb(&url, &[("LispM", SITE), ("Desktop", &url)]),
                feature_page(
                    DESKTOP,
                    "LispM Desktop: Three classic desktops, one workspace for agents.",
                    DESKTOP_DESCRIPTION,
                    &["h1", ".lead", ".theme-cards"],
                    &images,
                    &clips,
                ),
            ];
            list.extend(clips.into_iter().map(video));
            list
        }
        Kind::Apps => {
            let url = absolute(APPS);
            let images = page_illustrations(&[], &STORE_STAGES, &[]);
            let clips = stage_videos(&STORE_STAGES, &[]);
            let mut list = vec![
                organization(),
                website(),
                application(),
                breadcrumb(&url, &[("LispM", SITE), ("Apps", &url)]),
                feature_page(
                    APPS,
                    "LispM App Store: Emacs Apps & AI Agent Harnesses",
                    APPS_DESCRIPTION,
                    &["h1", ".lead", ".points"],
                    &images,
                    &clips,
                ),
            ];
            list.extend(clips.into_iter().map(video));
            list
        }
        Kind::Remote => {
            let url = absolute(REMOTE);
            let images = page_illustrations(&[], &REMOTE_STAGES, &[]);
            let clips = stage_videos(&REMOTE_STAGES, &[]);
            let mut list = vec![
                organization(),
                website(),
                application(),
                breadcrumb(&url, &[("LispM", SITE), ("Remote", &url)]),
                feature_page(
                    REMOTE,
                    "LispM Remote: Your agents, in your pocket.",
                    REMOTE_DESCRIPTION,
                    &["h1", ".lead", ".points"],
                    &images,
                    &clips,
                ),
            ];
            list.extend(clips.into_iter().map(video));
            list
        }
        Kind::Questions => {
            let url = absolute(QA);
            vec![
                organization(),
                website(),
                application(),
                breadcrumb(&url, &[("LispM", SITE), ("Q&A", &url)]),
                questions_page(),
            ]
        }
        Kind::Blog => vec![
            organization(),
            website(),
            application(),
            breadcrumb(&blog_url, &[("LispM", SITE), (NAME, &blog_url)]),
            blog_page(),
            blog(),
        ],
        Kind::Article(post) => {
            let url = absolute(&post.path());
            vec![
                organization(),
                website(),
                application(),
                breadcrumb(
                    &url,
                    &[("LispM", SITE), (NAME, &blog_url), (post.title, &url)],
                ),
                post_page(post),
                posting(post),
            ]
        }
        Kind::Page(page) => {
            let url = absolute(page.path);
            vec![
                organization(),
                website(),
                application(),
                breadcrumb(&url, &[("LispM", SITE), (page.title, &url)]),
                web_page(page),
            ]
        }
        Kind::Missing => return None,
    };
    Some(graph(nodes.into_iter()))
}

/// Returns a page as a whole document: `head`, the shared `links` and `page`,
/// the page's body as Yew rendered it.
pub fn document(head: &Head, links: &str, page: &str) -> String {
    let url = absolute(&head.path);
    let (card, card_alt) = head.card;
    let card = absolute(card);
    let (kind, article) = match head.kind {
        Kind::Agent
        | Kind::Desktop
        | Kind::Apps
        | Kind::Remote
        | Kind::Questions
        | Kind::Blog
        | Kind::Page(_)
        | Kind::Missing => ("website", String::new()),
        Kind::Article(post) => (
            "article",
            format!(
                "\n    <meta name=\"author\" content=\"{}\" />\
                 \n    <meta property=\"article:published_time\" content=\"{}\" />\
                 \n    <meta property=\"article:author\" content=\"{}\" />\
                 \n    <meta name=\"twitter:creator\" content=\"@{}\" />",
                escape(post.author.name),
                post.date.iso(),
                escape(&post.author.profile()),
                escape(post.author.handle),
            ),
        ),
    };
    // Search engines may show a page's pictures large, and should keep the
    // missing page out; nothing there to name as the page's own address.
    let (robots, canonical) = match head.kind {
        Kind::Missing => ("noindex", String::new()),
        Kind::Agent
        | Kind::Desktop
        | Kind::Apps
        | Kind::Remote
        | Kind::Questions
        | Kind::Blog
        | Kind::Article(_)
        | Kind::Page(_) => {
            let keywords = match head.kind {
                Kind::Article(post) => post.keywords,
                _ => KEYWORDS,
            };
            let url = escape(&url);
            let [short, full] = LLMS.map(absolute);
            (
                "max-image-preview:large",
                format!(
                    "\n    <meta name=\"keywords\" content=\"{keywords}\" />\
                     \n    <link rel=\"canonical\" href=\"{url}\" />\
                     \n    <link rel=\"alternate\" hreflang=\"en\" href=\"{url}\" />\
                     \n    <link rel=\"alternate\" hreflang=\"x-default\" href=\"{url}\" />\
                     \n    <link rel=\"llms-txt\" href=\"{short}\" />\
                     \n    <link rel=\"llms-full-txt\" href=\"{full}\" />\
                     \n    <link rel=\"sitemap\" type=\"application/xml\" href=\"{SITE}sitemap.xml\" />",
                    keywords = escape(keywords),
                ),
            )
        }
    };
    let json_ld = json_ld(head.kind)
        .map(|ld| format!("\n    <script type=\"application/ld+json\">{ld}</script>"))
        .unwrap_or_default();
    format!(
        "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\" />\n    \
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n    \
         <title>{title}</title>\n    \
         <meta name=\"description\" content=\"{description}\" />\n    \
         <meta name=\"robots\" content=\"{robots}\" />\n    \
         <meta name=\"referrer\" content=\"strict-origin-when-cross-origin\" />\n    \
         <meta name=\"theme-color\" content=\"#000000\" />{canonical}\n    \
         <link rel=\"alternate\" type=\"application/atom+xml\" title=\"{NAME}\" href=\"{FEED}\" />\n    \
         <meta property=\"og:site_name\" content=\"LispM\" />\n    \
         <meta property=\"og:type\" content=\"{kind}\" />\n    \
         <meta property=\"og:url\" content=\"{url}\" />\n    \
         <meta property=\"og:title\" content=\"{headline}\" />\n    \
         <meta property=\"og:description\" content=\"{description}\" />\n    \
         <meta property=\"og:image\" content=\"{card}\" />\n    \
         <meta property=\"og:image:type\" content=\"image/png\" />\n    \
         <meta property=\"og:image:width\" content=\"1200\" />\n    \
         <meta property=\"og:image:height\" content=\"630\" />\n    \
         <meta property=\"og:image:alt\" content=\"{card_alt}\" />\n    \
         <meta name=\"twitter:card\" content=\"summary_large_image\" />\n    \
         <meta name=\"twitter:title\" content=\"{headline}\" />\n    \
         <meta name=\"twitter:description\" content=\"{description}\" />\n    \
         <meta name=\"twitter:image\" content=\"{card}\" />\n    \
         <meta name=\"twitter:image:alt\" content=\"{card_alt}\" />{article}\n    \
         {links}\n    \
         <link rel=\"apple-touch-icon\" href=\"/apple-touch-icon.png\" />{json_ld}\n  </head>\n  \
         <body>\n{page}\n  </body>\n</html>\n",
        title = escape(&head.title),
        description = escape(head.description),
        url = escape(&url),
        headline = escape(head.headline),
        card = escape(&card),
        card_alt = escape(card_alt),
    )
}

/// Date the site's main and feature pages were last updated.
const SITE_UPDATED: &str = "2026-11-24";

/// Returns the stills visibly rendered on the home page: the film's poster,
/// the three classic desktop themes, each still of `IDEA_STAGES`, and the
/// pillar cards' screenshots.
fn home_stills() -> Vec<&'static str> {
    std::iter::once(FILM.poster)
        .chain(
            home_illustrations()
                .into_iter()
                .map(|item| item.media.still()),
        )
        .collect()
}

/// Returns the stills visibly rendered on a feature page (`themes`, `stages`,
/// and `spotlights`).
fn page_stills(
    themes: &'static [DesktopTheme],
    stages: &'static [Stage],
    spotlights: &'static [Spotlight],
) -> Vec<&'static str> {
    themes
        .iter()
        .map(|theme| theme.picture.image)
        .chain(stages.iter().map(|stage| stage.media.still()))
        .chain(
            spotlights
                .iter()
                .filter_map(|spotlight| spotlight.clip.map(|clip| clip.poster)),
        )
        .collect()
}

/// Returns the site's map: the home page and each dedicated feature page with
/// the stills and videos visibly rendered on it, the blog (as new as its
/// latest post), the posts, and the pages that stand alone.
pub fn sitemap() -> String {
    let mut out = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" \
         xmlns:image=\"http://www.google.com/schemas/sitemap-image/1.1\" \
         xmlns:video=\"http://www.google.com/schemas/sitemap-video/1.1\">\n",
    );
    let latest = POSTS.iter().map(|post| post.date).max();
    let mut desktop_stills = page_stills(&DESKTOP_THEMES, &IDEA_STAGES, &[]);
    desktop_stills.extend(page_stills(&[], &DESKTOP_STAGES, &DESKTOP_SPOTLIGHTS));
    let mut desktop_videos = stage_videos(&IDEA_STAGES, &[]);
    desktop_videos.extend(stage_videos(&DESKTOP_STAGES, &DESKTOP_SPOTLIGHTS));
    let pages = [
        (
            absolute("/"),
            Some(SITE_UPDATED.to_owned()),
            home_stills(),
            home_videos().collect::<Vec<_>>(),
        ),
        (
            absolute(AGENT),
            Some(SITE_UPDATED.to_owned()),
            page_stills(&[], &AGENTS_STAGES, &AGENTS_SPOTLIGHTS),
            stage_videos(&AGENTS_STAGES, &AGENTS_SPOTLIGHTS),
        ),
        (
            absolute(DESKTOP),
            Some(SITE_UPDATED.to_owned()),
            desktop_stills,
            desktop_videos,
        ),
        (
            absolute(APPS),
            Some(SITE_UPDATED.to_owned()),
            page_stills(&[], &STORE_STAGES, &[]),
            stage_videos(&STORE_STAGES, &[]),
        ),
        (
            absolute(REMOTE),
            Some(SITE_UPDATED.to_owned()),
            page_stills(&[], &REMOTE_STAGES, &[]),
            stage_videos(&REMOTE_STAGES, &[]),
        ),
        (
            absolute(QA),
            Some(SITE_UPDATED.to_owned()),
            Vec::new(),
            Vec::new(),
        ),
        (
            absolute(BLOG),
            latest.map(|date| date.iso()),
            Vec::new(),
            Vec::new(),
        ),
    ]
    .into_iter()
    .chain(POSTS.iter().map(|post| {
        let images = std::iter::once(post.cover.src)
            .chain(
                super::named_paths(post.body)
                    .into_iter()
                    .filter(|path| !path.ends_with(".mp4")),
            )
            .map(|path| path.trim_start_matches('/'))
            .collect();
        (
            absolute(&post.path()),
            Some(post.date.iso()),
            images,
            Vec::new(),
        )
    }))
    .chain(PAGES.iter().map(|page| {
        (
            absolute(page.path),
            Some(SITE_UPDATED.to_owned()),
            Vec::new(),
            Vec::new(),
        )
    }));
    for (url, modified, images, clips) in pages {
        let _ = write!(out, "  <url>\n    <loc>{}</loc>\n", escape(&url));
        if let Some(modified) = modified {
            let _ = writeln!(out, "    <lastmod>{modified}</lastmod>");
        }
        for image in images {
            let _ = writeln!(
                out,
                "    <image:image>\n      <image:loc>{}</image:loc>\n    </image:image>",
                escape(&format!("{SITE}{image}"))
            );
        }
        for (headline, clip) in clips {
            let _ = writeln!(
                out,
                "    <video:video>\n      <video:thumbnail_loc>{}</video:thumbnail_loc>\n      \
                 <video:title>{}</video:title>\n      <video:description>{}</video:description>\n      \
                 <video:content_loc>{}</video:content_loc>\n      \
                 <video:publication_date>{}</video:publication_date>\n    </video:video>",
                escape(&format!("{SITE}{}", clip.poster)),
                escape(headline),
                escape(clip.description),
                escape(&format!("{SITE}{}", clip.video)),
                clip.published.iso(),
            );
        }
        out.push_str("  </url>\n");
    }
    out.push_str("</urlset>\n");
    out
}

/// Where llms.txt is served, and llms-full.txt, the same with every post's
/// words.
pub const LLMS: [&str; 2] = ["/llms.txt", "/llms-full.txt"];

/// Returns the blog as a section of llms.txt: a link to each post, with
/// what it is about, and where all their words are.
pub fn llms_section() -> String {
    let mut out = format!("## Blog\n\n{HEADLINE} {LEAD}\n\n");
    for post in &POSTS {
        let _ = writeln!(
            out,
            "- [{}]({}): {} ({}, {})",
            post.title,
            absolute(&post.path()),
            post.description,
            post.author.name,
            post.date.long(),
        );
    }
    let [_, full] = LLMS;
    let _ = writeln!(
        out,
        "\nEvery post's words, after all of this: [llms-full.txt]({}). New posts, as they come: \
         [the feed]({}).",
        absolute(full),
        absolute(FEED),
    );
    out
}

/// Returns `markdown` with the links and pictures it names from the root
/// named by their whole address, to be read away from the site.
fn absolute_links(markdown: &str) -> String {
    markdown.replace("](/", &format!("]({SITE}"))
}

/// Returns every post as Markdown, the latest first, for llms-full.txt: its
/// title, author, date and address, then its words, their headings two
/// levels down, under the post's.
pub fn llms_posts() -> String {
    let mut out = String::from("## The posts, in full\n");
    for post in &POSTS {
        let body = format!("\n{}", absolute_links(post.body))
            .replace("\n## ", "\n#### ")
            .replace("\n### ", "\n##### ");
        let _ = write!(
            out,
            "\n### {}\n\nBy {}, {}. {}\n\n{}",
            post.title,
            post.author.name,
            post.date.long(),
            absolute(&post.path()),
            body.trim_start(),
        );
        if !out.ends_with('\n') {
            out.push('\n');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const BUILT: &str = "<head><link rel=\"canonical\" href=\"https://lispm.ai/\" />\
        <link rel=\"icon\" href=\"/lispm-lambda-43b8.png\" integrity=\"sha384-a\"/>\
        <link rel=\"stylesheet\" href=\"/site-8ec6.css\" integrity=\"sha384-b\"/>\
        <link rel=\"modulepreload\" href=\"/lispm-site-ae01.js\"></head>";

    #[test]
    fn the_pages_share_the_built_stylesheet_and_icon_only() {
        assert_eq!(
            shared_links(BUILT),
            Ok(
                "<link rel=\"icon\" href=\"/lispm-lambda-43b8.png\" integrity=\"sha384-a\"/>\n    \
                <link rel=\"stylesheet\" href=\"/site-8ec6.css\" integrity=\"sha384-b\"/>"
                    .to_owned()
            )
        );
        assert_eq!(
            shared_links("<head><link rel=\"icon\" href=\"/a.png\"></head>"),
            Err(BlogError::NoStylesheet)
        );
    }

    #[test]
    fn every_page_has_its_own_card_title_and_address() {
        for (head, path) in [
            (agent_head(), AGENT.to_owned()),
            (desktop_head(), DESKTOP.to_owned()),
            (apps_head(), APPS.to_owned()),
            (remote_head(), REMOTE.to_owned()),
            (questions_head(), QA.to_owned()),
            (blog_head(), "/blog/".to_owned()),
        ]
        .into_iter()
        .chain(
            POSTS
                .iter()
                .map(|post| (post_head(post), format!("/blog/{}/", post.slug))),
        )
        .chain(
            PAGES
                .iter()
                .map(|page| (page_head(page), page.path.to_owned())),
        ) {
            let html = document(
                &head,
                "<link rel=\"stylesheet\" href=\"/s.css\"/>",
                "<main/>",
            );
            let url = format!("https://lispm.ai{path}");
            assert!(html.contains(&format!("<link rel=\"canonical\" href=\"{url}\" />")));
            assert!(html.contains(&format!(
                "<link rel=\"alternate\" hreflang=\"en\" href=\"{url}\" />"
            )));
            assert!(html.contains(&format!(
                "<link rel=\"alternate\" hreflang=\"x-default\" href=\"{url}\" />"
            )));
            assert!(html.contains("<link rel=\"llms-txt\" href=\"https://lispm.ai/llms.txt\" />"));
            assert!(html.contains(
                "<link rel=\"llms-full-txt\" href=\"https://lispm.ai/llms-full.txt\" />"
            ));
            assert!(html.contains(
                "<link rel=\"sitemap\" type=\"application/xml\" href=\"https://lispm.ai/sitemap.xml\" />"
            ));
            assert!(html.contains(&format!("<meta property=\"og:url\" content=\"{url}\" />")));
            assert!(html.contains(&format!(
                "<meta property=\"og:image\" content=\"https://lispm.ai{}\" />",
                head.card.0
            )));
            assert!(html.contains(&format!(
                "<meta name=\"twitter:image\" content=\"https://lispm.ai{}\" />",
                head.card.0
            )));
            assert!(html.contains("<link rel=\"stylesheet\" href=\"/s.css\"/>"));
            assert!(html.contains("<body>\n<main/>\n  </body>"));
            assert!(html.contains("<meta name=\"robots\" content=\"max-image-preview:large\" />"));
            assert!(html.contains("<link rel=\"alternate\" type=\"application/atom+xml\""));
            assert_eq!(html.matches("<title>").count(), 1);
            assert!(head.title.chars().count() <= 65, "{}", head.title);
            assert!(head.description.chars().count() <= super::super::DESCRIPTION_MOST);
        }
    }

    #[test]
    fn the_missing_page_is_kept_out_of_search_engines() {
        let html = document(&missing_head(), "", "<main/>");
        assert!(html.contains("<meta name=\"robots\" content=\"noindex\" />"));
        assert!(!html.contains("rel=\"canonical\""));
        assert!(!html.contains("application/ld+json"));
        assert_eq!(json_ld(Kind::Missing), None);
    }

    #[test]
    fn a_post_is_shared_as_an_article_by_its_author() {
        for post in &POSTS {
            let html = document(&post_head(post), "", "");
            assert!(html.contains("<meta property=\"og:type\" content=\"article\" />"));
            assert!(html.contains(&format!(
                "<meta property=\"article:published_time\" content=\"{}\" />",
                post.date.iso()
            )));
            assert!(html.contains(&format!(
                "<meta name=\"twitter:creator\" content=\"@{}\" />",
                post.author.handle
            )));
            assert!(html.contains(&format!(
                "<title>{} — LispM Blog</title>",
                escape(post.title)
            )));
            assert!(html.contains(&format!(
                "<meta name=\"description\" content=\"{}\" />",
                escape(post.summary)
            )));
            assert!(html.contains(&format!(
                "<meta name=\"author\" content=\"{}\" />",
                escape(post.author.name)
            )));
            assert!(html.contains(&format!(
                "<meta name=\"keywords\" content=\"{}\" />",
                escape(post.keywords)
            )));
        }
        let html = document(&blog_head(), "", "");
        assert!(html.contains("<meta property=\"og:type\" content=\"website\" />"));
        assert!(!html.contains("article:"));
    }

    #[test]
    fn a_page_that_stands_alone_is_a_web_page_of_the_site_with_its_card() {
        for page in &PAGES {
            let html = document(&page_head(page), "", "");
            assert!(html.contains("<meta property=\"og:type\" content=\"website\" />"));
            assert!(!html.contains("article:"));
            assert!(html.contains(&format!("<title>{}</title>", escape(page.meta_title))));
            assert!(html.contains(&format!(
                "<meta property=\"og:title\" content=\"{}\" />",
                escape(page.meta_title)
            )));
            assert!(html.contains(&format!(
                "<meta name=\"description\" content=\"{}\" />",
                escape(page.description)
            )));
            assert!(html.contains(&format!(
                "<meta property=\"og:image\" content=\"https://lispm.ai{}\" />",
                pages::CARD.src
            )));
            let ld = json_ld(Kind::Page(page)).unwrap_or_default();
            assert!(ld.contains(&format!(
                ",{{\"@type\":\"WebPage\",\"@id\":{}",
                json_string(&absolute(page.path))
            )));
            assert!(ld.contains(&format!("\"isPartOf\":{}", Node::WebSite.reference())));
            assert!(!ld.contains("</script"));
            assert_eq!(ld.matches('{').count(), ld.matches('}').count());
        }
    }

    #[test]
    fn heads_are_escaped() {
        let mut head = blog_head();
        head.title = "a \"b\" <c>".to_owned();
        let html = document(&head, "", "");
        assert!(html.contains("<title>a &quot;b&quot; &lt;c&gt;</title>"));
    }

    #[test]
    fn the_json_ld_names_every_post_and_no_script_ends_in_it() {
        let blog = json_ld(Kind::Blog).unwrap_or_default();
        let organization = format!(
            "{{\"@context\":\"https://schema.org\",\"@graph\":[{{\"@type\":\"Organization\",\
             \"@id\":{}",
            json_string(&Node::Organization.id())
        );
        for post in &POSTS {
            assert!(blog.contains(&json_string(&absolute(&post.path()))));
            let article = json_ld(Kind::Article(post)).unwrap_or_default();
            assert!(article.starts_with(&organization), "{article}");
            assert!(article.contains(",{\"@type\":\"BlogPosting\""));
            assert!(article.contains(&format!("\"datePublished\":\"{}\"", post.date.iso())));
            assert!(article.contains(&format!(
                "\"mainEntityOfPage\":{}",
                json_string(&absolute(&post.path()))
            )));
            for ld in [&blog, &article] {
                assert!(!ld.contains("</script"));
                assert_eq!(ld.matches('{').count(), ld.matches('}').count());
                assert_eq!(ld.matches('[').count(), ld.matches(']').count());
            }
        }
        assert!(blog.starts_with(&organization));
    }

    #[test]
    fn lispm_writes_as_the_organization_and_people_as_people() {
        assert_eq!(author(LISPM), Node::Organization.reference());
        let person = author(super::super::RYAN_KUNG);
        assert!(person.starts_with("{\"@type\":\"Person\",\"name\":\"Ryan Kung\""));
    }

    #[test]
    fn the_map_lists_the_home_page_the_blog_every_post_and_every_page() {
        let map = sitemap();
        assert!(map.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset "));
        assert!(map.ends_with("</urlset>\n"));
        assert!(map.contains("<loc>https://lispm.ai/</loc>"));
        assert!(!map.contains("llms"), "pages only");
        let latest = POSTS
            .iter()
            .map(|post| post.date)
            .max()
            .map(|date| date.iso());
        assert_eq!(
            latest.map(|date| map.contains(&format!(
                "<loc>https://lispm.ai/blog/</loc>\n    <lastmod>{date}</lastmod>"
            ))),
            Some(true)
        );
        for post in &POSTS {
            assert!(map.contains(&format!(
                "<loc>{}</loc>\n    <lastmod>{}</lastmod>",
                absolute(&post.path()),
                post.date.iso()
            )));
        }
        for page in &PAGES {
            assert!(map.contains(&format!(
                "<loc>{}</loc>\n    <lastmod>{SITE_UPDATED}</lastmod>\n  </url>",
                absolute(page.path)
            )));
        }
        for path in [AGENT, DESKTOP, APPS, REMOTE, QA] {
            assert!(map.contains(&format!(
                "<loc>{}</loc>\n    <lastmod>{SITE_UPDATED}</lastmod>",
                absolute(path)
            )));
        }
        assert_eq!(map.matches("<url>").count(), 7 + POSTS.len() + PAGES.len());
    }

    #[test]
    fn the_map_attaches_stills_and_videos_to_the_pages_that_render_them() {
        let map = sitemap();
        let home = map.split("</url>").next().unwrap_or_default();
        assert!(home.contains("<loc>https://lispm.ai/</loc>"));
        for still in home_stills() {
            assert!(
                home.contains(&format!("<image:loc>{SITE}{still}</image:loc>")),
                "{still}"
            );
        }
        for (headline, clip) in home_videos() {
            assert!(
                home.contains(&format!(
                    "<video:content_loc>{SITE}{}</video:content_loc>",
                    clip.video
                )),
                "{headline}"
            );
        }
        assert!(!home.contains("assets/video/agent.mp4"));
        assert!(map.contains("assets/video/agent.mp4"));
        assert!(map.contains("assets/video/desktop.mp4"));
        assert!(map.contains("assets/video/app-store-ai.mp4"));
        assert!(map.contains("assets/video/remote-pair.mp4"));
        assert!(map.contains(
            "<image:loc>https://lispm.ai/assets/blog/zero-lock-how-lispm-kernel-made-emacs-157x-faster/cover.webp</image:loc>"
        ));
        assert!(map.contains(
            "<image:loc>https://lispm.ai/assets/blog/zero-lock-how-lispm-kernel-made-emacs-157x-faster/benchmark.webp</image:loc>"
        ));
    }

    #[test]
    fn llms_txt_links_every_post() {
        let section = llms_section();
        assert!(section.starts_with("## Blog\n\n"));
        for post in &POSTS {
            assert!(section.contains(&format!("- [{}]({})", post.title, absolute(&post.path()))));
        }
        assert!(section.contains("(https://lispm.ai/llms-full.txt)"));
        assert!(section.ends_with('\n'));
    }

    #[test]
    fn llms_full_txt_has_every_post_whole_its_links_whole_too() {
        let posts = llms_posts();
        for post in &POSTS {
            assert!(posts.contains(&format!("\n### {}\n\nBy {}", post.title, post.author.name)));
            let first = post
                .body
                .lines()
                .find(|line| !line.is_empty() && !line.starts_with('#') && !line.contains("]("));
            assert!(
                first.is_some_and(|line| posts.contains(line)),
                "{}",
                post.slug
            );
        }
        assert!(!posts.contains("](/"));
        assert!(
            !posts.contains("\n## "),
            "a post's headings are under its title"
        );
        assert!(posts.ends_with('\n'));
    }
}
