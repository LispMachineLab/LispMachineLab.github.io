//! Renders the page into the index.html Trunk staged, so it reads before
//! WebAssembly loads, and writes llms.txt beside it. Trunk runs it after
//! every build (see Trunk.toml), naming the directory in
//! `TRUNK_STAGING_DIR`; it can also be given the directory as its argument.

#![deny(missing_docs)]

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::fmt;
    use std::path::{Path, PathBuf};
    use std::process::ExitCode;

    use lispm_site::publish::{self, PublishError};
    use lispm_site::view::page::App;

    /// Why the page was not prerendered.
    #[derive(Debug)]
    pub enum Failure {
        /// Neither `TRUNK_STAGING_DIR` nor an argument named the directory.
        NoDirectory,
        /// A file could not be read.
        Read(PathBuf, std::io::Error),
        /// A file could not be written.
        Write(PathBuf, std::io::Error),
        /// No runtime could be made to render in.
        Runtime(std::io::Error),
        /// index.html had no place for the page.
        Publish(PublishError),
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
                Failure::Runtime(error) => write!(f, "starting the renderer: {error}"),
                Failure::Publish(error) => write!(f, "{error}"),
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

    /// Returns the page as Yew renders it on a server, ready to be hydrated.
    fn render() -> Result<String, Failure> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .map_err(Failure::Runtime)?;
        let renderer = yew::LocalServerRenderer::<App>::new().hydratable(true);
        Ok(tokio::task::LocalSet::new().block_on(&runtime, renderer.render()))
    }

    fn write(path: &Path, text: &str) -> Result<(), Failure> {
        std::fs::write(path, text).map_err(|error| Failure::Write(path.to_owned(), error))
    }

    fn prerender() -> Result<(), Failure> {
        let directory = staging_directory()?;
        let index_path = directory.join("index.html");
        let index = std::fs::read_to_string(&index_path)
            .map_err(|error| Failure::Read(index_path.clone(), error))?;
        let html =
            publish::insert_page(&index, &render()?, &publish::head()).map_err(Failure::Publish)?;
        write(&index_path, &html)?;
        write(&directory.join("llms.txt"), &publish::llms_txt())
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
