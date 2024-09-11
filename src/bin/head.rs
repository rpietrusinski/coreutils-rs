use coreutils_rs::{
    config::HeadCli,
    runner::run_head,
};
use std::process;
use clap::Parser;

/// GNU's head functionality. Returns top NUM_LINES from a FILE
///
/// # Examples
///
/// ```
/// ./head -f FILE -n NUM_LINES
/// ./head -file FILE -num_lines NUM_LINES
/// ./head poem.txt 4
/// ```
fn main() {
    let cli = HeadCli::parse();

    if let Err(e) = run_head(&cli) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
