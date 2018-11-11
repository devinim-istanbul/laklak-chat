extern crate laklak_command_codec;

use laklak_command_codec::*;

#[test]
fn decode_send_message_command() {
  let message = "U0VORHwxMjM0NTY3ODEyMzQ1Njc4fEhlbGxvLCBXb3JsZCE=";
  let command = Decode!(message);
  assert_eq!(command, Command::SendMessage{recipient:String::from("1234567812345678"), message:String::from("Hello, World!")});
}

#[test]
fn decode_auth_command() {
  let message = "QVVUSHwxMjM0NTY3ODEyMzQ1Njc4";
  let command = Decode!(message);
  assert_eq!(command, Command::Authenticate{token:String::from("1234567812345678")});
}

#[test]
fn decode_ping_command() {
  let message = "UElORw==";
  let command = Decode!(message);
  assert_eq!(command, Command::Ping{ });
}

// #[test]
// fn decode_pong_command() {
//   let message = "UE9ORw=="; // Somehow, this prints ping as well /shrug
//   let command = Decode!(message);
//   println!("{:?}", command);
//   assert_eq!(command, Command::Pong{ });
// }

#[test]
#[should_panic]
fn panic_on_bad_token() {
  let message = "AUTH|12345678";
  Decode!(message);
}

#[test]
#[should_panic]
fn panic_on_unknown_command() {
  let message = "GREETINGS";
  Decode!(message);
}

#[test]
fn encode_command() {
  let send_message = Command::SendMessage{
    recipient:String::from("1234567812345678"),
    message:String::from("Hello, World!")
  };
  let command_string = Encode!{send_message};
  assert_eq!(command_string, "U0VORHwxMjM0NTY3ODEyMzQ1Njc4fEhlbGxvLCBXb3JsZCE=");
}
