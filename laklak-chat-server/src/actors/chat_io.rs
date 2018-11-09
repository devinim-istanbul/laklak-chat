use super::*;

use super::chat::{ChatActor};
use super::messages::{ChatMessage, Introduction};

/**
 * The actor responsible for handling I/O for an individual chat actor.
 */
pub struct ChatIOActor {
    id: String,
    chat_actor: Addr<ChatActor>
}

impl ChatIOActor {
    pub fn new(chat_actor: Addr<ChatActor>) -> ChatIOActor {
        ChatIOActor {
            id: nanoid::simple(),
            chat_actor
        }
    }

    pub fn spawn(stream: TcpStream, addr: SocketAddr) -> Addr<ChatIOActor> {
        ChatIOActor::create(move |ctx| {
            let codec = LinesCodec::new_with_max_length(65535);

            let (writer, reader) = codec.framed(stream).split();
            
            let message_stream = reader
                .map_err(|e| println!("Failed to read from stream {:?}", e))
                .map(move |st| ChatMessage::Inbound(st));

            ctx.add_message_stream(message_stream);

            let chat_actor = ChatActor::new(addr);
            let io_actor = ChatIOActor::new(chat_actor.start());

            println!("Initializing I/O handler for {:?}", addr);

            io_actor
        })
    }
    
    fn reply_with(&mut self, cmd: Command) {
        //self.writer.send("PING\n".to_string());
    }
}

impl Actor for ChatIOActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.chat_actor.do_send(Introduction(addr));
    }
}

impl Handler<ChatMessage> for ChatIOActor {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, _: &mut Self::Context) {
        match msg {
            ChatMessage::Inbound(msg) => {
                match parse_command(CompleteStr(&msg)) {
                    Ok(result) => {
                        let command = result.1;
                        self.chat_actor.do_send(command)
                    },
                    Err(e) => println!("Failed to parse command string {:?} due to {:?}", msg, e)
                }
            },

            ChatMessage::Outbound(cmd) => {
                self.reply_with(cmd);
            }
        }        
    }
}

impl std::fmt::Debug for ChatIOActor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ChatIOActor #{:?}", self.id)
    }
}
