use crate::KeyValuePair;
use rusqlite::Connection;
use std::rc::Rc;

pub fn save_public_pair(key_value: KeyValuePair, connection: Rc<Connection>) -> Result<(), String> {
    connection
        .execute(
            "insert into public (key, value) values (?1, ?2)",
            (&key_value.key, &key_value.value),
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}
