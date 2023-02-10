use nom::{
    bytes::complete::take_while_m_n, sequence::tuple,
    IResult, Parser,
};
use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::tag;
use nom_supreme::ParserExt;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex(
    input: &str,
) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(
    input: &str,
) -> IResult<&str, u8, ErrorTree<&str>> {
    take_while_m_n(2, 2, is_hex_digit)
        .context("Should be a 2 digit hex code")
        .map_res(from_hex)
        .parse(input)
}

fn hex_color(
    input: &str,
) -> IResult<&str, Color, ErrorTree<&str>> {
    tuple((hex_primary, hex_primary, hex_primary))
        .preceded_by(tag("#"))
        .parse(input)
        .map(|(input, (red, green, blue))| {
            (input, Color { red, green, blue })
        })
}

fn main() {
    assert_eq!(
        hex_color("#2F14DF").unwrap(),
        (
            "",
            Color {
                red: 47,
                green: 20,
                blue: 223,
            }
        )
    );

    dbg!(hex_color("#2"));
    dbg!(hex_color("234567"));

    hex_color("#12").unwrap();
}
