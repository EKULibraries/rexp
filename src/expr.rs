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
pub enum Quote {
    Quote(Box<Sexp>),
    Quasi(Box<Sexp>),
    UnQuote(Box<Sexp>),
    Splice(Box<Sexp>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Sexp {
    Constant(Atom),
    List(Vec<Sexp>),
    Quote(Quote),
}
