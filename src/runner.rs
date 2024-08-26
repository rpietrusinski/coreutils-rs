use crate::config;
use crate::search::string_search_funcs::{search_case_sensitive, search_case_insensitive};
use std::error::Error;
use std::fs;

pub fn run_grep(config: config::ConfigGrep) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let search_results: Vec<&str> = match config.ignore_case {
        true => search_case_insensitive(&config.query, &contents),
        false => search_case_sensitive(&config.query, &contents),
    };

    let _: Vec<_> = search_results
        .into_iter()
        .map(|x| println!("{}", x))
        .collect();

    Ok(())
}

pub fn run_cp(config: config::ConfigCp) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.source_file)?;
    fs::write(config.dest_file, contents)?;
    Ok(())
}

pub fn run_mv(config: config::ConfigCp) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.source_file)?;
    fs::write(config.dest_file, contents)?;
    fs::remove_file(config.source_file)?;
    Ok(())
}

pub fn run_ls(config: config::ConfigLs) -> Result<(), Box<dyn Error>> {

    if let Ok(entries) = fs::read_dir(config.dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    println!("{:?}: {:?}", entry.path(), metadata.permissions());
                } else {
                    println!("Couldn't get metadata for {:?}", entry.path());
                }
            }
        }
    }
    Ok(())
}
