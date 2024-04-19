use std::str::Utf8Error;

use percent_encoding::{percent_decode, utf8_percent_encode, AsciiSet, CONTROLS};
use url::form_urlencoded::{byte_serialize, parse};

fn main() {
    percent_encode_string().unwrap();
    encode_string_www_form_urlencoded();
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
