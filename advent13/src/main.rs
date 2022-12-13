use std::error::Error;
use std::fs;
use nom::{
    branch::alt,
    character::complete::char,
    combinator,
    error::{ParseError, context},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair, terminated},
    Err, IResult,
};
use std::collections::HashMap;


const TEST_PAIR: &str = "[[1],[2,3,4]]
[[1],4]";


const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Int(i32),
    Array(Vec<Value>)
}

// parse either an int or an array
fn value<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Value, E> {
    alt((
        combinator::map(array, Value::Array),
        combinator::map(nom::character::complete::i32, Value::Int)
    ))(i)
}

fn array<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Vec<Value>, E> {
    preceded(
        char('['),
        terminated(
            separated_list0(char(','), value),
            char(']')
        )
    )(i)
}

fn input_pair<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (Value, Value), E> {
    let (i, first) = value(i)?;
    let (i, _) = char('\n')(i)?;
    let (i, second) = value(i)?;
    Ok((i, (first, second)))
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (i, (first, second)) = input_pair(TEST_PAIR).unwrap();
        dbg!(i, first, second);
    }
}
