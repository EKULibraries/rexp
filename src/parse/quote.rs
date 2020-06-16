use crate::{
    expr::*,
    parse::sexp,
};

use nom::{
    branch::alt,
    sequence::preceded,
    bytes::complete::tag,
    combinator::map,
    error::{
        VerboseError,
        VerboseErrorKind::Context,
    },
    IResult, Err,
};

fn quote_bouncer<'a>(
    mut parser: impl FnMut(&'a str) -> IResult<&'a str, Sexp, VerboseError<&'a str>>,
    builder: impl Fn(Box<Sexp>) -> Quote,
    msg: &'static str,
) -> impl FnMut(&'a str) -> IResult<&'a str, Quote, VerboseError<&'a str>> {
    move |i: &'a str| match parser(i) {
        Ok((ii, ss)) => match ss {
            Sexp::Constant(_) => Err(Err::Failure(VerboseError {
                errors: vec![(ii, Context(msg))],
            })),
            _ => Ok((ii, builder(Box::new(ss)))),
        },
        Err(err) => Err(err),
    }
}

pub fn quote<'a>(i: &'a str) -> IResult<&'a str, Quote, VerboseError<&'a str>> {
    alt((
        map(preceded(tag("'"), sexp), |s| Quote::Quote(Box::new(s))),
        map(preceded(tag("`"), sexp), |s| Quote::Quasi(Box::new(s))),
        quote_bouncer(
            preceded(tag(","), sexp),
            Quote::UnQuote,
            "can't unquote literals",
        ),
        quote_bouncer(
            preceded(tag("@"), sexp),
            Quote::Splice,
            "can't splice literals",
        ),
    ))(i)
}
