use super::{DeleteCommand, ReadSubcommand};
use crate::cli::delete::secret::delete_secret_pair;
use public::delete_public_pair;
use rusqlite::Connection;
use std::rc::Rc;

mod public;
mod secret;

#[derive(Debug)]
pub enum DeleteAction {
    Canceled,
    Deleted,
}

pub fn delete_entry(command: DeleteCommand, conn: Rc<Connection>) -> Result<String, String> {
    let delete_action = match command.visibility {
        ReadSubcommand::Public(entry) => delete_public_pair(entry, Rc::clone(&conn)),
        ReadSubcommand::Secret(entry) => delete_secret_pair(entry, Rc::clone(&conn)),
    }?;

    Ok(match delete_action {
        DeleteAction::Canceled => String::from("Aborting password deleting"),
        DeleteAction::Deleted => String::from("Successfully deleted key-value pair"),
    })
}
