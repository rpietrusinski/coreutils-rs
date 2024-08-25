use std::env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build<T>(mut args: T) -> Result<Self, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next();

        let query: String = match args.next() {
            Some(x) => x,
            None => return Err("Didn't get query parameter"),
        };

        let file_path: String = match args.next() {
            Some(x) => x,
            None => return Err("Didn't get file path parameter"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_build() {
        let args_iterator = [
            "binary_name".to_string(),
            "query".to_string(),
            "file_path".to_string(),
        ]
        .into_iter();

        let config = Config::build(args_iterator).unwrap();
        assert_eq!(config.query, "query".to_string());
        assert_eq!(config.file_path, "file_path".to_string());
        assert_eq!(config.ignore_case, false);
    }
}
