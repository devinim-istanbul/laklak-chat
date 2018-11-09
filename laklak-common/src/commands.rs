use actix::prelude::*;

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

    Pong { }
}
