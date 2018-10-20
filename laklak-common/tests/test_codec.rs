extern crate bytes;
extern crate tokio_io;
extern crate laklak_common;

use bytes::BytesMut;
use tokio_io::codec::{Decoder, Encoder};
use laklak_common::codec::*;

#[test]
fn test_encode_pong() {
    let mut c = ChatCodec::new();

    let input = Message::Pong();
    let expected = BytesMut::from("PONG");

    let mut actual = BytesMut::new();
    c.encode(input, &mut actual).unwrap();

    assert_eq!(expected, actual);
}
