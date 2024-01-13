use super::ListSubcommand;
use rusqlite::Connection;
use std::rc::Rc;

#[derive(Debug)]
struct Entry {
    key: String,
}

pub fn list_entry(sub: ListSubcommand, conn: Rc<Connection>) -> Result<(), rusqlite::Error> {
    let entries = match sub {
        ListSubcommand::Public => list_public_entry(Rc::clone(&conn)),
        ListSubcommand::Secret => list_secret_entry(Rc::clone(&conn)),
    }?;

    display_entries(&sub, &entries);

    Ok(())
}

fn display_entries(visibility: &ListSubcommand, entries: &[Entry]) {
    match visibility {
        ListSubcommand::Public => println!("List of public entries"),
        ListSubcommand::Secret => println!("List of secret entries"),
    };

    entries.into_iter().for_each(|r| {
        println!("{}", r.key);
    })
}

fn list_public_entry(conn: Rc<Connection>) -> Result<Vec<Entry>, rusqlite::Error> {
    let mut stmt = conn.prepare("select key from public")?;
    let rows = stmt
        .query_map([], |row| Ok(Entry { key: row.get(0)? }))?
        .collect::<Vec<_>>();

    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }

    Ok(entries)
}

fn list_secret_entry(conn: Rc<Connection>) -> Result<Vec<Entry>, rusqlite::Error> {
    let mut stmt = conn.prepare("select key from secret")?;
    let rows = stmt
        .query_map([], |row| Ok(Entry { key: row.get(0)? }))?
        .collect::<Vec<_>>();

    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }

    Ok(entries)
}
