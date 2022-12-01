use std::fs;

pub const TEST_INPUT: &'static str = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";


fn max_weight(input: &str) -> i64 {
    let lines = input.split("\n");
    let mut max: i64 = 0;
    let mut current_weight: i64 = 0;
    for line in lines {
        if line.len() == 0 {
            if current_weight > max {
                max = current_weight;
            }
            current_weight = 0;
        } else {
            let val: i64 = line.parse().unwrap();
            current_weight += val;
        }
    }
    max
}

fn max_weight_3(input: &str) -> i64 {
    let lines = input.split("\n");
    let mut weights: Vec<i64> = Vec::new();
    let mut current_weight: i64 = 0;
    for line in lines {
        if line.len() == 0 {
            weights.push(current_weight);
            current_weight = 0;
        } else {
            let val: i64 = line.parse().unwrap();
            current_weight += val;
        }
    }
    weights.sort();
    weights.reverse();
    let total = weights[0..3].iter().sum();
    total
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", max_weight(&input));
    println!("{}", max_weight_3(&input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(max_weight(TEST_INPUT), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(max_weight_3(TEST_INPUT), 45000);
    }
}

