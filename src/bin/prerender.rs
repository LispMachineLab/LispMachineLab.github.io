//! Renders the page into the index.html Trunk staged, so it reads before
//! WebAssembly loads, and writes llms.txt beside it; writes the blog's
//! pages, the list at blog/index.html and each post at
//! blog/<slug>/index.html; and writes the site's map, sitemap.xml. Trunk
//! runs it after every build (see Trunk.toml), naming the directory in
//! `TRUNK_STAGING_DIR`; it can also be given the directory as its argument.

#![deny(missing_docs)]

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::fmt;
    use std::path::{Path, PathBuf};
    use std::process::ExitCode;

    use lispm_site::blog::POSTS;
    use lispm_site::blog::publish::{self as blog, BlogError};
    use lispm_site::publish::{self, PublishError};
    use lispm_site::view::blog::{Blog, PostPage, PostProps};
    use lispm_site::view::page::App;
    use yew::{BaseComponent, LocalServerRenderer};

    /// Why the page was not prerendered.
    #[derive(Debug)]
    pub enum Failure {
        /// Neither `TRUNK_STAGING_DIR` nor an argument named the directory.
        NoDirectory,
        /// A file could not be read.
        Read(PathBuf, std::io::Error),
        /// A file could not be written.
        Write(PathBuf, std::io::Error),
        /// A directory could not be made.
        Directory(PathBuf, std::io::Error),
        /// No runtime could be made to render in.
        Runtime(std::io::Error),
        /// index.html had no place for the page.
        Publish(PublishError),
        /// The blog's pages could not be made.
        Blog(BlogError),
    }

    impl fmt::Display for Failure {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Failure::NoDirectory => write!(
                    f,
                    "name the staging directory: TRUNK_STAGING_DIR or an argument"
                ),
                Failure::Read(path, error) => write!(f, "reading {}: {error}", path.display()),
                Failure::Write(path, error) => write!(f, "writing {}: {error}", path.display()),
                Failure::Directory(path, error) => {
                    write!(f, "making {}: {error}", path.display())
                }
                Failure::Runtime(error) => write!(f, "starting the renderer: {error}"),
                Failure::Publish(error) => write!(f, "{error}"),
                Failure::Blog(error) => write!(f, "{error}"),
            }
        }
    }

    /// Returns the directory Trunk staged the site in.
    fn staging_directory() -> Result<PathBuf, Failure> {
        std::env::var_os("TRUNK_STAGING_DIR")
            .or_else(|| std::env::args_os().nth(1))
            .map(PathBuf::from)
            .ok_or(Failure::NoDirectory)
    }

    /// Returns what `renderer` renders, as Yew renders it on a server.
    fn render<C: BaseComponent>(renderer: LocalServerRenderer<C>) -> Result<String, Failure> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .map_err(Failure::Runtime)?;
        Ok(tokio::task::LocalSet::new().block_on(&runtime, renderer.render()))
    }

    fn write(path: &Path, text: &str) -> Result<(), Failure> {
        std::fs::write(path, text).map_err(|error| Failure::Write(path.to_owned(), error))
    }

    /// Writes `page`, with `head` and the shared `links`, as `directory`'s index.html.
    fn write_page(
        directory: &Path,
        head: &blog::Head,
        links: &str,
        page: &str,
    ) -> Result<(), Failure> {
        std::fs::create_dir_all(directory)
            .map_err(|error| Failure::Directory(directory.to_owned(), error))?;
        write(
            &directory.join("index.html"),
            &blog::document(head, links, page),
        )
    }

    /// Writes the blog's pages under `directory`, sharing `links`.
    fn write_blog(directory: &Path, links: &str) -> Result<(), Failure> {
        let directory = directory.join("blog");
        let list = render(LocalServerRenderer::<Blog>::new().hydratable(false))?;
        write_page(&directory, &blog::blog_head(), links, &list)?;
        for post in &POSTS {
            let page = render(
                LocalServerRenderer::<PostPage>::with_props(PostProps { post }).hydratable(false),
            )?;
            write_page(
                &directory.join(post.slug),
                &blog::post_head(post),
                links,
                &page,
            )?;
        }
        Ok(())
    }

    fn prerender() -> Result<(), Failure> {
        let directory = staging_directory()?;
        let index_path = directory.join("index.html");
        let index = std::fs::read_to_string(&index_path)
            .map_err(|error| Failure::Read(index_path.clone(), error))?;
        let links = blog::shared_links(&index).map_err(Failure::Blog)?;
        let page = render(LocalServerRenderer::<App>::new().hydratable(true))?;
        let html =
            publish::insert_page(&index, &page, &publish::head()).map_err(Failure::Publish)?;
        write(&index_path, &html)?;
        write(
            &directory.join("llms.txt"),
            &format!("{}\n{}", publish::llms_txt(), blog::llms_section()),
        )?;
        write(&directory.join("sitemap.xml"), &blog::sitemap())?;
        write_blog(&directory, &links)
    }

    /// Prerenders the page, telling Trunk whether it could.
    pub fn main() -> ExitCode {
        match prerender() {
            Ok(()) => ExitCode::SUCCESS,
            Err(failure) => {
                eprintln!("prerender: {failure}");
                ExitCode::FAILURE
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> std::process::ExitCode {
    native::main()
}

/// In WebAssembly there is nothing to prerender.
#[cfg(target_arch = "wasm32")]
fn main() {}
