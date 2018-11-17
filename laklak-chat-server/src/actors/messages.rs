use super::*;

// CHAT I/O
#[derive(Message)]
#[derive(Debug)]
pub enum ChatMessage {
    Inbound(String),
    Outbound(Command)
}

#[derive(Message)]
#[derive(Debug)]
pub struct Introduction<A>(pub Addr<A>) where A: Actor;

/**
 * A struct that represents a single chat session.
 */
#[derive(Message)]
#[derive(Debug)]
pub struct ConnectionRequest(pub TcpStream, pub SocketAddr);
