use std::collections::{HashSet, VecDeque};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let problems = parse_input(reader)?;

        for problem in &problems {
            let steps = solve_problem(problem)?;
            answer += steps;
        }

        Ok(answer)
    }

    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let problems = parse_input(reader)?;

        for problem in &problems {
            let steps = solve_problem2(problem)?;
            answer += steps;
        }

        Ok(answer)
    }

    assert_eq!(33, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug)]
struct Problem {
    goal: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages : Vec<usize>,
}

fn solve_problem2(problem: &Problem) -> Result<usize> {
    let n = problem.joltages.len();
    let start = vec![0; n];
    let mut todo = VecDeque::new();
    todo.push_back((start, 0));
    let mut visited = HashSet::new();

    while let Some((state, steps)) = todo.pop_front() {
        if state == problem.joltages {
            return Ok(steps);
        }

        visited.insert(state_str2(&state));

        for button in &problem.buttons {
            let mut new_state = state.clone();
            for &pos in button {
                if pos < n {
                    new_state[pos] +=1;
                }
            }
            let new_state_str = state_str2(&new_state);
            if !visited.contains(&new_state_str) && is_goal_reachable(&new_state, &problem.joltages) {
                todo.push_back((new_state, steps + 1));
            }
        }
    }

    Err(anyhow!("No solution found"))
}

fn is_goal_reachable(state: &Vec<usize>, goal: &Vec<usize>) -> bool {
    for (s, g) in state.iter().zip(goal.iter()) {
        if s > g {
            return false;
        }
    }
    true
}

fn state_str2(state: &Vec<usize>) -> String {
    state.iter().map(|&i| i.to_string()).collect::<Vec<String>>().join(",")
}

fn solve_problem(problem: &Problem) -> Result<usize> {
    let n = problem.goal.len();
    let start = vec![false; n];
    let mut todo = VecDeque::new();
    todo.push_back((start, 0));
    let mut visited = HashSet::new();

    while let Some((state, steps)) = todo.pop_front() {
        if state == problem.goal {
            return Ok(steps);
        }

        visited.insert(state_str(&state));

        for button in &problem.buttons {
            let mut new_state = state.clone();
            for &pos in button {
                if pos < n {
                    new_state[pos] = !new_state[pos];
                }
            }
            let new_state_str = state_str(&new_state);
            if !visited.contains(&new_state_str) {
                todo.push_back((new_state, steps + 1));
            }
        }
    }

    Err(anyhow!("No solution found"))
}

fn state_str(state: &Vec<bool>) -> String {
    state.iter().map(|&b| if b { '#' } else { '.' }).collect()
}


fn parse_input<R: BufRead>(reader: R) -> Result<Vec<Problem>> {
    let mut problems = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let problem = parse_line(&line)?;
        problems.push(problem);
    }
    Ok(problems)
}

fn parse_line(line: &str) -> Result<Problem> {
    let parts: Vec<&str> = line.split(' ').collect();
    let mut goal = Vec::new();
    let mut buttons = Vec::new();
    let mut joltages = Vec::new();

    for part in parts {
        if part.starts_with('[') {
            for ch in part.chars().skip(1) {
                match ch {
                    '#' => goal.push(true),
                    '.' => goal.push(false),
                    ']' => break,
                    _ => return Err(anyhow!("Invalid character in goal")),
                }
            }
        }

        if part.starts_with('(') {
            let button_str = &part[1..part.len()-1];
            let button = button_str
                .split(',')
                .map(|s| s.parse::<usize>())
                .collect::<Result<Vec<usize>, _>>()?;
            // Store buttons as needed
            buttons.push(button);
        }

        if part.starts_with('{') {
            let joltage_str = &part[1..part.len()-1];
            joltages = joltage_str
                .split(',')
                .map(|s| s.parse::<usize>())
                .collect::<Result<Vec<usize>, _>>()?;
        }

    }

    Ok(Problem { goal, buttons, joltages })
}