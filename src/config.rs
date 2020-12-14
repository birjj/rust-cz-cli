use quick_error::quick_error;
use serde_json::Value;
use std::fs::File;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: std::io::Error) {
            from()
            display("I/O error: {}", err)
        }
        Serde(err: serde_json::Error) {
            from()
        }
        ConfigError(msg: &'static str) {
            display("{}", msg)
        }
    }
}

/// Load our config from a package.json-like file
pub fn load_from_packagejson(path: &str) -> std::result::Result<Value, Error> {
    let file = File::open(path)?;
    let data: Value = serde_json::from_reader(file)?;

    Ok(data["config"]["commitizen"].clone())
}

/// Load our config from a JSON file (e.g. .czrc)
pub fn load_from_json(path: &str) -> std::result::Result<Value, Error> {
    let file = File::open(path)?;
    let data: Value = serde_json::from_reader(file)?;

    Ok(data.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_from_packagejson() -> std::result::Result<(), Error> {
        let config = load_from_packagejson("./test/package.json")?;
        assert_eq!(config["path"], "./node_modules/cz-conventional-changelog",);

        let config = load_from_packagejson("./test/package-empty.json")?;
        assert_eq!(config["path"], Value::Null);

        Ok(())
    }

    #[test]
    fn test_load_from_json() -> std::result::Result<(), Error> {
        let config = load_from_json("./test/.czrc")?;
        assert_eq!(config["path"], "cz-conventional-changelog",);

        let config = load_from_json("./test/.czrc-empty")?;
        assert_eq!(config["path"], Value::Null);

        Ok(())
    }
}
