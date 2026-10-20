//! LispM's website: a Mac OS 9 desktop in the browser, drawn in Platinum as
//! LispM Desktop draws Emacs, its windows introducing the three projects.
//!
//! The crate is three layers. [`model`] is pure: the desktop's state and its
//! transitions, tested natively. [`effects`] is the boundary with the browser.
//! [`view`] draws the state with Yew and turns what the visitor does into events.

#![deny(missing_docs)]

mod effects;
mod model;
mod view;

fn main() {
    yew::Renderer::<view::app::App>::new().render();
}
