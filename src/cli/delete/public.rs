use super::DeleteAction;
use crate::cli::KeyEntry;
use rusqlite::Connection;
use std::{io, rc::Rc};

pub fn delete_public_pair(
    key_entry: KeyEntry,
    conn: Rc<Connection>,
) -> Result<DeleteAction, String> {
    let continue_deleting = verify_action(&key_entry);

    if !continue_deleting {
        return Ok(DeleteAction::Canceled);
    }

    delete_pair(&key_entry, Rc::clone(&conn)).map_err(|e| e.to_string())?;
    Ok(DeleteAction::Deleted)
}

pub fn verify_action(entry: &KeyEntry) -> bool {
    println!(
        "Are you sure you want to continue deliting {} [Y/n]: ",
        entry.key
    );

    let mut delete_value = String::new();
    let stdin_err = io::stdin().read_line(&mut delete_value);

    if stdin_err.is_err() {
        return false;
    }

    match delete_value.trim().to_lowercase().as_str() {
        "y" => true,
        _ => false,
    }
}

fn delete_pair(KeyEntry { key }: &KeyEntry, conn: Rc<Connection>) -> Result<(), rusqlite::Error> {
    conn.execute("delete from public where key = ?1", [&key])?;
    Ok(())
}
