use crate::config;
use crate::search::string_search_funcs::{search_case_insensitive, search_case_sensitive};
use std::error::Error;
use std::fs;

pub fn run_grep(config: &config::GrepCli) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file)?;

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

pub fn run_cp(config: &config::CpCli) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.source)?;
    fs::write(&config.dest, contents)?;
    Ok(())
}

pub fn run_mv(config: &config::CpCli) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.source)?;
    fs::write(&config.dest, contents)?;
    fs::remove_file(&config.source)?;
    Ok(())
}

pub fn run_ls(config: &config::LsCli) -> Result<(), Box<dyn Error>> {
    if let Ok(entries) = fs::read_dir(&config.dir) {
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

pub fn run_head(config: &config::HeadCli) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file)?;

    for (item, idx) in contents.lines().zip(0u32..) {
        if &idx < &config.num_lines {
            println!("{}", item);
        } else {
            break;
        }
    }
    Ok(())
}
