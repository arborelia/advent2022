// An example to build from each day
use std::fs;
use parse_display::{Display, FromStr};

pub const TEST_INPUT: &str =
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

pub const TEST_STACKS: &str = 
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display("move {howmany} from {source} to {target}")]
struct CrateMove {
    howmany: usize,
    source: usize,
    target: usize
}
 
fn parse_crate_picture(lines: &[&str]) -> Vec<Vec<char>> {
    let nlines = lines.len();
    let nstacks = (lines[0].len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _i in 0..nstacks {
        stacks.push(Vec::new());
    }

    for line_idx in (0..nlines).rev() {
        // convert the line from a string to a Vec<char> so we can index it arbitrarily
        let chars: Vec<char> = lines[line_idx].chars().collect();
        for stack_idx in 0..nstacks {
            let box_char: char = chars[stack_idx * 4 + 1];
            if box_char != ' ' {
                stacks[stack_idx].push(box_char);
            }
        }
    }

    stacks
}

fn top_crates_str(stacks: &[Vec<char>]) -> String {
    let top_chars: Vec<char> = stacks.into_iter().map(|stack| stack[stack.len() - 1]).collect();
    top_chars.into_iter().collect()
}

impl CrateMove {
    fn apply(&self, stacks: &mut [Vec<char>]) {
        for _i in 0..self.howmany {
            let got: char = stacks[self.source - 1].pop().unwrap();
            stacks[self.target - 1].push(got);
        }
    }
}

fn apply_moves(input: &str) -> Vec<Vec<char>> {
    let input_lines: Vec<&str> = input.lines().collect();
    let mut boundary: usize = 0;
    for i in 0..input_lines.len() {
        if input_lines[i].len() == 0 {
            boundary = i;
            break;
        }
    }
    if boundary == 0 {
        panic!("couldn't find the boundary");
    }
    
    let mut stacks: Vec<Vec<char>> = parse_crate_picture(&input_lines[0..boundary]);
    for &line in input_lines[(boundary + 1)..].iter() {
        let crate_move: CrateMove = line.parse().unwrap();
        crate_move.apply(&mut stacks);
    }
    stacks
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let stacks: Vec<Vec<char>> = apply_moves(&input);
    println!("top crates: {}", top_crates_str(&stacks));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let lines: Vec<&str> = TEST_STACKS.lines().collect();
        let crates: Vec<Vec<char>> = parse_crate_picture(&lines);
        assert_eq!(top_crates_str(&crates), "NDP");
    }
    #[test]
    fn test_example() {
        assert_eq!(top_crates_str(&apply_moves(TEST_INPUT)), "CMZ");
    }

}
