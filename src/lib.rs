use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        alpha0,
        alpha1,
        char,
        digit1,
        multispace0,
        multispace1,
        one_of,
    },
    combinator::{ cut, map, map_res, opt },
    error::{ context, VerboseError },
    multi::many0,
    sequence::{ delimited, preceded, terminated, tuple },
    IResult,
    Parser,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
