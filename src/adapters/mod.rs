pub mod conventional_changelog;

use quick_error::quick_error;

quick_error! {
    /// Commitizen error
    #[derive(Debug)]
    pub enum Error {
        UserError(msg: &'static str) {
            display("{}", msg)
        }
        CommitizenError(msg: &'static str) {
            display("Error: {}", msg)
        }
        Io(err: std::io::Error) {
            from()
            display("I/O error: {}", err)
        }
    }
}
