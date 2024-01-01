use db::db_client;
use std::process;

mod db;

fn main() {
    let db_client = db_client().unwrap_or_else(|err| {
        eprintln!("Failed to make db connection: {}", err);
        process::exit(1)
    });

    db::create_default_db_schema(&db_client).unwrap_or_else(|err| {
        eprintln!("Error during creating default db schema: {}", err);
        process::exit(1);
    });

    // todo: parse command line arguments
}
