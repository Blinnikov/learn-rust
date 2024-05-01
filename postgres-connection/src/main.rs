use postgres::{Client, NoTls, Error};

fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let host = "postgres://postgres:postgres@localhost:5432";
    let db = "travel";

    let connection_str = format!("{host}/{db}");

    let mut client = Client::connect(connection_str.as_str(), NoTls)?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS planes (
            id SERIAL PRIMARY KEY,
            model TEXT
        )
    ")?;

    Ok(())
}
