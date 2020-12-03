mod args_filter;

fn main() {
    let raw_git_args = std::env::args().skip(1);
    let filtered_args = args_filter::filter(raw_git_args);
    println!("Parsed args: {:?}", filtered_args);

    if filtered_args.contains(&"--amend".to_string()) {
        // TODO: implement --amend override as in commitizen/cli/strategies/git-cz.js
    }
}
