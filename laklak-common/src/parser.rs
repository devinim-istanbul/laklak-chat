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

macro_rules! command {
    ($tag:expr => $parent:ident::$name:ident) => {
        named!(cmd_$name<CompleteStr, $parent>,
            do_parse!(
                tag!($tag) >>
                ($parent::$type { })
            )
        );
    };

    (
        $tag:expr => $parent:ident::$name:ident {
            $(
                $field:ident: $type:ident @ $arg_type:ident
            ),*
        }
    ) => {
        named!(cmd_$name<CompleteStr, $parent>,
            do_parse!(
                tag!($tag) >>
                $(char!('|') >> $key: $arg_type)>>+ >>
                ($parent::$type { $($key),+ })
            )
        );
    };

    (pub $tag:expr => $parent:ident::$name:ident) => {
        named!(cmd_$name<CompleteStr, $parent>,
            do_parse!(
                tag!($tag) >>
                ($parent::$type { })
            )
        );
    };

    (
        pub $tag:expr => $parent:ident::$name:ident {
            $(
                $field:ident: $type:ident @ $arg_type:ident
            ),*
        }
    ) => {
        named!(cmd_$name<CompleteStr, $parent>,
            do_parse!(
                tag!($tag) >>
                $(char!('|') >> $key: $arg_type)>>+ >>
                ($parent::$type { $($key),+ })
            )
        );
    };
}

#[macro_export]
macro_rules! command_parsers {
    (
        #[derive($($derive:ident),*)]
        pub enum $parent:ident {
            $(
                [$tag:expr] $name:ident {
                    $([$arg_type:ident] $field:ident: $type:ident),*
                }
            ),*
        }
    ) => {
        #[derive($($derive),*)]
        pub enum $parent {
            $(
                $name {
                    $($field: $type),*
                }
            ),*
        }
    };
}
