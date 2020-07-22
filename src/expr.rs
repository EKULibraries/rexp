/// Symbolic Expression Types.
#[derive(Debug, PartialEq, Clone)]
pub enum Num {
    Int(i64),
    Float(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Num(Num),
    Char(char),
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
    Quote(Quote),
    Constant(Atom),
    List(Vec<Sexp>),
    Vector(Vec<Sexp>),
}
