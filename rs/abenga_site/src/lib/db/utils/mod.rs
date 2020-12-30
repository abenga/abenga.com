pub mod posts;

use std::env;

use crate::lib::config;


pub fn get_db_connection() -> Result<postgres::Connection, postgres::Error> {
    let conf = config::get_config();
    let db_env = env::var("DATABASE_ENV_NAME").expect("Database environment name not set!");
    let connect_str = format!("postgresql://{user}:{password}@{host}:{port}/{database}",
                              user=conf.databases[&db_env].user,
                              password=conf.databases[&db_env].password,
                              host=conf.databases[&db_env].host,
                              port=conf.databases[&db_env].port,
                              database=conf.databases[&db_env].database);
    let conn = postgres::Connection::connect(connect_str,
                                             postgres::TlsMode::None).unwrap();
    Ok(conn)
}
