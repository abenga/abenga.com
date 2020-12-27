pub mod posts;

use crate::lib::config;

// use postgres;


pub fn get_connection() -> Result<postgres::Client, postgres::Error> {
    let conf = config::get_config();
    let connect_str = format!("postgresql://postgres:{user}@host:port/db_name", user=conf.databases["local"].user);
    let client = postgres::Client::connect(&connect_str, postgres::NoTls)?;
    Ok(client)
}
