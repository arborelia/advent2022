use std::fs;
use parse_display::{Display, FromStr};

pub const TEST_INPUT: &str = "A Y
B X
C Z";

// Implement the string representation of these rock-paper-scissors moves,
// by parsing them with parse_display.

/// An RPS move, represented as either ABC or XYZ.
#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
enum RPSMove {
    #[from_str(regex = "[AX]")]
    Rock,
    #[from_str(regex = "[BY]")]
    Paper,
    #[from_str(regex = "[CZ]")]
    Scissors
}

/// A plan for whether to lose, draw, or win, represented with XYZ.
#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
enum RPSPlan {
    #[display("X")]
    Lose,
    #[display("Y")]
    Draw,
    #[display("Z")]
    Win
}

/// A round in the format of part 1, with their move followed by my move. 
#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{their_move} {my_move}")]
struct RPSRound {
  their_move: RPSMove,
  my_move: RPSMove
}

/// A round in the format of part 2, with their move followed by my plan
/// (whether I should win, lose, or draw).
#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{their_move} {my_plan}")]
struct RPSFixedRound {
    their_move: RPSMove,
    my_plan: RPSPlan
}

impl RPSMove {
    /// The inherent score for playing each move, according to the rules of the problem
    fn move_score(&self) -> i64 {
        match self {
            RPSMove::Rock => 1,
            RPSMove::Paper => 2,
            RPSMove::Scissors => 3
        }
    }

    /// Respond to a move with another move, to fit the plan
    fn respond(&self, plan: RPSPlan) -> RPSMove {
        let plan_diff: i64 = match plan {
            RPSPlan::Win => 1,
            RPSPlan::Draw => 0,
            RPSPlan::Lose => -1
        };
        let value = self.move_score() + plan_diff;
        match value.rem_euclid(3) {
            0 => RPSMove::Scissors,
            1 => RPSMove::Rock,
            2 => RPSMove::Paper,
            _ => panic!("remainder broke")
        }
    }
}


impl RPSRound {
    /// Score a round, awarding 6, 3, or 0 points based on winning, plus the move score
    fn score(&self) -> i64 {
        let diff: i64 = self.my_move.move_score() - self.their_move.move_score();
        // look at the difference mod 3 -- rem_euclid works as 'mod' should work on
        // negative numbers, unlike %
        let win_score: i64 = match diff.rem_euclid(3) {
            1 => 6,
            0 => 3,
            2 => 0,
            _ => panic!("remainder broke")
        };
        self.my_move.move_score() + win_score
    }
}


impl RPSFixedRound {
    fn as_round(&self) -> RPSRound {
        let my_move = self.their_move.respond(self.my_plan);
        RPSRound { their_move: self.their_move, my_move: my_move }
    }

    fn score(&self) -> i64 {
        self.as_round().score()
    }
}

/// The main function for part 1: score a list of move pairs
fn score_strategy(movelist: &str) -> i64 {
    let mut total_score: i64 = 0;
    for line in movelist.split("\n") {
        let round: RPSRound = line.trim().parse().unwrap();
        total_score += round.score();
    }
    total_score
}

/// The main function for part 2: score the string representing their moves and my plan
fn score_plans(movelist: &str) -> i64 {
    let mut total_score: i64 = 0;
    for line in movelist.split("\n") {
        // had to trim \r whitespace because I made input.txt on a Windows computer. :/
        let round: RPSFixedRound = line.trim().parse().unwrap();
        total_score += round.score();
    }
    total_score
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", score_strategy(&input));
    println!("{}", score_plans(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rps() {
        assert_eq!(score_strategy(TEST_INPUT), 15);
    }

    #[test]
    fn test_rps_part2() {
        assert_eq!(score_plans(TEST_INPUT), 12);
    }
}
