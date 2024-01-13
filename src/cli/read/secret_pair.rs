use super::PairValue;
use crate::cli::save::secret_save::{secret_password, verify_user_password, VerifyPassword};
use crate::cli::KeyEntry;
use crate::crypto::decrypt_value;
use rusqlite::Connection;
use std::rc::Rc;

pub fn read_secret_pair(key_entry: KeyEntry, conn: Rc<Connection>) -> Result<PairValue, String> {
    let hash = secret_password(Rc::clone(&conn)).map_err(|e| e.to_string())?;
    let pass = match verify_user_password(&hash)? {
        VerifyPassword::Password(pass) => pass,
        VerifyPassword::NotMatching => return Err(String::from("Passwords are not matching")),
    };

    let encrypted_value =
        read_key_value(Rc::clone(&conn), &key_entry).map_err(|e| e.to_string())?;
    let key_value = decrypt_value(&encrypted_value, &pass).map_err(|e| e.to_string())?;

    Ok(PairValue { value: key_value })
}

fn read_key_value(conn: Rc<Connection>, key_entry: &KeyEntry) -> Result<String, rusqlite::Error> {
    let mut stmt = conn.prepare("select value from secret where key = ?1")?;
    stmt.query_row([&key_entry.key], |row| Ok(row.get(0)?))
}
