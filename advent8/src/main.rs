// An example to build from each day
use std::fs;
use std::default::Default;
use array2d::Array2D;

pub const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

fn parse_grid(input: &str) -> Array2D<u8> {
    let lines: Vec<&str> = input.split("\n").collect();
    let nrows = lines.len();
    let ncols = lines[0].len();
    let mut grid = Array2D::filled_with(0u8, nrows, ncols);
    for (row, line) in input.split("\n").enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let val: u8 = ch as u8 - '0' as u8;
            grid.set(row, col, val).unwrap();
        }
    }
    grid
}

fn rotate_grid<T: Default + Clone + Copy>(grid: Array2D<T>) -> Array2D<T> {
    let nrows = grid.num_rows();
    let ncols = grid.num_columns();
    let mut newgrid = Array2D::filled_with(T::default(), grid.num_rows(), grid.num_columns());
    for col in 0..ncols {
        for row in 0..nrows {
            newgrid.set(ncols - col - 1, row, *grid.get(row, col).unwrap()).unwrap();
        }
    }
    newgrid
}

/// Update a grid of which trees are visible, setting those that are visible from the left to 'true'.
/// Rotating the tree grid and the visibility grid, and applying this four times, should tell us which
/// trees are visible from any direction.
fn update_visibility(grid: &Array2D<u8>, visibility: &mut Array2D<bool>) {
    for row in 0..grid.num_rows() {
        let mut highest_seen: Option<u8> = None;
        for col in 0..grid.num_columns() {
            let &this_height = grid.get(row, col).unwrap();
            let taller = match highest_seen {
                Some(height) => this_height > height,
                None => true
            };
            if taller {
                highest_seen = Some(this_height);
                visibility.set(row, col, true).unwrap();
            }
        }
    }
}

fn num_visible_trees(grid: &Array2D<u8>) -> i64 {
    let mut mygrid = grid.clone();
    let mut visibility = Array2D::filled_with(false, mygrid.num_rows(), mygrid.num_columns());
    for _ in 0..4 {
        update_visibility(&mygrid, &mut visibility);
        mygrid = rotate_grid(mygrid);
        visibility = rotate_grid(visibility);
    }
    let num_visible = visibility.elements_row_major_iter().map(
        |&elem| elem as i64
    ).sum();
    num_visible
}

fn scenic_score(grid: &Array2D<u8>, row: usize, col: usize) -> i64 {
    let nrows = grid.num_rows();
    let ncols = grid.num_columns();
    let start_height = grid.get(row, col).unwrap();

    // look east
    let mut scenery_east: i64 = 0;
    for seen_col in (col + 1)..ncols {
        scenery_east += 1;
        if grid.get(row, seen_col).unwrap() >= start_height {
            break;
        }
    }

    // look west
    let mut scenery_west: i64 = 0;
    for seen_col in (0..col).rev() {
        scenery_west += 1;
        if grid.get(row, seen_col).unwrap() >= start_height {
            break;
        }
    }

    // look south
    let mut scenery_south: i64 = 0;
    for seen_row in (row + 1)..nrows {
        scenery_south += 1;
        if grid.get(seen_row, col).unwrap() >= start_height {
            break;
        }
    }

    // look north
    let mut scenery_north: i64 = 0;
    for seen_row in (0..row).rev() {
        scenery_north += 1;
        if grid.get(seen_row, col).unwrap() >= start_height {
            break;
        }
    }
    scenery_north * scenery_south * scenery_east * scenery_west
}

fn best_scenery(grid: &Array2D<u8>) -> i64 {
    let mut best = 0;
    for row in 0..grid.num_rows() {
        for col in 0..grid.num_columns() {
            let scenery = scenic_score(grid, row, col);
            if scenery > best {
                best = scenery;
            }
        }
    }
    best
}

#[allow(dead_code)]
fn show_visibility(grid: &Array2D<bool>) {
    for row in 0..grid.num_rows() {
        for col in 0..grid.num_columns() {
            let &visible = grid.get(row, col).unwrap();
            let ch = if visible { '#' } else { '.' };
            print!("{}", ch);
        }
        println!("");
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("visible trees: {}", num_visible_trees(&parse_grid(&input)));
    println!("best scenery score: {}", best_scenery(&parse_grid(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        parse_grid(TEST_INPUT);
    }

    #[test]
    fn test_rotate() {
        let orig_grid = parse_grid(TEST_INPUT);
        let mut grid = orig_grid.clone();
        for _ in 0..4 {
            grid = rotate_grid(grid);
        }
        assert_eq!(orig_grid, grid);
        assert_ne!(orig_grid, rotate_grid(grid));
    }

    #[test]
    fn test_example() {
        let grid = parse_grid(TEST_INPUT);
        assert_eq!(num_visible_trees(&grid), 21);
    }

    #[test]
    fn test_scenery() {
        let grid = parse_grid(TEST_INPUT);
        assert_eq!(scenic_score(&grid, 1, 2), 4);
        assert_eq!(scenic_score(&grid, 3, 2), 8);
    }

    #[test]
    fn test_example_part2() {
        let grid = parse_grid(TEST_INPUT);
        assert_eq!(best_scenery(&grid), 8);
    }
}
