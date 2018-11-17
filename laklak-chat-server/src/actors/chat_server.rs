use super::*;

use super::chat_io::{ChatIOActor};
use super::messages::{ConnectionRequest};

/**
 * A global actor that handles incoming connections.
 * Usually spawns a new ChatActor that is responsible for handling the connection request.
 */
pub struct ChatServer;

impl Actor for ChatServer {
    type Context = Context<ChatServer>;
}

impl Handler<ConnectionRequest> for ChatServer {
    type Result = ();

    fn handle(&mut self, session: ConnectionRequest, _: &mut Self::Context) {
        println!("Accepted connection from {:?}, launching a new chat session", session.1);
        ChatIOActor::spawn(session.0, session.1);
    }
}
