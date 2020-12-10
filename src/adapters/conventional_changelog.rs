use super::Error;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use lazy_static::lazy_static;

lazy_static! {
    static ref ITEMS: Vec<CommitType> = vec![
        CommitType{
            tag: "feat",
            description: "A new feature"
        },
        CommitType {
            tag: "fix",
            description: "A bug fix"
        },
        CommitType {
            tag: "docs",
            description: "Documentation only changes"
        },
        CommitType {
            tag: "style",
            description: "Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)"
        },
        CommitType {
            tag: "refactor",
            description: "A code change that neither fixes a bug nor adds a feature"
        },
        CommitType {
            tag: "perf",
            description: "A code change that improves performance"
        },
        CommitType {
            tag: "test",
            description: "Adding missing tests or correcting existing tests"
        },
        CommitType {
            tag: "build",
            description: "Changes that affect the build system or external dependencies (example scopes: gulp, broccoli, npm)"
        },
        CommitType {
            tag: "ci",
            description: "Changes to our CI configuration files and scripts (example scopes: Travis, Circle, BrowserStack, SauceLabs)"
        },
        CommitType {
            tag: "chore",
            description: "Other changes that don't modify src or test files"
        },
        CommitType {
            tag: "revert",
            description: "Reverts a previous commit"
        }
    ];
}

struct PromptOutput {
    tag: String,
    scope: String,
    subject: String,
    body: String,
    breaking: bool,
    breaking_body: String,
    issues: String,
}

fn header_len(data: &PromptOutput) -> usize {
    let mut outp = data.tag.len() + 2;
    if !data.scope.trim().is_empty() {
        outp += data.scope.trim().len() + 2;
    }
    return outp;
}

fn max_summary_len(data: &PromptOutput) -> usize {
    100 - header_len(data)
}

fn filter_subject(subject: &String) -> String {
    // TODO: implement disableSubjectLowerCase once options are supported
    let mut chars = subject.chars();
    let mut outp = match chars.next() {
        None => String::new(),
        Some(c) => c.to_lowercase().collect::<String>() + chars.as_str(),
    };

    while outp.ends_with('.') {
        outp.pop();
    }

    return outp;
}

fn create_message(data: &PromptOutput) -> String {
    let mut message = String::with_capacity(512);

    // head
    message.push_str(&data.tag);
    if !data.scope.is_empty() {
        message.push_str(&["(", &data.scope, ")"].concat());
    }
    message.push_str(": ");
    message.push_str(&filter_subject(&data.subject));

    // body
    if !data.body.is_empty() {
        message.push_str("\n\n");
        message.push_str(&textwrap::fill(&data.body, 100));
    }

    // breaking
    if data.breaking {
        message.push_str("\n\n");
        message.push_str("BREAKING CHANGE: ");
        message.push_str(&data.breaking_body.replace("BREAKING CHANGE: ", ""));
    }

    // issues
    if !data.issues.is_empty() {
        message.push_str("\n\n");
        message.push_str(&data.issues);
    }

    return message;
}

/// Implementation of the cz-conventional-changelog adapter in Rust
pub fn prompt() -> Result<String, Error> {
    let mut data = PromptOutput {
        tag: String::new(),
        scope: String::new(),
        subject: String::new(),
        body: String::new(),
        breaking: false,
        breaking_body: String::new(),
        issues: String::new(),
    };

    // tag
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the type of change that you're committing:")
        .items(&ITEMS)
        .default(0)
        .interact_opt()?;
    if selection.is_none() {
        return Err(Error::UserError("User quit, aborting."));
    }
    data.tag = ITEMS[selection.unwrap()].tag.to_string();

    // scope
    data.scope = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "What is the scope of this change (e.g. component or file name): (press enter to skip)",
        )
        .allow_empty(true)
        .interact_text()?
        .trim()
        .to_string();

    // subject
    data.subject = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Write a short, imperative tense description of the change (max {} chars):",
            max_summary_len(&data)
        ))
        .interact_text()?
        .trim()
        .to_string();
    while data.subject.is_empty() || data.subject.len() > max_summary_len(&data) {
        data.subject = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Subject is required, and must be less than {} characters. Current length is {}:",
                max_summary_len(&data),
                data.subject.trim().len()
            ))
            .interact_text()?
            .trim()
            .to_string();
    }

    // body
    data.body = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Provide a longer description of the change: (press enter to skip)")
        .allow_empty(true)
        .interact_text()?
        .trim()
        .to_string();

    // breaking
    data.breaking = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Are there any breaking changes?")
        .default(false)
        .interact()?;
    while data.body.is_empty() && data.breaking {
        data.body = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("A BREAKING CHANGE commit requires a body. Please enter a longer description of the commit itself:")
            .interact_text()?
            .trim().to_string();
    }
    if data.breaking {
        data.breaking_body = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Describe the breaking changes:")
            .interact_text()?
            .trim()
            .to_string();
    }

    // issues
    let is_issue_affected = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Does this change affect any open issues?")
        .default(false)
        .interact()?;
    if is_issue_affected && data.body.is_empty() && data.breaking_body.is_empty() {
        data.body = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("If issues are closed, the commit requires a body. Please enter a longer description of the commit itself:")
            .interact_text()?
            .trim().to_string();
    }
    if is_issue_affected {
        data.issues = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Add issue references (e.g. \"fix #123\", \"re #123\".):")
            .interact_text()?
            .trim()
            .to_string();
    }

    return Ok(create_message(&data));
}

struct CommitType {
    tag: &'static str,
    description: &'static str,
}
impl std::fmt::Display for CommitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:9} {}", format!("{}:", self.tag), self.description)
    }
}
