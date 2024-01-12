use clap::{command, Args, Parser, Subcommand};
pub use save::save_entry;

mod save;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "This CLI allows you to save, view, edit, and delete encrypted key-value pairs. Values can be stored either publicly or secretly."
)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Commands for the keychain CLI
#[derive(Debug, Subcommand)]
pub enum Commands {
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
pub struct SaveCommand {
    #[command(subcommand)]
    pub visibility: Visibility,
}

/// Whether the key-value pair should be public or secret
#[derive(Debug, Subcommand)]
pub enum Visibility {
    /// Store the value publicly
    #[clap(visible_alias = "p")]
    Public(KeyValuePair),
    /// Encrypt and store the value secretly
    #[clap(visible_alias = "s")]
    Secret(KeyValuePair),
}

/// The key-value pair
#[derive(Debug, Parser)]
pub struct KeyValuePair {
    /// Provide key for the value
    #[arg(short, long)]
    pub key: String,
    /// Provide value
    #[arg(short, long)]
    pub value: String,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
