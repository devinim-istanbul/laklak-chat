use super::*;

/**
 * An actor that handles the communication for a single chat session.
 */
pub struct ChatActor {
    id: String,
    addr: SocketAddr
}

impl ChatActor {
    pub fn new(addr: SocketAddr) -> ChatActor {
        ChatActor {
            id: nanoid::simple(),
            addr
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

impl Handler<Command> for ChatActor {
    type Result = Command;

    fn handle(&mut self, cmd: Command, _: &mut Self::Context) -> Command {
        println!("Received command {:?}", cmd);

        match cmd {
            Command::Ping {} => Command::Pong {},
            Command::Pong {} => Command::Ping {},
            _ => {
                let error_code = ErrorCode::InvalidCommand("asda".to_string());
                Command::Error(error_code, "Invalid command string")
            }
        }
    }
}
