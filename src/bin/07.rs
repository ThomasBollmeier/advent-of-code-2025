use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let setup = parse_input(reader)?;
        let answer = count_splits(&setup);
        Ok(answer)
    }

    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let setup = parse_input(reader)?;
        let answer = count_paths(&setup);
        Ok(answer)
    }

    assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug)]
struct Setup {
    start: usize,
    width: usize,
    splitters: Vec<HashSet<usize>>,
}

fn count_paths(setup: &Setup) -> usize {
    let mut beams = HashMap::new();
    beams.insert(setup.start, 1_usize);

    for splitter_row in &setup.splitters {
        let mut new_beams = HashMap::new();
        for splitter in splitter_row {
            if let Some(&count) = beams.get(splitter) {
                // Split the beam
                if *splitter > 0 {
                    *new_beams.entry(splitter - 1).or_insert(0) += count;
                }
                if *splitter + 1 < setup.width {
                    *new_beams.entry(splitter + 1).or_insert(0) += count;
                }
                beams.remove(splitter);
            }
        }
        for (beam, count) in &beams {
            *new_beams.entry(*beam).or_insert(0) += *count;
        }
        beams = new_beams;
    }

    beams.values().sum()
}

fn count_splits(setup: &Setup) -> usize {
    let mut ret = 0;
    let mut beams = HashSet::new();
    beams.insert(setup.start);

    for splitter_row in &setup.splitters {
        let mut new_beams = HashSet::new();
        for splitter in splitter_row {
            if beams.contains(splitter) {
                // Split the beam
                if *splitter > 0 {
                    new_beams.insert(splitter - 1);
                }
                if *splitter + 1 < setup.width {
                    new_beams.insert(splitter + 1);
                }
                beams.remove(splitter);
                ret += 1;
            }
        }
        for beam in &beams {
            new_beams.insert(*beam);
        }
        beams = new_beams;
    }
    ret
}

fn parse_input<R: BufRead>(reader: R) -> Result<Setup> {
    let mut start = None;
    let mut width = None;
    let mut splitters: Vec<HashSet<usize>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if width.is_none() {
            width = Some(line.len());
        } else if width.unwrap() != line.len() {
            return Err(anyhow!("Inconsistent line widths in input"));
        }
        let mut splitter_row: HashSet<usize> = HashSet::new();
        for (x, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    if start.is_some() {
                        return Err(anyhow!("Multiple start positions found"));
                    }
                    start = Some(x);
                }
                '^' => {
                    splitter_row.insert(x);
                }
                '.' => {}
                _ => return Err(anyhow!("Invalid character in input: {}", ch)),
            }
        }
        if !splitter_row.is_empty() {
            splitters.push(splitter_row);
        }
    }

    let start = start.ok_or_else(|| anyhow!("No start position found"))?;
    let width = width.ok_or_else(|| anyhow!("No input lines found"))?;

    Ok(Setup {
        start,
        width,
        splitters,
    })
}
