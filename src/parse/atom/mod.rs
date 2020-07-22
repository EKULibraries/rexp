use nom::{
    branch::alt,
    bytes::complete::{ is_not, tag },
    character::complete::digit1,
    combinator::{ map, map_res },
    sequence::{
        preceded,
        delimited,
        terminated,
        separated_pair,
    },
    error::{
        VerboseError,
        VerboseErrorKind::Context,
    },
    IResult, Err,
};

use crate::expr::{ Atom, Num };



// Public

pub fn atom<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    alt((
        map(num, Atom::Num),
        map(lit_char, Atom::Char),
        map(string, Atom::String),
        map(symbol, Atom::Symbol),
    ))(i)
}



// String parsing

fn string<'a>(i: &'a str) -> IResult<&'a str, String, VerboseError<&'a str>> {
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



// Symbol parsing

fn symbol<'a>(i: &'a str) -> IResult<&'a str, String, VerboseError<&'a str>> {
    alt((
        map(delimited(tag("|"), is_not("|"), tag("|")), |s: &str| {
            s.to_owned()
        }),
        map(is_not(" \t\r\n()"), |s: &str| s.to_owned()),
    ))(i)
}



// Number parsing

fn num<'a>(i: &'a str) -> IResult<&'a str, Num, VerboseError<&'a str>> {
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



// Character literal parsing

fn lit_char<'a>(i: &'a str) -> IResult<&'a str, char, VerboseError<&'a str>> {
    map(preceded(tag("#\\"), is_not(" \t\r\n")), process_char)(i)
}

// Currently this panics if the char is invalid
fn process_char<'a>(i: &'a str) -> char {
    if i.len() == 1 {
        i.chars().next().unwrap()
    } else {
        match i {
            "nul"   => 0x00 as char,
            // Short names for the `C0` characters
            "soh"                          => 0x01 as char,
            "stx"                          => 0x02 as char,
            "etx"                          => 0x03 as char,
            "eot"                          => 0x04 as char,
            "enq"                          => 0x05 as char,
            "ack"                          => 0x06 as char,
            "bel" | "alarm"                => 0x07 as char,
            "bs"  | "backspace"            => 0x08 as char,
            "ht"  | "tab"                  => 0x09 as char,
            "lf"  | "linefeed" | "newline" => 0x0A as char,
            "vt"  | "vtab"                 => 0x0B as char,
            "ff"  | "page"                 => 0x0C as char,
            "cr"  | "return"               => 0x0D as char,
            "so"                           => 0x0E as char,
            "si"                           => 0x0F as char,
            "dle"                          => 0x10 as char,
            "dc1"                          => 0x11 as char,
            "dc2"                          => 0x12 as char,
            "dc3"                          => 0x13 as char,
            "dc4"                          => 0x14 as char,
            "nak"                          => 0x15 as char,
            "syn"                          => 0x16 as char,
            "etb"                          => 0x17 as char,
            "can"                          => 0x18 as char,
            "em"                           => 0x19 as char,
            "sub"                          => 0x1A as char,
            "esc" | "escape" /* R7RS */    => 0x1B as char,
            "fs"                           => 0x1C as char,
            "gs"                           => 0x1D as char,
            "rs"                           => 0x1E as char,
            "us"                           => 0x1F as char,
            "sp"  | "space"                => 0x20 as char,
            // End of `C0` characters
            "delete"                      => 0x7F as char,
            _ => panic!("Should find a way to handle this more gracefully"),
        }
    }
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
    fn symbol_cant_containt_parens() {
        assert_eq!(
            symbol("symbol("),
            Ok(("(", "symbol".to_owned()))
        );

        assert_eq!(
            symbol("fun-an)d-games"),
            Ok((")d-games", "fun-an".to_owned()))
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

    // Atoms

    // Strings

    #[test]
    fn parse_whole_atom_string() {
        assert_eq!(
            atom("\"This is a test\""),
            Ok(("", Atom::String("This is a test".to_owned())))
        );
    }

    #[test]
    fn atom_string_with_escaped_quotes() {
        assert_eq!(
            atom("\"This is a \\\"test\\\"\""),
            Ok(("", Atom::String("This is a \"test\"".to_owned())))
        );
        // With unclosed escaped string too
        assert_eq!(
            atom("\"This is a \\\"test\" and some more stuff"),
            Ok((
                " and some more stuff",
                Atom::String("This is a \"test".to_owned())
            ))
        );
    }


    // Chars

    #[test]
    fn regular_char_literals() {
        assert_eq!(atom("#\\a"), Ok(("", Atom::Char('a'))));
        assert_eq!(atom("#\\b"), Ok(("", Atom::Char('b'))));
        assert_eq!(atom("#\\Z"), Ok(("", Atom::Char('Z'))));
    }


    // Symbols

    #[test]
    fn atom_symbols() {
        assert_eq!(atom("map"), Ok(("", Atom::Symbol("map".to_owned()))));
        assert_eq!(
            atom("^!symbols#$%legal"),
            Ok(("", Atom::Symbol("^!symbols#$%legal".to_owned())))
        );
        assert_eq!(atom("regular-name"), Ok(("", Atom::Symbol("regular-name".to_owned()))));
        // only get first symbol
        assert_eq!(
            atom("this is a test"),
            Ok((" is a test", Atom::Symbol("this".to_owned())))
        );
        // parse delimited symbol
        assert_eq!(
            atom("|this is a symbol|"),
            Ok(("", Atom::Symbol("this is a symbol".to_owned())))
        );
        // delimited symbol with unmatched delimiters
        assert_eq!(atom("|this"), Ok(("", Atom::Symbol("|this".to_owned()))));
        assert_eq!(
            atom("|these are many symbols"),
            Ok((" are many symbols", Atom::Symbol("|these".to_owned())))
        );
        assert_eq!(atom("this|"), Ok(("", Atom::Symbol("this|".to_owned()))));
        assert_eq!(
            atom("this|is many symbols"),
            Ok((" many symbols", Atom::Symbol("this|is".to_owned())))
        );
    }
}
