use nom::{IResult, combinator::map_res, bytes::complete::{take_while_m_n}, sequence::{tuple, preceded}};
use nom::character::complete::char;

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
  u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
  c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
  map_res(
    take_while_m_n(2, 2, is_hex_digit),
    from_hex
  )(input)
}

pub fn hex_color(input: &str) -> (u8,u8,u8) {
  let (_input, result) = preceded(char('#'), tuple((hex_primary, hex_primary, hex_primary)))(input).unwrap();

  result
}