use rexp::*;
use rexp::parse::{string, symbol, num};

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

// Higher level parsing tests

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

// Symbols

#[test]
fn atom_symbols() {
    assert_eq!(symbol("map"), Ok(("", "map".to_owned())));
    assert_eq!(
        symbol("^!symbols#$%legal"),
        Ok(("", "^!symbols#$%legal".to_owned()))
    );
    assert_eq!(symbol("regular-name"), Ok(("", "regular-name".to_owned())));
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

#[test]
fn fails_on_empy_sexp() {
    assert!(sexp("").is_err());
}

// Constants

#[test]
fn int_constant() {
    assert_eq!(
        sexp("345"),
        Ok(("", Sexp::Constant(Atom::Num(Num::Int(345)))))
    );
    assert_eq!(
        sexp("-345"),
        Ok(("", Sexp::Constant(Atom::Num(Num::Int(-345)))))
    );
}

#[test]
fn float_constant() {
    assert_eq!(
        sexp("756.314"),
        Ok(("", Sexp::Constant(Atom::Num(Num::Float(756.314)))))
    );
    assert_eq!(
        sexp("-756.314"),
        Ok(("", Sexp::Constant(Atom::Num(Num::Float(-756.314)))))
    );
}

#[test]
fn symbol_constant() {
    assert_eq!(
        sexp("name"),
        Ok(("", Sexp::Constant(Atom::Symbol("name".to_owned()))))
    );
}

#[test]
fn string_constant() {
    assert_eq!(
        sexp("\"This is a \\\"string\\\"!\""),
        Ok((
            "",
            Sexp::Constant(Atom::String("This is a \"string\"!".to_owned()))
        ))
    );
}

// Quoted Constants
use rexp::Quote::{
    Quasi,
    Quote,
//    Splice,
//    UnQuote,
};

#[test]
fn quoted_int() {
    assert_eq!(
        sexp("'345"),
        Ok((
            "",
            Sexp::Quote(Quote(Box::new(Sexp::Constant(Atom::Num(Num::Int(345))))))
        ))
    );
    assert_eq!(
        sexp("'-345"),
        Ok((
            "",
            Sexp::Quote(Quote(Box::new(Sexp::Constant(Atom::Num(Num::Int(-345))))))
        ))
    );
    // Quasi
    assert_eq!(
        sexp("`345"),
        Ok((
            "",
            Sexp::Quote(Quasi(Box::new(Sexp::Constant(Atom::Num(Num::Int(345))))))
        ))
    );
}

#[test]
fn cant_splice_unquote_int() {
    assert!(sexp(",345").is_err());

    assert!(sexp("@345").is_err());
}

#[test]
fn quoted_float() {
    assert_eq!(
        sexp("'756.314"),
        Ok((
            "",
            Sexp::Quote(Quote(Box::new(Sexp::Constant(Atom::Num(Num::Float(
                756.314
            ))))))
        ))
    );
    assert_eq!(
        sexp("'-756.314"),
        Ok((
            "",
            Sexp::Quote(Quote(Box::new(Sexp::Constant(Atom::Num(Num::Float(
                -756.314
            ))))))
        ))
    );
    // Quasi
    assert_eq!(
        sexp("`756.314"),
        Ok((
            "",
            Sexp::Quote(Quasi(Box::new(Sexp::Constant(Atom::Num(Num::Float(
                756.314
            ))))))
        ))
    );
}

#[test]
fn cant_splice_or_quote_float() {
    assert!(sexp(",756.314").is_err());

    assert!(sexp("@756.314").is_err());
}

#[test]
fn quoted_string() {
    assert_eq!(
        sexp("'\"this is a quoted string\""),
        Ok((
            "",
            Sexp::Quote(Quote(Box::new(Sexp::Constant(Atom::String(
                "this is a quoted string".to_owned()
            )))))
        ))
    );
}

#[test]
fn quoted_symbol() {
    assert_eq!(
        sexp("'symbol"),
        Ok((
            "",
            Sexp::Quote(Quote(Box::new(Sexp::Constant(Atom::Symbol(
                "symbol".to_owned()
            )))))
        ))
    );
    assert_eq!(
        sexp("'|this symbol has spaces|"),
        Ok((
            "",
            Sexp::Quote(Quote(Box::new(Sexp::Constant(Atom::Symbol(
                "this symbol has spaces".to_owned()
            )))))
        ))
    );
}
