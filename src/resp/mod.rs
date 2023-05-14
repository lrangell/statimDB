use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::not_line_ending;
use nom::character::complete::{crlf, digit1};
use nom::combinator::map_res;
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Expr {
    String(String),
    Integer(i64),
    Array(Vec<Expr>),
    Null,
    Error(String),
}

pub fn null(input: &str) -> IResult<&str, Expr> {
    map_res(tag("$-1\r\n"), |_| Ok::<Expr, &str>(Expr::Null))(input)
}

pub fn error(input: &str) -> IResult<&str, Expr> {
    map_res(
        delimited(tag("+"), not_line_ending, crlf),
        |content: &str| Ok::<Expr, &str>(Expr::Error(content.into())),
    )(input)
}

pub fn simple_string(input: &str) -> IResult<&str, Expr> {
    map_res(
        delimited(tag("+"), not_line_ending, crlf),
        |content: &str| Ok::<Expr, &str>(Expr::String(content.into())),
    )(input)
}

pub fn string(input: &str) -> IResult<&str, Expr> {
    let (remaining, size) = map_res(delimited(tag("$"), digit1, crlf), |i: &str| {
        i.parse::<usize>()
    })(input)?;
    let (remaining, content) = terminated(take(size), crlf)(remaining)?;
    Ok((remaining, Expr::String(content.to_string())))
}

pub fn integer(input: &str) -> IResult<&str, Expr> {
    map_res(delimited(tag(":"), digit1, crlf), |i: &str| {
        i.parse::<i64>().map(Expr::Integer)
    })(input)
}

pub fn array(input: &str) -> IResult<&str, Expr> {
    let element = many0(alt((string, integer, null)));
    let array_size = tuple((tag("*"), digit1, crlf));
    let (remaining, array) = preceded(array_size, element)(input)?;
    Ok((remaining, Expr::Array(array)))
}
