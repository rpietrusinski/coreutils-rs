use coreutils_rs::config::ConfigCp;
use coreutils_rs::runner::run_cp;
use std::env;
use std::process;

/// GNU's cp functionality. Copies SOURCE_FILE into DEST_FILE
///
/// # Examples
///
/// ```
/// ./cp SOURCE_FILE DEST_FILE
/// ./cp poem.txt poem.md
/// ```
fn main() {
    let config: ConfigCp = ConfigCp::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run_cp(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
