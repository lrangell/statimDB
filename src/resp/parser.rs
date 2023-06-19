use super::Expr;
use nom::{
    branch::alt,
    bytes::complete::take,
    character::complete::{crlf, digit1, not_line_ending},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use nom_supreme::tag::complete::tag;
use nom_supreme::{error::ErrorTree, final_parser::final_parser};

macro_rules! delimited_exp {
    ($tag: literal, $to:expr) => {
        map(
            delimited(
                tag::<&str, &str, ErrorTree<&str>>($tag),
                not_line_ending,
                crlf,
            ),
            |content: &str| $to(content.into()),
        )
    };
}

pub fn parse(input: &str) -> Result<Expr, ErrorTree<&str>> {
    final_parser(alt((parse_element, parse_array)))(input)
}
fn parse_element(input: &str) -> IResult<&str, Expr, ErrorTree<&str>> {
    let error = delimited_exp!("-", Expr::Error);
    let null = delimited_exp!("-1", |_: &str| { Expr::Null });
    let simple_string = delimited_exp!("+", Expr::String);
    let integer = delimited_exp!(":", |i: &str| {
        i.parse::<i64>().map(Expr::Integer).unwrap()
    });
    let mut elements = alt((null, error, simple_string, integer, string));
    elements(input)
}
fn parse_array(input: &str) -> IResult<&str, Expr, ErrorTree<&str>> {
    let array_elements = map(many0(parse_element), Expr::Array);
    let array_size = tuple((tag("*"), opt(tag("-")), digit1, crlf));
    preceded(array_size, array_elements)(input)
}

fn string(input: &str) -> IResult<&str, Expr, ErrorTree<&str>> {
    let (remaining, size) = map(
        delimited(tag::<_, &str, _>("$"), tuple((opt(tag("-")), digit1)), crlf),
        |(neg, i)| match neg {
            Some(_) => -1,
            None => i.parse::<isize>().unwrap(),
        },
    )(input)?;
    match size {
        -1 => Ok((remaining, Expr::Null)),
        _ => map(terminated(take(size as usize), crlf), |s: &str| {
            Expr::String(s.into())
        })(remaining),
    }
}
