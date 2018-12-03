/*-------------------------------------------------------------------------------

# Copyright (C) 2018 Opuslink development team
# 
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.
# 
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
# 
# You should have received a copy of the GNU General Public License along
# with this program; if not, write to the Free Software Foundation, Inc.,
# 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
-------------------------------------------------------------------------------*/
extern crate futures;
extern crate tokio_core;
extern crate websocket;

use std::fmt::Debug;

use websocket::async::Server;
use websocket::message::{Message, OwnedMessage};
use websocket::server::InvalidConnection;

use futures::{Future, Sink, Stream};
use tokio_core::reactor::{Core, Handle};

extern crate serde;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Error;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum OlPacket {
    Play {
        opcode: u16,
        #[serde(default)]
        auth: String,
        #[serde(default)]
        justatest: String
    },
    Default {
        opcode: u16,
        #[serde(default)]
        auth: String
    }
}


#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::*;
use std::fs::File;

#[derive(Serialize, Deserialize)]
struct OlConfig {
    port: u16,
    authkey: String,
}

fn loadconfig(location: &str) -> OlConfig {
    let conf = OlConfig {
        port: 8080,
        authkey: String::from("catsrkewl"),
    };

    return conf;
}
fn generateauthpacket() -> String {
    let packet = json!({
		"opcode": 1
	});
    return packet.to_string();
}

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
        WriteLogger::new(
            LevelFilter::Warn,
            Config::default(),
            File::create("opuslink.log").unwrap(),
        ),
    ]).unwrap();

    info!("×××××××××××××××××××××××××××××××××××××××××××××××××××");
    info!("  __  ____  _  _  ____  __    __  __ _  __ _ ");
    info!(" /  \\(  _ \\/ )( \\/ ___)(  )  (  )(  ( \\(  / )");
    info!("(  O )) __/) \\/ (\\___ \\/ (_/\\ )( /    / )  ( ");
    info!(" \\__/(__)  \\____/(____/\\____/(__)\\_)__)(__\\_)");
    info!(
        "Version {} | by untocodes & dondish               ",
        env!("CARGO_PKG_VERSION")
    );
    info!("×××××××××××××××××××××××××××××××××××××××××××××××××××");
    info!("Loading config");
    let conf: OlConfig = loadconfig("conf.json");
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let ipport = ["127.0.0.1:", &conf.port.to_string()].join("");
    let server = Server::bind(ipport, &handle).unwrap();
    info!("Websocket server started on port 8080");
    let f = server
        .incoming()
        .map_err(|InvalidConnection { error, .. }| error)
        .for_each(|(upgrade, addr)| {
            info!("Got a connection from: {}", addr);
            let auth: bool = false;
            let f = upgrade
                .accept()
                .and_then(|(s, _)| s.send(Message::text(generateauthpacket()).into()))
                .and_then(|s| {
                    let (sink, stream) = s.split();
                    stream
                        .take_while(|m| Ok(!m.is_close()))
                        .filter_map(|m| {
                            match m {
                                OwnedMessage::Text(p) => {
                                    info!("{}", p);
                                    let x: Result<OlPacket, Error> = serde_json::from_str(&p);
                                    match x {
                                        Ok(v) => {
                                            match v {
                                                OlPacket::Play { opcode: 2, auth, justatest } => {
//                                                    if auth == conf.authkey {
//                                                        info!("Good muhahaha");
//                                                    }
                                                    info!("works haha yes")
                                                },
                                                OlPacket::Default {opcode: 5, auth } => {},
                                                OlPacket::Default {opcode: 6, auth } => {},
                                                OlPacket::Default {opcode: 7, auth } => {},
                                                OlPacket::Default {opcode: 8, auth } => {},
                                                OlPacket::Default {opcode: 9, auth } => {},
                                                OlPacket::Default {opcode, auth} => {
                                                    warn!("Unknown opcode {} with data {}", opcode, p);
                                                },
                                                _ => {}
                                            };
                                            None
                                        },
                                        Err(_) => None
                                    }
                                }
                                _ => Some(m),
                            }
                        }).forward(sink)
                        .and_then(|(_, sink)| sink.send(OwnedMessage::Close(None)))
                });

            spawn_future(f, "Client Status", &handle);
            Ok(())
        });

    core.run(f).unwrap();
}

fn spawn_future<F, I, E>(f: F, desc: &'static str, handle: &Handle)
where
    F: Future<Item = I, Error = E> + 'static,
    E: Debug,
{
    handle.spawn(
        f.map_err(move |e| println!("{}: '{:?}'", desc, e))
            .map(move |_| println!("{}: Finished.", desc)),
    );
}
