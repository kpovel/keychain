use clap::{Args, Parser, Subcommand};
use db::db_client;
use std::process;

mod db;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "This CLI allows you to save, view, edit, and delete encrypted key-value pairs. Values can be stored either publicly or secretly."
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Commands for the keychain CLI
#[derive(Debug, Subcommand)]
enum Commands {
    /// Save a new keychain entry
    #[clap(visible_alias = "s")]
    Save(SaveCommand),
    /// Read a keychain entry
    #[clap(visible_alias = "r")]
    Read,
    /// Edit a keychain entry
    #[clap(visible_alias = "e")]
    Edit,
    /// Delete a keychain entry
    #[clap(visible_alias = "d")]
    Delete,
}

/// Save a new public or secret key-value pair
#[derive(Debug, Args)]
struct SaveCommand {
    #[command(subcommand)]
    visibility: Visibility,

    #[command(flatten)]
    key_value: KeyValuePair,
}

/// Whether the key-value pair should be public or secret
#[derive(Debug, Subcommand)]
enum Visibility {
    /// Store the value publicly
    Public,
    /// Encrypt and store the value secretly
    Secret,
}

/// The key-value pair to save
#[derive(Debug, Args)]
struct KeyValuePair {
    /// Provide key for the value
    #[arg(short, long)]
    key: String,
    /// Provide value
    #[arg(short, long)]
    value: String,
}

fn main() {
    let _args = Cli::parse();
    match _args.command {
        Commands::Save(value) => println!("Key value: {:?}", value),
        Commands::Read => todo!(),
        Commands::Edit => todo!(),
        Commands::Delete => todo!(),
    };

    let db_client = db_client().unwrap_or_else(|err| {
        eprintln!("Failed to make db connection: {}", err);
        process::exit(1)
    });

    db::create_default_db_schema(&db_client).unwrap_or_else(|err| {
        eprintln!("Error during creating default db schema: {}", err);
        process::exit(1);
    });
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
