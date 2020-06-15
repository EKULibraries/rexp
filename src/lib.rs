// Only turned off for now
#![allow(dead_code, unused_imports)]

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha0, alpha1, char, digit1, multispace0, multispace1, one_of},
    combinator::{cut, map, map_res, not, opt},
    error::{context, VerboseError, VerboseErrorKind::Context},
    multi::many0,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    take_while, IResult, Parser, Err, Needed,
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
pub enum Quote {
    Quote(Box::<Sexp>),
    Quasi(Box::<Sexp>),
    UnQuote(Box::<Sexp>),
    Splice(Box::<Sexp>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Sexp {
    Constant(Atom),
    List(Vec<Sexp>),
    Quote(Quote),
}

fn unquote<'a>(i: &'a str) -> IResult<&'a str, Quote, VerboseError<&'a str>> {
    match preceded(tag(","), sexp)(i) {
        Ok((ii, ss)) => {
            match ss {
                Sexp::Constant(_) =>
                    Err(Err::Failure(
                        VerboseError {
                            errors: vec![(ii, Context("can't unsplice literals"))]
                        }
                    )),
                _ => Ok((ii, Quote::UnQuote(Box::new(ss))))
            }
        },
        Err(err) => Err(err),
    }
}

fn splice<'a>(i: &'a str) -> IResult<&'a str, Quote, VerboseError<&'a str>> {
    match preceded(tag("@"), sexp)(i) {
        Ok((ii, ss)) => {
            match ss {
                Sexp::Constant(_) =>
                    Err(Err::Failure(
                        VerboseError {
                            errors: vec![(ii, Context("can't splice literals"))]
                        }
                    )),
                _ => Ok((ii, Quote::Splice(Box::new(ss))))
            }
        },
        Err(err) => Err(err),
    }
}

pub fn quote<'a>(i: &'a str) -> IResult<&'a str, Quote, VerboseError<&'a str>> {
    alt((
        map(preceded(tag("'"), sexp), |s| Quote::Quote(Box::new(s))),
        map(preceded(tag("`"), sexp), |s| Quote::Quasi(Box::new(s))),
        unquote,
        splice,
    ))(i)
}

pub fn sexp<'a>(i: &'a str) -> IResult<&'a str, Sexp, VerboseError<&'a str>> {
    alt((
        map(quote, Sexp::Quote),
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
    terminated(preceded(tag("\""), string_inner), tag("\""))(i)
}

fn string_inner<'a>(s: &'a str) -> IResult<&'a str, String, VerboseError<&'a str>> {
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
    Err(Err::Error(
        VerboseError { errors: vec![(s, Context("string missing closing \"")) ] }
    ))
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
        map(num, Atom::Num),
        map(string, Atom::String),
        map(symbol, Atom::Symbol),
    ))(i)
}
