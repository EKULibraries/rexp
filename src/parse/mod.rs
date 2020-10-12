/// Limit the exposed interface of the parser internals.
use nom::{
    IResult,
    branch,
    multi,
    combinator,
    error::{ self, VerboseError },
    sequence,
    character::complete,
};

use crate::expr::Sexp;

pub mod quote;
use quote::quote;

pub mod atom;
use atom::atom;

pub fn sexp<'a>(i: &'a str) -> IResult<&'a str, Sexp, VerboseError<&'a str>> {
    use combinator::map;
    branch::alt((
        map(quote, Sexp::Quote),
        map(list, Sexp::List),
        map(vector, Sexp::Vector),
        // `atom` is very greedy, so it needs to come last
        map(atom, Sexp::Constant),
    ))(i)
}

fn list<'a>(i: &'a str) -> IResult<&'a str, Vec<Sexp>, VerboseError<&'a str>> {
    use sequence::{preceded, delimited};
    use complete::{char, multispace0};
    delimited(
        char('('),
        multi::many0(preceded(multispace0, sexp)),
        error::context(
            "closing paren",
            combinator::cut(
                preceded(
                    multispace0,
                    char(')'))))
    )(i)
}

fn vector<'a>(i: &'a str) -> IResult<&'a str, Vec<Sexp>, VerboseError<&'a str>> {
    sequence::preceded(complete::char('#'), list)(i)
}
