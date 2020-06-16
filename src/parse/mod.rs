/// Limit the exposed interface of the parser internals.
use nom::{
    IResult,
    branch::alt,
    multi::many0,
    combinator::{ cut, map },
    error::{ context, VerboseError },
    sequence::{ delimited, preceded },
    character::complete::{ char, multispace0 },
};

use crate::expr::*;

pub mod quote;
use quote::quote;

pub mod atom;
use atom::atom;

pub fn sexp<'a>(i: &'a str) -> IResult<&'a str, Sexp, VerboseError<&'a str>> {
    alt((
        map(quote, Sexp::Quote),
        map(
            delimited(
                char('('),
                many0(preceded(multispace0, sexp)),
                context("closing paren", cut(preceded(multispace0, char(')')))),
            ),
            Sexp::List,
        ),
        map(atom, Sexp::Constant),
    ))(i)
}
