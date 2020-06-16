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
    take_while, Err, IResult, Needed, Parser,
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

#[cfg(test)]
mod tests {
    use super::*;
}
