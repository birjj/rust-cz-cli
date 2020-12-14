use quick_error::quick_error;
use serde_json::Value;
use std::{
    fs::{canonicalize, File},
    path::PathBuf,
};

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
fn load_from_packagejson(file: &File) -> std::result::Result<Value, Error> {
    let data: Value = serde_json::from_reader(file)?;

    Ok(data["config"]["commitizen"].clone())
}

/// Load our config from a JSON file (e.g. .czrc)
fn load_from_json(file: &File) -> std::result::Result<Value, Error> {
    let data: Value = serde_json::from_reader(file)?;

    Ok(data.clone())
}

/// Load the closest config, walking up from the current path
/// Returns Ok(Value::Null) if no config is found
pub fn load(from: Option<&PathBuf>) -> std::result::Result<Value, Error> {
    let mut path: PathBuf;
    if from == None {
        path = std::env::current_dir()?;
    } else {
        path = from.unwrap().clone();
    }

    // for relative paths, popping the last entry might not mean that parent == None
    if path.is_relative() {
        path = canonicalize(path)?;
    }

    type Loader = fn(&File) -> Result<Value, Error>;
    let files = [
        (".czrc", load_from_json as Loader),
        (".cz.json", load_from_json as Loader),
        ("package.json", load_from_packagejson as Loader),
    ];

    // check every parent directory for a configuration file
    loop {
        println!("Checking out {:?}", path);
        for (f, loader) in files.iter() {
            path.push(f);
            let open = File::open(&path);
            match open {
                Ok(file) => return loader(&file),
                Err(_) => (),
            }
            path.pop();
        }

        // stop if we've reached root
        if !path.pop() {
            break;
        }
    }

    Ok(Value::Null)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() -> std::result::Result<(), Error> {
        let path = PathBuf::from("/");
        let config = load(Some(&path))?;
        assert_eq!(config, Value::Null);

        let path = PathBuf::from("./test/");
        let config = load(Some(&path))?;
        assert_eq!(config["path"], "cz-conventional-changelog");

        Ok(())
    }

    #[test]
    fn test_load_from_packagejson() -> std::result::Result<(), Error> {
        let file = File::open("./test/package.json")?;
        let config = load_from_packagejson(&file)?;
        assert_eq!(config["path"], "./node_modules/cz-conventional-changelog",);

        let file = File::open("./test/package-empty.json")?;
        let config = load_from_packagejson(&file)?;
        assert_eq!(config["path"], Value::Null);

        Ok(())
    }

    #[test]
    fn test_load_from_json() -> std::result::Result<(), Error> {
        let file = File::open("./test/.czrc")?;
        let config = load_from_json(&file)?;
        assert_eq!(config["path"], "cz-conventional-changelog",);

        let file = File::open("./test/.czrc-empty")?;
        let config = load_from_json(&file)?;
        assert_eq!(config["path"], Value::Null);

        Ok(())
    }
}
