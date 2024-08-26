use coreutils_rs::config::ConfigLs;
use coreutils_rs::runner::run_ls;
use std::env;
use std::process;

fn main() {
    let config: ConfigLs = ConfigLs::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run_ls(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
