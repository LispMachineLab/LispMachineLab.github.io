# LispMachineLab.github.io

LispM's website, at https://lispm.ai/: what LispM is, told with its films. A
headline and the 57-second film, the idea, then chapters on what LispM does: its agents, the
desktop, the App Store, LispM Remote, and why you stay in charge. They are told with clips from
the films and stills, and followed by how to install LispM and the projects it is made of. The
chapters follow what a visitor can do, not how the code is split into repositories, so each
feature is told once. Its blog, at https://lispm.ai/blog/, holds
LispM's Articles from X. It is written in Rust with [Yew](https://yew.rs) and built with
[Trunk](https://trunk-rs.github.io/trunk/).

The page reads before WebAssembly loads, for crawlers and language models as much as for slow
connections: after each build Trunk runs `prerender` (see `Trunk.toml`), which renders the same
Yew components natively into `index.html`'s `<div id="app">`, puts JSON-LD describing the site
and its projects in the head, and writes `llms.txt`, the page as Markdown, and `llms-full.txt`,
the same with every post's words. In the browser the page is hydrated, not drawn again.
`prerender` also draws the blog's pages, the list of posts at `blog/index.html` and each post at
`blog/<slug>/index.html`, as plain pages that load no WebAssembly; the blog's Atom feed at
`blog/feed.xml`; `404.html`, the page GitHub Pages shows at addresses not on the site, kept out
of search engines; and `sitemap.xml`, which lists the pages, and the stills the home page shows.

For search engines and the answer engines built on them, every page has its own title and a
description within the 160 characters they show (tests keep both), and its pictures their width
and height. What LispM is, `content::DEFINITION` (an AI agent harness built on GNU Emacs and the
ideas of the Lisp Machine), is told in the same words wherever they look: the description, the
JSON-LD, llms.txt and the first of the questions at the foot of the page, which are also the
page's FAQPage. The JSON-LD is one graph whose nodes name each other by id: LispM, the organization,
is the same node on every page, with its logo and its accounts on GitHub and X, and the app, the
site, the blog and the posts point to it. So that what LispM looks like is not guessed at, the
app's screenshots are the stills beside the chapters, the clips' posters and the pictures, and
every clip, the film's too, is a VideoObject about the app, named by the headline it illustrates
and dated the day it was first published; llms.txt shows them all under Screenshots, and the
sitemap names them as the home page's images. After each deploy, `scripts/indexnow.sh` tells Bing and
the other engines that take IndexNow that the sitemap's pages are new. Google and Bing learn of
the site through their webmaster tools, where `sitemap.xml` is submitted by hand.

## Layout

The crate is a library and two binaries: `src/main.rs`, the page in the browser, and
`src/bin/prerender.rs`, run natively after each build.

- `src/content.rs` holds the page's words and media as data: the sections, the chapters with
  their stories, clips, stills and points, and the projects with their repositories and install
  snippets, for those that stand on their own in your own Emacs; `illustrations()` walks the
  clips and stills top to bottom, each with the headline it illustrates. Its tests, in
  `src/content/tests.rs`, check, among other things, that every clip and poster it names is in
  `assets/`, and that every still is there at the size the page gives it.
- `src/date.rs` is the days posts and clips were first published.
- `src/copy.rs` is the copy buttons' states, pure and tested.
- `src/publish.rs` is what is written beside the page, pure and tested: the rendered page put
  into `index.html`, the JSON-LD, and `llms.txt`, all from `content.rs`.
- `src/blog/` is the blog, built natively only: `mod.rs` the posts as data, newest first, each
  with its title, date, author, cover, share card and the X post it first was; `markdown.rs` a
  post's Markdown as HTML, a picture alone in its paragraph drawn as a figure, its title the
  caption, its size its width and height, and an `.mp4` as a video on the `.jpg` of the same
  name; `size.rs` a WebP's size, read from its header; `publish.rs` each page's head (Open Graph,
  Twitter and JSON-LD), the stylesheet and icons it shares with the home page, the missing page's
  head, the site's map, llms.txt's list of posts and llms-full.txt's words of them; `feed.rs` the
  Atom feed.
- `src/effects.rs` is the only code that touches the browser beyond Yew's rendering: the
  clipboard, the motion the visitor prefers, playing a clip while it is in sight.
- `src/view/` draws the page: `page.rs` its sections, `media.rs` the film and the clips,
  `code.rs` the snippets, `blog.rs` the blog's list and posts, and the missing page.
- `blog/` holds each post's words, `<slug>.md`, in Markdown; `assets/blog/<slug>/` its cover
  (5:2), its share card (1200 by 630) and its pictures and films. To add a post, put its Markdown
  and assets there and add it at the top of `POSTS` in `src/blog/mod.rs`; the tests check that
  everything it names is there and every card is 1200 by 630.
- `style/site.css` is the films' keynote look.
- `static/` holds what is copied to the site's root: `robots.txt`, `install.sh`, the icons at
  addresses that stay put (`favicon.ico`, 16 to 48 pixels, and `apple-touch-icon.png`, 180), and
  `<key>.txt`, the IndexNow key, its own name.
- `scripts/remote-page.sh` puts [LispM Remote](https://github.com/LispMachineLab/lispm-remote)'s
  page at `/remote/`, where LispM's QR code points: it downloads the page's release, pinned by
  version and SHA-256, and unpacks it into `dist/remote` after Trunk's build, with a robots tag
  that keeps the app out of search engines. To publish a new page, pin the new release there.
- `scripts/indexnow.sh` sends the deployed sitemap's pages to IndexNow; `--print` shows the
  request and sends nothing.
- `assets/social/lispm-card.png` is the card shared links show, 1200 by 630, its words clear of
  X's 2:1 crop; `index.html` names it in its Open Graph and Twitter tags, and
  `src/share_card.rs` tests that the tags agree and the file is that size.
- `assets/video/` holds the film and the clips cut from it, from the longer LispM Desktop film,
  from the LispM Remote film and from the Classic AI film, each with its poster; `content.rs`
  dates each clip the day it was first published, for its VideoObject. The model line
  some frames of the agent's chat showed is covered in them, so no model server is named. The
  stills beside some steps are the blog's own pictures, under `assets/blog/`.

Clips play muted and looped while a quarter of them is in sight, and only then load; with less
motion asked for, they wait on their posters with their controls. The film, with sound, starts
when asked.

## Working on it

Needs Rust with the `wasm32-unknown-unknown` target, and Trunk:

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
trunk serve            # http://127.0.0.1:8080, rebuilt on every change
cargo test             # the content's, copy buttons', publishing's, blog's and share cards' tests
```

Every push to `main` is built, checked (`cargo fmt --check`, `cargo clippy` denying `unwrap`,
`expect`, `panic` and unchecked indexing, `cargo test`) and deployed to GitHub Pages by
`.github/workflows/pages.yml`; pull requests are built and checked only. The domain lispm.ai is
set in the repository's Pages settings (an Actions deployment ignores a `CNAME` file); its DNS,
at Cloudflare, points to GitHub Pages unproxied, so GitHub can issue the HTTPS certificate.

## License

AGPL-3.0-or-later, as the LispM projects are. See [LICENSE](LICENSE). The photographs in the film
are from Wikimedia Commons; their authors and licenses are credited at its end.
