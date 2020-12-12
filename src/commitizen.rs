use std::{fs, write};

use directories::ProjectDirs;
use quick_error::quick_error;

use super::adapters;
use super::git;

quick_error! {
    /// Commitizen error
    #[derive(Debug)]
    pub enum Error {
        Error(msg: &'static str) {
            display("{}", msg)
        }
        Io(err: std::io::Error) {
            from()
            display("I/O error: {}", err)
        }
        Adapter(err: adapters::Error) {
            from()
            display("Adapter error: {}", err)
        }
        Git(err: git::Error) {
            from()
            display("Git error: {}", err)
        }
    }
}

/// Start the commit process, prompting the user for input
pub fn commit() -> Result<(), Error> {
    // make sure our cache directory exists
    let proj_dirs = ProjectDirs::from("org", "commitizen", "cli")
        .ok_or_else(|| Error::Error("Couldn't locate cache directory"))?;
    fs::create_dir_all(proj_dirs.cache_dir())?;

    // TODO: implement retryLastCommit

    let msg = adapters::conventional_changelog::prompt()?;

    git::commit(msg)?;

    Ok(())
}
