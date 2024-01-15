use crate::cli::edit::prompt_new_entry_value;
use crate::cli::save::secret_save::{secret_password, verify_user_password, VerifyPassword};
use crate::cli::KeyEntry;
use crate::crypto;
use rusqlite::Connection;
use std::rc::Rc;

pub fn edit_secret_value(entry: KeyEntry, conn: Rc<Connection>) -> Result<String, String> {
    let hash = secret_password(Rc::clone(&conn)).map_err(|e| e.to_string())?;
    let pass = match verify_user_password(&hash)? {
        VerifyPassword::Password(pass) => pass,
        VerifyPassword::NotMatching => return Err(String::from("Passwords are not matching")),
    };

    let new_entry_value = prompt_new_entry_value(&entry).map_err(|e| e.to_string())?;
    let encrypted_value = crypto::encrypt_value(&new_entry_value, &pass);

    edit_entry_value(&entry, &encrypted_value, Rc::clone(&conn)).map_err(|e| e.to_string())?;
    Ok(String::from("Successfully edited secret key value"))
}

fn edit_entry_value(
    KeyEntry { key }: &KeyEntry,
    new_value: &str,
    conn: Rc<Connection>,
) -> Result<(), rusqlite::Error> {
    // todo: check if such key doesn't exist

    conn.execute(
        "update secret set value = ?1 where key = ?2;",
        [new_value, &key],
    )?;

    Ok(())
}
