use coreutils_rs::{
    config::LsCli,
    runner::run_ls,
};
use std::process;
use clap::Parser;

/// GNU's ls functionality. Returns contents of a DIR
///
/// # Examples
///
/// ```
/// ./ls -d DIR
/// ./ls -dir DIR
/// ./ls .
/// ```
fn main() {
    let cli = LsCli::parse();

    if let Err(e) = run_ls(&cli) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
