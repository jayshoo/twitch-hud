use nom::character::complete::char;
use nom::{bytes::complete::take_while1, combinator::opt, sequence::preceded, IResult};

fn is_host_char(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '-')
}

#[derive(Debug, PartialEq)]
pub struct Source {
    pub user: String, // FOR NOW
}

fn parse(i: &str) -> IResult<&str, &str> {
    let mut user_part = opt(preceded(char('!'), take_while1(is_host_char)));
    let mut host_part = opt(preceded(char('@'), take_while1(is_host_char)));

    let (i, _) = char(':')(i)?;
    let (i, user) = take_while1(is_host_char)(i)?;
    let (i, _) = user_part(i)?;
    let (i, _) = host_part(i)?;
    let (i, _) = char(' ')(i)?;

    Ok((i, user))
}

pub fn source(i: &str) -> IResult<&str, Option<Source>> {
    opt(parse)(i).map(|(i, res)| {
        (
            i,
            res.map(|user| Source {
                user: String::from(user),
            }),
        )
    })
}

#[test]
fn test() {
    assert_eq!(
        source(":wolfguy84!wolfguy84@wolfguy84.tmi.twitch.tv "),
        Ok(("", Some(Source{ user: String::from("wolfguy84") })))
    )
}
