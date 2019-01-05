use super::*;

use std::io;
use failure::Fail;

use super::chat::{ChatActor};
use super::messages::*;

#[derive(Debug, Fail)]
pub enum ChatIOError {
    #[fail(display = "Failed to read from stream")]
    NetworkReadError,

    #[fail(display = "Failed to parse command string")]
    ParseFailure,

    #[fail(display = "Internal communication error")]
    MailboxError
}

impl ChatIOError {
    pub fn as_io_error(self) -> io::Error {
        let kind = match self {
            ChatIOError::NetworkReadError => io::ErrorKind::BrokenPipe,
            ChatIOError::ParseFailure => io::ErrorKind::InvalidInput,
            ChatIOError::MailboxError => io::ErrorKind::Other
        };

        io::Error::new(kind, "asda".to_string())
    }
}

fn serialize_command(cmd: Command) -> String {
    match cmd {
        Command::Ping {} => "PING\n".to_string(),
        Command::Pong {} => "PONG\n".to_string(),
        _ => "NOPE".to_string()
    }
}

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

        let codec = LinesCodec::new_with_max_length(65535);

        let (writer, reader) = codec.framed(session.0).split();

        let chat_actor = ChatActor::new(session.1).start();

        let f = &mut reader
                .map_err(|e| ChatIOError::NetworkReadError)
                .map(move |st| ChatMessage(st))
                .and_then(|msg|
                    parse_command(CompleteStr(&msg.0))
                        .map_err(|e| ChatIOError::ParseFailure)
                        .and_then(|(_, command)| chat_actor.send(command).wait().map_err(|e| ChatIOError::MailboxError))
                        .map(|response| serialize_command(response))
                )
                .map_err(|e| e.as_io_error());
        
        f.for_each(|asd| println!("asdasd {:?}", asd));
        
    }
}
