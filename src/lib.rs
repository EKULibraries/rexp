use nom::{
    branch::alt,
    bytes::complete::{ tag, is_not },
    character::complete::{
        alpha0,
        alpha1,
        char,
        digit1,
        multispace0,
        multispace1,
        one_of,
    },
    combinator::{ cut, map, map_res, opt, not },
    error::{ context, VerboseError },
    multi::many0,
    sequence::{ delimited, preceded, terminated, tuple },
    IResult,
    Parser,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Num {
    Int(i64),
    //Float(f64),
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

fn escaped_double_quote<'a>(i: &'a str) -> IResult<&'a str, char, VerboseError<&'a str>> {
    map(preceded(tag("\\"), one_of("\"")), |_| '"')(i)
}

fn ending_double_quote<'a>(i: &'a str) -> IResult<&'a str, char, VerboseError<&'a str>> {
    map(preceded(not(tag("\\")), one_of("\"")), |_| '"')(i)
}

fn parse_string<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    map(
        preceded(tag("\""), many0(preceded(not(ending_double_quote), is_not("\"\\")))),
        |s: Vec<&str>| Atom::String(s.join(""))
    )(i)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
