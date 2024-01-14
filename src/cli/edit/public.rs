use crate::cli::edit::prompt_new_entry_value;
use crate::cli::KeyEntry;
use rusqlite::Connection;
use std::rc::Rc;

pub fn edit_public_value(entry: KeyEntry, conn: Rc<Connection>) -> Result<String, String> {
    let new_entry_value = prompt_new_entry_value(&entry).map_err(|e| e.to_string())?;
    edit_entry_value(&entry, &new_entry_value, Rc::clone(&conn)).map_err(|e| e.to_string())?;

    Ok(String::from("Successfully edited key value"))
}

fn edit_entry_value(
    KeyEntry { key }: &KeyEntry,
    new_value: &str,
    conn: Rc<Connection>,
) -> Result<(), rusqlite::Error> {
    // todo: check if such key doesn't exist

    conn.execute(
        "update public set value = ?1 where key = ?2;",
        [new_value, &key],
    )?;

    Ok(())
}
