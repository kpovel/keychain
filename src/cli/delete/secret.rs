use super::{public::verify_action, DeleteAction};
use crate::cli::{
    save::secret_save::{secret_password, verify_user_password, VerifyPassword},
    KeyEntry,
};
use rusqlite::Connection;
use std::rc::Rc;

pub fn delete_secret_pair(
    key_entry: KeyEntry,
    conn: Rc<Connection>,
) -> Result<DeleteAction, String> {
    let continue_deleting = verify_action(&key_entry);

    if !continue_deleting {
        return Ok(DeleteAction::Canceled);
    }

    match verify_password(Rc::clone(&conn)) {
        Ok(VerifyPassword::Password(_)) => (),
        Ok(VerifyPassword::NotMatching) => return Err(String::from("Passwords are not matching")),
        Err(e) => return Err(e),
    };

    delete_pair(&key_entry, Rc::clone(&conn)).map_err(|e| e.to_string())?;
    Ok(DeleteAction::Deleted)
}

fn verify_password(conn: Rc<Connection>) -> Result<VerifyPassword, String> {
    let hash = secret_password(Rc::clone(&conn)).map_err(|e| e.to_string())?;
    verify_user_password(&hash)
}

fn delete_pair(KeyEntry { key }: &KeyEntry, conn: Rc<Connection>) -> Result<(), rusqlite::Error> {
    conn.execute("delete from secret where key = ?1", [&key])?;
    Ok(())
}
