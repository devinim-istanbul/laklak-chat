use super::*;

use super::chat::{ChatActor};
use super::messages::{ChatMessage, Introduction};

type ChatSink = futures::stream::SplitSink<tokio_codec::Framed<tokio_core::net::TcpStream, tokio_codec::LinesCodec>>;

fn serialize_command(cmd: Command) -> Option<String> {
    match cmd {
        Command::Ping {} => Some("PING\n".to_string()),
        Command::Pong {} => Some("PONG\n".to_string()),
        _ => None
    }
}

/**
 * The actor responsible for handling I/O for an individual chat actor.
 */
pub struct ChatIOActor {
    id: String,
    chat_actor: Addr<ChatActor>,
    writer: ChatSink
}

impl ChatIOActor {
    pub fn new(chat_actor: Addr<ChatActor>, writer: ChatSink) -> ChatIOActor {
        ChatIOActor {
            id: nanoid::simple(),
            chat_actor,
            writer
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
            let io_actor = ChatIOActor::new(chat_actor.start(), writer);

            println!("Initializing I/O handler for {:?}", addr);

            io_actor
        })
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
                        self.chat_actor
                            .send(command)
                            .and_then(|response| {
                                println!("Actor replied with {:?}", response);
                                Ok(())
                            });
                    },
                    Err(e) => println!("Failed to parse command string {:?} due to {:?}", msg, e)
                }
            },

            ChatMessage::Outbound(cmd) => {
                let writer = self.writer;
                let as_string = serialize_command(cmd);

                as_string.map(|s| writer.send(s));
            }
        }        
    }
}

impl std::fmt::Debug for ChatIOActor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ChatIOActor #{:?}", self.id)
    }
}
