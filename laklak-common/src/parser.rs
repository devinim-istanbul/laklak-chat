extern crate std;
extern crate base64;
extern crate nom;

use nom::types::CompleteStr;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

use commands::*;

lazy_static! {
    static ref VALID_BASE64_CHARS: HashMap<char, bool> = {
        let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
        let mut map: HashMap<char, bool> = HashMap::new();
        charset.chars().for_each(|c| {
            map.insert(c, true);
        });
        map
    };
}

#[derive(Debug, Fail)]
pub enum ParserFailure {
    #[fail(display = "Failed to decode input using Base64 due to {}", _0)]
    Base64DecodeError(#[fail(cause)] base64::DecodeError),

    #[fail(display = "Failed to decode input as a valid UTF-8 string {}", _0)]
    Utf8EncodingError(#[fail(cause)] std::string::FromUtf8Error)
}

fn decode_base64(input: CompleteStr) -> Result<String, ParserFailure> {
    base64::decode(input.as_bytes())
        .map_err(|e| ParserFailure::Base64DecodeError(e))
        .and_then(|buf|
            String::from_utf8(buf).map_err(|e| ParserFailure::Utf8EncodingError(e))
        )
}

fn from_hex(input: CompleteStr) -> Result<String, std::num::ParseIntError> {
    Ok(input.to_string())
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn is_valid_base64_char(c: char) -> bool {
    VALID_BASE64_CHARS.contains_key(&c)
}

named_args!(typ_hex(min_len: usize, max_len: usize)<CompleteStr, String>,
    map_res!(take_while_m_n!(min_len, max_len, is_hex_digit), from_hex)
);

named!(arg_idlike<CompleteStr, String>, call!(typ_hex, 16, 16));

named!(arg_base64_encoded<CompleteStr, String>,
    map_res!(take_while_m_n!(1, 65536, is_valid_base64_char), decode_base64)
);

named!(pub cmd_ping<CompleteStr, Command>,
    do_parse!(
        tag!("PING") >>
        (Command::Ping { })
    )
);

named!(pub cmd_pong<CompleteStr, Command>,
    do_parse!(
        tag!("PONG") >>
        (Command::Pong { })
    )
);

named!(pub cmd_authenticate<CompleteStr, Command>,
    do_parse!(
        tag!("AUTH") >>
        char!('|') >>
        token: arg_idlike >>
        (Command::Authenticate { token })
    )
);

named!(pub cmd_send_message<CompleteStr, Command>,
    do_parse!(
        tag!("SEND") >>
        char!('|') >>
        recipient: arg_idlike >>
        char!('|') >>
        message: arg_base64_encoded >>
        (Command::SendMessage { recipient, message })
    )
);

named!(pub parse_command<CompleteStr, Command>,
    alt!(
        complete!(cmd_authenticate) |
        complete!(cmd_send_message) |
        complete!(cmd_ping) |
        complete!(cmd_pong)
    )
);
