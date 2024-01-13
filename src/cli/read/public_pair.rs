use super::PairValue;
use crate::cli::KeyEntry;
use rusqlite::Connection;
use std::rc::Rc;

pub fn read_public_pair(key_entry: KeyEntry, conn: Rc<Connection>) -> Result<PairValue, String> {
    let mut stmt = conn
        .prepare("select value from public where key = ?1")
        .map_err(|e| e.to_string())?;
    let pair_value = stmt
        .query_row([&key_entry.key], |row| Ok(PairValue { value: row.get(0)? }))
        .map_err(|e| e.to_string())?;

    Ok(pair_value)
}
