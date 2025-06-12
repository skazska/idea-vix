use std::fs;
use std::error::Error;

pub mod config;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_build_config_tl(r#in: &[String], expected: Result<Config, Box<dyn Error>>) {
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
            &vec![
                "minigrep".to_string(),
                "query".to_string(),
                "filename.txt".to_string()
            ],
            Ok(Config {
                query: "query".to_string(),
                filename: "filename.txt".to_string()
            })
        );

        test_build_config_tl(
            &vec!["minigrep".to_string()],
            Err("Please provide a query string".into())
        );
        test_build_config_tl(
            &vec!["minigrep".to_string(), "query".to_string()],
            Err("Please provide a filename".into())
        );
    }

    fn test_run_tl(r#in: Config, expected: Result<(), Box<dyn Error>>) {
        let result = run(r#in);

        match expected {
            Ok(_) => {
                assert_eq!(result.is_ok(), true);
            },
            Err(expected_err) => {
                assert_eq!(result.is_err(), true);
                assert_eq!(result.unwrap_err().to_string(), expected_err.to_string());
            }
        }
    }

    #[test]
    fn test_run() {
        test_run_tl(
            Config {
                query: "nobody".to_string(),
                filename: "poem.txt".to_string(),
            },
            Ok(()),
        );
    }
}