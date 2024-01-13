use super::{SaveCommand, VisibilitySave};
use public_save::save_public_pair;
use rusqlite::Connection;
use secret_save::save_secret_pair;
use std::rc::Rc;

mod public_save;
pub mod secret_save;

pub fn save_entry(value: SaveCommand, connection: Rc<Connection>) -> Result<(), String> {
    match value.visibility {
        VisibilitySave::Public(v) => save_public_pair(v, connection),
        VisibilitySave::Secret(v) => save_secret_pair(v, connection),
    }
}
