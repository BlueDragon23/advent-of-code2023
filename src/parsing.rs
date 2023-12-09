use std::str::FromStr;

use nom::{
    character::complete::{char, digit1, space1},
    combinator::{map_res, opt},
    multi::separated_list1,
    sequence::pair,
    IResult,
};

pub fn parse_number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(
        pair(opt(char('-')), digit1),
        |(sign, x): (_, &str)| match sign {
            Some(_) => ("-".to_string() + x).parse::<T>(),
            None => x.parse::<T>(),
        },
    )(input)
}

/// Parse a list of whitespace separated numbers
pub fn parse_numbers<T: FromStr>(input: &str) -> IResult<&str, Vec<T>> {
    separated_list1(space1, parse_number)(input)
}
