use std::cmp::max;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
            solve(reader, 2)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        solve(reader, 12)
    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn parse_input<R: BufRead>(reader: R) -> Result<Vec<Vec<i32>>> {
    let mut ret = Vec::new();
    let lines = reader
        .lines()
        .collect::<Result<Vec<String>, _>>()?;

    for line in lines {
        let bank = line
            .chars()
            .map(|c| c.to_digit(10).map(|d| d as i32).ok_or_else(|| anyhow!("Invalid digit")))
            .collect::<Result<Vec<i32>, _>>()?;
        ret.push(bank);
    }

    Ok(ret)
}

fn find_max_joltage_path(banks: &Vec<i32>, selection_size: usize) -> Result<usize> {
    let len = banks.len();

    if len < selection_size {
        return Err(anyhow!("Not enough banks to determine max joltage path"));
    }

    let mut selected: Vec<Option<i32>> = vec![None; selection_size];

    for (idx, &joltage) in banks.iter().enumerate() {
        // NUM_SELECTED - i <= len - idx
        let diff = selection_size as i32 - len as i32 + idx as i32;
        let i = if diff > 0 {
            diff as usize
        } else {
            0
        };
        for sel_idx in i..selection_size {
            if selected[sel_idx].is_none() || joltage > selected[sel_idx].unwrap() {
                selected[sel_idx] = Some(joltage);
                for j in (sel_idx + 1)..selection_size {
                    selected[j] = None;
                }
                break;
            }
        }
    }

    if selected.iter().any(|x| x.is_none()) {
        return Err(anyhow!("Not enough banks to determine max joltage path"));
    }

    let selection = selected
        .iter()
        .filter_map(|j_opt| match j_opt {
            Some(j) => Some(j.to_string()),
            None => None,
        })
        .collect::<String>();

    Ok(selection.parse::<usize>()?)
}

fn solve<R: BufRead>(reader: R, selection_size: usize) -> Result<usize> {
    let banks = parse_input(reader)?;
    let answer = banks.iter()
        .map(|bank| find_max_joltage_path(bank, selection_size))
        .collect::<Result<Vec<usize>>>()?
        .iter()
        .sum::<usize>() ;
    Ok(answer)
}
