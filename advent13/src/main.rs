use std::cmp::{Ord, Ordering};
use std::fs;
use nom::{
    branch::alt,
    character::complete::char,
    combinator,
    multi::separated_list0,
    sequence::{preceded, terminated},
    IResult,
};


pub const TEST_PAIR: &str = "[[1],[2,3,4]]
[[1],4]";


pub const TEST_INPUT: &str = "[1,1,3,1,1]
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

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    Int(i32),
    Array(Vec<Value>)
}

impl Value {
    fn to_array(&self) -> Vec<Value> {
        match &self {
            Value::Int(n) => vec![Value::Int(*n)],
            Value::Array(arr) => arr.clone()
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Value::Int(my_int) = self {
            if let Value::Int(other_int) = other {
                return my_int.cmp(other_int);
            }
        }
        self.to_array().cmp(&other.to_array())
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// parse either an int or an array
fn value<'a>(i: &'a str) -> IResult<&'a str, Value> {
    // skip a blank line before an array (useful in the outermost array)
    let (i, _) = combinator::opt(char('\n'))(i)?;
    alt((
        combinator::map(array, Value::Array),
        combinator::map(nom::character::complete::i32, Value::Int)
    ))(i)
}

fn array<'a>(i: &'a str) -> IResult<&'a str, Vec<Value>> {
    preceded(
        char('['),
        terminated(
            separated_list0(char(','), value),
            char(']')
        )
    )(i)
}

fn parse_value_exact(i: &str) -> Value {
    let (i, value) = value(i).unwrap();
    if i.len() > 0 {
        panic!("There was leftover input: {}", i);
    }
    value
}

fn input_pair_parser<'a>(i: &'a str) -> IResult<&'a str, (Value, Value)> {
    let (i, first) = value(i)?;
    let (i, _) = char('\n')(i)?;
    let (i, second) = value(i)?;
    Ok((i, (first, second)))
}

fn input_list_parser<'a>(i: &'a str) -> IResult<&'a str, Vec<Value>> {
    separated_list0(char('\n'), value)(i)
}

fn sum_ordered(input: &str) -> i32 {
    let mut sum = 0;
    for (idx, pair_input) in input.split("\n\n").enumerate() {
        let (remaining, (first, second)) = input_pair_parser(pair_input).unwrap();
        if remaining.trim().len() > 0 {
            panic!("There was input left over: {}", remaining);
        }
        if first <= second {
            sum += idx as i32 + 1;
        }
    }
    sum
}

fn find_divider_packets(input: &str) -> i32 {
    let dividers: Vec<Value> = vec![
        parse_value_exact("[[2]]"),
        parse_value_exact("[[6]]")
    ];

    let (_, mut packets) = input_list_parser(input).unwrap();
    packets.extend(dividers.clone());
    packets.sort();
    let mut decoder_key: i32 = 1;
    for (i, packet) in packets.iter().enumerate() {
        if packet == &dividers[0] || packet == &dividers[1] {
            decoder_key *= (i + 1) as i32;
        }
    }
    decoder_key
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Sum of ordered packets: {}", sum_ordered(&input));
    println!("Decoder key: {}", find_divider_packets(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair() {
        let (i, (first, second)) = input_pair_parser(TEST_PAIR).unwrap();
        dbg!(i, first.clone(), second.clone());
        assert!(first < second);
        assert!(second > first);
        assert!(first == first);
    }

    #[test]
    fn test_list() {
        let (i, values) = input_list_parser(TEST_INPUT).unwrap();
        assert_eq!(values.len(), 16);
        assert_eq!(i.len(), 0);
    }

    #[test]
    fn test_example() {
        assert_eq!(sum_ordered(TEST_INPUT), 13);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(find_divider_packets(TEST_INPUT), 140);
    }
}
