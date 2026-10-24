//! A post's Markdown as HTML: CommonMark, with one addition. A picture
//! alone in its paragraph is drawn as a figure, its title, if it has one,
//! as the caption; and when it is a film, an `.mp4`, as a video, the `.jpg`
//! of the same name its poster.

use pulldown_cmark::{Event, Parser, Tag, TagEnd, html};

/// Returns `markdown` as HTML.
pub fn to_html(markdown: &str) -> String {
    let mut out = String::with_capacity(markdown.len() * 3 / 2);
    html::push_html(&mut out, figures(Parser::new(markdown)).into_iter());
    out
}

/// Returns `text` escaped for HTML, in text or in a quoted attribute.
pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            c => out.push(c),
        }
    }
    out
}

/// What the events held so far are.
enum Held {
    /// The start of a paragraph holding a picture alone, perhaps.
    Maybe,
    /// Anything else: they pass on as they are.
    No,
    /// A paragraph holding a picture alone, drawn as a figure.
    Figure(String),
}

/// Returns `events` with each paragraph holding a picture alone replaced by
/// its figure.
fn figures<'a>(events: impl IntoIterator<Item = Event<'a>>) -> Vec<Event<'a>> {
    let mut out = Vec::new();
    let mut held = Vec::new();
    for event in events {
        held.push(event);
        match lone_picture(&held) {
            Held::Maybe => {}
            Held::No => out.append(&mut held),
            Held::Figure(figure) => {
                out.push(Event::Html(figure.into()));
                held.clear();
            }
        }
    }
    out.append(&mut held);
    out
}

/// Returns what `held`, the events since the last that passed on, are.
fn lone_picture(held: &[Event<'_>]) -> Held {
    let [Event::Start(Tag::Paragraph), rest @ ..] = held else {
        return Held::No;
    };
    let [
        Event::Start(Tag::Image {
            dest_url, title, ..
        }),
        inner @ ..,
    ] = rest
    else {
        return if rest.is_empty() {
            Held::Maybe
        } else {
            Held::No
        };
    };
    match inner {
        [
            alt @ ..,
            Event::End(TagEnd::Image),
            Event::End(TagEnd::Paragraph),
        ] => Held::Figure(figure(dest_url, title, &plain_text(alt))),
        // Words after the picture: an ordinary paragraph.
        [.., Event::End(TagEnd::Image), _] => Held::No,
        _ => Held::Maybe,
    }
}

/// Returns the text in `events`, without its markup, as alt text is.
fn plain_text(events: &[Event<'_>]) -> String {
    events
        .iter()
        .filter_map(|event| match event {
            Event::Text(text) | Event::Code(text) => Some(text.as_ref()),
            Event::SoftBreak | Event::HardBreak => Some(" "),
            _ => None,
        })
        .collect()
}

/// Returns the figure of the picture or film at `src`, showing `alt`, with
/// `caption` under it unless it is empty.
fn figure(src: &str, caption: &str, alt: &str) -> String {
    let media = match src.strip_suffix(".mp4") {
        Some(stem) => format!(
            "<video src=\"{}\" poster=\"{}.jpg\" controls preload=\"none\" playsinline \
             aria-label=\"{}\"></video>",
            escape(src),
            escape(stem),
            escape(alt)
        ),
        None => format!(
            "<img src=\"{}\" alt=\"{}\" loading=\"lazy\" decoding=\"async\" />",
            escape(src),
            escape(alt)
        ),
    };
    let caption = if caption.is_empty() {
        String::new()
    } else {
        format!("<figcaption>{}</figcaption>", escape(caption))
    };
    format!("<figure>{media}{caption}</figure>\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_reads_as_commonmark() {
        assert_eq!(
            to_html("## A *head*\n\nWords, [a link](https://lispm.ai/).\n\n```sh\na | b\n```\n"),
            "<h2>A <em>head</em></h2>\n<p>Words, <a href=\"https://lispm.ai/\">a link</a>.</p>\n\
             <pre><code class=\"language-sh\">a | b\n</code></pre>\n"
        );
    }

    #[test]
    fn a_picture_alone_is_a_figure_its_title_the_caption() {
        assert_eq!(
            to_html("Before.\n\n![A *λ*, \"lit\"](/a.webp \"Its caption & more\")\n\nAfter.\n"),
            "<p>Before.</p>\n<figure><img src=\"/a.webp\" alt=\"A λ, &quot;lit&quot;\" \
             loading=\"lazy\" decoding=\"async\" /><figcaption>Its caption &amp; more\
             </figcaption></figure>\n<p>After.</p>\n"
        );
        assert_eq!(
            to_html("![Alt](/a.webp)\n"),
            "<figure><img src=\"/a.webp\" alt=\"Alt\" loading=\"lazy\" decoding=\"async\" />\
             </figure>\n"
        );
    }

    #[test]
    fn a_film_alone_is_a_video_on_its_poster() {
        assert_eq!(
            to_html("![A film](/f/film.mp4 \"With sound.\")\n"),
            "<figure><video src=\"/f/film.mp4\" poster=\"/f/film.jpg\" controls preload=\"none\" \
             playsinline aria-label=\"A film\"></video><figcaption>With sound.</figcaption>\
             </figure>\n"
        );
    }

    #[test]
    fn a_picture_among_words_stays_in_its_paragraph() {
        assert_eq!(
            to_html("![A](/a.webp) and words.\n"),
            "<p><img src=\"/a.webp\" alt=\"A\" /> and words.</p>\n"
        );
        assert_eq!(
            to_html("Words and ![A](/a.webp)\n"),
            "<p>Words and <img src=\"/a.webp\" alt=\"A\" /></p>\n"
        );
    }

    #[test]
    fn escaping_covers_text_and_attributes() {
        assert_eq!(
            escape("<a href=\"x\">'&'</a>"),
            "&lt;a href=&quot;x&quot;&gt;&#39;&amp;&#39;&lt;/a&gt;"
        );
    }
}
