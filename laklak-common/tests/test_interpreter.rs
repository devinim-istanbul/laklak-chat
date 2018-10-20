extern crate futures;

use futures::streams::Stream;
use laklak_common::interpreter::*;

#[test]
fn test_interpreter() {
    let input: Stream<u8> = Stream::new();
    let mut interpreter = MessageInterpreter::new();

    interpreter.attach(input);

    std::thread::sleep_ms(5000);
}
