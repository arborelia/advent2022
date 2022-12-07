// An example to build from each day
use std::fs;
use parse_display::{Display, FromStr};
use std::collections::HashSet;

pub const TEST_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{word} {num}")]
struct Example {
  word: String,
  num: i64,
}

fn find_n_different(input: &str, n: usize) -> usize {
    let input_chars: Vec<char> = input.chars().collect();
    for endpt in n..input_chars.len() {
        let startpt = endpt - n;
        let charset: HashSet<char> = input_chars[startpt..endpt].iter().cloned().collect();
        if charset.len() == n {
            return endpt;
        }
    }
    panic!("There were never {} different characters", n);
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", find_n_different(&input, 4));
    println!("{}", find_n_different(&input, 14));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(find_n_different(TEST_INPUT, 4), 7);
    }
}
