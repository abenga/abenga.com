pub mod posts;

// use postgres;


pub fn get_connection() -> Result<postgres::Client, postgres::Error> {
    let connect_str = "postgresql://postgres:postgres@localhost/library";
    let mut client = postgres::Client::connect(connect_str, postgres::NoTls)?;
    Ok(client)
}
