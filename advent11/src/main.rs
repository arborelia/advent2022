// An example to build from each day
use std::cell::RefCell;
use std::error::Error;
use std::fs;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;
use parse_display::{Display, FromStr};

const DEBUG: bool = false;

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
struct ItemList(RefCell<Vec<i64>>);

impl FromStr for ItemList {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums: Vec<i64> = Vec::new();
        for num_str in s.split(", ") {
            nums.push(num_str.parse()?);
        }
        let cell = RefCell::new(nums);
        Ok(ItemList(cell))
    }
}

impl fmt::Display for ItemList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let items: &Vec<i64> = &self.0.borrow();
        for (idx, num) in items.iter().enumerate() {
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
    fn inspect_and_throw_item(&self, worry: i64, chill_out: bool) -> (usize, i64) {
        let mut value = worry;
        // println!("Monkey {} inspects an item with a worry level of {}.", self.id, worry);
        value = self.op.apply(value);
        // println!("Value after operation is {}.", value);
        if chill_out {
            value /= 3;
        }
        // println!("Value is divided by 3 to give {}.", value);
        let target = if value % self.div == 0 {
            // println!("Value is divisible by {}.", self.div);
            self.true_target
        } else {
            // println!("Value is not divisible by {}.", self.div);
            self.false_target
        };
        // println!("Item with worry level {} is thrown to monkey {}.\n", value, target);
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

fn run_monkey_game(input: &str, num_rounds: usize, chill_out: bool) -> Result<i64, Box<dyn Error>> {
    let monkeys: Vec<Monkey> = parse_monkey_list(input)?;

    let modulo: i64 = monkeys.iter().map(|monkey| monkey.div).product();
    dbg!(modulo);
    let mut monkey_business: Vec<i64> = vec![0; monkeys.len()];
    for _ in 0..num_rounds {
        if DEBUG {
            for monkey in &monkeys {
                println!("Monkey {}: {}", monkey.id, monkey.items);
            }
            println!();
        }

        for monkey in &monkeys {
            let items: &mut Vec<i64> = &mut monkey.items.0.borrow_mut();
            for item in items.iter() {
                let (target, value) = monkey.inspect_and_throw_item(*item, chill_out);
                monkey_business[monkey.id] += 1;
                let target_item_list: &mut Vec<i64> = &mut monkeys[target].items.0.borrow_mut();
                target_item_list.push(value % modulo);
            }
            items.drain(..);
        }
    }
    monkey_business.sort();
    monkey_business.reverse();
    dbg!(monkey_business.clone());
    Ok(monkey_business[0] * monkey_business[1])
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt").unwrap();
    let monkey_business = run_monkey_game(&input, 20, true)?;
    println!("monkey business after 20 rounds: {}", monkey_business);
    let monkey_business_2 = run_monkey_game(&input, 10000, false)?;
    println!("monkey business after 10000 non-chill rounds: {}", monkey_business_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<(), Box<dyn Error>> {
        let monkey_business = run_monkey_game(TEST_INPUT, 20, true)?;
        assert_eq!(monkey_business, 10605);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<(), Box<dyn Error>> {
        let monkey_business = run_monkey_game(TEST_INPUT, 10000, false)?;
        assert_eq!(monkey_business, 2713310158);
        Ok(())
    }
}
