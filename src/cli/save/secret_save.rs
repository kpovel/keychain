use crate::{
    cli::KeyValuePair,
    crypto::{encrypt_value, hash_password, verify_password},
};
use rusqlite::Connection;
use std::{io, rc::Rc};

#[derive(Debug)]
enum VerifyPassword {
    Password(String),
    NotMatching,
}

pub fn save_secret_pair(key_value: KeyValuePair, conn: Rc<Connection>) -> Result<(), String> {
    let hashed_pass = secret_password(Rc::clone(&conn));

    let pass = match hashed_pass {
        Ok(hash) => {
            let pass = verify_user_password(&hash)?;
            match pass {
                VerifyPassword::Password(pass) => pass,
                VerifyPassword::NotMatching => {
                    return Err(String::from("Passwords are not matching"))
                }
            }
        }
        Err(e) => {
            if let rusqlite::Error::QueryReturnedNoRows = e {
                let new_pass = create_new_password(Rc::clone(&conn))?;
                new_pass
            } else {
                return Err(e.to_string());
            }
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

fn verify_user_password(hashed_pass: &str) -> Result<VerifyPassword, String> {
    let pass = prompt_password().map_err(|e| e.to_string())?;
    match verify_password(&pass, &hashed_pass) {
        Ok(true) => Ok(VerifyPassword::Password(pass)),
        Ok(false) => Ok(VerifyPassword::NotMatching),
        Err(e) => Err(e.to_string()),
    }
}

fn prompt_password() -> Result<String, io::Error> {
    println!("Enter your password to verify your identity: ");

    // todo: hide the password
    let mut pass = String::new();
    io::stdin().read_line(&mut pass)?;

    Ok(pass)
}

fn create_new_password(conn: Rc<Connection>) -> Result<String, String> {
    let new_pass = create_password().map_err(|e| e.to_string())?;

    match new_pass {
        VerifyPassword::Password(pass) => {
            save_password(Rc::clone(&conn), &pass)?;
            return Ok(pass);
        }
        VerifyPassword::NotMatching => Err(String::from("Passwords are not matching")),
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

fn create_password() -> Result<VerifyPassword, io::Error> {
    println!("Let's create a secret password: ");

    // todo: hide the password
    let mut pass = String::new();
    io::stdin().read_line(&mut pass)?;

    println!("Repeat the password: ");
    let mut pass_repeat = String::new();
    io::stdin().read_line(&mut pass_repeat)?;

    match pass == pass_repeat {
        true => Ok(VerifyPassword::Password(pass)),
        false => Ok(VerifyPassword::NotMatching),
    }
}

fn secret_password(conn: Rc<Connection>) -> Result<String, rusqlite::Error> {
    let mut stmt = conn.prepare("select value from credentials where name = 'password'")?;
    stmt.query_row([], |row| Ok(row.get(0)?))
}
