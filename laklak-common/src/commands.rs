use actix::prelude::*;

use actix::dev::{MessageResponse, ResponseChannel};

use super::parser::*;

#[derive(Debug, Message, PartialEq, Eq)]
pub enum ErrorCode {
    InvalidCommand(String),
    Unauthenticated
}

command_parsers!(
    #[derive(Debug, Message, PartialEq, Eq)]
    pub enum Command {
        ["PING"] Ping { },

        ["PONG"] Pong { },

        ["AUTH"] Authenticate {
            [arg_idlike] token: String
        },

        ["SEND"] SendMessage {
            [arg_idlike] recipient: String,
            [arg_base64_encoded] message: String
        }
    }
);


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
