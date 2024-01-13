use clap::{command, Args, Parser, Subcommand};
pub use list::list_entry;
pub use save::save_entry;

mod list;
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
    #[clap(visible_alias = "l")]
    #[command(subcommand)]
    /// List of all keychain entries
    List(ListSubcommand),
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

#[derive(Debug, Subcommand)]
pub enum ListSubcommand {
    #[clap(visible_alias = "p")]
    /// List of public keychain entries
    Public,
    #[clap(visible_alias = "s")]
    /// List of secret keychain entries
    Secret,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
