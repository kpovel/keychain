use clap::Parser;
use cli::{Cli, Commands};
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
        Commands::Read => todo!(),
        Commands::Edit => todo!(),
        Commands::Delete => todo!(),
    };
}
