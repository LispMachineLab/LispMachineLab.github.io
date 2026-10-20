//! The film under the headline and the clips beside each project's story.

use yew::prelude::*;

use crate::content::Clip;
use crate::effects;

/// The properties of a [`Loop`] or the [`Film`].
#[derive(Properties, PartialEq)]
pub struct ClipProps {
    /// The clip.
    pub clip: Clip,
}

/// A clip, muted and looped, playing while in sight. With less motion asked
/// for, it waits on its poster with its controls.
///
/// It is drawn without asking the browser anything, as prerender draws it
/// at build time; the motion the visitor prefers is read once it is in the
/// page.
#[function_component(Loop)]
pub fn clip_loop(props: &ClipProps) -> Html {
    let video = use_node_ref();
    let still = use_state_eq(|| false);
    {
        let video = video.clone();
        let still = still.clone();
        use_effect_with((), move |_| {
            let reduced = effects::prefers_reduced_motion();
            still.set(reduced);
            let watch = video
                .cast::<web_sys::HtmlVideoElement>()
                .filter(|_| !reduced)
                .and_then(|element| effects::play_in_sight(&element));
            move || drop(watch)
        });
    }
    let clip = props.clip;
    html! {
        <video ref={video} class="clip" src={clip.video} poster={clip.poster}
               muted=true loop=true playsinline=true preload="none" controls={*still}
               aria-label={clip.description} />
    }
}

/// The film, with sound: its poster and a play button until it is started,
/// then the browser's own controls.
#[function_component(Film)]
pub fn film(props: &ClipProps) -> Html {
    let video = use_node_ref();
    let started = use_state_eq(|| false);
    let start = {
        let video = video.clone();
        let started = started.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(element) = video.cast::<web_sys::HtmlVideoElement>() {
                effects::play(&element);
            }
            started.set(true);
        })
    };
    let clip = props.clip;
    html! {
        <div class={classes!("film", started.then_some("started"))}>
            <video ref={video} src={clip.video} poster={clip.poster} preload="none"
                   playsinline=true controls={*started} aria-label={clip.description} />
            if !*started {
                <button type="button" class="play" onclick={start}>
                    <span class="play-icon" aria-hidden="true" />
                    <span>{ "Watch the film" }</span>
                    <span class="length">{ "57 s" }</span>
                </button>
            }
        </div>
    }
}
