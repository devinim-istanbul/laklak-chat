extern crate laklak_common;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate actix;
extern crate tokio;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_codec;
extern crate futures;
extern crate regex;
extern crate nom;
extern crate nanoid;

use actix::prelude::*;
use std::net::{ToSocketAddrs};
use tokio_core::net::{TcpListener};
use futures::Stream;

use laklak_common::settings::{SettingsLoader};

use actors::chat_server::ChatServer;
use actors::messages::*;

mod actors;
mod config;

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
            .map(|(stream, addr)| ConnectionRequest(stream, addr));

        ChatServer::create(|ctx| {
            ctx.add_message_stream(connections);
            ChatServer {}
        });

        println!("Server is now listening at {:?}", addr);
    });

    std::process::exit(exit_code);
}
