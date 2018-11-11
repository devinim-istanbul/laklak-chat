extern crate base64;

use base64::{encode, decode};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Authenticate {
        token: String
    },

    SendMessage {
        recipient: String,
        message: String
    },

    Ping { },

    Pong { }
}

#[macro_export]
macro_rules! Decode {
    (SEND;$recipient:expr;$message:expr) => {{
        Command::SendMessage { recipient: $recipient, message: $message }
    }};
    (AUTH;$token:expr) => {{
        Command::Authenticate { token: id_like!($token) }
    }};
    (PING) => {{ Command::Ping {} }};
    (PONG) => {{ Command::Pong {} }};
    ($command:expr) => {{
        let command = decode_base64($command);
        let parts: Vec<&str> = command.split("|").collect();
        let result: Command = match parts[0] {
            "SEND" => Decode!{SEND;parts[1].to_string();parts[2].to_string()},
            "AUTH" => Decode!{AUTH;parts[1].to_string()},
            "PING" => Decode!{PING},
            "PONG" => Decode!{PING},
            undefined_command => panic!("Undefined command: {}", undefined_command),
        };

        result
    }};
}

#[macro_export]
macro_rules! Encode {
    ($command:expr) => {{
        match $command {
            Command::SendMessage{recipient, message} => encode_base46(&format!("SEND|{}|{}", recipient, message)),
            Command::Authenticate{token} => encode_base46(&format!("AUTH|{}", token)),
            Command::Ping{} => encode_base46(&format!("PING")),
            Command::Pong{} => encode_base46(&format!("PONG")),
        }
    }};
}

#[macro_export]
macro_rules! id_like {
    ($token:expr) => {
        if $token.len() != 16 {
            panic!("Invalid token: {}", $token);
        } else {
            $token
        };
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
