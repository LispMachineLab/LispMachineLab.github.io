//! The blog's pages: the list of posts, and each post. Prerender draws
//! them at build time, under the home page's navigation bar and over its
//! footer; they hold no state, and no WebAssembly runs on them.

use yew::prelude::*;

use crate::blog::{HEADLINE, LEAD, POSTS, Post, markdown};
use crate::content::{Anchor, BLOG};

use super::page::{Place, footer, navigation, outward};

/// The properties of a [`PostPage`].
#[derive(Properties, PartialEq)]
pub struct PostProps {
    /// The post.
    pub post: &'static Post,
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

/// Returns when a post was published, as a `<time>`.
fn date(post: &Post) -> Html {
    html! { <time datetime={post.date.iso()}>{ post.date.long() }</time> }
}

/// Returns a post in the list: its cover, date, author, title and what it
/// is about, all one link to it.
fn teaser(post: &'static Post) -> Html {
    html! {
        <li>
            <a href={post.path()}>
                <img src={post.cover.src} alt="" loading="lazy" decoding="async" />
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
    let body = Html::from_html_unchecked(markdown::to_html(post.body).into());
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
                    <img class="cover" src={post.cover.src} alt={post.cover.alt} />
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
                            <a class="button primary" href={Place::Blog.href(Anchor::Start)}>
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
