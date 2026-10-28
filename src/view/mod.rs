//! The pages, drawn with Yew: static sections from [`crate::content`], and
//! the few components that keep state, the film, the clips and the copy
//! buttons; and, at build time only, the blog's pages and the pages that
//! stand alone.

#[cfg(not(target_arch = "wasm32"))]
pub mod blog;
mod code;
mod media;
pub mod page;
#[cfg(not(target_arch = "wasm32"))]
pub mod pages;
