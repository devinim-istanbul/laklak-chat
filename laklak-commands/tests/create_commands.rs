extern crate laklak_commands;

use laklak_commands::commands::*;
use laklak_commands::codec::*;

#[test]
fn encode_send_message_command() {
  let message = "U0VORHwxMjM0NTY3ODEyMzQ1Njc4fEhlbGxvLCBXb3JsZCE=";
  let command: &Command = &SendMessage::new(String::from("1234567812345678"), String::from("Hello, World!"));
  assert_eq!(
    command.encode(),
    message
  );
}

#[test]
fn encode_auth_command() {
  let message = "QVVUSHwxMjM0NTY3ODEyMzQ1Njc4fA==";
  let command: &Command = &Authenticate::new(String::from("1234567812345678"));
  assert_eq!(
    command.encode(),
    message
  );
}

#[test]
fn encode_ping_command() {
  let message = "UElORw==";
  let command: &Command = &Ping::new();
  assert_eq!(
    command.encode(),
    message
  );
}

#[test]
fn encode_pong_command() {
  let message = "UE9ORw==";
  let command: &Command = &Pong::new();
  assert_eq!(
    command.encode(),
    message
  );
}

#[test]
#[should_panic]
fn panic_on_bad_token() {
  Authenticate::new(String::from("AUTH|12345678"));
}

#[test]
fn decode_command() {
  let decoded_message = decode_base64("U0VORHwxMjM0NTY3ODEyMzQ1Njc4fEhlbGxvLCBXb3JsZCE=");
  assert_eq!(decoded_message, "SEND|1234567812345678|Hello, World!");
}
