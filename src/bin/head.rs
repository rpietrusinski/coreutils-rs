use coreutils_rs::config::ConfigHead;
use coreutils_rs::runner::run_head;
use std::env;
use std::process;

/// GNU's head functionality. Returns top NUM_LINES from a FILE
///
/// # Examples
///
/// ```
/// ./head FILE NUM_LINES
/// ./head poem.txt 4
/// ```
fn main() {
    let config: ConfigHead = ConfigHead::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run_head(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
