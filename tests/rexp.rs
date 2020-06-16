use rexp::*;

// Quoted Constants
use rexp::Quote::{
    Quasi,
    Quote,
//    Splice,
//    UnQuote,
};

// Higher level parsing tests
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



// Lists

#[test]
fn simple_list_of_ints() {
    assert_eq!(
        sexp("(1 2 3 4 5)"),
        Ok((
            "",
            Sexp::List(
                vec![
                    Sexp::Constant(Atom::Num(Num::Int(1))),
                    Sexp::Constant(Atom::Num(Num::Int(2))),
                    Sexp::Constant(Atom::Num(Num::Int(3))),
                    Sexp::Constant(Atom::Num(Num::Int(4))),
                    Sexp::Constant(Atom::Num(Num::Int(5))),
                ]
            )
        ))
    );
}

#[test]
fn simple_list_of_atoms() {
    assert_eq!(
        sexp("(func \"some message\" 14 56.3 -3)"),
        Ok((
            "",
            Sexp::List(
                vec![
                    Sexp::Constant(Atom::Symbol("func".to_owned())),
                    Sexp::Constant(Atom::String("some message".to_owned())),
                    Sexp::Constant(Atom::Num(Num::Int(14))),
                    Sexp::Constant(Atom::Num(Num::Float(56.3))),
                    Sexp::Constant(Atom::Num(Num::Int(-3))),
                ]
            )
        ))
    );
}

#[test]
fn nested_list() {
    assert_eq!(
        sexp("(lambda (msg) (println msg))"),
        Ok((
            "",
            Sexp::List(
                vec![
                    Sexp::Constant(Atom::Symbol("lambda".to_owned())),
                    Sexp::List(
                        vec![
                            Sexp::Constant(Atom::Symbol("msg".to_owned())),
                        ]
                    ),
                    Sexp::List(
                        vec![
                            Sexp::Constant(Atom::Symbol("println".to_owned())),
                            Sexp::Constant(Atom::Symbol("msg".to_owned())),
                        ]
                    ),
                ]
            )
        ))
    );
}

// Quoted Lists, Quoted Symbols, Quoted Quotes, and miscellaneous

#[test]
fn quoted_list() {
    assert_eq!(
        sexp("'()"),
        Ok(("", Sexp::Quote(Quote(Box::new(Sexp::List(vec![]))))))
    );
}
