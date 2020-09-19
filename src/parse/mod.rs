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
    branch::alt((
        combinator::map(quote, Sexp::Quote),
        combinator::map(list, Sexp::List),
        combinator::map(vector, Sexp::Vector),
        // `atom` is very greedy, so it needs to come last
        combinator::map(atom, Sexp::Constant),
    ))(i)
}

fn list<'a>(i: &'a str) -> IResult<&'a str, Vec<Sexp>, VerboseError<&'a str>> {
    sequence::delimited(
        complete::char('('),
        multi::many0(sequence::preceded(complete::multispace0, sexp)),
        error::context(
            "closing paren",
            combinator::cut(
                sequence::preceded(
                    complete::multispace0,
                    complete::char(')'))))
    )(i)
}

fn vector<'a>(i: &'a str) -> IResult<&'a str, Vec<Sexp>, VerboseError<&'a str>> {
    sequence::preceded(complete::char('#'), list)(i)
}
