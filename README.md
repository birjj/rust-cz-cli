# rust-cz-cli

An implementation of [cz-cli](https://github.com/commitizen/cz-cli) in Rust. Aims to be a drop-in replacement for git-cz with significantly better performance.

Currently only implements the basic workflow (run, get asked questions, the tool then generates a commit message and commits). The following still needs to be implemented:

-   [x] Prompt user for information
-   [x] Create commit message from information and use it to commit
-   [ ] Check if running in a commitizen-friendly repo
-   [ ] Support configuration of various options (as in cz-cli) through package.json, .czrc or .cz.json files.
-   [ ] Support --retry, --amend, hook mode
-   [ ] Add support for pluggable adapters
