use rusqlite::{Connection, Result};
use std::{env, fs, path::Path, process, rc::Rc};

pub fn db_client() -> Result<Connection, rusqlite::Error> {
    let home_dir = env::var("HOME").unwrap_or_else(|_| {
        eprintln!("Cannot find the HOME environment variable.");
        process::exit(1);
    });

    let keychain_dir = Path::new(&home_dir).join(".keychain");
    fs::create_dir_all(&keychain_dir).unwrap_or_else(|err| {
        eprintln!("Can't create a directory to store credentials: {}", err);
        process::exit(2);
    });

    Connection::open(keychain_dir.join("credentials.db"))
}

pub fn create_default_db_schema(connection: Rc<Connection>) -> Result<()> {
    connection.execute(
        "\n
create table if not exists secret (
    id    integer primary key autoincrement,
    key   varchar(50) not null unique,
    value text not null
);",
        (),
    )?;

    connection.execute(
        "\n
create table if not exists public (
    id    integer primary key autoincrement,
    key   varchar(50) not null unique,
    value text not null
);",
        (),
    )?;

    connection.execute(
        "\n
create table if not exists credentials (
    id    integer primary key autoincrement,
    name  varchar(50) not null unique,
    value text not null
);",
        (),
    )?;

    Ok(())
}
