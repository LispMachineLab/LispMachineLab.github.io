//! The film under the headline and the clips beside each project's story.

use std::rc::Rc;

use yew::prelude::*;

use crate::content::Clip;
use crate::effects;
use crate::playback::{Event, Playback};

/// The properties of a [`Loop`] or the [`Film`].
#[derive(Properties, PartialEq)]
pub struct ClipProps {
    /// The clip.
    pub clip: Clip,
}

/// A looping clip's playback, for [`use_reducer`]: what it shows, moved on
/// by what happens to it.
#[derive(Clone, Copy, Default, PartialEq)]
struct Shown(Playback);

impl Reducible for Shown {
    type Action = Event;

    fn reduce(self: Rc<Self>, event: Event) -> Rc<Self> {
        Rc::new(Shown(self.0.next(event)))
    }
}

/// A clip, muted and looped, playing while in sight. With less motion asked
/// for, it waits on its poster with its controls. When the browser will not
/// start it on its own, saving power or data, it waits on its poster with a
/// play button, so that it reads as a video and a click starts it.
///
/// It is drawn without asking the browser anything, as prerender draws it
/// at build time; the motion the visitor prefers is read once it is in the
/// page.
#[function_component(Loop)]
pub fn clip_loop(props: &ClipProps) -> Html {
    let video = use_node_ref();
    let shown = use_reducer_eq(Shown::default);
    {
        let video = video.clone();
        let shown = shown.dispatcher();
        use_effect_with((), move |_| {
            let reduced = effects::prefers_reduced_motion();
            shown.dispatch(Event::Motion { reduced });
            let watch = video
                .cast::<web_sys::HtmlVideoElement>()
                .filter(|_| !reduced)
                .and_then(|element| {
                    effects::play_in_sight(&element, move |result| {
                        shown.dispatch(Event::Played(result));
                    })
                });
            move || drop(watch)
        });
    }
    let start = {
        let video = video.clone();
        let shown = shown.dispatcher();
        Callback::from(move |_: MouseEvent| {
            if let Some(element) = video.cast::<web_sys::HtmlVideoElement>() {
                let shown = shown.clone();
                effects::play(&element, move |result| {
                    shown.dispatch(Event::Clicked(result));
                });
            }
        })
    };
    let clip = props.clip;
    let playback = shown.0;
    html! {
        <>
            <video ref={video} class="clip" src={clip.video} poster={clip.poster}
                   muted=true loop=true playsinline=true preload="none" controls={playback.controls()}
                   aria-label={clip.description} />
            if playback.offers_play() {
                <button type="button" class="play" onclick={start}>
                    <span class="play-icon" aria-hidden="true" />
                    <span>{ "Play" }</span>
                </button>
            }
        </>
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
                // Started by a click, it plays; refused all the same, its
                // controls show, as they do once it is started.
                effects::play(&element, drop);
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
