use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let tiles = parse_input(reader)?;
        let answer = determine_max_area(&tiles) as usize;
        Ok(answer)
    }

    assert_eq!(50, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let tiles = parse_input(reader)?;
        let edges = calculate_edges(&tiles);
        let mut max_area: Option<i64> = None;

        for a in &tiles {
            for b in &tiles {
                if is_valid_area(a, b, &edges) {
                    let area = area(a, b);
                    if max_area.is_none() || area > max_area.unwrap() {
                        max_area = Some(area);
                    }
                }
            }
        }

        Ok(max_area.unwrap() as usize)
    }

    assert_eq!(24, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

type Position = (i64, i64);

#[derive(Debug)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
struct Edge {
    orientation: Orientation,
    position: i64,
    start: i64,
    end: i64,
}

fn is_valid_area(
    a: &Position,
    b: &Position,
    edges: &Vec<Edge>,
) -> bool {
    let left = if a.0 < b.0 { a.0 } else { b.0 };
    let right = if a.0 > b.0 { a.0 } else { b.0 };
    let top = if a.1 < b.1 { a.1 } else { b.1 };
    let bottom = if a.1 > b.1 { a.1 } else { b.1 };

    // Check that no edge crosses the rectangle defined by (left, top) and (right, bottom)
    for edge in edges {
        match edge.orientation {
            Orientation::Horizontal => {
                // Horizontal edge at y = edge.position
                if edge.position > top && edge.position < bottom {
                    // Edge is within vertical bounds of rectangle
                    if edge.start <= right && edge.end >= left {
                        // Edge crosses the rectangle
                        return false;
                    }
                }
            }
            Orientation::Vertical => {
                // Vertical edge at x = edge.position
                if edge.position > left && edge.position < right {
                    // Edge is within horizontal bounds of rectangle
                    if edge.start <= top && edge.end >= bottom {
                        // Edge crosses the rectangle
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn calculate_edges(tiles: &Vec<Position>) -> Vec<Edge> {
    let mut edges = Vec::new();
    let n = tiles.len();

    for i in 0..n-1 {
        let pos_a = tiles[i];
        let pos_b = tiles[i+1];
        let edge = create_edge(&pos_a, &pos_b);
        edges.push(edge);
    }

    let pos_a = tiles[n-1];
    let pos_b = tiles[0];
    let edge = create_edge(&pos_a, &pos_b);
    edges.push(edge);

    edges
}

fn create_edge(
    a: &Position,
    b: &Position,
) -> Edge {
    if a.0 == b.0 {
        // Vertical edge
        let start = if a.1 < b.1 { a.1 } else { b.1 };
        let end = if a.1 > b.1 { a.1 } else { b.1 };
        Edge {
            orientation: Orientation::Vertical,
            position: a.0,
            start,
            end,
        }
    } else {
        // Horizontal edge
        let start = if a.0 < b.0 { a.0 } else { b.0 };
        let end = if a.0 > b.0 { a.0 } else { b.0 };
        Edge {
            orientation: Orientation::Horizontal,
            position: a.1,
            start,
            end,
        }
    }
}


fn determine_max_area(tiles: &Vec<Position>) -> i64 {
    let mut max_area: Option<i64> = None;

    for tile in tiles {
        for other_tile in tiles {
            let area = area(tile, other_tile);
            if max_area.is_none() || area > max_area.unwrap() {
                max_area = Some(area);
            }
        }
    }

    max_area.unwrap()
}

fn area(a: &Position, b: &Position) -> i64 {
    let width = if a.0 > b.0 {
        a.0 - b.0 + 1
    } else {
        b.0 - a.0 + 1
    };
    let height = if a.1 > b.1 {
        a.1 - b.1 + 1
    } else {
        b.1 - a.1 + 1
    };

    width * height
}

fn parse_input<R: BufRead>(reader: R) -> Result<Vec<Position>> {
    let mut ret = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split(',');
        let x = parts
            .next()
            .ok_or_else(|| anyhow!("Missing X part"))?
            .trim()
            .parse::<i64>()?;
        let y = parts
            .next()
            .ok_or_else(|| anyhow!("Missing Y part"))?
            .trim()
            .parse::<i64>()?;
        ret.push((x, y));
    }

    Ok(ret)
}