# LispMachineLab.github.io

LispM's website, at https://lispmachinelab.github.io/: what LispM is, told with its films. A
headline and the 57-second film, the idea, then LispM Agent, LispM Desktop and LispM Browser, each
with clips from the films, and how to install them. It is written in Rust with
[Yew](https://yew.rs) and built with [Trunk](https://trunk-rs.github.io/trunk/).

## Layout

- `src/content.rs` holds the page's words and media as data: the sections, the clips, each
  project's story, points and install snippet. Its tests check, among other things, that every
  clip and poster it names is in `assets/`.
- `src/copy.rs` is the copy buttons' states, pure and tested.
- `src/effects.rs` is the only code that touches the browser beyond Yew's rendering: the
  clipboard, the motion the visitor prefers, playing a clip while it is in sight.
- `src/view/` draws the page: `page.rs` its sections, `media.rs` the film and the clips,
  `code.rs` the snippets.
- `style/site.css` is the films' keynote look.
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
cargo test             # the content's and the copy buttons' tests
```

Every push to `main` is built, checked (`cargo fmt --check`, `cargo clippy` denying `unwrap`,
`expect`, `panic` and unchecked indexing, `cargo test`) and deployed to GitHub Pages by
`.github/workflows/pages.yml`; pull requests are built and checked only.

## License

GPL-3.0-or-later, as the LispM projects are. See [LICENSE](LICENSE). The photographs in the film
are from Wikimedia Commons; their authors and licenses are credited at its end.
