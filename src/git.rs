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
        GitError(exit_code: i32, stderr: Vec<u8>) {
            display("Git exited with error {}: {:?}", exit_code, stderr)
        }
    }
}

fn execute<I, S>(args: I) -> Result<Output, Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let outp = Command::new("git").args(args).output()?;
    if !outp.status.success() {
        return Err(Error::GitError(outp.status.code().unwrap(), outp.stderr));
    }
    Ok(outp)
}

/// Check if the staging area is clean
pub fn staging_is_clean() -> Result<bool, Error> {
    let non_cached = execute(&["diff", "--no-ext-diff", "--name-only"])?;
    let cached = execute(&["diff", "--no-ext-diff", "--name-only"])?;
    Ok(non_cached.stdout.len() == 0 && cached.stdout.len() == 0)
}
