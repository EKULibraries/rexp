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
    assert_eq!(
        symbol("regular-name"),
        Ok(("", "regular-name".to_owned()))
    );
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
    assert_eq!(num("45"), Ok(("", Num::Int(45))));

    assert_eq!(num("-562"), Ok(("", Num::Int(-562))));
}

#[test]
fn parse_atom_float() {
    assert_eq!(num("67.432"), Ok(("", Num::Float(67.432))));

    assert_eq!(num("-254.345"), Ok(("", Num::Float(-254.345))));
}

// Symbols

#[test]
fn parse_atom_symbols() {
    assert_eq!(symbol("map"), Ok(("", "map".to_owned())));
    assert_eq!(
        symbol("^!symbols#$%legal"),
        Ok(("", "^!symbols#$%legal".to_owned()))
    );
    assert_eq!(
        symbol("regular-name"),
        Ok(("", "regular-name".to_owned()))
    );
    // only get first symbol
    assert_eq!(
        symbol("this is a test"),
        Ok((" is a test", "this".to_owned()))
    );
    // parse delimited symbol
    assert_eq!(
        symbol("|this is a symbol|"),
        Ok(("", "this is a symbol".to_owned()))
    );
    // delimited symbol with unmatched delimiters
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

// Sexp

// Constants
#[test]
fn symbol_constant() {
    assert_eq!(sexp("name"), Ok("", Sexp::Constant(Atom::Symbol("name".to_owned()))));
}
