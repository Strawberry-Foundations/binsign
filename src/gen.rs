use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use std::fs;

pub(crate) fn gen_cmd(out: &str) {
    let pkey = generate_key();
    let pem = pkey.private_key_to_pem_pkcs8().expect("Unable to convert key to PEM");

    let mut output_pre: String = out.to_string();

    if !output_pre.ends_with(".pem") {
        output_pre.push_str(".pem");
    }

    let output = &output_pre;

    fs::write(output, &pem).expect("Unable to write key to file");
    println!("Generated certificate and saved to {:?}", output);
}

fn generate_key() -> PKey<Private> {
    let rsa = Rsa::generate(2048).expect("Unable to generate RSA key");
    PKey::from_rsa(rsa).expect("Unable to create PKey")
}
