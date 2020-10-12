use crate::{
    expr::{
        Quote,
        Sexp,
    },
    parse,
};

use nom::{
    branch,
    sequence,
    bytes::complete,
    combinator,
    error::{
        VerboseError,
        VerboseErrorKind,
    },
    IResult,
};

fn quote_bouncer<'a>(
    mut parser: impl FnMut(&'a str) -> IResult<&'a str, Sexp, VerboseError<&'a str>>,
    builder: impl Fn(Box<Sexp>) -> Quote,
    msg: &'static str,
) -> impl FnMut(&'a str) -> IResult<&'a str, Quote, VerboseError<&'a str>> {
    move |i: &'a str| match parser(i) {
        Ok((ii, ss)) => match ss {
            Sexp::Constant(_) => Err(nom::Err::Failure(VerboseError {
                errors: vec![(ii, VerboseErrorKind::Context(msg))],
            })),
            _ => Ok((ii, builder(Box::new(ss)))),
        },
        Err(err) => Err(err),
    }
}

pub fn quote<'a>(i: &'a str) -> IResult<&'a str, Quote, VerboseError<&'a str>> {
    use complete::tag;
    use combinator::map;
    use sequence::preceded;
    branch::alt((
        map(preceded(tag("'"), parse::sexp), |s| Quote::Quote(Box::new(s))),
        map(preceded(tag("`"), parse::sexp), |s| Quote::Quasi(Box::new(s))),
        quote_bouncer(
            preceded(tag(","), parse::sexp),
            Quote::UnQuote,
            "can't unquote literals",
        ),
        quote_bouncer(
            preceded(tag("@"), parse::sexp),
            Quote::Splice,
            "can't splice literals",
        ),
    ))(i)
}
