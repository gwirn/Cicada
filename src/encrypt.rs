use orion::kdf;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub fn get_pwd_file(file_path: &str) -> String {
    // get password from file
    let pwd: String = fs::read_to_string(file_path)
        .expect("Couldn't read pwd file")
        .trim()
        .to_string();
    pwd
}

pub fn gen_key_pwd(
    // generate salt and password from password string
    my_pwd: &str,
    salt: orion::kdf::Salt,
) -> (orion::kdf::Salt, orion::aead::SecretKey) {
    let pwd =
        kdf::Password::from_slice(my_pwd.as_bytes()).expect("Error converting password to bytes");
    let derived_key =
        kdf::derive_key(&pwd, &salt, 3, 1 << 16, 32).expect("Couldn't convert password to key");
    (salt, derived_key)
}

pub fn write_bin(text: &Vec<u8>, file_path: &str) {
    // write encrypted text to file
    let mut old_path: String = file_path.to_owned();
    old_path.push_str("_old");
    match fs::copy(&file_path, &old_path) {
        Ok(_) => {
            println!("Copied {} to {}_old file", &file_path, &file_path)
        }
        Err(_) => {
            println!("Didn't copy {} to {}_old file", &file_path, &file_path)
        }
    }
    let mut file = std::fs::File::create(file_path).expect("Couldn't create file");
    file.write_all(&text)
        .expect("Couldn't write content to binary file");
}
pub fn read_bin(file_path: &str) -> Vec<u8> {
    // read binary file
    let mut file = File::open(file_path).expect("Couldn't open binary file");
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer)
        .expect("Couldn't read binary file");
    buffer
}
