//! What makes the blog's pages whole documents, all from [`super`]: each
//! page's head, its title, description, share card and JSON-LD, with the
//! stylesheet Trunk built for the home page; and, with the posts, the
//! site's map and llms.txt's list of them.

use std::fmt::{self, Write as _};

use super::markdown::escape;
use super::{CARD, HEADLINE, LEAD, POSTS, Post, absolute};
use crate::content::{BLOG, ORGANIZATION};
use crate::publish::json_string;

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

/// Returns the head of the list of posts.
pub fn blog_head() -> Head {
    Head {
        title: "LispM Blog".to_owned(),
        headline: "LispM Blog: notes from the lab",
        description: LEAD,
        path: BLOG.to_owned(),
        kind: Kind::Blog,
        card: (CARD.src, CARD.alt),
    }
}

/// Returns the head of `post`'s page.
pub fn post_head(post: &'static Post) -> Head {
    Head {
        title: format!("{} — LispM Blog", post.title),
        headline: post.title,
        description: post.description,
        path: post.path(),
        kind: Kind::Article(post),
        card: (post.card, post.cover.alt),
    }
}

/// Returns `post` as a schema.org BlogPosting.
fn posting(post: &Post) -> String {
    format!(
        "{{\"@type\":\"BlogPosting\",\"headline\":{},\"description\":{},\"url\":{},\
         \"datePublished\":{},\"image\":{},\"author\":{{\"@type\":\"Person\",\"name\":{},\
         \"url\":{}}},\"inLanguage\":\"en\"{}}}",
        json_string(post.title),
        json_string(post.description),
        json_string(&absolute(&post.path())),
        json_string(&post.date.iso()),
        json_string(&absolute(post.card)),
        json_string(post.author.name),
        json_string(&post.author.profile()),
        post.on_x
            .map(|on_x| format!(",\"sameAs\":{}", json_string(on_x)))
            .unwrap_or_default(),
    )
}

/// Returns the JSON-LD of a page.
pub fn json_ld(kind: Kind) -> String {
    match kind {
        Kind::Blog => format!(
            "{{\"@context\":\"https://schema.org\",\"@type\":\"Blog\",\"name\":\"LispM Blog\",\
             \"url\":{},\"description\":{},\"publisher\":{{\"@type\":\"Organization\",\
             \"name\":\"LispMachineLab\",\"url\":{}}},\"blogPost\":[{}]}}",
            json_string(&absolute(BLOG)),
            json_string(LEAD),
            json_string(ORGANIZATION),
            POSTS.iter().map(posting).collect::<Vec<_>>().join(","),
        ),
        Kind::Article(post) => {
            let posting = posting(post);
            let body = posting.strip_prefix('{').unwrap_or(&posting);
            format!("{{\"@context\":\"https://schema.org\",{body}")
        }
    }
}

/// Returns a page as a whole document: `head`, the shared `links` and `page`,
/// the page's body as Yew rendered it.
pub fn document(head: &Head, links: &str, page: &str) -> String {
    let url = absolute(&head.path);
    let (card, card_alt) = head.card;
    let card = absolute(card);
    let (kind, article) = match head.kind {
        Kind::Blog => ("website", String::new()),
        Kind::Article(post) => (
            "article",
            format!(
                "\n    <meta property=\"article:published_time\" content=\"{}\" />\
                 \n    <meta property=\"article:author\" content=\"{}\" />\
                 \n    <meta name=\"twitter:creator\" content=\"@{}\" />",
                post.date.iso(),
                escape(&post.author.profile()),
                escape(post.author.handle),
            ),
        ),
    };
    format!(
        "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\" />\n    \
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n    \
         <title>{title}</title>\n    \
         <meta name=\"description\" content=\"{description}\" />\n    \
         <meta name=\"theme-color\" content=\"#000000\" />\n    \
         <link rel=\"canonical\" href=\"{url}\" />\n    \
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
         <link rel=\"apple-touch-icon\" href=\"/assets/lispm-lambda.png\" />\n    \
         <script type=\"application/ld+json\">{json_ld}</script>\n  </head>\n  \
         <body>\n{page}\n  </body>\n</html>\n",
        title = escape(&head.title),
        description = escape(head.description),
        url = escape(&url),
        headline = escape(head.headline),
        card = escape(&card),
        card_alt = escape(card_alt),
        json_ld = json_ld(head.kind),
    )
}

/// Returns the site's map: the home page, llms.txt, the blog and its posts.
pub fn sitemap() -> String {
    let mut out = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    let pages = ["/", "/llms.txt", BLOG]
        .into_iter()
        .map(|path| (absolute(path), None))
        .chain(
            POSTS
                .iter()
                .map(|post| (absolute(&post.path()), Some(post.date.iso()))),
        );
    for (url, modified) in pages {
        let _ = write!(out, "  <url>\n    <loc>{}</loc>\n", escape(&url));
        if let Some(modified) = modified {
            let _ = writeln!(out, "    <lastmod>{modified}</lastmod>");
        }
        out.push_str("  </url>\n");
    }
    out.push_str("</urlset>\n");
    out
}

/// Returns the blog as a section of llms.txt: a link to each post, with
/// what it is about.
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
        for (head, path) in std::iter::once((blog_head(), "/blog/".to_owned())).chain(
            POSTS
                .iter()
                .map(|post| (post_head(post), format!("/blog/{}/", post.slug))),
        ) {
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
            assert_eq!(html.matches("<title>").count(), 1);
        }
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
        }
        let html = document(&blog_head(), "", "");
        assert!(html.contains("<meta property=\"og:type\" content=\"website\" />"));
        assert!(!html.contains("article:"));
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
        let blog = json_ld(Kind::Blog);
        for post in &POSTS {
            assert!(blog.contains(&json_string(&absolute(&post.path()))));
            let article = json_ld(Kind::Article(post));
            assert!(
                article
                    .starts_with("{\"@context\":\"https://schema.org\",\"@type\":\"BlogPosting\"")
            );
            assert!(article.contains(&format!("\"datePublished\":\"{}\"", post.date.iso())));
            for ld in [&blog, &article] {
                assert!(!ld.contains("</script"));
                assert_eq!(ld.matches('{').count(), ld.matches('}').count());
                assert_eq!(ld.matches('[').count(), ld.matches(']').count());
            }
        }
    }

    #[test]
    fn the_map_lists_the_home_page_the_blog_and_every_post() {
        let map = sitemap();
        assert!(map.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset "));
        assert!(map.ends_with("</urlset>\n"));
        for url in [
            "https://lispm.ai/",
            "https://lispm.ai/llms.txt",
            "https://lispm.ai/blog/",
        ] {
            assert!(map.contains(&format!("<loc>{url}</loc>")), "{url}");
        }
        for post in &POSTS {
            assert!(map.contains(&format!(
                "<loc>{}</loc>\n    <lastmod>{}</lastmod>",
                absolute(&post.path()),
                post.date.iso()
            )));
        }
        assert_eq!(map.matches("<url>").count(), 3 + POSTS.len());
    }

    #[test]
    fn llms_txt_links_every_post() {
        let section = llms_section();
        assert!(section.starts_with("## Blog\n\n"));
        for post in &POSTS {
            assert!(section.contains(&format!("- [{}]({})", post.title, absolute(&post.path()))));
        }
        assert!(section.ends_with('\n'));
    }
}
