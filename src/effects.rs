//! The site's boundary with the browser: the clipboard, the motion the
//! visitor prefers, the system it runs on, and playing videos. Nothing else
//! touches the page but Yew's rendering.

use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    HtmlVideoElement, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit,
};

use std::rc::Rc;

use crate::content::Platform;
use crate::copy::CopyError;
use crate::playback::PlayError;

/// The share of a clip that must be in sight for it to play.
const IN_SIGHT: f64 = 0.25;

/// Returns whether the visitor asked for less motion.
pub fn prefers_reduced_motion() -> bool {
    web_sys::window()
        .and_then(|window| {
            window
                .match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
        })
        .is_some_and(|query| query.matches())
}

/// Returns the system the visitor's browser runs on, the Mac when it does
/// not say.
pub fn platform() -> Platform {
    web_sys::window()
        .and_then(|window| window.navigator().user_agent().ok())
        .map_or(Platform::Mac, |agent| Platform::of_user_agent(&agent))
}

/// Scrolls to the section the address names after `#`, if any. The browser
/// looks for it before the page is drawn, so it is looked for again after.
pub fn reveal_addressed_section() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let id = window
        .location()
        .hash()
        .ok()
        .and_then(|hash| hash.strip_prefix('#').map(str::to_owned));
    let section = id
        .filter(|id| !id.is_empty())
        .and_then(|id| window.document()?.get_element_by_id(&id));
    if let Some(section) = section {
        section.scroll_into_view();
    }
}

/// Puts `text` on the clipboard.
pub async fn copy(text: &str) -> Result<(), CopyError> {
    let window = web_sys::window().ok_or(CopyError::Unavailable)?;
    // The clipboard exists only in a secure context; asking for it elsewhere throws.
    if !window.is_secure_context() {
        return Err(CopyError::Unavailable);
    }
    let written = window.navigator().clipboard().write_text(text);
    JsFuture::from(written)
        .await
        .map(drop)
        .map_err(|_| CopyError::Refused)
}

/// Starts `video`, then calls `settled` with whether it plays. A browser
/// may refuse, saving power or data or waiting for a gesture: Safari in Low
/// Power Mode starts no video on its own, muted or not.
pub fn play(video: &HtmlVideoElement, settled: impl FnOnce(Result<(), PlayError>) + 'static) {
    match video.play() {
        Ok(started) => spawn_local(async move {
            settled(
                JsFuture::from(started)
                    .await
                    .map(drop)
                    .map_err(|_| PlayError::Refused),
            );
        }),
        // A browser without promises from play() throws when it refuses.
        Err(_) => settled(Err(PlayError::Refused)),
    }
}

/// Plays a video while it is in sight and pauses it out of sight, for as
/// long as it is kept; dropping it stops watching.
pub struct InSight {
    observer: IntersectionObserver,
    _changed: Closure<dyn FnMut(js_sys::Array)>,
}

impl Drop for InSight {
    fn drop(&mut self) {
        self.observer.disconnect();
    }
}

/// Returns a watch playing `video` while a quarter of it is in sight, or
/// nothing if the browser cannot watch, the video then staying on its poster.
/// Each start it tries is told to `settled`, see [`play`].
pub fn play_in_sight(
    video: &HtmlVideoElement,
    settled: impl Fn(Result<(), PlayError>) + 'static,
) -> Option<InSight> {
    // A second handle on the same element, for the callback to own.
    let watched = video.clone();
    let settled = Rc::new(settled);
    let changed = Closure::<dyn FnMut(js_sys::Array)>::new(move |entries: js_sys::Array| {
        let seen = entries
            .iter()
            .filter_map(|entry| entry.dyn_into::<IntersectionObserverEntry>().ok())
            .any(|entry| entry.is_intersecting());
        if seen {
            let settled = Rc::clone(&settled);
            play(&watched, move |result| settled(result));
        } else {
            // Pausing a video cannot fail in a way worth telling: it stays as it was.
            let _ = watched.pause();
        }
    });
    let options = IntersectionObserverInit::new();
    options.set_threshold(&JsValue::from_f64(IN_SIGHT));
    let observer =
        IntersectionObserver::new_with_options(changed.as_ref().unchecked_ref(), &options).ok()?;
    observer.observe(video);
    Some(InSight {
        observer,
        _changed: changed,
    })
}
