extern crate laklak_common;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate actix;
extern crate tokio;
extern crate tokio_core;
extern crate tokio_codec;
extern crate futures;
extern crate regex;

use std::io::{BufReader, BufRead};
//use std::net::{SocketAddr, ToSocketAddrs, TcpListener, TcpStream};
use std::net::{SocketAddr, ToSocketAddrs};
use std::error::Error;
use std::ops::Range;
use tokio_core::net::{TcpListener, TcpStream};
use tokio_codec::{FramedRead, LinesCodec};
use laklak_common::settings::{SettingsLoader};
use actix::prelude::*;
use futures::{Stream, Future};
use regex::{Regex, RegexSet};

mod config;

// CHAT ACTOR
static PATTERN_SEND: &'static str = r"(SEND)\|([\d+]{16})";
static PATTERN_AUTH: &'static str = r"(AUTH)\|([A-Z\d+]{16})";

static PATTERNS: &'static [&'static str] = &[
    PATTERN_SEND,
    PATTERN_AUTH
];

lazy_static! {
    static ref RE_ALL: RegexSet = RegexSet::new(PATTERNS).unwrap();
}

#[derive(Message)]
#[derive(Debug)]
enum Command {
    SendMessage,
    Authenticate
}

#[derive(Message)]
#[derive(Debug)]
struct SendMessage {
    sender: String,
    recipient: String,
    content: String
}

#[derive(Message)]
#[derive(Debug)]
struct Authenticate {
    token: String
}

/**
 * An actor that handles the communication for a single chat session.
 */
#[derive(Debug)]
struct ChatActor {
    addr: SocketAddr,
    io_actor: Option<Addr<ChatIOActor>>
}

impl Actor for ChatActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Actor is now handling communication with {:?}", self.addr);
    }
}

impl Handler<Command> for ChatActor {
    type Result = ();

    fn handle(&mut self, cmd: Command, _: &mut Self::Context) {
        println!("Received command {:?}", cmd);

        let keke = &[PATTERN_SEND, PATTERN_AUTH];
    }
}

/**
 * The actor responsible for handling I/O for an individual chat actor.
 */
#[derive(Debug)]
struct ChatIOActor {
    chat_actor: Addr<ChatActor>
}



// CHAT I/O
#[derive(Message)]
#[derive(Debug)]
struct InboundMessage(String);

#[derive(Message)]
#[derive(Debug)]
struct Introduction(Addr<ChatIOActor>);

impl ChatIOActor {
    pub fn spawn(stream: TcpStream, addr: SocketAddr) -> Addr<ChatIOActor> {
        ChatIOActor::create(move |ctx| {
            let codec = LinesCodec::new_with_max_length(65535);
            let buf_reader = BufReader::new(stream);
            let reader = FramedRead::new(buf_reader, codec);

            let message_stream = reader
                .map_err(|e| println!("Failed to read from stream {:?}", e))
                .map(move |st| InboundMessage(st));

            ctx.add_message_stream(message_stream);

            let chat_actor = ChatActor { io_actor: None, addr: addr }.start();

            println!("Initialized I/O handler for {:?}", addr);

            ChatIOActor { chat_actor: chat_actor }
        })
    }
}

fn parse_command<'s>(command_string: &'s String) -> Result<Option<Command>, String> {
    match RE_ALL.matches(command_string).into_iter().next() {
        Some(idx) => {
            let matched_pattern = PATTERNS[idx];

            match Regex::new(matched_pattern).unwrap().captures(command_string) {
                Some(captures) => {
                    let command_key = captures.get(1).unwrap().as_str();
                    let arg_indices = [2..command_key.len()];
                    println!("KEKEKEKE {:?}", command_key);
                    Ok(None)
                },
                None => Err("Parser failure".to_string())
            }
        },
        None => Err("Invalid command string".to_string())
    }
}

impl Handler<InboundMessage> for ChatIOActor {
    type Result = ();

    fn handle(&mut self, msg: InboundMessage, _: &mut Self::Context) {
        let msg = &msg.0;
        let command = parse_command(msg);
        println!("Received command: {:?}", command);
    }
}

impl Actor for ChatIOActor {
    type Context = Context<Self>;
}



// CHAT SERVER
/**
 * A struct that represents a single chat session.
 */
#[derive(Message)]
#[derive(Debug)]
struct ConnectionRequest {
    stream: TcpStream,
    addr: SocketAddr
}

impl ConnectionRequest {
    fn new(stream: TcpStream, addr: SocketAddr) -> ConnectionRequest {
        ConnectionRequest { stream: stream, addr: addr }
    }
}

/**
 * A global actor that handles incoming connections.
 * Usually spawns a new ChatActor that is responsible for handling the connection request.
 */
struct ChatServer;

impl Actor for ChatServer {
    type Context = Context<ChatServer>;
}

impl Handler<ConnectionRequest> for ChatServer {
    type Result = ();

    fn handle(&mut self, session: ConnectionRequest, ctx: &mut Self::Context) {
        println!("Accepted connection from {:?}, launching a new chat session", session.addr);
        ChatIOActor::spawn(session.stream, session.addr);
    }
}



fn main() {
    let settings: config::Config = SettingsLoader::load().unwrap();
    let listen_url = &format!("{}:{}", settings.tcp.hostname, settings.tcp.port);
    let addr = listen_url.to_socket_addrs().unwrap().next().unwrap();

    let listener = TcpListener::bind2(&addr).expect("Failed to listen");

    let exit_code = System::run(move || {
        let connections = listener.incoming()
            .map_err(|e| {
                println!("A connection attempt has failed {:?}", e);
            })
            .map(|(stream, addr)| {
                ConnectionRequest::new(stream, addr)
            });

        ChatServer::create(|ctx| {
            ctx.add_message_stream(connections);
            ChatServer {}
        });

        println!("Server is now listening at {:?}", addr);
    });

    std::process::exit(exit_code);
}
