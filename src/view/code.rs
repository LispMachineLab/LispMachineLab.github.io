//! A snippet of code with a button that copies it.

use gloo_timers::callback::Timeout;
use yew::prelude::*;

use crate::content::Snippet;
use crate::copy::CopyState;
use crate::effects;

/// Milliseconds a copy button shows its result before it is ready again.
const SETTLED_MS: u32 = 1_800;

/// The properties of a [`Code`] block.
#[derive(Properties, PartialEq)]
pub struct CodeProps {
    /// The snippet.
    pub snippet: Snippet,
}

/// A snippet, its language named over it and a copy button beside.
#[function_component(Code)]
pub fn code(props: &CodeProps) -> Html {
    let state = use_state_eq(CopyState::default);
    {
        let state = state.clone();
        use_effect_with(*state, move |shown| {
            let reset = shown
                .is_settled()
                .then(|| Timeout::new(SETTLED_MS, move || state.set(CopyState::Ready)));
            move || drop(reset)
        });
    }
    let text = props.snippet.code;
    let copy = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                state.set(CopyState::settled(effects::copy(text).await));
            });
        })
    };
    html! {
        <figure class="code">
            <figcaption>
                <span>{ props.snippet.language.name() }</span>
                <button type="button" class={classes!("copy", state.is_settled().then_some("settled"))}
                        onclick={copy} aria-live="polite">
                    { state.label() }
                </button>
            </figcaption>
            <pre><code>{ text }</code></pre>
        </figure>
    }
}
