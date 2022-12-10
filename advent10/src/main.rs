// An example to build from each day
use std::fs;
use parse_display::{Display, FromStr};

pub const SMALL_EXAMPLE: &str = "noop
addx 3
addx -5";

pub const SMALL_EXAMPLE_2: &str = "noop
addx 3
addx -5
noop";


pub const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

#[derive(Display, FromStr, PartialEq, Debug)]
enum CPUInstruction {
    #[display("addx {0}")]
    AddX(i64),
    #[display("noop")]
    Noop,
}

fn signal_strength(input: &str, start: i64, interval: i64, stop: i64) -> i64 {
    let mut elapsed: i64 = 0;
    let mut next_check: i64 = start;
    let mut register: i64 = 1;
    let mut signal: i64 = 0;
    for line in input.split("\n") {
        let inst: CPUInstruction = line.parse().unwrap();
        let mut queued_change: i64 = 0;
        match inst {
            CPUInstruction::Noop => { elapsed += 1 },
            CPUInstruction::AddX(val) => {
                elapsed += 2;
                queued_change = val;
            }
        }

        if elapsed >= next_check {
            signal += next_check * register;
            next_check += interval;
        }
        register += queued_change;
        if elapsed >= stop {
            break;
        }

    }
    signal
}


fn draw_pixel(register: i64, elapsed: i64, row_size: i64) {
    let xpos = elapsed % row_size;
    if xpos == 0 {
        println!();
    }
    if (register - xpos).abs() <= 1 {
        print!("#");
    } else {
        print!(".");
    }
}


fn draw_sprite(input: &str, row_size: i64) {
    let mut elapsed: i64 = 0;
    let mut register: i64 = 1;
    for line in input.split("\n") {
        let inst: CPUInstruction = line.parse().unwrap();
        match inst {
            CPUInstruction::Noop => {
                draw_pixel(register, elapsed, row_size);
                elapsed += 1;
            },
            CPUInstruction::AddX(val) => {
                draw_pixel(register, elapsed, row_size);
                elapsed += 1;
                draw_pixel(register, elapsed, row_size);
                elapsed += 1;
                register += val;
            }
        }
    }
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("signal strength: {}", signal_strength(&input, 20, 40, 220));

    draw_sprite(&input, 40);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let parsed: CPUInstruction = "addx 2".parse().unwrap();
        assert_eq!(parsed, CPUInstruction::AddX(2));
    }

    #[test]
    fn test_small() {
        assert_eq!(
            signal_strength(SMALL_EXAMPLE, 1, 2, 5),
            (1 * 1) + (3 * 1) + (5 * 4)
        );
        assert_eq!(
            signal_strength(SMALL_EXAMPLE_2, 2, 2, 6),
            (2 * 1) + (4 * 4) + (6 * -1)
        );
    }

    #[test]
    fn test_example() {
        assert_eq!(
            signal_strength(TEST_INPUT, 20, 40, 220),
            13140
        )
    }
}
