use std::fs::File;
use std::io::{BufReader, Error, Read, Write};

use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use ring::error::Unspecified;
use ring::hmac;
use ring::rand::{self, SecureRandom};

fn main() {
    calculate_sha_256_digist().expect("Error calculating sha 256");
    sign_and_verify_message_hmac().expect("Error signing and verifying the message");
}
fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Error> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    Ok(context.finish())
}

fn calculate_sha_256_digist() -> Result<(), Error> {
    println!("calculate_sha_256_digist - starts");
    let path = std::env::var("FILE_PATH").unwrap_or(String::from(""));

    if path == "" {
        println!("No file path!");
        return Ok(());
    }

    let mut output = File::create(&path)?;
    write!(output, "We will generate a digest of this text")?;

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));

    Ok(())
}

fn sign_and_verify_message_hmac()  -> Result<(), Unspecified>{
    println!("\nsign_and_verify_message_hmac - starts");
    let mut key_value = [0u8; 48];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value)?;
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    let message = "Legitimate and important message.";
    let signature = hmac::sign(&key, message.as_bytes());
    hmac::verify(&key, message.as_bytes(), signature.as_ref())?;
    println!("sign_and_verify_message_hmac - OK");
    Ok(())
}
