extern crate bytes;

use std::io;
use tokio_io::codec::{Decoder, Encoder};
use bytes::{BufMut, BytesMut};


lazy_static! {
    static ref MESSAGE_SEND: String = "SEND".to_string();
    static ref MESSAGE_PING: String = "PING".to_string();
    static ref MESSAGE_PONG: String = "PONG".to_string();
}

pub enum Message {
    // good old ping
    Ping(),

    // good old pong
    Pong()
}

pub struct ChatCodec;

impl ChatCodec {
    pub fn new() -> ChatCodec { ChatCodec {} }
}

impl Decoder for ChatCodec {
    type Item = Message;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 5 {
            return Ok(None);
        }

        Ok(Some(Message::Ping()))
    }
}

impl Encoder for ChatCodec {
    type Item = Message;
    type Error = io::Error;

    fn encode(&mut self, message: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let as_string: Result<String, Self::Error> = match message {
            Message::Ping() => Ok("PING".to_string()),
            Message::Pong() => Ok("PONG".to_string())
        };

        as_string.map(|s| {
            let msg_ref: &[u8] = s.as_ref();
            dst.put(msg_ref);
            ()
        })
    }
}
