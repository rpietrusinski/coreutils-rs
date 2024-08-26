use coreutils_rs::config::ConfigCp;
use coreutils_rs::runner::run_mv;
use std::env;
use std::process;

fn main() {
    let config: ConfigCp = ConfigCp::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run_mv(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
