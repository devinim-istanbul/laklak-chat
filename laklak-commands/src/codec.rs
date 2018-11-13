extern crate base64;

use base64::{encode, decode};
use std;

pub fn encode_base64(s: &str) -> String {
    encode(s)
}

pub fn decode_base64<'a>(s: &'a str) -> String {
    let r_command = decode(s)
        .map_err(|e| ParserFailure::Base64DecodeError(e))
        .and_then(|buf|
            String::from_utf8(buf).map_err(|e| ParserFailure::Utf8EncodingError(e))
        );

    match r_command {
        Ok(command) => command,
        Err(error) => panic!(error),
    }
}

#[derive(Debug)]
pub enum ParserFailure {
    Base64DecodeError(base64::DecodeError),
    Utf8EncodingError(std::string::FromUtf8Error)
}
