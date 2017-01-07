extern crate irc;
extern crate reqwest;
extern crate rodio;
extern crate url;

use std::default::Default;
use irc::client::prelude::*;
use url::Url;
use std::io::BufReader;

fn say(text: &str, sink: &rodio::Sink) -> () {
    // INPUT_TEXT=Welcome+to+the+world+of+speech+synthesis%21%0A&INPUT_TYPE=TEXT&OUTPUT_TYPE=AUDIO&LOCALE=en_US&AUDIO=AU_STREAM&VOICE=cmu-slt-hsmm&STYLE=HTTP/

    let url_str = "http://localhost:59125/process";
    let mut url = Url::parse(url_str).unwrap();
    url.query_pairs_mut()
        .append_pair("INPUT_TYPE", "TEXT")
        .append_pair("OUTPUT_TYPE", "AUDIO")
        .append_pair("LOCALE", "en_US")
        .append_pair("AUDIO", "WAVE_FILE")
        .append_pair("INPUT_TEXT", text);

    println!("url: {}", url.as_str());

    let mut res = reqwest::get(url.as_str()).unwrap();

    println!("Status: {}", res.status());
    println!("Headers:\n{}", res.headers());

    let mut data = Vec::new();
    ::std::io::copy(&mut res, &mut data).unwrap();

    let source = rodio::Decoder::new(BufReader::new(std::io::Cursor::new(data))).unwrap();
    sink.append(source);

}


fn main() {
    let config = Config {
        nickname: Some(format!("omnbot")),
        alt_nicks: Some(vec![format!("omnbot_"), format!("omnbot__")]),
        server: Some(format!("irc.freenode.net")),
        channels: Some(vec![format!("##destinychat")]),
        .. Default::default()
    };

    let endpoint = rodio::get_default_endpoint().unwrap();
    let sink = rodio::Sink::new(&endpoint);

    let server = IrcServer::from_config(config).unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        let message = message.unwrap(); // We'll just panic if there's an error.
        print!("{}", message);
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => match message.source_nickname() {
              Some(speaker) => say(&format!("{}: {}", speaker, msg), &sink),
              _ => ()
            },
            _ => (),
        }
    }
}

