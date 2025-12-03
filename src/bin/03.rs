use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;

    for line in lines {
        let bank = line
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|d| d as i32)
                    .ok_or_else(|| anyhow!("Invalid digit"))
            })
            .collect::<Result<Vec<i32>, _>>()?;
        ret.push(bank);
    }

    Ok(ret)
}

fn select_max(numbers: &[i32], idx_from: usize, remaining: usize) -> Result<Vec<i32>> {
    if remaining == 0 {
        return Ok(Vec::new());
    }

    let idx_to = numbers.len() as i32 - remaining as i32;
    if idx_to < 0 {
        return Err(anyhow!(
            "Cannot select {} out of {}",
            remaining,
            numbers.len()
        ));
    }
    let idx_to = idx_to as usize;
    let mut max_value: Option<i32> = None;
    let mut max_idx = -1;

    for i in idx_from..=idx_to {
        if max_value.is_none() || numbers[i] > max_value.unwrap() {
            max_value = Some(numbers[i]);
            max_idx = i as i32;
        }
    }

    if max_idx == -1 {
        return Err(anyhow!(("No maximum found")));
    }

    let mut ret = vec![max_value.unwrap()];
    ret.extend(select_max(numbers, (max_idx + 1) as usize, remaining - 1)?);

    Ok(ret)
}

fn select_max_joltage(bank: &Vec<i32>, selection_size: usize) -> Result<usize> {
    let selection = select_max(bank, 0, selection_size)?
        .iter()
        .map(|j| j.to_string())
        .collect::<String>();

    Ok(selection.parse::<usize>()?)
}

fn solve<R: BufRead>(reader: R, selection_size: usize) -> Result<usize> {
    let banks = parse_input(reader)?;
    let answer = banks
        .iter()
        .map(|bank| select_max_joltage(bank, selection_size))
        .collect::<Result<Vec<usize>>>()?
        .iter()
        .sum::<usize>();
    Ok(answer)
}
