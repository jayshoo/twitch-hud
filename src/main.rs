use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use crate::irc::message::Message;

mod irc;

const TWITCH_IRC_ADDRESS: &str = "irc.chat.twitch.tv:6667";
const TWITCH_CAPS_STANZA: &[u8] =
    b"CAP REQ :twitch.tv/tags twitch.tv/membership twitch.tv/commands\r\n";
const TWITCH_AUTH_STANZA: &[u8] = b"PASS justinfan0\r\nNICK justinfan0\r\n";
const TWITCH_JOIN_STANZA: &[u8] = b"JOIN #admiralbahroo\r\nJOIN #LEC\r\nJOIN #auronplay\r\nJOIN #ZeratoR\r\nJOIN #LVPes\r\nJOIN #tarik\r\nJOIN #VALORANT\r\n";

fn main() -> anyhow::Result<()> {
    println!("connecting");
    let mut stream = TcpStream::connect(TWITCH_IRC_ADDRESS)?;
    println!("connected");

    stream.write(TWITCH_AUTH_STANZA)?;
    stream.write(TWITCH_CAPS_STANZA)?;
    stream.write(TWITCH_JOIN_STANZA)?;

    let reader = BufReader::new(stream);
    for line in reader.lines() {
        let line = line?;
        let message = Message::parse(&line);
        println!("{:#?}", message);
    }

    Ok(())
}
