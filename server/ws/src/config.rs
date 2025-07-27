///Configurations module for web server
use serde::{Deserialize};

/// Configuration structure for the web server
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Host address for the server
    pub host: String,
    /// Host address for the server
    pub port: u16,
    /// Log level for the server
    pub log_level: String,
    /// URL for the database
    pub database_url: String,
    /// Number of connections in the pool
    pub database_pool: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct ConfigFile {
    host: Option<String>,
    port: Option<u16>,
    log_level: Option<String>,
    database_url: Option<String>,
    database_pool: Option<u8>,
}

/// ENV points to the configuration file
/// if not set, defaults to "config.toml" 
const DEFAULT_CONFIG_FN: &str = "config.toml";

/// Default values for the web server configuration
/// These values are used if the configuration file is not found or if the environment variables are not set.
/// host
const DEFAULT_HOST: &str = "0.0.0.0";
/// port
const DEFAULT_PORT: u16 = 7878;
/// log level
const DEFAULT_LOG_LEVEL: &str = "info";

// const DEFAULT_DATABASE_URL: &str = "file:shapes.db?mode=memory&cache=shared";
//const DEFAULT_DATABASE_URL: &str = "sqlite://.sqlite.db";
const DEFAULT_DATABASE_URL: &str = "sqlite.db";

/// Default database pool size
const DEFAULT_DATABASE_POOL: u8 = 5;

/// Configuration implementstion
/// provides default values for the web server
/// searches for a configuration .toml file in the current directory and loads it overriding defaults
/// overrides default and loaded values with environment variables
impl Config {
    pub fn new() -> Self {
        let mut config = Config {
            host: DEFAULT_HOST.to_string(),
            port: DEFAULT_PORT,
            log_level: DEFAULT_LOG_LEVEL.to_string(),
            database_url: DEFAULT_DATABASE_URL.to_string(),
            database_pool: DEFAULT_DATABASE_POOL,
        };

        let config_file_fn = &std::env::var("WS_CONFIG_FILE").unwrap_or(DEFAULT_CONFIG_FN.to_string());

        if let Ok(toml_str) = std::fs::read_to_string(config_file_fn) {
            // If the file is found, parse it and set the values
            Self::set_from_toml(&toml_str, &mut config);
        } else {
            eprintln!("Configuration file not found: {}", config_file_fn);
            eprintln!("Using default configuration values.");
        }

        dotenv::dotenv().ok();

        // Override with environment variables if set
        if let Ok(host) = std::env::var("WS_HOST") {
            config.host = host;
        }
        if let Ok(port) = std::env::var("WS_PORT") {
            config.port = port.parse().unwrap_or(DEFAULT_PORT);
        }
        if let Ok(log_level) = std::env::var("WS_LOG_LEVEL") {
            config.log_level = log_level;
        }


        if let Ok(database_url) = std::env::var("WS_DATABASE_URL") {
            config.database_url = database_url;
        }
        if let Ok(database_pool) = std::env::var("WS_DATABASE_POOL") {
            config.database_pool = database_pool.parse().unwrap_or(DEFAULT_DATABASE_POOL);
        }

        config
    }

    // overrides values from a configuration file contents
    fn set_from_toml(toml_str: &str, config: &mut Self) {
        let file_config: Result<ConfigFile, _> = toml::from_str(&toml_str);
        match file_config {
            Ok(file_config) => {
                if let Some(host) = file_config.host {
                    config.host = host;
                }

                if let Some(port) = file_config.port {
                    config.port = port;
                }

                if let Some(log_level) = file_config.log_level {
                    config.log_level = log_level;
                }

                if let Some(database_url) = file_config.database_url {
                    config.database_url = database_url;
                }

                if let Some(database_pool) = file_config.database_pool {
                    config.database_pool = database_pool;
                }
            }
            Err(e) => {
                eprintln!("Failed to parse configuration file: {}, error: {}", toml_str, e);
                eprintln!("Using default configuration values.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::new();
        assert_eq!(config.host, DEFAULT_HOST);
        assert_eq!(config.port, DEFAULT_PORT);
        assert_eq!(config.log_level, DEFAULT_LOG_LEVEL);
    }

    fn get_from_config_file(pref: &str, config_str: &str) -> Config {
        // Create a temporary config file
        let temp_file_path = &format!("{}_temp_config.toml", pref);
        std::fs::write(temp_file_path, config_str).unwrap();

        // Set the environment variable to point to the temp file
        unsafe {
            std::env::set_var("WS_CONFIG_FILE", temp_file_path);
        }

        let config = Config::new();

        // Clean up the temporary file
        std::fs::remove_file(temp_file_path).unwrap();

        config
    }

    #[test]
    fn test_config_from_file() {
        // Create a temporary config file
        let temp_config = r#"
            host = "host.example.com"
            port = 8080
            log_level = "debug"
        "#;

        let config = get_from_config_file("test1", temp_config);

        assert_eq!(config.host, "host.example.com");
        assert_eq!(config.port, 8080);
        assert_eq!(config.log_level, "debug");

        // Create a temporary config file with missing fields
        let temp_config = r#"
            host = "host.example.com"
        "#;
        let config = get_from_config_file("test2", temp_config);
        assert_eq!(config.host, "host.example.com");
        assert_eq!(config.port, DEFAULT_PORT);
        assert_eq!(config.log_level, DEFAULT_LOG_LEVEL);
    }

    #[test]
    fn test_config_from_env() {
        // Set environment variables
        unsafe {
            std::env::set_var("WS_HOST", "env.example.com");
            std::env::set_var("WS_PORT", "9090");
            std::env::set_var("WS_LOG_LEVEL", "warn");
        }
        let config = Config::new();

        assert_eq!(config.host, "env.example.com");
        assert_eq!(config.port, 9090);
        assert_eq!(config.log_level, "warn");

        // Clean up environment variables
        unsafe {
            std::env::remove_var("WS_HOST");
            std::env::remove_var("WS_PORT");
            std::env::remove_var("WS_LOG_LEVEL");
        }
    }
}


