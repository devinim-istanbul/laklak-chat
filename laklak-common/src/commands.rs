use actix::prelude::*;

use actix::dev::{MessageResponse, ResponseChannel};

#[derive(Debug, Message, PartialEq, Eq)]
pub enum ErrorCode {
    InvalidCommand(String),
    Unauthenticated
}

#[derive(Debug, Message, PartialEq, Eq)]
pub enum Command {
    Authenticate {
        token: String
    },

    SendMessage {
        recipient: String,
        message: String
    },

    Ping { },

    Pong { },

    Error(ErrorCode, &'static str)
}


impl<A, M> MessageResponse<A, M> for Command
where
    A: Actor,
    M: Message<Result = Command>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}
