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
use crate::content::{BLOG, FILM, SITE, illustrations};
use crate::pages::{self, PAGES, Page};
use crate::publish::{Node, graph, json_array, json_string, organization};

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
    /// The list of posts.
    Blog,
    /// A post.
    Article(&'static Post),
    /// A page that stands alone: LispM Mail's, or the privacy policy.
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

/// Returns the head of the list of posts.
pub fn blog_head() -> Head {
    let headline = "LispM Blog: notes from the lab";
    Head {
        title: headline.to_owned(),
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
         \"url\":{url},\"mainEntityOfPage\":{url},\"datePublished\":{},\"image\":{},\
         \"author\":{},\"publisher\":{},\"isPartOf\":{{\"@id\":{}}},\"inLanguage\":\"en\"{}}}",
        json_string(&format!("{url}#article")),
        json_string(post.title),
        json_string(post.description),
        json_string(&post.date.iso()),
        json_string(&absolute(post.card)),
        author(post.author),
        Node::Organization.reference(),
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

/// Returns `page`, a page that stands alone, as a schema.org WebPage of
/// the site.
fn web_page(page: &Page) -> String {
    format!(
        "{{\"@type\":\"WebPage\",\"@id\":{url},\"url\":{url},\"name\":{},\"description\":{},\
         \"inLanguage\":\"en\",\"isPartOf\":{},\"publisher\":{}}}",
        json_string(page.meta_title),
        json_string(page.description),
        Node::WebSite.reference(),
        Node::Organization.reference(),
        url = json_string(&absolute(page.path)),
    )
}

/// Returns the JSON-LD of a page, LispM the organization in it, or nothing
/// for a page kept out of search engines.
pub fn json_ld(kind: Kind) -> Option<String> {
    let page = match kind {
        Kind::Blog => blog(),
        Kind::Article(post) => posting(post),
        Kind::Page(page) => web_page(page),
        Kind::Missing => return None,
    };
    Some(graph([organization(), page].into_iter()))
}

/// Returns a page as a whole document: `head`, the shared `links` and `page`,
/// the page's body as Yew rendered it.
pub fn document(head: &Head, links: &str, page: &str) -> String {
    let url = absolute(&head.path);
    let (card, card_alt) = head.card;
    let card = absolute(card);
    let (kind, article) = match head.kind {
        Kind::Blog | Kind::Page(_) | Kind::Missing => ("website", String::new()),
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
        Kind::Blog | Kind::Article(_) | Kind::Page(_) => (
            "max-image-preview:large",
            format!("\n    <link rel=\"canonical\" href=\"{}\" />", escape(&url)),
        ),
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

/// Returns the stills the home page shows, top to bottom, each a path from
/// the root: the film's poster, then each clip's poster and each picture
/// beside the chapters.
fn stills() -> impl Iterator<Item = &'static str> {
    std::iter::once(FILM.poster)
        .chain(illustrations().map(|illustration| illustration.media.still()))
}

/// Returns the site's map: the home page, with the stills it shows, the
/// blog, as new as its latest post, the posts, and the pages that stand
/// alone. Its pages only: llms.txt, the home page again in Markdown, is for
/// language models, which find it at its known address.
pub fn sitemap() -> String {
    let mut out = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" \
         xmlns:image=\"http://www.google.com/schemas/sitemap-image/1.1\">\n",
    );
    let latest = POSTS.iter().map(|post| post.date).max();
    let pages = [
        (absolute("/"), None, stills().collect()),
        (absolute(BLOG), latest.map(|date| date.iso()), Vec::new()),
    ]
    .into_iter()
    .chain(
        POSTS
            .iter()
            .map(|post| (absolute(&post.path()), Some(post.date.iso()), Vec::new())),
    )
    .chain(
        PAGES
            .iter()
            .map(|page| (absolute(page.path), None, Vec::new())),
    );
    for (url, modified, images) in pages {
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
        for (head, path) in std::iter::once((blog_head(), "/blog/".to_owned()))
            .chain(
                POSTS
                    .iter()
                    .map(|post| (post_head(post), format!("/blog/{}/", post.slug))),
            )
            .chain(
                PAGES
                    .iter()
                    .map(|page| (page_head(page), page.path.to_owned())),
            )
        {
            let html = document(
                &head,
                "<link rel=\"stylesheet\" href=\"/s.css\"/>",
                "<main/>",
            );
            let url = format!("https://lispm.ai{path}");
            assert!(html.contains(&format!("<link rel=\"canonical\" href=\"{url}\" />")));
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
            assert!(map.contains(&format!("<loc>{}</loc>\n  </url>", absolute(page.path))));
        }
        assert_eq!(map.matches("<url>").count(), 2 + POSTS.len() + PAGES.len());
    }

    #[test]
    fn the_map_shows_the_home_pages_stills_there_only() {
        let map = sitemap();
        let home = map.split("</url>").next().unwrap_or_default();
        assert!(home.contains("<loc>https://lispm.ai/</loc>"));
        let mut shown = 0;
        for still in stills() {
            assert!(
                home.contains(&format!("<image:loc>{SITE}{still}</image:loc>")),
                "{still}"
            );
            shown += 1;
        }
        assert!(shown > 1);
        assert_eq!(map.matches("<image:image>").count(), shown);
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
