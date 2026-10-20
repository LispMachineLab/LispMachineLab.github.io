//! The page in the browser. It hydrates the page prerender wrote into
//! index.html's `#app`, so the words are there before WebAssembly loads;
//! without that element it draws the page afresh.

#![deny(missing_docs)]

#[cfg(target_arch = "wasm32")]
fn main() {
    use lispm_site::view::page::App;

    let root = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id(lispm_site::publish::ROOT_ID));
    match root {
        Some(root) => {
            yew::Renderer::<App>::with_root(root).hydrate();
        }
        None => {
            yew::Renderer::<App>::new().render();
        }
    }
}

/// Natively there is no page: the site is built with Trunk, see the README.
#[cfg(not(target_arch = "wasm32"))]
fn main() {}
