use indoc::indoc;
use quick_error::quick_error;
use std::process::{Command, Output};

quick_error! {
    /// Git Error
    #[derive(Debug)]
    pub enum Error {
        Io(err: std::io::Error) {
            from()
            display("I/O error: {}", err)
        }
        GitError(exit_code: i32, stderr: String) {
            display("Git exited with error {}: {}", exit_code, stderr)
        }
    }
}

/// Execute a git command
/// If strict is true, the Result is an error if git exits with code != 0
pub fn execute<I, S>(args: I, strict: bool) -> Result<Output, Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let outp = Command::new("git").args(args).output()?;
    if strict && !outp.status.success() {
        return Err(Error::GitError(
            outp.status.code().unwrap(),
            String::from_utf8(outp.stdout).unwrap(),
        ));
    }
    Ok(outp)
}

/// Check if the staging area is clean
pub fn staging_is_clean() -> Result<bool, Error> {
    let cached = execute(&["diff", "--no-ext-diff", "--cached", "--name-only"], true)?;
    Ok(cached.stdout.len() == 0)
}

/// Commit the current staging area with a given message
pub fn commit(msg: String) -> Result<(), Error> {
    // TODO: implement hookMode

    let args = ["commit", "-m", &msg];
    let outp = execute(&args, false)?;

    if !outp.status.success() {
        let code = outp.status.code().unwrap();
        if code == 128 {
            println!(indoc! {"
                Git exited with code 128. Did you forget to run:
                
                  git config --global user.email \"you@example.com\"
                  git config --global user.name \"Your Name\""});
        }
        return Err(Error::GitError(
            code,
            String::from_utf8(outp.stdout).unwrap(),
        ));
    }

    Ok(())
}
