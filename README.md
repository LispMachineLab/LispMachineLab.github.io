# LispMachineLab.github.io

LispM's website, at https://lispmachinelab.github.io/: what LispM is, told with its films. A
headline and the 57-second film, the idea, then LispM Agent, LispM Desktop and LispM Browser, each
with clips from the films, and how to install them. It is written in Rust with
[Yew](https://yew.rs) and built with [Trunk](https://trunk-rs.github.io/trunk/).

The page reads before WebAssembly loads, for crawlers and language models as much as for slow
connections: after each build Trunk runs `prerender` (see `Trunk.toml`), which renders the same
Yew components natively into `index.html`'s `<div id="app">`, puts JSON-LD describing the site
and its projects in the head, and writes `llms.txt`, the page as Markdown. In the browser the
page is hydrated, not drawn again.

## Layout

The crate is a library and two binaries: `src/main.rs`, the page in the browser, and
`src/bin/prerender.rs`, run natively after each build.

- `src/content.rs` holds the page's words and media as data: the sections, the clips, each
  project's story, points and install snippet. Its tests check, among other things, that every
  clip and poster it names is in `assets/`.
- `src/copy.rs` is the copy buttons' states, pure and tested.
- `src/publish.rs` is what is written beside the page, pure and tested: the rendered page put
  into `index.html`, the JSON-LD, and `llms.txt`, all from `content.rs`.
- `src/effects.rs` is the only code that touches the browser beyond Yew's rendering: the
  clipboard, the motion the visitor prefers, playing a clip while it is in sight.
- `src/view/` draws the page: `page.rs` its sections, `media.rs` the film and the clips,
  `code.rs` the snippets.
- `style/site.css` is the films' keynote look.
- `static/` holds `robots.txt` and `sitemap.xml`, copied to the site's root.
- `assets/social/lispm-card.png` is the card shared links show, 1200 by 630, its words clear of
  X's 2:1 crop; `index.html` names it in its Open Graph and Twitter tags, and
  `src/share_card.rs` tests that the tags agree and the file is that size.
- `assets/video/` holds the film and the clips cut from it and from the longer LispM Desktop film,
  each with its poster. The model line some frames of the agent's chat showed is covered in
  them, so no model server is named.

Clips play muted and looped while a quarter of them is in sight, and only then load; with less
motion asked for, they wait on their posters with their controls. The film, with sound, starts
when asked.

## Working on it

Needs Rust with the `wasm32-unknown-unknown` target, and Trunk:

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
trunk serve            # http://127.0.0.1:8080, rebuilt on every change
cargo test             # the content's, copy buttons', publishing's and share card's tests
```

Every push to `main` is built, checked (`cargo fmt --check`, `cargo clippy` denying `unwrap`,
`expect`, `panic` and unchecked indexing, `cargo test`) and deployed to GitHub Pages by
`.github/workflows/pages.yml`; pull requests are built and checked only.

## License

GPL-3.0-or-later, as the LispM projects are. See [LICENSE](LICENSE). The photographs in the film
are from Wikimedia Commons; their authors and licenses are credited at its end.
