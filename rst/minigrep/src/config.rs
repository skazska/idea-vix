#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let query = args.get(1).ok_or("Please provide a query string")?;
        let filename = args.get(2).ok_or("Please provide a filename")?;

        Ok(Config {
            query: query.to_string(),
            filename: filename.to_string()
        })
    }
}