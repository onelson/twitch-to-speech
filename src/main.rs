extern crate irc;
extern crate flite;
extern crate rand;

use std::default::Default;
use rand::{thread_rng, Rng};
use irc::client::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use flite::Flite;


fn get_default_config() -> Config {
    return Config {
        nickname: Some(format!("omnbot")),
        alt_nicks: Some(vec![format!("omnbot_"), format!("omnbot__")]),
        server: Some(format!("irc.freenode.net")),
        channels: Some(vec![format!("##omnbot_test")]),
        .. Default::default()
    };
}

fn main() {

    let config_file = std::env::args().nth(1);

    let config = match config_file {
        Some(path) => {
            let fp = std::path::Path::new(&path);
            if !fp.is_file() {
                println!("can't find config file at `{:?}`", fp);
                std::process::exit(-1);
            }
            else {
                println!("loading config from {:?}", fp);
            }
            Config::load(fp).unwrap()
        },
        _ => get_default_config()
    };

    let server = IrcServer::from_config(config).unwrap();
    server.identify().unwrap();

    let tts = Flite::new();
    let voice_list = tts.voices.keys().map(|v| v.clone()).collect::<Vec<String>>();

    let mut rng = thread_rng();

    let mut last_speaker = "".to_string();

    let mut speaker_voices = HashMap::new();

    for message in server.iter() {
        let message = message.unwrap(); // We'll just panic if there's an error.
        print!("{}", message);
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => match message.source_nickname() {
              Some(speaker) => {

                  let voice = speaker_voices
                      .entry(speaker.to_owned())
                      .or_insert_with(|| rng.choose(&voice_list).unwrap());

                  if speaker != last_speaker {
                      println!("{} read by {}", speaker, voice);
                      tts.say(&format!("{} says:", speaker), voice.to_string());
                  }

                  tts.say(&msg, voice.to_string());
                  last_speaker = speaker.to_string();
              },
              _ => ()
            },
            _ => (),
        }
    }
}

