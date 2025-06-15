use std::env;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // This is the program name, we can ignore it

        let query = args.next().ok_or("Please provide a query string")?;
        let filename = args.next().ok_or("Please provide a filename")?;

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query: query.to_string(),
            filename: filename.to_string(),
            ignore_case: ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use super::*;

    // test case for building the config
    fn test_build_config_tl(r#in: impl Iterator<Item = String>, expected: Result<Config, Box<dyn Error>>) {
        let result = Config::build(r#in);

        match expected {
            Ok(expected_config) => {
                assert_eq!(result.is_ok(), true);
                assert_eq!(result.unwrap(), expected_config);
            }
            Err(expected_err) => {
                assert_eq!(result.is_err(), true);
                assert_eq!(result.unwrap_err().to_string(), expected_err.to_string());
            }
        }
    }

    #[test]
    fn test_build_config() {
        test_build_config_tl(
            vec![String::from("minigrep"), String::from("query"), String::from("filename.txt")].into_iter(),
            Ok(Config {
                query: String::from("query"), filename: String::from("filename.txt"), ignore_case: false
            })
        );

        unsafe { env::set_var("IGNORE_CASE", "1"); }

        test_build_config_tl(
            vec![String::from("minigrep"), String::from("query"), String::from("filename.txt")].into_iter(),
            Ok(Config {
                query: String::from("query"), filename: String::from("filename.txt"), ignore_case: true
            })
        );

        unsafe { env::remove_var("IGNORE_CASE"); }

        test_build_config_tl(
            vec![String::from("minigrep")].into_iter(),
            Err("Please provide a query string".into())
        );
        test_build_config_tl(
            vec![String::from("minigrep"), String::from("query")].into_iter(),
            Err("Please provide a filename".into())
        );
    }
}