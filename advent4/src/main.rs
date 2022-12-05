// An example to build from each day
use std::fs;
use parse_display::{Display, FromStr};

pub const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display("{start}-{end}")]
struct Range {
  start: i64,
  end: i64
}

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display("{first},{second}")]
struct RangePair {
    first: Range,
    second: Range
}

impl Range {
    fn contains(self, other: Range) -> bool {
        (self.start <= other.start) && (self.end >= other.end)
    }

    fn contains_endpoint(self, other: Range) -> bool {
        (self.start <= other.start && other.start <= self.end) ||
        (self.start <= other.end && other.end <= self.end)
    }
}

impl RangePair {
    fn is_containment(self) -> bool {
        self.first.contains(self.second) || self.second.contains(self.first)
    }

    fn is_overlap(self) -> bool {
        self.is_containment() || self.first.contains_endpoint(self.second) || self.second.contains_endpoint(self.first)
    }
}

fn num_containments(input: &str) -> i64 {
    let mut containments: i64 = 0;
    for line in input.lines() {
        let rpair: RangePair = line.parse().unwrap();
        if rpair.is_containment() {
            containments += 1;
        }
    }
    containments
}

fn num_overlaps(input: &str) -> i64 {
    let mut containments: i64 = 0;
    for line in input.lines() {
        let rpair: RangePair = line.parse().unwrap();
        if rpair.is_overlap() {
            containments += 1;
        }
    }
    containments
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("part 1: {}", num_containments(&input));
    println!("part 2: {}", num_overlaps(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_containment() {
        let range1 = Range { start: 2, end: 9 };
        let range2 = Range { start: 2, end: 5 };
        assert!(range1.contains(range2));
        assert!(!range2.contains(range1));
        assert!(range1.contains(range1));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            RangePair { first: Range {start: 2, end: 3}, second: Range {start: 4, end: 5} },
            "2-3,4-5".parse().unwrap()
        )
    }

    #[test]
    fn test_example() {
        assert_eq!(num_containments(TEST_INPUT), 2);
    }

    #[test]
    fn test_example2() {
        assert_eq!(num_overlaps(TEST_INPUT), 4);
    }
}
