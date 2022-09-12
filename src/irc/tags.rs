use nom::bytes::complete::{is_a, is_not};
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair};
use nom::{bytes::complete::take_while1, combinator::opt, sequence::preceded, IResult};

#[derive(Debug, PartialEq)]
pub struct Tag {
    key: String,
    value: Option<String>,
}

fn host(i: &str) -> IResult<&str, &str> {
    let is_host_char = |c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '-');
    take_while1(is_host_char)(i)
}

fn escaped_value(i: &str) -> IResult<&str, &str> {
    is_not("\0\r\n; ")(i)
}

fn key(i: &str) -> IResult<&str, &str> {
    // HACK: don't validate, just slurp the optionals <client_prefix> and <vendor/>
    let is_key_char = |c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '.' | '+' | '/');
    take_while1(is_key_char)(i)
}

fn key_try2(i: &str) -> IResult<&str, &str> {
    // HACK: don't validate, just slurp the optionals <client_prefix> and <vendor/>
    is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789./-+")(i)
}

fn tag(i: &str) -> IResult<&str, Tag> {
    pair(key, opt(preceded(char('='), escaped_value)))(i).map(|(i, res)| {
        (
            i,
            Tag {
                key: String::from(res.0),
                value: res.1.map(String::from),
            },
        )
    })
}

pub fn tags(i: &str) -> IResult<&str, Option<Vec<Tag>>> {
    opt(delimited(
        char('@'),
        separated_list1(char(';'), tag),
        char(' '),
    ))(i)
}

// <message>       ::= ['@' <tags> <SPACE>] [':' <prefix> <SPACE> ] <command> [params] <crlf>
// <tags>          ::= <tag> [';' <tag>]*
// <tag>           ::= <key> ['=' <escaped_value>]
// <key>           ::= [ <client_prefix> ] [ <vendor> '/' ] <key_name>
// <client_prefix> ::= '+'
// <key_name>      ::= <non-empty sequence of ascii letters, digits, hyphens ('-')>
// <escaped_value> ::= <sequence of zero or more utf8 characters except NUL, CR, LF, semicolon (`;`) and SPACE>
// <vendor>        ::= <host>

#[test]
fn test() {
    assert_eq!(
        tags("@key=value;other=value rest of message"),
        Ok((
            "rest of message",
            Some(vec![
                Tag {
                    key: String::from("key"),
                    value: Some(String::from("value"))
                },
                Tag {
                    key: String::from("other"),
                    value: Some(String::from("value"))
                }
            ])
        ))
    )
}
