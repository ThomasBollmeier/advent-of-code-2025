use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        solve(reader, &parse_input)
    }

    assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        solve(reader, &parse_input2)
    }

    assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

fn solve<R: BufRead>(reader: R, parser: &dyn Fn(R) -> Result<Vec<(Operator, Vec<usize>)>>) -> Result<usize> {
    let mut answer = 0;
    let operations = parser(reader)?;

    for (op, operands) in operations {
        let result: usize = match op {
            Operator::Add => operands.iter().sum(),
            Operator::Multiply => operands.iter().product(),
        };
        answer += result;
    }

    Ok(answer)
}

fn parse_input<R: BufRead>(reader: R) -> Result<Vec<(Operator, Vec<usize>)>> {
    let mut all_operands: Vec<Vec<usize>> = vec![];
    let mut operators: Vec<Operator> = vec![];

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let first = parts[0];
        match first {
            "+" | "*" => {
                operators = parts
                    .iter()
                    .map(|s| match *s {
                        "+" => Ok(Operator::Add),
                        "*" => Ok(Operator::Multiply),
                        _ => Err(anyhow!("Invalid operator: {}", s)),
                    })
                    .collect::<Result<Vec<Operator>, _>>()?;
            }
            _ => {
                let operands = parts
                    .iter()
                    .map(|s| s.parse::<usize>())
                    .collect::<Result<Vec<usize>, _>>()?;
                if !all_operands.is_empty() {
                    for (i, operand) in operands.iter().enumerate() {
                        all_operands[i].push(*operand);
                    }
                } else {
                    all_operands = operands.iter().map(|i| vec![*i]).collect();
                }
            }
        }
    }

    let ret = operators
        .iter()
        .zip(all_operands)
        .map(|(op, ops)| (op.clone(), ops))
        .collect();

    Ok(ret)
}

fn parse_input2<R: BufRead>(reader: R) -> Result<Vec<(Operator, Vec<usize>)>> {
    let mut lines = reader
        .lines()
        .flat_map(|line| line.ok())
        .collect::<Vec<String>>();

    let operators = lines
        .last()
        .ok_or_else(|| anyhow!("No lines found"))?
        .chars()
        .flat_map(|c| match c {
            '+' => Some(Operator::Add),
            '*' => Some(Operator::Multiply),
            _ => None,
        })
        .collect::<Vec<Operator>>();

    let operands_lines = &lines[..lines.len() - 1];
    let max_len = operands_lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .ok_or_else(|| anyhow!("No operand lines found"))?;

    let mut column_strs = vec![String::new(); max_len];
    for operand_line in operands_lines {
        for (i, c) in operand_line.chars().enumerate() {
            if c != ' ' {
                column_strs[i].push(c);
            }
        }
    }

    let mut all_operands: Vec<Vec<usize>> = vec![];
    let mut operands: Vec<usize> = vec![];

    for column_str in column_strs {
        if column_str.is_empty() {
            if !operands.is_empty() {
                all_operands.push(operands);
                operands = vec![];
            }
            continue;
        }
        let operand = column_str.parse::<usize>()?;
        operands.push(operand);
    }

    if !operands.is_empty() {
        all_operands.push(operands);
    }

    let ret = operators
        .iter()
        .zip(all_operands)
        .map(|(op, ops)| (op.clone(), ops))
        .collect();

    Ok(ret)
}
