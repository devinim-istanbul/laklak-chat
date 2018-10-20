extern crate laklak_common;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate actix;
extern crate tokio;
extern crate tokio_core;

use std::net::{SocketAddr, ToSocketAddrs};
use actix::prelude::*;
use tokio::prelude::*;
use tokio_core::net::{TcpListener, TcpStream};
use laklak_common::settings::{SettingsLoader};

mod config;

#[derive(Message)]
pub struct TcpConnect {
    pub stream: TcpStream,
    pub addr: SocketAddr
}

impl TcpConnect {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> TcpConnect {
        TcpConnect {
            stream: stream,
            addr: addr
        }
    }
}

pub struct Server {}

impl Server {
    pub fn listen(listener: TcpListener) {
        let _ = Server::create(move |ctx| {
            // Handle incoming connection
            ctx.add_message_stream(listener.incoming()
                .map_err(|_| ()).map(|(st, addr)| TcpConnect::new(st, addr)));

            Server {}
        });
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<TcpConnect> for Server {
    type Result = ();

    fn handle(&mut self, mut msg: TcpConnect, ctx: &mut Context<Self>) {
        println!("Accepting connection from {:?}", msg.addr);
        let mut message = String::new();
        msg.stream.read_to_string(&mut message).unwrap();
        println!("Herif sunu dedi: {:?}", message);
    }
}

fn main() {
    let settings: config::Config = SettingsLoader::load().unwrap();
    let listen_url = &format!("{}:{}", settings.tcp.hostname, settings.tcp.port);
    let addr = listen_url.to_socket_addrs().unwrap().next().unwrap();

    let system = System::new("laklak-chat");

    let listener = TcpListener::bind2(&addr).unwrap();
    println!("Chat is now listening on {:?}", addr);

    Server::listen(listener);

    system.run();
}
