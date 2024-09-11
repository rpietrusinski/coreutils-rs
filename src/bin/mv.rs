use coreutils_rs::{
    config::CpCli,
    runner::run_mv,
};
use std::process;
use clap::Parser;

/// GNU's mv functionality. Moves SOURCE_FILE into DEST_FILE
///
/// # Examples
///
/// ```
/// ./mv -s SOURCE_FILE -d DEST_FILE
/// ./mv -source SOURCE_FILE -dest DEST_FILE
/// ./mv -s poem.txt -d poem.md
/// ```
fn main() {
    let cli = CpCli::parse();

    if let Err(e) = run_mv(&cli) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
