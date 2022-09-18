use ansi_colours::ansi256_from_rgb;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use crate::irc::hex_color::hex_color;
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
    let reader = BufReader::new(stream.try_clone()?);
    println!("connected");

    stream.write(TWITCH_AUTH_STANZA)?;
    stream.write(TWITCH_CAPS_STANZA)?;
    stream.write(TWITCH_JOIN_STANZA)?;

    for line in reader.lines() {
        let line = line?;
        let message = Message::parse(&line);

        match message.command.as_str() {
            "PING" => {
                println!("{:>15} {}", "ping?", "pong!");
                stream.write(b"PONG :tmi.twitch.tv\r\n")?;
            }
            "JOIN" | "PART" => {
                println!("{:>15} {}", message.command, message.source.unwrap().user);
            }
            "PRIVMSG" => {
                let mut user = String::from(message.source.unwrap().user);
                let mut color = String::from("#FFFFFF");
                let tags = message.tags.unwrap();
                let _tagkeys = tags
                    .iter()
                    .fold(String::new(), |a, e| format!("{}, {}", a, e.key));

                for tag in tags {
                    if tag.key == "display-name" {
                        user = tag.value.unwrap()
                    } else if tag.key == "color" {
                        if let Some(value) = tag.value {
                            color = value;
                        }
                    }
                }
                let ansi_color = ansi256_from_rgb(hex_color(&color));
                println!(
                    "\x1b[0;1;38;5;{ansi_color}m{user:>15}\x1b[0m {message}",
                    message = message.params[1]
                );
            }
            _ => {
                println!("{:>15} {:?}", message.command, message.params)
            }
        }
    }

    Ok(())
}
