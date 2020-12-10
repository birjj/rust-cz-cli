mod args_filter;
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
    }
}

fn main() -> Result<(), Error> {
    let raw_git_args = std::env::args().skip(1);
    let filtered_args = args_filter::filter(raw_git_args);

    if filtered_args.contains(&"--amend".to_string()) {
        // TODO: implement --amend override as in commitizen/cli/strategies/git-cz.js
    }

    let retry_last_commit = filtered_args.get(0) == Some(&"--retry".to_string());
    let staging_is_clean = git::staging_is_clean()?;

    if staging_is_clean && !filtered_args.contains(&"--allow-empty".to_string()) {
        return Err(Error::AppError(
            "No files added to staging! Did you forget to run git add?",
        ));
    }

    // TODO: implement hook mode

    Ok(())
}
