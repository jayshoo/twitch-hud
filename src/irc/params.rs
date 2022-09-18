use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::{char, space1};

use nom::multi::separated_list0;

use nom::{sequence::preceded, IResult};

fn param_single(i: &str) -> IResult<&str, String> {
    is_not(" ")(i).map(|(i, res)| (i, String::from(res)))
}

fn param_final(i: &str) -> IResult<&str, String> {
    preceded(char(':'), is_not("\r\n"))(i).map(|(i, res)| (i, String::from(res)))
}

fn param(i: &str) -> IResult<&str, String> {
    alt((param_final, param_single))(i)
}

pub fn params(i: &str) -> IResult<&str, Vec<String>> {
    separated_list0(space1, param)(i)
}

#[test]
fn test() {
    assert_eq!(
        params("#moonmoon :message text here"),
        Ok((
            "",
            vec![String::from("#moonmoon"), String::from("message text here")]
        ))
    )
}
