use crate::{crypto::hash_password, KeyValuePair, SaveCommand};
use rusqlite::Connection;
use std::{io, rc::Rc};

#[derive(Debug)]
enum CreatePassword {
    Password(String),
    NotMatching,
}

pub fn save_entry(value: SaveCommand, connection: Rc<Connection>) -> Result<(), String> {
    match value.visibility {
        crate::Visibility::Public(v) => save_public_pair(v, connection),
        crate::Visibility::Secret(v) => save_private_pair(v, connection),
    }
}

fn save_public_pair(value: KeyValuePair, connection: Rc<Connection>) -> Result<(), String> {
    connection
        .execute(
            "insert into public (key, value) values (?1, ?2)",
            (&value.key, &value.value),
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn save_private_pair(value: KeyValuePair, conn: Rc<Connection>) -> Result<(), String> {
    let pass = match secret_password(Rc::clone(&conn)) {
        Ok(sp) => sp,
        Err(e) => match e {
            rusqlite::Error::QueryReturnedNoRows => {
                let new_pass = create_password().map_err(|e| e.to_string())?;

                match new_pass {
                    CreatePassword::Password(pass) => match save_password(conn, pass) {
                        Ok(hashed_pass) => hashed_pass,
                        Err(e) => return Err(e),
                    },
                    CreatePassword::NotMatching => {
                        return Err(String::from("Passwords are not matching"))
                    }
                }
            }
            _ => {
                return Err(e.to_string());
            }
        },
    };

    println!("Pass: {}", pass);

    todo!("save private key-value");

    Ok(())
}

fn create_password() -> Result<CreatePassword, std::io::Error> {
    println!("Let's create a secret password: ");

    // todo: hide the password
    let mut pass = String::new();
    io::stdin().read_line(&mut pass)?;

    println!("Repeat the password: ");
    let mut pass_repeat = String::new();
    io::stdin().read_line(&mut pass_repeat)?;

    match pass == pass_repeat {
        true => Ok(CreatePassword::Password(pass)),
        false => Ok(CreatePassword::NotMatching),
    }
}

fn save_password(conn: Rc<Connection>, pass: String) -> Result<String, String> {
    let hashed_pass = hash_password(&pass).map_err(|e| e.to_string())?;

    conn.execute(
        "insert into credentials (name, value) values ($1, $2)",
        ("password", &hashed_pass),
    )
    .map_err(|e| e.to_string())?;

    Ok(hashed_pass)
}

fn secret_password(conn: Rc<Connection>) -> Result<String, rusqlite::Error> {
    let mut stmt = conn.prepare("select value from credentials where name = 'password'")?;
    stmt.query_row([], |row| Ok(row.get(0)?))
}
