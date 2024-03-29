use clap::Parser;
use cli::{delete_entry, edit_entry, list_entry, read_entry, Cli, Commands};
use db::db_client;
use std::{process, rc::Rc};

mod cli;
mod crypto;
mod db;

fn main() {
    let cli = Cli::parse();

    let db_client = Rc::new(db_client().unwrap_or_else(|err| {
        eprintln!("Failed to make db connection: {}", err);
        process::exit(1)
    }));

    db::create_default_db_schema(Rc::clone(&db_client)).unwrap_or_else(|err| {
        eprintln!("Error during creating default db schema: {}", err);
        process::exit(1);
    });

    match cli.command {
        Commands::Save(value) => {
            let saving_result = cli::save_entry(value, Rc::clone(&db_client));

            match saving_result {
                Ok(_) => println!("Successfully saved values"),
                Err(e) => eprintln!("Error during saving results: {}", e),
            }
        }
        Commands::Read(command) => match read_entry(command, Rc::clone(&db_client)) {
            Ok(v) => println!("{}", v),
            Err(e) => eprintln!("Error during reading value: {}", e),
        },
        Commands::List(subcommand) => {
            list_entry(subcommand, Rc::clone(&db_client)).unwrap_or_else(|e| {
                eprintln!("Error during showing list of entries: {}", e.to_string());
            });
        }
        Commands::Edit(command) => match edit_entry(command, Rc::clone(&db_client)) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("Error during editing key value: {}", e),
        },
        Commands::Delete(command) => match delete_entry(command, Rc::clone(&db_client)) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("Error during deleting key-value pair: {}", e),
        },
    };
}
