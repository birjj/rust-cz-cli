mod adapters;
mod args_filter;
mod commitizen;
mod config;
mod git;

use quick_error::quick_error;

quick_error! {
    /// Application Error
    #[derive(Debug)]
    pub enum Error {
        AppError(msg: &'static str) {
            display("Error: {}", msg)
        }
        GitError(err: git::Error) {
            from()
            display("Git error: {}", err)
        }
        CommitizenError(err: commitizen::Error) {
            from()
            display("Commitizen error: {}", err)
        }
    }
}

fn main() -> Result<(), Error> {
    let raw_git_args: Vec<String> = std::env::args().skip(1).collect();
    // if we're amending, just run git as normal
    if raw_git_args.contains(&"--amend".to_string()) {
        git::execute::<Vec<String>, String>(raw_git_args, true)?;
        return Ok(());
    }

    // otherwise remove any potential message and generate our own
    let filtered_args = args_filter::filter(raw_git_args.iter());

    let retry_last_commit = filtered_args.get(0) == Some(&"--retry".to_string());
    let staging_is_clean = git::staging_is_clean()?;

    if staging_is_clean && !filtered_args.contains(&"--allow-empty".to_string()) {
        return Err(Error::AppError(
            "No files added to staging! Did you forget to run git add?",
        ));
    }

    // TODO: implement hook mode

    commitizen::commit()?;

    Ok(())
}
