use std::env;

#[derive(Debug)]
pub struct ConfigGrep {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

#[derive(Debug)]
pub struct ConfigCp {
    pub source_file: String,
    pub dest_file: String,
}

#[derive(Debug)]
pub struct ConfigLs {
    pub dir: String,
}

#[derive(Debug)]
pub struct ConfigHead {
    pub file: String,
    pub num_lines: u32,
}

impl ConfigGrep {
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

impl ConfigCp {
    pub fn build<T>(mut args: T) -> Result<Self, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next();

        let source_file: String = match args.next() {
            Some(x) => x,
            None => return Err("Didn't get source file parameter"),
        };

        let dest_file: String = match args.next() {
            Some(x) => x,
            None => return Err("Didn't get dest file parameter"),
        };

        Ok(Self {
            source_file,
            dest_file,
        })
    }
}

impl ConfigLs {
    pub fn build<T>(mut args: T) -> Result<Self, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next();

        let dir: String = match args.next() {
            Some(x) => x,
            None => return Err("Didn't get directory parameter"),
        };

        Ok(Self { dir })
    }
}

impl ConfigHead {
    pub fn build<T>(mut args: T) -> Result<Self, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next();

        let file: String = match args.next() {
            Some(x) => x,
            None => return Err("Didn't get file parameter"),
        };

        let num_lines: String = match args.next() {
            Some(x) => x,
            None => return Err("Didn't get num_lines parameter"),
        };

        let num_lines_int: u32 = match num_lines.parse() {
            Ok(t) => t,
            Err(_z) => return Err("Unable to parse num_lines to int"),
        };

        Ok(Self {
            file,
            num_lines: num_lines_int,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_grep_build() {
        let args_iterator = [
            "binary_name".to_string(),
            "query".to_string(),
            "file_path".to_string(),
        ]
        .into_iter();

        let config = ConfigGrep::build(args_iterator).unwrap();
        assert_eq!(config.query, "query".to_string());
        assert_eq!(config.file_path, "file_path".to_string());
        assert_eq!(config.ignore_case, false);
    }

    #[test]
    fn test_config_cp_build() {
        let args_iterator = [
            "binary_name".to_string(),
            "source_file".to_string(),
            "dest_file".to_string(),
        ]
        .into_iter();

        let config = ConfigCp::build(args_iterator).unwrap();
        assert_eq!(config.source_file, "source_file".to_string());
        assert_eq!(config.dest_file, "dest_file".to_string());
    }

    #[test]
    fn test_config_ls_build() {
        let args_iterator = ["binary_name".to_string(), "dir".to_string()].into_iter();

        let config = ConfigLs::build(args_iterator).unwrap();
        assert_eq!(config.dir, "dir".to_string());
    }
}
