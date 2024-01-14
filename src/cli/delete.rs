use super::{DeleteCommand, ReadSubcommand};
use public::delete_public_pair;
use rusqlite::Connection;
use std::rc::Rc;

mod public;

#[derive(Debug)]
pub enum DeleteAction {
    Canceled,
    Deleted,
}

pub fn delete_entry(command: DeleteCommand, conn: Rc<Connection>) -> Result<(), String> {
    let delete_action = match command.visibility {
        ReadSubcommand::Public(entry) => delete_public_pair(entry, Rc::clone(&conn)),
        ReadSubcommand::Secret(entry) => todo!(),
    };

    dbg!(&delete_action);

    Ok(())
}
