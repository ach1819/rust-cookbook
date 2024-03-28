use std::fs::File;
use std::io::{BufReader, Error, Read, Write};
use std::num::NonZeroU32;

use data_encoding::HEXUPPER;
use ring::digest::{self, Context, Digest, SHA256};
use ring::error::Unspecified;
use ring::rand::{self, SecureRandom};
use ring::{hmac, pbkdf2};

fn main() {
    calculate_sha_256_digist().expect("Error calculating sha 256");
    sign_and_verify_message_hmac().expect("Error signing and verifying the message");
    salt_hash_password_pbkdf2().expect("Error calculating and verifying the password");
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

fn sign_and_verify_message_hmac() -> Result<(), Unspecified> {
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

fn salt_hash_password_pbkdf2() -> Result<(), Unspecified> {
    println!("\nsalt_hash_password_pbkdf2 - starts");
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)?;

    let password = "Guess Me If You Can!";
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    println!("Salt: {}", HEXUPPER.encode(&salt));
    println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));

    let should_secceed = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &pbkdf2_hash,
    );

    let wrong_password = "Definitely not the correct password";
    let should_fail = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        wrong_password.as_bytes(),
        &pbkdf2_hash,
    );

    assert!(should_secceed.is_ok());
    assert!(!should_fail.is_ok());

    Ok(())
}
