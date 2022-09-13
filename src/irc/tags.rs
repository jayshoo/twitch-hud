use nom::bytes::complete::{is_a, is_not};
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, terminated};
use nom::{bytes::complete::take_while1, combinator::opt, IResult};

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
    pair(terminated(key, char('=')), opt(escaped_value))(i).map(|(i, res)| {
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
    let TAG = |key: &str, value: &str| Tag {
        key: String::from(key),
        value: Some(String::from(value)),
    };
    let TAGNONE = |key: &str| Tag {
        key: String::from(key),
        value: None,
    };

    assert_eq!(
        tags("@key=value;other=value rest of message"),
        Ok((
            "rest of message",
            Some(vec![TAG("key", "value"), TAG("other", "value"),])
        ))
    );

    assert_eq!(
        tags("@badge-info=;badges=;client-nonce=1edccf36d7b90525fd32b161e7a85996;color=#727082;display-name=The_Assault_Corgi;emotes=;first-msg=0;flags=;id=3884c9e2-793d-442d-8c2a-1c9b0f4171c3;mod=0;returning-chatter=0;room-id=121605691;subscriber=0;tmi-sent-ts=1663040755420;turbo=0;user-id=78466127;user-type= :the_assault_corgi!the_assault_corgi@the_assault_corgi.tmi.twitch.tv PRIVMSG #lilmanic :HAPPY BIRTHDAY @Mk22222"),
        Ok((
            ":the_assault_corgi!the_assault_corgi@the_assault_corgi.tmi.twitch.tv PRIVMSG #lilmanic :HAPPY BIRTHDAY @Mk22222",
            Some(vec![
                TAGNONE("badge-info"),
                TAGNONE("badges"),
                TAG("client-nonce", "1edccf36d7b90525fd32b161e7a85996"),
                TAG("color", "#727082"),
                TAG("display-name","The_Assault_Corgi"),
                TAGNONE("emotes"),
                TAG("first-msg","0"),
                TAGNONE("flags"),
                TAG("id","3884c9e2-793d-442d-8c2a-1c9b0f4171c3"),
                TAG("mod","0"),
                TAG("returning-chatter","0"),
                TAG("room-id","121605691"),
                TAG("subscriber","0"),
                TAG("tmi-sent-ts","1663040755420"),
                TAG("turbo","0"),
                TAG("user-id","78466127"),
                TAGNONE("user-type"),
            ])
        ))
    )
}
