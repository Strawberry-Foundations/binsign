use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::sign::Verifier;
use openssl::hash::MessageDigest;
use std::fs::File;
use std::io::Read;

pub(crate) fn check_cmd(file: &str, key: &str) {
    if verify(file, key) {
        println!("Match!");
    } else {
        eprintln!("No match!");
    }
}

fn verify(file_path: &str, key_path: &str) -> bool {
    let mut key_file = File::open(key_path).expect("Unable to open key file");
    let mut key_pem = Vec::new();
    key_file.read_to_end(&mut key_pem).expect("Unable to read key file");
    let private_key = Rsa::private_key_from_pem(&key_pem).expect("Unable to load private key");
    let pkey = PKey::from_rsa(private_key).expect("Unable to extract private key");

    let mut file = File::open(file_path).expect("Unable to open file");
    let mut file_contents = Vec::new();
    file.read_to_end(&mut file_contents).expect("Unable to read file");

    let signature_size = 256;
    let data = &file_contents[..file_contents.len() - signature_size];
    let signature = &file_contents[file_contents.len() - signature_size..];

    let mut verifier = Verifier::new(MessageDigest::sha256(), &pkey).expect("Unable to create verifier");
    verifier.update(data).expect("Unable to add data to verifier");

    verifier.verify(signature).expect("Unable to verify signature")
}