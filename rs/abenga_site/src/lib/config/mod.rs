
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path;


#[derive(serde::Deserialize)]
pub struct DBConfig {
    pub protocol: String,
    pub host: String,
    pub port: u32,
    pub database: String,
    pub user: String,
    pub password: String,
    pub echo: bool,
}


#[derive(serde::Deserialize)]
pub struct Config {
    pub databases: HashMap<String, DBConfig>,
}


pub fn get_config() -> Config {
    // FIXME: Make this path relative, or a useful system-wide value
    let config_file_path = path::Path::new("/app/configuration.toml");
    let config_file_path = path::Path::new("/app/configuration.toml");
    let mut f = fs::File::open(&config_file_path).expect("Unable to open configuration file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Could not read file to string");
    let config: Config = toml::from_str(&s).expect("Unable to parse config");
    return config;
}
