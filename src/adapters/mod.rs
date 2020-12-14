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

pub trait Adapter {
    fn prompt(&self) -> Result<String, Error>;
}
impl Adapter for fn() -> Result<String, Error> {
    fn prompt(&self) -> Result<String, Error> {
        self()
    }
}

pub fn get_by_name(name: &String) -> Option<Box<dyn Adapter>> {
    return Some(Box::new(
        conventional_changelog::prompt as fn() -> Result<String, Error>,
    ));
}
