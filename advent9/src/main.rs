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

#[derive(PartialEq, Debug)]
struct Rope {
    head: Position,
    tail: Position,
}

/// Move the rope and return the new position of the
fn move_rope(rope: &mut Rope, mov: &RopeMove, visited: &mut HashSet<Position>) {
    for _ in 0..mov.num {
        match mov.dir {
            Direction::U => rope.head.y -= 1,
            Direction::D => rope.head.y += 1,
            Direction::L => rope.head.x -= 1,
            Direction::R => rope.head.x += 1,
        }
        let dist = max(
            (rope.head.x - rope.tail.x).abs(),
            (rope.head.y - rope.tail.y).abs(),
        );
        if dist >= 2 {
            if rope.head.x > rope.tail.x {
                rope.tail.x += 1;
            } else if rope.head.x < rope.tail.x {
                rope.tail.x -= 1;
            }
            if rope.head.y > rope.tail.y {
                rope.tail.y += 1;
            } else if rope.head.y < rope.tail.y {
                rope.tail.y -= 1;
            }
        }
        visited.insert(rope.tail);
    }
}

fn count_tail_positions(input: &str) -> i64 {
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(Position { x: 0, y: 0 });

    let mut rope = Rope {
        head: Position { x: 0, y: 0 },
        tail: Position { x: 0, y: 0 },
    };

    for line in input.split("\n") {
        let mov: RopeMove = line.parse().unwrap();
        move_rope(&mut rope, &mov, &mut visited)
    }
    visited.len() as i64
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("tail positions visited: {}", count_tail_positions(&input));
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
        assert_eq!(count_tail_positions(TEST_INPUT), 13);
    }
}
