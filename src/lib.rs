// Only turned off for now
#![allow(dead_code, unused_imports)]

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha0, alpha1, char, digit1, multispace0, multispace1, one_of},
    combinator::{cut, map, map_res, not, opt},
    error::{context, VerboseError},
    multi::many0,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    take_while, IResult, Parser,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Num {
    Int(i64),
    Float(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Num(Num),
    Symbol(String),
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Sexp {
    Constant(Atom),
    List(Vec<Sexp>),
    Quote(Box<Sexp>),
}

pub fn sexp<'a>(i: &'a str) -> IResult<&'a str, Sexp, VerboseError<&'a str>> {
    alt((
        map(preceded(tag("'"), sexp), |s| Sexp::Quote(Box::new(s))),
        map(
            delimited(
                char('('),
                many0(preceded(multispace0, sexp)),
                context("closing paren", cut(preceded(multispace0, char(')'))))
            ),
            Sexp::List
        ),
        map(atom, Sexp::Constant)
    ))(i)
}

pub fn string<'a>(i: &'a str) -> IResult<&'a str, String, VerboseError<&'a str>> {
    terminated(preceded(tag("\""), in_quotes), tag("\""))(i)
}

fn in_quotes<'a>(s: &'a str) -> IResult<&'a str, String, VerboseError<&'a str>> {
    let mut result = String::new();
    let mut skip = false;

    for (i, ch) in s.char_indices() {
        if ch == '\\' && !skip {
            skip = true;
        } else if ch == '"' && !skip {
            return Ok((&s[i..], result));
        } else {
            result.push(ch);
            skip = false;
        }
    }
    Err(nom::Err::Incomplete(nom::Needed::Unknown))
}

pub fn symbol<'a>(i: &'a str) -> IResult<&'a str, String, VerboseError<&'a str>> {
    alt((
        map(delimited(tag("|"), is_not("|"), tag("|")), |s: &str| {
            s.to_owned()
        }),
        map(is_not(" \t\n"), |s: &str| s.to_owned()),
    ))(i)
}

pub fn num<'a>(i: &'a str) -> IResult<&'a str, Num, VerboseError<&'a str>> {
    alt((
        // Floats
        map_res(
            separated_pair(digit1, tag("."), digit1),
            |(whole, part): (&str, &str)| {
                (whole.to_owned() + "." + part)
                    .parse::<f64>()
                    .map(Num::Float)
            },
        ),
        map_res(
            preceded(tag("-"), separated_pair(digit1, tag("."), digit1)),
            |(whole, part): (&str, &str)| {
                (whole.to_owned() + "." + part)
                    .parse::<f64>()
                    .map(|f| Num::Float(-f))
            },
        ),
        // Ints
        map_res(digit1, |d: &str| d.parse::<i64>().map(Num::Int)),
        map_res(preceded(tag("-"), digit1), |d: &str| {
            d.parse::<i64>().map(|i| Num::Int(-i))
        }),
    ))(i)
}

pub fn atom<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    alt((
        map(string, Atom::String),
        map(symbol, Atom::Symbol),
        map(num, Atom::Num),
    ))(i)
}
