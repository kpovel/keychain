use super::{SaveCommand, Visibility};
use public_save::save_public_pair;
use rusqlite::Connection;
use secret_save::save_secret_pair;
use std::rc::Rc;

mod public_save;
mod secret_save;

pub fn save_entry(value: SaveCommand, connection: Rc<Connection>) -> Result<(), String> {
    match value.visibility {
        Visibility::Public(v) => save_public_pair(v, connection),
        Visibility::Secret(v) => save_secret_pair(v, connection),
    }
}
