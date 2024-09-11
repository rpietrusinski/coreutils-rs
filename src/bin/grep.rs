use coreutils_rs::{
    config::GrepCli,
    runner::run_grep,
};
use std::process;
use clap::Parser;

/// GNU's grep functionality. Returns lines from a FILE that match particular QUERY
///
/// # Examples
///
/// ```
/// ./grep -q QUERY -f FILE <-i>
/// ./grep -query QUERY -file FILE <-ignore_case>
/// ./grep -q somebody -f poem.txt
/// ./grep -query somebody -file poem.txt
/// ```
fn main() {
    let cli = GrepCli::parse();

    if let Err(e) = run_grep(&cli) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
