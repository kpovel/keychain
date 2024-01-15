use secret::edit_secret_value;
use super::{EditCommand, KeyEntry};
use crate::cli::EditSubcommand;
use public::edit_public_value;
use rusqlite::Connection;
use std::{io, rc::Rc};

mod public;
mod secret;

pub fn edit_entry(command: EditCommand, conn: Rc<Connection>) -> Result<String, String> {
    match command.visibility {
        EditSubcommand::Public(entry) => edit_public_value(entry, Rc::clone(&conn)),
        EditSubcommand::Secret(entry) => edit_secret_value(entry, Rc::clone(&conn)),
    }
}

pub fn prompt_new_entry_value(KeyEntry { key }: &KeyEntry) -> Result<String, io::Error> {
    println!("Type new value for {}: ", key);

    let mut edit_to = String::new();
    io::stdin().read_line(&mut edit_to)?;

    Ok(edit_to)
}
