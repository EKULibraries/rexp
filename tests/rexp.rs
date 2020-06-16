use rexp::*;
use rexp::parse::{string, symbol, num};

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
