use bcrypt::{BcryptError, DEFAULT_COST};
use magic_crypt::{new_magic_crypt, MagicCryptError, MagicCryptTrait};

pub fn encrypt_value(value: &str, pass: &str) -> String {
    let mc = new_magic_crypt!(pass, 256);
    mc.encrypt_str_to_base64(value)
}

pub fn decrypt_value(encrypted_string: &str, pass: &str) -> Result<String, MagicCryptError> {
    let mc = new_magic_crypt!(pass, 256);
    let decrypted_val = mc.decrypt_base64_to_string(encrypted_string)?;

    Ok(decrypted_val)
}

pub fn hash_password(pass: &str) -> Result<String, BcryptError> {
    bcrypt::hash(pass, DEFAULT_COST)
}

pub fn verify_password(pass: &str, hash: &str) -> Result<bool, BcryptError> {
    bcrypt::verify(pass, hash)
}
