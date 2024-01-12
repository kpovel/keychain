use crate::SaveCommand;
use public_save::save_public_pair;
use rusqlite::Connection;
use secret_save::save_secret_pair;
use std::rc::Rc;

mod public_save;
mod secret_save;

pub fn save_entry(value: SaveCommand, connection: Rc<Connection>) -> Result<(), String> {
    match value.visibility {
        crate::Visibility::Public(v) => save_public_pair(v, connection),
        crate::Visibility::Secret(v) => save_secret_pair(v, connection),
    }
}
