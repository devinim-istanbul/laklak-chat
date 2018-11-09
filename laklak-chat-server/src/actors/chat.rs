use super::*;

use super::chat_io::{ChatIOActor};
use super::messages::{Introduction, ChatMessage};

/**
 * An actor that handles the communication for a single chat session.
 */
pub struct ChatActor {
    id: String,
    addr: SocketAddr,
    io_actor: Option<Addr<ChatIOActor>>
}

impl ChatActor {
    pub fn new(addr: SocketAddr) -> ChatActor {
        ChatActor {
            id: nanoid::simple(),
            addr,
            io_actor: None
        }
    }

    fn reply_with(&mut self, cmd: Command) {
        match self.io_actor {
            Some(ref actor) => { actor.do_send(ChatMessage::Outbound(cmd)); },
            None => println!("I/O Actor not yet met, reply not sent")
        }
    }
}

impl std::fmt::Debug for ChatActor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ChatActor #{:?} @ {:?}", self.id, self.addr)
    }
}

impl Actor for ChatActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("Actor is now handling communication with {:?}", self.addr);
    }
}

impl Handler<Introduction<ChatIOActor>> for ChatActor {
    type Result = ();

    fn handle(&mut self, intro: Introduction<ChatIOActor>, _: &mut Self::Context) {
        self.io_actor = Some(intro.0);
    }
}

impl Handler<Command> for ChatActor {
    type Result = ();

    fn handle(&mut self, cmd: Command, _: &mut Self::Context) {
        println!("Received command {:?}", cmd);

        match cmd {
            Command::Ping {} => self.reply_with(Command::Pong {}),
            Command::Pong {} => self.reply_with(Command::Ping {}),
            _ => ()
        }
    }
}
