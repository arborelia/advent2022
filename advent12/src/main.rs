use array2d::Array2D;
use queues::*;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

const DEBUG: bool = false;

pub const TEST_INPUT: &str = "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    row: i32,
    column: i32,
}

#[derive(Debug, PartialEq, Clone)]
struct State {
    pos: Pos,
    steps: i32,
}

#[derive(Debug)]
struct Map {
    grid: Array2D<u8>,
    start_pos: Pos,
    end_pos: Pos,
}

impl Map {
    fn get(&self, pos: &Pos) -> u8 {
        // check for out-of-bounds, act like the map has huge cliffs around it
        if pos.row < 0 || pos.row >= self.grid.num_rows() as i32 {
            return 100;
        }
        if pos.column < 0 || pos.column >= self.grid.num_columns() as i32 {
            return 100;
        }
        *self
            .grid
            .get(pos.row as usize, pos.column as usize)
            .unwrap()
    }

    fn reachable_positions(&self, pos: &Pos) -> Vec<Pos> {
        let height = self.get(pos);
        let candidates: Vec<Pos> = vec![
            Pos {
                row: pos.row - 1,
                column: pos.column,
            },
            Pos {
                row: pos.row + 1,
                column: pos.column,
            },
            Pos {
                row: pos.row,
                column: pos.column - 1,
            },
            Pos {
                row: pos.row,
                column: pos.column + 1,
            },
        ];
        candidates
            .into_iter()
            .filter(|&candidate| self.get(&candidate) <= height + 1)
            .collect()
    }

    fn search_from(&self, start: Pos) -> Result<i32, Box<dyn Error>> {
        let mut queue: Queue<State> = Queue::new();
        let mut seen: HashSet<Pos> = HashSet::new();
        queue.add(State {
            pos: start,
            steps: 0,
        })?;
        seen.insert(start);

        loop {
            let state: State = match queue.remove() {
                Ok(s) => s,
                Err(_) => {
                    // the queue is empty; return a very long length
                    return Ok(1000000);
                }
            };
            if DEBUG {
                println!(
                    "steps={}, row={}, col={}",
                    state.steps, state.pos.row, state.pos.column
                );
            }
            if state.pos == self.end_pos {
                return Ok(state.steps);
            }
            for next_pos in self.reachable_positions(&state.pos) {
                if !seen.contains(&next_pos) {
                    queue.add(State {
                        pos: next_pos,
                        steps: state.steps + 1,
                    })?;
                    seen.insert(next_pos);
                }
            }
        }
    }

    fn search(&self) -> Result<i32, Box<dyn Error>> {
        Ok(self.search_from(self.start_pos)?)
    }

    fn best_path(&self) -> Result<i32, Box<dyn Error>> {
        let mut best = i32::MAX;
        for row in 0..self.grid.num_rows() {
            for column in 0..self.grid.num_columns() {
                let pos = Pos {
                    row: row as i32,
                    column: column as i32,
                };
                if self.get(&pos) == 0 {
                    let length = self.search_from(pos)?;
                    if length < best {
                        best = length;
                    }
                }
            }
        }
        Ok(best)
    }
}

fn parse_grid(input: &str) -> Map {
    let lines: Vec<&str> = input.trim().lines().collect();
    let nrows = lines.len();
    let ncols = lines[0].len();
    let mut grid: Array2D<u8> = Array2D::filled_with(0, nrows, ncols);

    let mut start_pos: Option<Pos> = None;
    let mut end_pos: Option<Pos> = None;
    for (row, line) in lines.iter().enumerate() {
        for (column, ch) in line.chars().enumerate() {
            let mut height = 0;
            match ch {
                'S' => {
                    start_pos = Some(Pos {
                        row: row as i32,
                        column: column as i32,
                    });
                }
                'E' => {
                    height = 'z' as u8 - 'a' as u8;
                    end_pos = Some(Pos {
                        row: row as i32,
                        column: column as i32,
                    });
                }
                _ => {
                    height = ch as u8 - 'a' as u8;
                }
            }
            grid.set(row, column, height).unwrap();
        }
    }
    Map {
        grid,
        start_pos: start_pos.unwrap(),
        end_pos: end_pos.unwrap(),
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let map: Map = parse_grid(&input);
    println!("path from start: {}", map.search().unwrap());
    println!("best path: {}", map.best_path().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let map: Map = parse_grid(TEST_INPUT);
        assert_eq!(map.search().unwrap(), 31);
    }

    #[test]
    fn test_best_path() {
        let map: Map = parse_grid(TEST_INPUT);
        assert_eq!(map.best_path().unwrap(), 29);
    }
}
