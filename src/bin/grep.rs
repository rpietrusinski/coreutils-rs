use coreutils_rs::config::ConfigGrep;
use coreutils_rs::runner::run_grep;
use std::env;
use std::process;

/// GNU's grep functionality. Returns lines from a FILE that match particular QUERY
///
/// # Examples
///
/// ```
/// ./grep QUERY FILE
/// ./grep somebody poem.txt
/// ```
fn main() {
    let config: ConfigGrep = ConfigGrep::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run_grep(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
