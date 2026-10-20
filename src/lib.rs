//! LispM's website: what LispM is, told with its films. A headline and the
//! film, the idea, then LispM Agent, LispM Desktop and LispM Browser, each
//! with clips from the films, and how to install them.
//!
//! [`content`] holds the page's words and media as data, [`copy`] the copy
//! buttons' states, [`playback`] the looping clips', [`publish`] what is
//! written beside the page for crawlers and language models; all four are
//! pure and tested natively. [`effects`]
//! is the boundary with the browser. [`view`] draws the page with Yew: in
//! the browser it hydrates what the prerender binary wrote into index.html
//! from the same components at build time.

#![deny(missing_docs)]

pub mod content;
pub mod copy;
pub mod effects;
pub mod playback;
pub mod publish;
#[cfg(test)]
mod share_card;
pub mod view;
