//! The site's boundary with the browser: the viewport and the time are read
//! here, pages are visited and windows revealed here, and nowhere else.

use gloo_events::EventListener;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{Element, KeyboardEvent, PointerEvent, ScrollBehavior, ScrollIntoViewOptions};

use crate::model::clock::ClockTime;
use crate::model::command::Link;
use crate::model::geometry::{Point, Viewport};
use crate::model::window::WindowId;

/// Returns `value` as whole pixels: truncated toward zero, saturating at
/// `i32`'s bounds, and zero for a value that is not a number.
fn pixels(value: f64) -> i32 {
    // Rust's float-to-integer `as` is total: it saturates and maps NaN to 0.
    value as i32
}

/// Returns the browser's viewport, or an empty one outside a browser.
pub fn viewport() -> Viewport {
    let extent = |read: Result<web_sys::wasm_bindgen::JsValue, _>| {
        read.ok().and_then(|value| value.as_f64()).map_or(0, pixels)
    };
    web_sys::window().map_or(Viewport::new(0, 0), |window| {
        Viewport::new(extent(window.inner_width()), extent(window.inner_height()))
    })
}

/// Calls `changed` with the new viewport whenever the browser's is resized,
/// for as long as the listener returned is kept.
pub fn on_resize(changed: impl Fn(Viewport) + 'static) -> Option<EventListener> {
    let window = web_sys::window()?;
    Some(EventListener::new(&window, "resize", move |_| {
        changed(viewport())
    }))
}

/// Calls `pressed` whenever Escape is pressed, for as long as the listener
/// returned is kept.
pub fn on_escape(pressed: impl Fn() + 'static) -> Option<EventListener> {
    let window = web_sys::window()?;
    Some(EventListener::new(&window, "keydown", move |event| {
        let escape = event
            .dyn_ref::<KeyboardEvent>()
            .is_some_and(|key| key.key() == "Escape");
        if escape {
            pressed();
        }
    }))
}

/// Returns the local time, if the browser can tell it.
pub fn now() -> Option<ClockTime> {
    let date = js_sys::Date::new_0();
    ClockTime::new(date.get_hours(), date.get_minutes())
}

/// Returns where `event` happened on the viewport.
pub fn pointer(event: &PointerEvent) -> Point {
    Point::new(event.client_x(), event.client_y())
}

/// Returns whether `event` is the primary button's, the one that drags.
pub fn is_primary(event: &PointerEvent) -> bool {
    event.button() == 0
}

/// Sends the pointer's later events to the element `event` pressed, so a
/// drag follows the pointer beyond it.
///
/// Yew listens for events at the page's root, so the element pressed, not
/// the listener, takes the capture; its events still reach the listener,
/// which Yew finds among the element's ancestors.
pub fn capture_pointer(event: &PointerEvent) {
    let pressed = event
        .target()
        .and_then(|target| target.dyn_into::<Element>().ok());
    if let Some(element) = pressed {
        // A pointer the browser no longer tracks cannot be captured; the
        // drag then follows it only while it is over the element.
        let _ = element.set_pointer_capture(event.pointer_id());
    }
}

/// Visits `link` in a new tab, or in this one if new tabs are blocked.
pub fn visit(link: Link) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let opened = window
        .open_with_url_and_target(link.url(), "_blank")
        .ok()
        .flatten();
    if opened.is_none() {
        // Leaving the page is the last resort; if even that fails, the page
        // stays as it was, which is all there is to keep.
        let _ = window.location().set_href(link.url());
    }
}

/// Scrolls the page until `window` is in sight, for the stacked layout.
pub fn reveal(window: WindowId) {
    let element = web_sys::window()
        .and_then(|page| page.document())
        .and_then(|document| document.get_element_by_id(window.element_id()));
    if let Some(element) = element {
        let options = ScrollIntoViewOptions::new();
        options.set_behavior(ScrollBehavior::Smooth);
        element.scroll_into_view_with_scroll_into_view_options(&options);
    }
}
