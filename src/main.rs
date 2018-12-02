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
struct ol_packet {
    opcode: u16,
    #[serde(default)]
    auth: String,
    #[serde(default)]
    justatest: String,
}

#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::*;
use std::fs::File;

#[derive(Serialize, Deserialize)]
struct ol_config {
    port: u16,
    authkey: String,
}

fn loadconfig(location: &str) -> ol_config {
    let conf = ol_config {
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
    let conf: ol_config = loadconfig("conf.json");
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
                                    let v: ol_packet = serde_json::from_str(&p).unwrap();
                                    match v.opcode {
                                        2 => {
                                            /*if v.auth == conf.authkey {
											auth = true;
										} else {
											
										}*/
                                        }
                                        5 => {}
                                        6 => {}
                                        7 => {}
                                        8 => {}
                                        9 => {}
                                        _ => {
                                            warn!("Unknown opcode {} with data {}", v.opcode, p);
                                        }
                                    }
                                    None
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
