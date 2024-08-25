use crate::config::Config;
use crate::search::search_funcs::{search, search_case_insensitive};
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let search_results: Vec<&str> = match config.ignore_case {
        true => search_case_insensitive(&config.query, &contents),
        false => search(&config.query, &contents),
    };

    let _: Vec<_> = search_results
        .into_iter()
        .map(|x| println!("{}", x))
        .collect();

    Ok(())
}
