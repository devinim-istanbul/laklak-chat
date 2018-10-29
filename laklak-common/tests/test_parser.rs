extern crate laklak_common;
extern crate nom;

use nom::types::CompleteStr;

use laklak_common::commands::*;
use laklak_common::parser::*;

#[test]
fn parse_authenticate_valid_string() {
    let input = CompleteStr("AUTH|ABCD1234EFAB6789");
    let expected = Command::Authenticate { token: "ABCD1234EFAB6789".to_string() };

    let actual = cmd_authenticate(input).unwrap().1;

    assert_eq!(expected, actual);
}

#[test]
#[should_panic]
fn test_parse_authenticate_invalid_string() {
    let input = CompleteStr("AUTH");

    cmd_authenticate(input).unwrap();
}

#[test]
fn test_parse_send_message_valid_input() {
    let input = CompleteStr("SEND|1234567812345678|Zm9vIGJhcg==");
    let expected = Command::SendMessage {
        recipient: "1234567812345678".to_string(),
        message: "foo bar".to_string()
    };

    let actual = cmd_send_message(input).unwrap().1;

    assert_eq!(expected, actual);
}

#[test]
fn test_parse_random_valid_input() {
    let input = CompleteStr("PING");
    let expected = Command::Ping {};
    let not_expected = Command::Pong {};

    let actual = parse_command(input).unwrap().1;

    assert_ne!(not_expected, actual);
    assert_eq!(expected, actual);
}
