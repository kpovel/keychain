use crate::{KeyValuePair, SaveCommand};
use rusqlite::Connection;
use std::rc::Rc;

pub fn save_entry(value: SaveCommand, connection: Rc<Connection>) -> Result<(), rusqlite::Error> {
    match value.visibility {
        crate::Visibility::Public(values) => save_public_pair(values, connection),
        crate::Visibility::Secret(_) => todo!("Saving secret entry in development process"),
    }
}

fn save_public_pair(
    value: KeyValuePair,
    connection: Rc<Connection>,
) -> Result<(), rusqlite::Error> {
    connection.execute(
        "insert into public (key, value) values (?1, ?2)",
        (&value.key, &value.value),
    )?;

    Ok(())
}
