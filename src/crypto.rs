use bcrypt::{BcryptError, DEFAULT_COST};
use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use std::error::Error;

pub fn encrypt_value(mcrypt: &MagicCrypt256, pass: &str) -> Vec<u8> {
    mcrypt.encrypt_str_to_bytes(pass)
}

pub fn decrypt_value(
    mcrypt: &MagicCrypt256,
    encrypted_string: &Vec<u8>,
) -> Result<String, Box<dyn Error>> {
    let decrypted_bytes = mcrypt.decrypt_bytes_to_bytes(&encrypted_string)?;
    let decrypted_str = std::str::from_utf8(&decrypted_bytes)?;

    Ok(decrypted_str.to_string())
}

pub fn hash_password(pass: &str) -> Result<String, BcryptError> {
    bcrypt::hash(pass, DEFAULT_COST)
}

pub fn verify_password(pass: &str, hash: &str) -> Result<bool, BcryptError> {
    bcrypt::verify(pass, hash)
}
