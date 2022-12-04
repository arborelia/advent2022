// An example to build from each day
use std::fs;
use parse_display::{Display, FromStr};
use std::collections::{HashSet};
use intersection::hash_set::intersection;

pub const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";


fn char_set(string: &str) -> HashSet<char> {
    HashSet::from_iter(string.chars())
}

fn intersect_halves(string: &str) -> char {
    let split: usize = string.len() / 2;
    let first_half: &str = &string[0..split];
    let second_half: &str = &string[split..];
    let first_chars = char_set(first_half);
    let second_chars = char_set(second_half);
    if let Some(&ch) = first_chars.intersection(&second_chars).next() {
        ch
    } else {
        panic!("No chars in intersection")
    }
}

fn intersect_triple(s1: &str, s2: &str, s3: &str) -> char {
    let set1 = char_set(s1);
    let set2 = char_set(s2);
    let set3 = char_set(s3);
    if let Some(&ch) = intersection([set1, set2, set3]).iter().next() {
        ch
    } else {
        panic!("No chars in intersection")
    }
}

fn letter_value(ch: char) -> i64 {
    if 'a' <= ch && ch <= 'z' {
        (ch as i64 - 'a' as i64) + 1
    } else if 'A' <= ch && ch <= 'Z' {
        (ch as i64 - 'A' as i64) + 27
    } else {
        panic!("not a letter")
    }
}

fn intersect_halves_sum(input: &str) -> i64 {
    let mut total: i64 = 0;
    for line in input.lines() {
        total += letter_value(intersect_halves(line.trim()));
    }
    total
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", intersect_halves_sum(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect_1() {
        assert_eq!(intersect_halves("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
    }

    #[test]
    fn test_intersect_2() {
        assert_eq!(intersect_halves("AA"), 'A');
    }

    #[test]
    fn test_letter_values() {
        assert_eq!(letter_value('p'), 16);
        assert_eq!(letter_value('L'), 38);
    }
}
