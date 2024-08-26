use coreutils_rs::config::ConfigGrep;
use coreutils_rs::runner::run_grep;
use std::env;
use std::process;

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
