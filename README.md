# LispMachineLab.github.io

LispM's website, at https://lispmachinelab.github.io/: a Mac OS 9 desktop in the browser, drawn in
Platinum as [LispM Desktop](https://github.com/LispMachineLab/lispm-desktop) draws Emacs, its
windows introducing the three LispM projects. It is written in Rust with
[Yew](https://yew.rs) and built with [Trunk](https://trunk-rs.github.io/trunk/).

On a wide screen the windows float over the desk: double-click an icon to open it, drag a window
by its title bar (a dotted outline follows the pointer and the window moves once, when let go, as
Mac OS 9's did), and use its close, zoom and collapse boxes. On a narrow screen the windows stand
one under another down the page, and a tap on an icon scrolls to its window.

## Layout

- `src/model/` is pure and tested natively: the geometry, the windows, the desktop's state machine
  (`Desktop::apply`, every transition `Desktop -> Event -> Desktop`), the menus and icons, the
  clock, and the title bar boxes as pixel maps ported from LispM Desktop's
  `lispm-desktop-platinum-draw-box`.
- `src/effects.rs` is the only code that touches the browser: the viewport, the time, pointer
  capture, visiting a page, scrolling to a window.
- `src/view/` draws the state with Yew and turns what the visitor does into events.
- `style/platinum.css` holds Platinum's greys, bevels and sizes, from LispM Desktop's platinum
  theme.
- `assets/` holds the icons (LispM Desktop's), the rainbow λ and the demo recording (LispM
  Agent's).

## Working on it

Needs Rust with the `wasm32-unknown-unknown` target, and Trunk:

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
trunk serve            # http://127.0.0.1:8080, rebuilt on every change
cargo test             # the model's tests
```

Every push to `main` is built, checked (`cargo fmt --check`, `cargo clippy` denying `unwrap`,
`expect`, `panic` and unchecked indexing, `cargo test`) and deployed to GitHub Pages by
`.github/workflows/pages.yml`; pull requests are built and checked only.

## License

GPL-3.0-or-later, as the LispM projects are. See [LICENSE](LICENSE).
