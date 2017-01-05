extern crate irc;

use std::default::Default;
use irc::client::prelude::*;

fn main() {
    let config = Config {
        nickname: Some(format!("omnbot")),
        alt_nicks: Some(vec![format!("omnbot_"), format!("omnbot__")]),
        server: Some(format!("irc.freenode.net")),
        channels: Some(vec![format!("##destinychat")]),
        .. Default::default()
    };
    let server = IrcServer::from_config(config).unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        let message = message.unwrap(); // We'll just panic if there's an error.
        print!("{}", message);
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => match message.source_nickname() {
              Some(speaker) => println!("{} is speaking", speaker),
              _ => ()
            },
            _ => (),
        }
    }
}

