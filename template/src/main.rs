// An example to build from each day
use std::error::Error;
use std::fs;
use parse_display::{Display, FromStr};

pub const TEST_INPUT: &str = "bee 20";

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{word} {num}")]
struct Example {
    word: String,
    num: i64,
}


fn get_num(input: &str) -> i64 {
    let parsed: Example = input.parse().unwrap();
    parsed.num
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
    }
}
