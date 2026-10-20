//! LispM's website: what LispM is, told with its films. A headline and the
//! film, the idea, then LispM Agent, LispM Desktop and LispM Browser, each
//! with clips from the films, and how to install them.
//!
//! [`content`] holds the page's words and media as data, [`copy`] the copy
//! buttons' states, both pure and tested natively. [`effects`] is the
//! boundary with the browser. [`view`] draws the page with Yew.

#![deny(missing_docs)]

mod content;
mod copy;
mod effects;
mod view;

fn main() {
    yew::Renderer::<view::page::App>::new().render();
}
