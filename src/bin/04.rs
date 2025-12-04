use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = parse_input(reader)?;
        Ok(count_accessible_rolls(&grid))
    }

    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut grid = parse_input(reader)?;
        let mut answer = 0;

        loop {
            let accessible_rolls = find_accessible_rolls(&grid);
            if accessible_rolls.is_empty() {
                break;
            }
            answer += accessible_rolls.len();

            for (x, y) in accessible_rolls {
                // Mark the cell as empty
                grid[y][x] = Cell::Empty;
            }
        }

        Ok(answer)
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn find_accessible_rolls(grid: &Vec<Vec<Cell>>) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    let h = grid.len();
    let w = grid[0].len();

    for (row_idx, row) in grid.iter().enumerate() {
        for (cell_idx, cell) in row.iter().enumerate() {
            match cell {
                Cell::Filled => {
                    let filled_neighbors = count_filled_neighbors(grid, h, w, row_idx, cell_idx);
                    if filled_neighbors < 4 {
                        ret.push((cell_idx, row_idx));
                    }
                }
                Cell::Empty => {}
            }
        }
    }

    ret
}

fn count_accessible_rolls(grid: &Vec<Vec<Cell>>) -> usize {
    let mut ret = 0;
    let h = grid.len();
    let w = grid[0].len();

    for (row_idx, row) in grid.iter().enumerate() {
        for (cell_idx, cell) in row.iter().enumerate() {
            match cell {
                Cell::Filled => {
                    let filled_neighbors = count_filled_neighbors(grid, h, w, row_idx, cell_idx);
                    if filled_neighbors < 4 {
                        ret += 1;
                    }
                }
                Cell::Empty => {}
            }
        }
    }

    ret
}

fn count_filled_neighbors(
    grid: &Vec<Vec<Cell>>,
    height: usize,
    width: usize,
    row: usize,
    col: usize,
) -> usize {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let valid_positions = directions.iter().filter_map(|(dr, dc)| {
        let x = col as isize + dc;
        if x < 0 || x >= width as isize {
            return None;
        }
        let y = row as isize + dr;
        if y < 0 || y >= height as isize {
            return None;
        }
        Some((x as usize, y as usize))
    });

    valid_positions
        .filter(|(x, y)| grid[*y][*x] == Cell::Filled)
        .count()
}

fn parse_input<R: BufRead>(reader: R) -> Result<Vec<Vec<Cell>>> {
    let lines = reader.lines();
    let mut grid = Vec::new();

    for line in lines {
        let line = line?;
        let mut row = Vec::new();
        for ch in line.chars() {
            let cell = match ch {
                '@' => Cell::Filled,
                '.' => Cell::Empty,
                _ => return Err(anyhow!("Invalid character in input: {}", ch)),
            };
            row.push(cell);
        }
        grid.push(row);
    }

    Ok(grid)
}

#[derive(Debug, PartialEq)]
enum Cell {
    Filled,
    Empty,
}
