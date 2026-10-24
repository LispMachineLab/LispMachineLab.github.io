//! What is written beside the page for crawlers and language models, all
//! from [`crate::content`]: the page rendered into index.html before
//! WebAssembly loads, JSON-LD describing the site and its projects, and
//! llms.txt, the page as Markdown.

use std::fmt::{self, Write as _};

use crate::content::{
    BACKENDS, CREDITS, DOWNLOAD_LEAD, DOWNLOAD_STEPS, EXPERIMENTAL, EXPERIMENTAL_BACK,
    EXPERIMENTAL_HEADLINE, FILM, IDEA, IDEA_HEADLINE, IDEA_POINTS, INSTALL_BACK,
    INSTALL_EXPERIMENTAL, INSTALLER, INSTALLER_DOES, INSTALLER_SOURCE, LICENSE, ORGANIZATION,
    OWN_EMACS, OWN_EMACS_HEADLINE, PROJECTS, Point, Project, REQUIREMENTS, SITE, SUMMARY, TAGLINE,
};

/// The id of the element the page is rendered into and hydrated in.
pub const ROOT_ID: &str = "app";

/// Why the page could not be put into index.html.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PublishError {
    /// index.html has no empty `<div id="app"></div>` to render into.
    NoRoot,
    /// index.html has no `</head>` to put the JSON-LD before.
    NoHead,
}

impl fmt::Display for PublishError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PublishError::NoRoot => {
                write!(f, "index.html has no empty <div id=\"{ROOT_ID}\"></div>")
            }
            PublishError::NoHead => write!(f, "index.html has no </head>"),
        }
    }
}

impl std::error::Error for PublishError {}

/// Returns `index` with `page` in its root element and `head` before `</head>`.
///
/// Pre: `index` holds one empty root element and a `</head>`.
/// Post: the result differs from `index` only by `page` and `head`, inserted there.
pub fn insert_page(index: &str, page: &str, head: &str) -> Result<String, PublishError> {
    let root = format!("<div id=\"{ROOT_ID}\"></div>");
    let (before_root, after_root) = index.split_once(&root).ok_or(PublishError::NoRoot)?;
    let (before_head, after_head) = before_root
        .split_once("</head>")
        .ok_or(PublishError::NoHead)?;
    Ok(format!(
        "{before_head}{head}</head>{after_head}<div id=\"{ROOT_ID}\">{page}</div>{after_root}"
    ))
}

/// Returns `text` as a JSON string, safe inside an HTML `<script>` too.
pub(crate) fn json_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            // `<` and `>` escaped, a `</script>` in the text cannot end the element.
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            c if u32::from(c) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", u32::from(c));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Returns a JSON array of `items`, each already JSON.
fn json_array(items: impl Iterator<Item = String>) -> String {
    format!("[{}]", items.collect::<Vec<_>>().join(","))
}

/// Returns a project as a schema.org SoftwareSourceCode.
fn source_code(project: &Project) -> String {
    format!(
        "{{\"@type\":\"SoftwareSourceCode\",\"name\":{},\"description\":{},\"codeRepository\":{},\
         \"programmingLanguage\":{},\"license\":{},\"runtimePlatform\":\"GNU Emacs\"}}",
        json_string(project.name),
        json_string(project.lead),
        json_string(project.repository),
        json_array(
            project
                .languages
                .iter()
                .map(|language| json_string(language))
        ),
        json_string(LICENSE),
    )
}

/// Returns LispM.app as a schema.org SoftwareApplication.
fn application() -> String {
    format!(
        "{{\"@type\":\"SoftwareApplication\",\"name\":\"LispM\",\"description\":{},\
         \"applicationCategory\":\"DeveloperApplication\",\"operatingSystem\":{},\
         \"downloadUrl\":{},\"isAccessibleForFree\":true,\"license\":{}}}",
        json_string(DOWNLOAD_LEAD),
        json_string(REQUIREMENTS),
        json_string(&format!("{SITE}{INSTALLER}")),
        json_string(LICENSE),
    )
}

/// Returns the JSON-LD describing the site, LispM.app and its projects.
pub fn json_ld() -> String {
    format!(
        "{{\"@context\":\"https://schema.org\",\"@type\":\"WebSite\",\"name\":\"LispM\",\"url\":{},\
         \"headline\":{},\"description\":{},\"publisher\":{{\"@type\":\"Organization\",\
         \"name\":\"LispMachineLab\",\"url\":{}}},\"about\":{}}}",
        json_string(SITE),
        json_string(TAGLINE),
        json_string(SUMMARY),
        json_string(ORGANIZATION),
        json_array(std::iter::once(application()).chain(PROJECTS.iter().map(source_code))),
    )
}

/// Returns what prerender puts before `</head>`: the JSON-LD.
pub fn head() -> String {
    format!(
        "<script type=\"application/ld+json\">{}</script>",
        json_ld()
    )
}

/// Returns `points` as a Markdown list.
fn markdown_points(points: &[Point]) -> String {
    points
        .iter()
        .map(|point| {
            format!(
                "- **{}**: {}\n",
                point.title.trim_end_matches('.'),
                point.text
            )
        })
        .collect()
}

/// Returns a project as a Markdown section.
fn markdown_project(project: &Project) -> String {
    let mut out = format!(
        "## {}: {}\n\n{}\n\n",
        project.name, project.headline, project.lead
    );
    for stage in project.stages {
        let _ = writeln!(out, "- **{}** {}", stage.headline, stage.text);
    }
    out.push('\n');
    if let Some(spotlight) = &project.spotlight {
        let _ = writeln!(out, "### {}\n\n{}\n", spotlight.headline, spotlight.text);
        out.push_str(&markdown_points(spotlight.points));
        out.push('\n');
    }
    out.push_str(&markdown_points(project.points));
    let _ = write!(out, "\n{}\n\n", project.audience);
    if let Some(install) = project.install {
        let _ = write!(
            out,
            "```{}\n{}\n```\n\n",
            install.language.fence(),
            install.code
        );
    }
    let _ = writeln!(out, "Repository: {}\n", project.repository);
    out
}

/// Returns how to install LispM.app, as a Markdown section.
fn markdown_download() -> String {
    let mut out = format!("## Download LispM for Mac\n\n{DOWNLOAD_LEAD} {REQUIREMENTS}\n\n");
    for (number, step) in DOWNLOAD_STEPS.iter().enumerate() {
        let _ = write!(out, "{}. **{}**", number + 1, step.title);
        if !step.text.is_empty() {
            let _ = write!(out, " {}", step.text);
        }
        out.push('\n');
        if let Some(snippet) = step.snippet {
            let _ = writeln!(
                out,
                "\n   ```{}\n   {}\n   ```\n",
                snippet.language.fence(),
                snippet.code
            );
        }
    }
    let _ = writeln!(
        out,
        "\nThe line runs [install.sh]({INSTALLER_SOURCE}), which{INSTALLER_DOES}\n"
    );
    let _ = writeln!(
        out,
        "### {EXPERIMENTAL_HEADLINE}\n\n{EXPERIMENTAL}\n\n```{}\n{}\n```\n\n{EXPERIMENTAL_BACK} `{INSTALL_BACK}`\n",
        INSTALL_EXPERIMENTAL.language.fence(),
        INSTALL_EXPERIMENTAL.code
    );
    out
}

/// Returns llms.txt: the page as Markdown, after llmstxt.org.
pub fn llms_txt() -> String {
    let mut out = format!("# LispM\n\n> {TAGLINE} {SUMMARY}\n\n");
    out.push_str(&markdown_download());
    let _ = write!(out, "## {IDEA_HEADLINE}\n\n{IDEA}\n\n");
    out.push_str(&markdown_points(&IDEA_POINTS));
    out.push('\n');
    let _ = write!(
        out,
        "## {OWN_EMACS_HEADLINE}\n\n{OWN_EMACS} Each of their sections ends with how.\n\n"
    );
    for project in &PROJECTS {
        out.push_str(&markdown_project(project));
    }
    let _ = writeln!(
        out,
        "## Links\n\n\
         - [LispMachineLab on GitHub]({ORGANIZATION})\n\
         - [The film]({SITE}{}): {}\n\
         - [Model servers for LispM Agent]({BACKENDS})\n\n\
         LispM's projects are all free software, under the GNU Affero General Public License, version 3 or later. {CREDITS}",
        FILM.video, FILM.description,
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::INSTALL;

    const INDEX: &str =
        "<html><head><title>t</title></head><body><div id=\"app\"></div></body></html>";

    #[test]
    fn insert_page_fills_the_root_and_the_head() {
        let html = insert_page(INDEX, "<main>page</main>", "<script>x</script>");
        assert_eq!(
            html.as_deref(),
            Ok(
                "<html><head><title>t</title><script>x</script></head><body><div id=\"app\"><main>page</main></div></body></html>"
            )
        );
    }

    #[test]
    fn insert_page_needs_an_empty_root_and_a_head() {
        assert_eq!(
            insert_page("<head></head><body></body>", "", ""),
            Err(PublishError::NoRoot)
        );
        assert_eq!(
            insert_page("<div id=\"app\"></div>", "", ""),
            Err(PublishError::NoHead)
        );
        let rendered = insert_page(INDEX, "x", "").unwrap_or_default();
        assert_eq!(insert_page(&rendered, "x", ""), Err(PublishError::NoRoot));
    }

    #[test]
    fn json_strings_cannot_end_a_script() {
        assert_eq!(json_string("a \"b\" \\ c"), "\"a \\\"b\\\" \\\\ c\"");
        assert_eq!(json_string("</script>"), "\"\\u003c/script\\u003e\"");
        assert_eq!(json_string("a\nb\u{1}"), "\"a\\nb\\u0001\"");
    }

    #[test]
    fn the_json_ld_names_every_project_and_no_script_ends_in_it() {
        let ld = json_ld();
        for project in PROJECTS {
            assert!(
                ld.contains(&json_string(project.repository)),
                "{}",
                project.name
            );
        }
        assert!(ld.contains("\"downloadUrl\":\"https://lispm.ai/install.sh\""));
        assert!(!head().trim_end_matches("</script>").contains("</script"));
        assert_eq!(ld.matches('{').count(), ld.matches('}').count());
        assert_eq!(ld.matches('[').count(), ld.matches(']').count());
    }

    #[test]
    fn llms_txt_tells_every_project_and_how_to_install_it() {
        let text = llms_txt();
        assert!(text.starts_with("# LispM\n\n> "));
        for project in PROJECTS {
            assert!(text.contains(project.headline), "{}", project.name);
            if let Some(install) = project.install {
                assert!(text.contains(install.code), "{}", project.name);
            }
        }
        assert!(text.contains("Chrome extensions, inside Emacs."));
        assert!(text.contains(INSTALL_EXPERIMENTAL.code));
        let download = text.find(INSTALL.code).unwrap_or(usize::MAX);
        let idea = text.find(IDEA_HEADLINE).unwrap_or(0);
        assert!(download < idea, "the download comes first");
        assert!(text.ends_with('\n'));
    }
}
