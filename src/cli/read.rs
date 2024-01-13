use super::{ReadCommand, ReadSubcommand};
use public_pair::read_public_pair;
use rusqlite::Connection;
use secret_pair::read_secret_pair;
use std::rc::Rc;

mod public_pair;
mod secret_pair;

#[derive(Debug)]
pub struct PairValue {
    value: String,
}

pub fn read_entry(command: ReadCommand, conn: Rc<Connection>) -> Result<String, String> {
    let pair_value = match command.visibility {
        ReadSubcommand::Public(entry) => read_public_pair(entry, Rc::clone(&conn)),
        ReadSubcommand::Secret(entry) => read_secret_pair(entry, Rc::clone(&conn)),
    }?;

    Ok(pair_value.value)
}
