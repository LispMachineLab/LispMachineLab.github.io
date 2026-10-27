//! The pages that stand alone: LispM Mail's and the privacy policy.
//! Prerender draws them at build time as it draws the blog's, under the
//! home page's navigation bar and over its footer; they hold no state, and
//! no WebAssembly runs on them.

use yew::prelude::*;

use crate::blog::{markdown, size};
use crate::pages::Page;

use super::page::{Place, footer, navigation};

/// The properties of a [`Standalone`] page.
#[derive(Properties, PartialEq)]
pub struct StandaloneProps {
    /// The page.
    pub page: &'static Page,
}

/// A page that stands alone: its eyebrow and headline over its words, set
/// as a post's are.
#[function_component(Standalone)]
pub fn standalone(props: &StandaloneProps) -> Html {
    let page = props.page;
    let body = Html::from_html_unchecked(markdown::to_html(page.body, size::of).into());
    html! {
        <>
            { navigation(Place::Elsewhere) }
            <main class="standalone">
                <article>
                    <header class="standalone-head">
                        <p class="eyebrow">{ page.eyebrow }</p>
                        <h1>{ page.title }</h1>
                    </header>
                    <div class="prose">{ body }</div>
                </article>
            </main>
            { footer(Place::Elsewhere) }
        </>
    }
}
