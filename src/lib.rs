//! LispM's website: what LispM is, told with its films. A headline and the
//! film, the idea, then LispM Agent, LispM Desktop, LispM Browser, the App
//! Store and LispM Remote, each with clips from the films or stills, and how
//! to install them.
//!
//! [`content`] holds the page's words and media as data, [`copy`] the copy
//! buttons' states, [`playback`] the looping clips', [`publish`] what is
//! written beside the page for crawlers and language models; all four are
//! pure and tested natively. [`effects`]
//! is the boundary with the browser. [`view`] draws the page with Yew: in
//! the browser it hydrates what the prerender binary wrote into index.html
//! from the same components at build time.
//!
//! `blog` holds the blog's posts, LispM's Articles from X, and makes their
//! pages; prerender draws them at build time as plain pages, so it is built
//! natively only.

#![deny(missing_docs)]

#[cfg(not(target_arch = "wasm32"))]
pub mod blog;
pub mod content;
pub mod copy;
pub mod effects;
pub mod playback;
pub mod publish;
#[cfg(test)]
mod share_card;
pub mod view;
