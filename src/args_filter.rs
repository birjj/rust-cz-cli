use lazy_static::lazy_static;
use regex::Regex;

/// Rust implementation of cz-cli/src/cli/parser/git-cz.js
/// Parses the given arguments into a Vec of args we've seen, while removing any message declaration
pub fn filter<A>(args: A) -> Vec<String>
where
    A: Iterator<Item = String>,
{
    lazy_static! {
        static ref RE_SHORT_MESSAGE: Regex = Regex::new(r"^-([a-zA-Z]*)m(.*)$").unwrap();
        static ref RE_LONG_MESSAGE: Regex = Regex::new(r"^--message(=.*)?$").unwrap();
    }

    let mut outp = Vec::new();
    let mut skip_next = false;

    for arg in args {
        if skip_next {
            skip_next = false;
            continue;
        }

        match RE_SHORT_MESSAGE.captures(&arg) {
            Some(our_match) => {
                let preceding_opts = our_match.get(1).unwrap().as_str();
                let following = our_match.get(2).unwrap().as_str();
                if preceding_opts != "" {
                    outp.push(format!("-{}", preceding_opts));
                }
                // if the last option was -m, next arg will be a message which we skip
                if following == "" {
                    skip_next = true;
                }
                continue;
            }
            None => {}
        }

        match RE_LONG_MESSAGE.captures(&arg) {
            Some(our_match) => {
                // if the message wasn't specified as "=message", next arg will be a message which we skip
                if our_match.get(1).unwrap().as_str() == "" {
                    skip_next = true;
                }
                continue;
            }
            None => {}
        }

        outp.push(arg);
    }

    outp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_filter() {
        let args = filter(
            vec![
                "--all",
                "-am",
                "stripped message",
                "-c",
                "123",
                "--fixup=321",
                "--message=test",
                "test",
            ]
            .iter()
            .map(|x| x.to_string()),
        );

        assert_eq!(
            args,
            vec!["--all", "-a", "-c", "123", "--fixup=321", "test"]
        );
    }
}
