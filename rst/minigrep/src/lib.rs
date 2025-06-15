use std::fs;
use std::error::Error;

pub mod config;
pub mod search;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.ignore_case { 
        search::find_line_ci(&config.query, &contents) 
     } else { 
        search::find_line(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_run_tl(r#in: config::Config, expected: Result<(), Box<dyn Error>>) {
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

    // FIXME tests with real file?
    #[test]
    fn test_run() {
        test_run_tl(
            config::Config { query: String::from("nobody"), filename: String::from("poem.txt"), ignore_case: true },
            Ok(()),
        );
        test_run_tl(
            config::Config { query: String::from("nobody"), filename: String::from("nonexistent.txt"), ignore_case: true },
            Err("No such file or directory (os error 2)".into()),
        );
    }

}