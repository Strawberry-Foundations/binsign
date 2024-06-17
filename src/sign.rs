use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::sign::Signer;

pub(crate) fn sign_cmd(file: &str, key: &str) {
    let pkey = load_key_from_pem(key);

    sign(file, pkey.as_ref());
}

fn load_key_from_pem(pem_path: &str) -> Option<PKey<Private>> {
    let mut file = File::open(pem_path).expect("Unable to open PEM file");
    let mut pem = Vec::new();

    file.read_to_end(&mut pem).expect("Unable to read PEM file");
    PKey::private_key_from_pem(&pem).ok()
}

fn sign(file_path: &str, pkey: Option<&PKey<Private>>) {
    // Check params
    if file_path.is_empty() {
        eprintln!("ERROR: No file provided");
        return;
    }
    let pkey = match pkey {
        Some(pkey) => pkey,
        None => {
            eprintln!("ERROR: No private key provided");
            return;
        }
    };

    // Read file
    let mut data = Vec::new();
    let mut file_to_read = File::open(file_path).expect("Unable to open file");
    file_to_read.read_to_end(&mut data).expect("Unable to read file");

    // Create Signer
    let mut signer = Signer::new(MessageDigest::sha256(), pkey).expect("Unable to create signer");
    signer.update(&data).expect("Unable to add file data to signer");

    // Sign
    let signature = signer.sign_to_vec().expect("Unable to sign");

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .expect("Unable to open file");

    file.write_all(&signature).expect("Unable to write signature");
    println!("\"{}\" has been signed successfully", file_path);
}