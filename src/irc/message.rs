use nom::bytes::complete::{tag, take_until1};
use nom::character::complete::char;
use nom::combinator::opt;
use nom::sequence::terminated;
use nom::IResult;

use super::params::params;
use super::source::{source, Source};
use super::tags::{tags, Tag};

#[derive(Debug, PartialEq)]
pub struct Message {
    pub tags: Option<Vec<Tag>>,
    pub source: Option<Source>,
    pub command: String,
    pub params: Vec<String>,
}

impl Message {
    pub fn parse(i: &str) -> Self {
        let (_, parsed) = message(i).unwrap();
        parsed
    }
}

fn message(i: &str) -> IResult<&str, Message> {
    let (i, tags) = tags(i)?;
    let (i, source) = source(i)?;
    let (i, command) = terminated(take_until1(" "), char(' '))(i)?;
    let (i, params) = params(i)?;
    let (i, _) = opt(tag("\r\n"))(i)?;

    Ok((
        i,
        Message {
            tags,
            source,
            command: String::from(command),
            params,
        },
    ))
}

#[test]
fn test() {
    assert_eq!(
        message(
            ":wolfguy84!wolfguy84@wolfguy84.tmi.twitch.tv PRIVMSG #moonmoon :INFINITE OOBA SAW\r\n"
        ),
        Ok((
            "",
            Message {
                tags: None,
                source: Some(Source {
                    user: String::from("wolfguy84")
                }),
                command: String::from("PRIVMSG"),
                params: vec![String::from("#moonmoon"), String::from("INFINITE OOBA SAW")],
            }
        ))
    )
}
