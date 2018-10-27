extern crate futures;

use futures::prelude::*;
use futures::Stream;
use std::error::Error;

static DELIM_ARGUMENT: u8 = '|' as u8;
static DELIM_MESSAGE: u8 = '\n' as u8;

pub enum Message {
    PING,
    PONG
}

enum State {
    ReadingCommand
}

type TypedStream<'a, T: 'a> = Stream<Item = T, Error = Error> + 'a;
type MessageStream<'a> = TypedStream<'a, Message>;
type ByteStream<'a> = TypedStream<'a, u8>;

pub struct MessageInterpreter {
    is_attached: bool,
    state: State,
    c_buf: Vec<u8>,
    a_buf: Vec<u8>
}

impl MessageInterpreter {
    pub fn new() -> MessageInterpreter {
        MessageInterpreter {
            is_attached: false,
            state: State::ReadingCommand,
            c_buf: Vec::new(),
            a_buf: Vec::new()
        }
    }

    pub fn attach<'a>(&mut self, from: &'a ByteStream, to: &'a MessageStream) -> bool {
        if self.is_attached {
            return false;
        }

        self.is_attached = true;

        //from

        //from.for_each(|b| self.read(b));

        true
    }

    fn read(&mut self, b: u8) {
        match self.state {
            State::ReadingCommand => {
                if b == DELIM_ARGUMENT {
                    println!("yohamuga")
                } else if b == DELIM_MESSAGE {
                    println!("sikerting {:?}", self.c_buf)
                } else {
                    println!("MEMEMRE {:?}", b)
                }
            }
        }
    }
}
