// An example to build from each day
use std::error::Error;
use std::fs;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;
use parse_display::{Display, FromStr};

pub const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("Monkey {id}:
  Starting items: {items}
  Operation: {op}
  Test: divisible by {div}
    If true: throw to monkey {true_target}
    If false: throw to monkey {false_target}")]
struct Monkey {
    id: usize,
    items: ItemList,
    op: Operation,
    div: i64,
    true_target: usize,
    false_target: usize,
    #[from_str(default)]
    activity: i64
}

#[derive(Display, FromStr, PartialEq, Debug)]
enum Operation {
    #[display("new = old + {0}")]
    Add(i64),
    #[display("new = old * old")]
    Square,
    #[display("new = old * {0}")]
    Mul(i64),
}
use Operation::*;

impl Operation {
    fn apply(&self, val: i64) -> i64 {
        match self {
            Add(n) => val + n,
            Square => val * val,
            Mul(n) => val * n
        }
    }
}

#[derive(PartialEq, Debug)]
struct ItemList(Vec<i64>);

impl FromStr for ItemList {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums: Vec<i64> = Vec::new();
        for num_str in s.split(", ") {
            nums.push(num_str.parse()?);
        }

        Ok(ItemList(nums))
    }
}

impl fmt::Display for ItemList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, num) in self.0.iter().enumerate() {
            if idx > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", num)?;
        }
        Ok(())
    }
}

impl Monkey {
    /// Implement the thing where a monkey inspects an item and throws it to another monkey.
    /// Returns a tuple of the target monkey index, and the new "worry value" of the item.
    fn inspect_and_throw_item(&self, worry: i64) -> (usize, i64) {
        let mut value = worry;
        println!("Monkey {} inspects an item with a worry level of {}.", self.id, worry);
        value = self.op.apply(value);
        println!("Value after operation is {}.", value);
        value /= 3;
        println!("Value is divided by 3 to give {}.", value);
        let target = if value % self.div == 0 {
            println!("Value is divisible by {}.", self.div);
            self.true_target
        } else {
            println!("Value is not divisible by {}.", self.div);
            self.false_target
        };
        println!("Item with worry level {} is thrown to monkey {}.\n", value, target);
        (target, value)
    }
}

fn parse_monkey_list(input: &str) -> Result<Vec<Monkey>, Box<dyn Error>> {
    let mut monkeys = Vec::new();
    for chunk in input.split("\n\n") {
        monkeys.push(chunk.parse()?);
    }
    Ok(monkeys)
}

fn run_monkey_game(input: &str, num_rounds: usize) -> Result<i64, Box<dyn Error>> {
    let mut monkeys: Vec<Monkey> = parse_monkey_list(input)?;
    for _ in 0..num_rounds {
        for monkey in &monkeys {
            println!("Monkey {}: {}", monkey.id, monkey.items);
        }
        println!();

        let mut airborne_items: Vec<Vec<i64>> = vec![Vec::new(); monkeys.len()];
        for monkey in &mut monkeys {
            for item in &monkey.items.0 {
                let (target, value) = monkey.inspect_and_throw_item(*item);
                airborne_items[target].push(value);
                monkey.activity += 1;
            }
        }
        for i in 0..monkeys.len() {
            monkeys[i].items = ItemList(airborne_items[i].clone());
        }
    }
    let mut monkey_business: Vec<i64> = monkeys.iter().map(|monkey| monkey.activity).collect();
    monkey_business.sort();
    monkey_business.reverse();
    dbg!(monkey_business.clone());
    Ok(monkey_business[0] * monkey_business[1])
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<(), Box<dyn Error>> {
        let monkey_business = run_monkey_game(TEST_INPUT, 2)?;
        assert_eq!(monkey_business, 10605);
        Ok(())
    }
}
