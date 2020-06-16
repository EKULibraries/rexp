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

pub fn atom<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    alt((
        map(num, Atom::Num),
        map(string, Atom::String),
        map(symbol, Atom::Symbol),
    ))(i)
}

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
    Err(Err::Error(VerboseError {
        errors: vec![(s, Context("string missing closing \""))],
    }))
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

#[cfg(test)]
mod tests {
    use super::*;
    // Strings

    #[test]
    fn parse_whole_scm_string() {
        assert_eq!(
            string("\"This is a test\""),
            Ok(("", "This is a test".to_owned()))
        );
    }

    #[test]
    fn parse_scm_string_with_escaped_quotes() {
        assert_eq!(
            string("\"This is a \\\"test\\\"\""),
            Ok(("", "This is a \"test\"".to_owned()))
        );
        // With unclosed escaped string too
        assert_eq!(
            string("\"This is a \\\"test\" and some more stuff"),
            Ok((" and some more stuff", "This is a \"test".to_owned()))
        );
    }

    #[test]
    fn fail_to_parse_scm_string_without_active_quotes() {
        assert!(string("this is a test").is_err());
        // And also with escaped quotes
        assert!(string("\\\"this is \\\" a \\\"test\\\"").is_err());
    }

    // Numbers

    #[test]
    fn parse_positive_integer() {
        assert_eq!(num("45"), Ok(("", Num::Int(45))));
    }

    #[test]
    fn parse_negative_integer() {
        assert_eq!(num("-562"), Ok(("", Num::Int(-562))));
    }

    #[test]
    fn parse_positive_float() {
        assert_eq!(num("67.432"), Ok(("", Num::Float(67.432))));
    }

    #[test]
    fn parse_negative_float() {
        assert_eq!(num("-254.345"), Ok(("", Num::Float(-254.345))));
    }

    // Symbols

    #[test]
    fn parse_simple_symbols() {
        assert_eq!(symbol("map"), Ok(("", "map".to_owned())));
        assert_eq!(
            symbol("^!symbols#$%legal"),
            Ok(("", "^!symbols#$%legal".to_owned()))
        );
        assert_eq!(symbol("regular-name"), Ok(("", "regular-name".to_owned())));
    }

    #[test]
    fn only_get_first_symbol() {
        assert_eq!(
            symbol("this is a test"),
            Ok((" is a test", "this".to_owned()))
        );
    }

    #[test]
    fn parse_delimited_symbol() {
        assert_eq!(
            symbol("|this is a symbol|"),
            Ok(("", "this is a symbol".to_owned()))
        );
    }

    #[test]
    fn delimited_symbol_with_unmatched_delimiters() {
        assert_eq!(symbol("|this"), Ok(("", "|this".to_owned())));
        assert_eq!(
            symbol("|these are many symbols"),
            Ok((" are many symbols", "|these".to_owned()))
        );
        assert_eq!(symbol("this|"), Ok(("", "this|".to_owned())));
        assert_eq!(
            symbol("this|is many symbols"),
            Ok((" many symbols", "this|is".to_owned()))
        );
    }

    // Numbers

    #[test]
    fn num_integer() {
        assert_eq!(num("45"), Ok(("", Num::Int(45))));

        assert_eq!(num("-562"), Ok(("", Num::Int(-562))));
    }

    #[test]
    fn num_float() {
        assert_eq!(num("67.432"), Ok(("", Num::Float(67.432))));

        assert_eq!(num("-254.345"), Ok(("", Num::Float(-254.345))));
    }

    #[test]
    fn atom_integer() {
        assert_eq!(atom("45"), Ok(("", Atom::Num(Num::Int(45)))));

        assert_eq!(atom("-562"), Ok(("", Atom::Num(Num::Int(-562)))));
    }

    #[test]
    fn atom_float() {
        assert_eq!(atom("67.432"), Ok(("", Atom::Num(Num::Float(67.432)))));

        assert_eq!(atom("-254.345"), Ok(("", Atom::Num(Num::Float(-254.345)))));
    }

}
