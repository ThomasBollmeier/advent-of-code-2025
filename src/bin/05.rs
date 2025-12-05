use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (intervals, food) = parse_input(reader)?;
        let mut answer = 0;

        for &item in &food {
            if intervals
                .iter()
                .any(|&(start, end)| item >= start && item <= end)
            {
                answer += 1;
            }
        }

        Ok(answer)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut intervals = parse_intervals(reader)?;
        merge_intervals(&mut intervals);

        let mut answer = 0;
        for interval in &intervals {
            answer += (interval.1 - interval.0 + 1) as usize;
        }
        Ok(answer)
    }

    assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

type FoodId = i64;

fn parse_input<R: BufRead>(reader: R) -> Result<(Vec<(FoodId, FoodId)>, Vec<FoodId>)> {
    let mut intervals = Vec::new();
    let mut food = Vec::new();
    let regex_interval = regex::Regex::new(r"^(\d+)-(\d+)$")?;

    for line in reader.lines() {
        let line = line?.trim().to_string();
        if regex_interval.is_match(&line) {
            let caps = regex_interval.captures(&line).unwrap();
            let start = caps.get(1).unwrap().as_str().parse()?;
            let end = caps.get(2).unwrap().as_str().parse()?;
            intervals.push((start, end));
        } else if !line.is_empty() {
            let value = line.parse()?;
            food.push(value);
        }
    }

    Ok((intervals, food))
}

fn parse_intervals<R: BufRead>(reader: R) -> Result<Vec<(FoodId, FoodId)>> {
    let mut intervals = Vec::new();
    let regex_interval = regex::Regex::new(r"^(\d+)-(\d+)$")?;

    for line in reader.lines() {
        let line = line?.trim().to_string();
        if regex_interval.is_match(&line) {
            let caps = regex_interval.captures(&line).unwrap();
            let start = caps.get(1).unwrap().as_str().parse()?;
            let end = caps.get(2).unwrap().as_str().parse()?;
            intervals.push((start, end));
        } else {
            break;
        }
    }

    Ok(intervals)
}

fn merge_intervals(intervals: &mut Vec<(FoodId, FoodId)>) {
    if intervals.len() <= 1 {
        return;
    }

    intervals.sort_by_key(|&(start, _)| start);
    let mut i = 0;
    while i < intervals.len() - 1 {
        if has_overlap(intervals[i], intervals[i + 1]) {
            let merged = merge(intervals[i], intervals[i + 1]);
            intervals.splice(i..=i + 1, [merged]);
        } else {
            i += 1;
        }
    }
}

fn has_overlap(a: (FoodId, FoodId), b: (FoodId, FoodId)) -> bool {
    let x = a.0.max(b.0);
    let y = a.1.min(b.1);
    x <= y
}

fn merge(a: (FoodId, FoodId), b: (FoodId, FoodId)) -> (FoodId, FoodId) {
    (a.0.min(b.0), a.1.max(b.1))
}