use std::str::{self, Utf8Error};

use base64::{decode, encode};
use data_encoding::HEXUPPER;
use percent_encoding::{percent_decode, utf8_percent_encode, AsciiSet, CONTROLS};
use url::form_urlencoded::{byte_serialize, parse};

fn main() {
    percent_encode_string().unwrap();
    encode_string_www_form_urlencoded();
    encode_and_decode_hex();
    encode_decode_base64();
}

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
fn percent_encode_string() -> Result<(), Utf8Error> {
    println!("\npercent_encode_string - starts");
    let input = "confident, productive systems programming";

    let iter = utf8_percent_encode(input, FRAGMENT);
    let encoded: String = iter.collect();
    assert_eq!(encoded, "confident,%20productive%20systems%20programming");

    let iter = percent_decode(encoded.as_bytes());
    let decoded = iter.decode_utf8()?;
    assert_eq!(decoded, "confident, productive systems programming");

    println!("percent_encode_string - OK");

    Ok(())
}

fn encode_string_www_form_urlencoded() {
    println!("\npercent_encode_string - starts");

    let urlencoded: String = byte_serialize("What is ❤?".as_bytes()).collect();
    assert_eq!(urlencoded, "What+is+%E2%9D%A4%3F");
    println!("urlencoded: '{}'", urlencoded);

    let decoded: String = parse(urlencoded.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    assert_eq!(decoded, "What is ❤?");
    println!("decoded: '{}'", decoded);

    println!("percent_encode_string - OK");
}

fn encode_and_decode_hex () {
    println!("\nencode_and_decode_hex - starts");
    let original = b"The quick brown fox jumps over the lazy dog";
    let expected = "54686520717569636B2062726F776E20666F78206A756D7073206F76657220746865206C617A7920646F67";

    let encoded = HEXUPPER.encode(original);
    assert_eq!(encoded, expected);

    let decode = HEXUPPER.decode(&encoded.into_bytes()).unwrap();
    assert_eq!(&decode[..], &original[..]);

    println!("encode_and_decode_hex - OK");
}

fn encode_decode_base64() {
    println!("\nencode_decode_base64 - starts");
    let hello = b"hello rustaceans";
    let encoded = encode(hello);
    let decoded = decode(&encoded).unwrap();

    println!("origin: {}", str::from_utf8(hello).unwrap());
    println!("base64 encoded: {}", encoded);
    println!("back to origin: {}", str::from_utf8(&decoded).unwrap());

    println!("encode_decode_base64 - OK");
}
