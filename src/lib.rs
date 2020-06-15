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
}

// Begin combinators

fn parse_string<'a>(i: &'a str) -> IResult<&'a str, String, VerboseError<&'a str>> {
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

fn parse_symbol<'a>(i: &'a str) -> IResult<&'a str, String, VerboseError<&'a str>> {
    alt((
        map(delimited(tag("|"), is_not("|"), tag("|")), |s: &str| {
            s.to_owned()
        }),
        map(is_not(" \t\n"), |s: &str| s.to_owned()),
    ))(i)
}

fn parse_num<'a>(i: &'a str) -> IResult<&'a str, Num, VerboseError<&'a str>> {
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

pub fn parse_atom<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    alt((
        map(parse_string, Atom::String),
        map(parse_symbol, Atom::Symbol),
        map(parse_num, Atom::Num),
    ))(i)
}

#[cfg(test)]
mod tests {

    use super::*;

    // Strings

    #[test]
    fn parse_whole_scm_string() {
        assert_eq!(
            parse_string("\"This is a test\""),
            Ok(("", "This is a test".to_owned()))
        );
    }

    #[test]
    fn parse_scm_string_with_escaped_quotes() {
        assert_eq!(
            parse_string("\"This is a \\\"test\\\"\""),
            Ok(("", "This is a \"test\"".to_owned()))
        );
        // With unclosed escaped string too
        assert_eq!(
            parse_string("\"This is a \\\"test\" and some more stuff"),
            Ok((" and some more stuff", "This is a \"test".to_owned()))
        );
    }

    #[test]
    fn fail_to_parse_scm_string_without_active_quotes() {
        assert!(parse_string("this is a test").is_err());
        // And also with escaped quotes
        assert!(parse_string("\\\"this is \\\" a \\\"test\\\"").is_err());
    }

    // Numbers

    #[test]
    fn parse_positive_integer() {
        assert_eq!(parse_num("45"), Ok(("", Num::Int(45))));
    }

    #[test]
    fn parse_negative_integer() {
        assert_eq!(parse_num("-562"), Ok(("", Num::Int(-562))));
    }

    #[test]
    fn parse_positive_float() {
        assert_eq!(parse_num("67.432"), Ok(("", Num::Float(67.432))));
    }

    #[test]
    fn parse_negative_float() {
        assert_eq!(parse_num("-254.345"), Ok(("", Num::Float(-254.345))));
    }

    // Symbols

    #[test]
    fn parse_simple_symbols() {
        assert_eq!(parse_symbol("map"), Ok(("", "map".to_owned())));
        assert_eq!(
            parse_symbol("^!symbols#$%legal"),
            Ok(("", "^!symbols#$%legal".to_owned()))
        );
        assert_eq!(
            parse_symbol("regular-name"),
            Ok(("", "regular-name".to_owned()))
        );
    }

    #[test]
    fn only_get_first_symbol() {
        assert_eq!(
            parse_symbol("this is a test"),
            Ok((" is a test", "this".to_owned()))
        );
    }

    #[test]
    fn parse_delimited_symbol() {
        assert_eq!(
            parse_symbol("|this is a symbol|"),
            Ok(("", "this is a symbol".to_owned()))
        );
    }

    #[test]
    fn delimited_symbol_with_unmatched_delimiters() {
        assert_eq!(parse_symbol("|this"), Ok(("", "|this".to_owned())));
        assert_eq!(
            parse_symbol("|these are many symbols"),
            Ok((" are many symbols", "|these".to_owned()))
        );
        assert_eq!(parse_symbol("this|"), Ok(("", "this|".to_owned())));
        assert_eq!(
            parse_symbol("this|is many symbols"),
            Ok((" many symbols", "this|is".to_owned()))
        );
    }

    // Higher level parsing tests

    // Atoms

    // Strings

    #[test]
    fn parse_whole_atom_string() {
        assert_eq!(
            parse_atom("\"This is a test\""),
            Ok(("", Atom::String("This is a test".to_owned())))
        );
    }

    #[test]
    fn parse_atom_string_with_escaped_quotes() {
        assert_eq!(
            parse_atom("\"This is a \\\"test\\\"\""),
            Ok(("", Atom::String("This is a \"test\"".to_owned())))
        );
        // With unclosed escaped string too
        assert_eq!(
            parse_atom("\"This is a \\\"test\" and some more stuff"),
            Ok((
                " and some more stuff",
                Atom::String("This is a \"test".to_owned())
            ))
        );
    }

    fn fail_atom_string_without_active_quotes_2() {
        // And also with escaped quotes
        assert!(parse_atom("\\\"this is \\\" a \\\"test\\\"").is_err());
    }

    // Numbers

    #[test]
    fn parse_atom_integer() {
        assert_eq!(parse_num("45"), Ok(("", Num::Int(45))));

        assert_eq!(parse_num("-562"), Ok(("", Num::Int(-562))));
    }

    #[test]
    fn parse_atom_float() {
        assert_eq!(parse_num("67.432"), Ok(("", Num::Float(67.432))));

        assert_eq!(parse_num("-254.345"), Ok(("", Num::Float(-254.345))));
    }

    // Symbols

    #[test]
    fn parse_atom_symbols() {
        assert_eq!(parse_symbol("map"), Ok(("", "map".to_owned())));
        assert_eq!(
            parse_symbol("^!symbols#$%legal"),
            Ok(("", "^!symbols#$%legal".to_owned()))
        );
        assert_eq!(
            parse_symbol("regular-name"),
            Ok(("", "regular-name".to_owned()))
        );
        // only get first symbol
        assert_eq!(
            parse_symbol("this is a test"),
            Ok((" is a test", "this".to_owned()))
        );
        // parse delimited symbol
        assert_eq!(
            parse_symbol("|this is a symbol|"),
            Ok(("", "this is a symbol".to_owned()))
        );
        // delimited symbol with unmatched delimiters
        assert_eq!(parse_symbol("|this"), Ok(("", "|this".to_owned())));
        assert_eq!(
            parse_symbol("|these are many symbols"),
            Ok((" are many symbols", "|these".to_owned()))
        );
        assert_eq!(parse_symbol("this|"), Ok(("", "this|".to_owned())));
        assert_eq!(
            parse_symbol("this|is many symbols"),
            Ok((" many symbols", "this|is".to_owned()))
        );
    }
}
