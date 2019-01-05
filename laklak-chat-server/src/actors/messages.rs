use super::*;

use actix::dev::{Message, MessageResponse, ResponseChannel};

/**
 * A struct that represents a single chat session.
 */
#[derive(Message)]
#[derive(Debug)]
pub struct ConnectionRequest(pub TcpStream, pub SocketAddr);

/**
 * A struct that represents an incoming chat message.
 */
#[derive(Debug)]
pub struct ChatMessage(pub String);

impl Message for ChatMessage {
    type Result = Option<String>;
}

impl<A, M> MessageResponse<A, M> for ChatMessage
where
    A: Actor,
    M: Message<Result = ChatMessage>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}
