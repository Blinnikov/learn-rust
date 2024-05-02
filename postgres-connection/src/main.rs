use postgres::{Client, NoTls, Error};

#[derive(Debug)]
struct City {
    _id: i64,
    name: String,
    country: String,
}

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
    
    let cities_to_add = get_cities();

    let _ = write_cities(&mut client, &cities_to_add);

    let cities = read_cities(&mut client);

    // for city in cities {
    //     println!("{:?}", city);
    // }

    println!("{:?}", cities);

    Ok(())
}

fn read_cities(client: &mut Client) -> Option<Vec<City>> {
    let query = "SELECT id, name, country FROM cities;";
    let mut res = Vec::new();

    for row in client.query(query, &[]).ok()? {
        let id: i64 = row.get(0);
        let name: String = row.get(1);
        let country: String = row.get(2);

        let city = City { _id: id, name, country };

        res.push(city);
    }

    Some(res)
}

fn get_cities() -> Vec<City> {
    vec![
        City { _id: 0, name: "Москва".to_string(), country: "Россия".to_string() },
        City { _id: 0, name: "Минск".to_string(), country: "Беларусь".to_string() },
        City { _id: 0, name: "Ростов-на-Дону".to_string(), country: "Россия".to_string() },
    ]
}

fn write_cities(client: &mut Client, cities: &Vec<City>) -> Result<(), Error> {
    for city in cities {
        let _row_added = client.execute(
            "INSERT INTO cities(name, country) VALUES($1, $2)",
            &[&city.name, &city.country]
        )?;
    }

    Ok(())
}
