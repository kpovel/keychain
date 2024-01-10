use crate::{
    crypto::{encrypt_value, hash_password, verify_password},
    KeyValuePair, SaveCommand,
};
use rusqlite::Connection;
use std::{io, rc::Rc};

#[derive(Debug)]
enum Password {
    Password(String),
    NotMatching,
}

pub fn save_entry(value: SaveCommand, connection: Rc<Connection>) -> Result<(), String> {
    match value.visibility {
        crate::Visibility::Public(v) => save_public_pair(v, connection),
        crate::Visibility::Secret(v) => save_private_pair(v, connection),
    }
}

fn save_public_pair(key_value: KeyValuePair, connection: Rc<Connection>) -> Result<(), String> {
    connection
        .execute(
            "insert into public (key, value) values (?1, ?2)",
            (&key_value.key, &key_value.value),
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn save_private_pair(key_value: KeyValuePair, conn: Rc<Connection>) -> Result<(), String> {
    let hashed_pass = secret_password(Rc::clone(&conn));

    let pass = match hashed_pass {
        Ok(hash) => {
            let pass = verify_user_passowrd(&hash)?;
            match pass {
                Password::Password(pass) => pass,
                Password::NotMatching => return Err(String::from("Passwords are not matching")),
            }
        }
        Err(e) => {
            let new_pass = create_new_password(e, Rc::clone(&conn))?;
            new_pass
        }
    };

    let encrypted_value = encrypt_value(&key_value.value, &pass);

    conn.execute(
        "insert into secret (key, value) values (?1, ?2)",
        (&key_value.key, &encrypted_value),
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

fn verify_user_passowrd(hashed_pass: &str) -> Result<Password, String> {
    let pass = prompt_password().map_err(|e| e.to_string())?;
    match verify_password(&pass, &hashed_pass) {
        Ok(v) if v == true => Ok(Password::Password(pass)),
        Ok(_) => Ok(Password::NotMatching),
        Err(e) => Err(e.to_string()),
    }
}

fn prompt_password() -> Result<String, std::io::Error> {
    println!("Enter your password to verify your identity: ");

    // todo: hide the password
    let mut pass = String::new();
    io::stdin().read_line(&mut pass)?;

    Ok(pass)
}

fn create_new_password(err: rusqlite::Error, conn: Rc<Connection>) -> Result<String, String> {
    if let rusqlite::Error::QueryReturnedNoRows = err {
        let new_pass = create_password().map_err(|e| e.to_string())?;

        match new_pass {
            Password::Password(pass) => {
                save_password(Rc::clone(&conn), &pass)?;
                return Ok(pass);
            }
            Password::NotMatching => Err(String::from("Passwords are not matching")),
        }
    } else {
        Err(err.to_string())
    }
}

fn save_password(conn: Rc<Connection>, pass: &str) -> Result<String, String> {
    let hashed_pass = hash_password(&pass).map_err(|e| e.to_string())?;

    conn.execute(
        "insert into credentials (name, value) values ($1, $2)",
        ("password", &hashed_pass),
    )
    .map_err(|e| e.to_string())?;

    Ok(hashed_pass)
}

fn create_password() -> Result<Password, std::io::Error> {
    println!("Let's create a secret password: ");

    // todo: hide the password
    let mut pass = String::new();
    io::stdin().read_line(&mut pass)?;

    println!("Repeat the password: ");
    let mut pass_repeat = String::new();
    io::stdin().read_line(&mut pass_repeat)?;

    match pass == pass_repeat {
        true => Ok(Password::Password(pass)),
        false => Ok(Password::NotMatching),
    }
}

fn secret_password(conn: Rc<Connection>) -> Result<String, rusqlite::Error> {
    let mut stmt = conn.prepare("select value from credentials where name = 'password'")?;
    stmt.query_row([], |row| Ok(row.get(0)?))
}
