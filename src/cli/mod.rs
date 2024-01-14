use clap::{command, Args, Parser, Subcommand};
pub use delete::delete_entry;
pub use edit::edit_entry;
pub use list::list_entry;
pub use read::read_entry;
pub use save::save_entry;

mod delete;
mod edit;
mod list;
mod read;
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
    Read(ReadCommand),
    #[clap(visible_alias = "l")]
    #[command(subcommand)]
    /// List of all keychain entries
    List(ListSubcommand),
    /// Edit a keychain entry
    #[clap(visible_alias = "e")]
    Edit(EditCommand),
    /// Delete a keychain entry
    #[clap(visible_alias = "d")]
    Delete(DeleteCommand),
}

/// Save a new public or secret key-value pair
#[derive(Debug, Args)]
pub struct SaveCommand {
    #[command(subcommand)]
    pub visibility: VisibilitySave,
}

/// Whether the key-value pair should be public or secret
#[derive(Debug, Subcommand)]
pub enum VisibilitySave {
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

#[derive(Debug, Args)]
pub struct ReadCommand {
    #[command(subcommand)]
    pub visibility: ReadSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ReadSubcommand {
    #[clap(visible_alias = "p")]
    /// Read public entry
    Public(KeyEntry),
    #[clap(visible_alias = "s")]
    /// Read secret entry
    Secret(KeyEntry),
}

#[derive(Debug, Args)]
pub struct KeyEntry {
    /// Provide key for the value
    #[arg(short, long)]
    pub key: String,
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

#[derive(Debug, Args)]
pub struct EditCommand {
    #[command(subcommand)]
    pub visibility: EditSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum EditSubcommand {
    #[clap(visible_alias = "p")]
    /// Edit public value
    Public(KeyEntry),
    #[clap(visible_alias = "s")]
    /// Edit secret value
    Secret(KeyEntry),
}

#[derive(Debug, Args)]
pub struct DeleteCommand {
    #[command(subcommand)]
    pub visibility: ReadSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum DeleteSubcommand {
    #[clap(visible_alias = "p")]
    /// Delete public entry
    Public(KeyEntry),
    #[clap(visible_alias = "s")]
    /// Delete secret entry
    Secret(KeyEntry),
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
