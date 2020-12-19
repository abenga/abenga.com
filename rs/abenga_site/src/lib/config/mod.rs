
use std::collections::HashMap;
use std::fs;


#[derive(serde::Deserialize)]
struct Config {
    databases: HashMap<String, DBConfig>,
}

#[derive(serde::Deserialize)]
struct DBConfig {
    protocol: String,
    host: String,
    port: u32,
    database: String,
    user: String,
    password: String,
    echo: bool,
}

pub fn get_config() -> Result<Config, Box<std::error::Error + 'static>> {
    let config_file_path = "/home/horace/Documents/Development/Rust/abenga_site/configuration.toml";
    let config_str = fs::read_to_string(config_file_path)?.parse()?;
    toml::from_str(config_str).unwrap()
}
