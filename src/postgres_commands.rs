use super::*;
use lazy_static::lazy_static;
use postgres::{Client, NoTls};

lazy_static! {
    static ref connection_string: String =
        postres_config::Config::get_from_cfg_json().connection_string;
}

pub fn create_person_table() {
    let mut client =
        Client::connect(&connection_string, NoTls).expect("PostgreSql client creation error.");

    client
        .batch_execute(
            "
        CREATE TABLE person (
            id      SERIAL PRIMARY KEY, 
            name    TEXT NOT NULL,
            age     SMALLINT
        );",
        )
        .unwrap_or_else(|err| panic!("{}", err));
}
pub fn add_to_person_table(name: &str, age: i16) {
    let mut client =
        Client::connect(&connection_string, NoTls).expect("PostgreSql client creation error.");

    client
        .execute(
            "
    INSERT INTO person (name, age) 
    VALUES ($1, $2);",
            &[&name, &age],
        )
        .unwrap_or_else(|err| panic!("{}", err));
}
pub fn delete_by_name(name: &str) {
    let mut client =
        Client::connect(&connection_string, NoTls).expect("PostgreSql client creation error.");

    client
        .execute(
            "
    DELETE FROM person WHERE name = $1;
    ",
            &[&name],
        )
        .unwrap_or_else(|err| panic!("{}", err));
}
