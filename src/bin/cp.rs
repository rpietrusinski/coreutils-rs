use coreutils_rs::{
    config::CpCli,
    runner::run_cp,
};
use std::process;
use clap::Parser;

/// GNU's cp functionality. Copies SOURCE_FILE into DEST_FILE
///
/// # Examples
///
/// ```
/// ./cp -s SOURCE -d DEST
/// ./cp -source SOURCE -dest DEST
/// ./cp -s poem.txt -d poem.md
/// ```
fn main() {
    let cli = CpCli::parse();

    if let Err(e) = run_cp(&cli) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

