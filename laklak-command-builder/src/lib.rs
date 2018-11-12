extern crate base64;

use base64::{encode, decode};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Command<T> {
    pub _marker: PhantomData<T>,
}

#[derive(Debug)]
pub struct SendMessage {
    recipient: String,
    message: String,
}

#[derive(Debug)]
pub struct Authenticate {
    token: String,
}

#[derive(Debug)]
pub struct Ping {}

#[derive(Debug)]
pub struct Pong {}

macro_rules! id_like {
    ($token:expr) => {
        if $token.len() != 16 {
            panic!("Invalid token: {}", $token);
        } else {
            $token
        };
    }
}

impl Command<SendMessage> {
    pub fn build(recipient: String, message: String) -> SendMessage {
        SendMessage { recipient: recipient, message: message }
    }
}

impl Command<Authenticate> {
    pub fn build(token: String) -> Authenticate {
        Authenticate { token: id_like!(token) }
    }
}

impl Command<Ping> {
    pub fn build() -> Ping {
        Ping {}
    }
}

impl Command<Pong> {
    pub fn build() -> Pong {
        Pong {}
    }
}

pub fn encode_base46(s: &str) -> String {
    encode(s)
}

pub fn decode_base64<'a>(s: &'a str) -> String {
    decode(s)
        .map_err(|e| ParserFailure::Base64DecodeError(e))
        .and_then(|buf|
            String::from_utf8(buf).map_err(|e| ParserFailure::Utf8EncodingError(e))
        )
        .unwrap()
}

#[derive(Debug)]
pub enum ParserFailure {
    Base64DecodeError(base64::DecodeError),
    Utf8EncodingError(std::string::FromUtf8Error)
}

// Side note, having structs be responsible from how they are encoded seems logical.
// As for decoding, I think a respective actor should be spawn right after decode,
// without any exposure to the rest of the code to keep side-effects in check.
// Thus we can move on with the spawned actor forwarding the result to its respective queue.
