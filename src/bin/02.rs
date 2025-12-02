use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2025::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        solve(reader, Some(2))
    }

    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        solve(reader, None)
    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn solve<R: BufRead>(reader: R, max_factor_opt: Option<usize>) -> Result<usize> {
    let input = reader.lines().next().unwrap()?;
    let ranges = parse_input_ranges(&input);

    let invalid_numbers = generate_invalid_numbers(10, max_factor_opt);

    Ok(invalid_numbers
        .into_iter()
        .filter(|num| {
            ranges.iter().any(|(low, high)| *num >= *low && *num <= *high)
        })
        .sum())
}

fn parse_input_ranges(input: &str) -> Vec<(usize, usize)> {
    input
        .split(',')
        .filter_map(|part| {
            let bounds: Vec<&str> = part.split('-').collect();
            let low = bounds.get(0)?.parse::<usize>().ok()?;
            let high = bounds.get(1)?.parse::<usize>().ok()?;
            Some((low, high))
        })
        .collect::<Vec<(usize, usize)>>()
}

fn generate_base_strings(size: usize) -> Vec<String> {
    let low = 10usize.pow((size - 1) as u32);
    let high = 10usize.pow(size as u32);
    (low..high).map(|n| n.to_string()).collect()
}

fn generate_invalid_numbers(max_size: usize, max_factor: Option<usize>) -> Vec<usize> {
    let mut invalid_nums = HashSet::new();
    let max_base_size = max_size / 2;

    for size in 1..=max_base_size {
        let base_strings = generate_base_strings(size);
        let mut factor = 2;
        while factor * size <= max_size {
            match max_factor {
                Some(mf) if factor > mf => break,
                _ => {}
            }
            for base in &base_strings {
                let mut num_str = String::new();
                for _ in 0..factor {
                    num_str.push_str(base);
                }
                if let Ok(num) = num_str.parse::<usize>() {
                    invalid_nums.insert(num);
                }
            }
            factor += 1;
        }
    }

    invalid_nums.into_iter().sorted().collect()
}
