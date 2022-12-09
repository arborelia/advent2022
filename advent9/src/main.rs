// An example to build from each day
use parse_display::{Display, FromStr};
use std::cmp::max;
use std::collections::HashSet;
use std::fs;

pub const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

pub const BIGGER_TEST_INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

// the rope has a head and tail, and is on an integer grid
// tail is always within 1 space of the head, orthogonally or diagonally
// it gets pulled toward the head, orthogonally or diagonally
// when the head makes the given moves, how many distinct locations does the tail visit?

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display("{}")]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display("{dir} {num}")]
struct RopeMove {
    dir: Direction,
    num: i64,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Position {
    x: i64,
    y: i64,
}

fn make_rope(length: usize) -> Vec<Position> {
    vec![Position { x: 0, y: 0 }; length]
}

/// Move the rope and return the new position of the
fn move_rope(rope: &mut Vec<Position>, mov: &RopeMove, visited: &mut HashSet<Position>) {
    for _ in 0..mov.num {
        match mov.dir {
            Direction::U => rope[0].y -= 1,
            Direction::D => rope[0].y += 1,
            Direction::L => rope[0].x -= 1,
            Direction::R => rope[0].x += 1,
        }
        for idx in 0..(rope.len() - 1) {
            let head = rope[idx];
            let link = &mut rope[idx + 1];
            let dist = max(
                (head.x - link.x).abs(),
                (head.y - link.y).abs(),
            );
            if dist >= 2 {
                if head.x > link.x {
                    link.x += 1;
                } else if head.x < link.x {
                    link.x -= 1;
                }
                if head.y > link.y {
                    link.y += 1;
                } else if head.y < link.y {
                    link.y -= 1;
                }
            }
        }
        let tail = rope[rope.len() - 1];
        visited.insert(tail);
    }
}

fn count_tail_positions(input: &str, rope_length: usize) -> i64 {
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(Position { x: 0, y: 0 });

    let mut rope = make_rope(rope_length);

    for line in input.split("\n") {
        let mov: RopeMove = line.parse().unwrap();
        move_rope(&mut rope, &mov, &mut visited)
    }
    visited.len() as i64
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("tail positions visited with length 2: {}", count_tail_positions(&input, 2));
    println!("tail positions visited with length 10: {}", count_tail_positions(&input, 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "U 4".parse::<RopeMove>().unwrap(),
            RopeMove {
                dir: Direction::U,
                num: 4
            }
        );
    }

    #[test]
    fn test_example() {
        assert_eq!(count_tail_positions(TEST_INPUT, 2), 13);
        assert_eq!(count_tail_positions(TEST_INPUT, 10), 1);
    }

    #[test]
    fn test_bigger_example() {
        assert_eq!(count_tail_positions(BIGGER_TEST_INPUT, 10), 36);
    }
}
