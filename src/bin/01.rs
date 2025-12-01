use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let mut position = 50; // Starting position
        for line in reader.lines() {
            let direction  = line?;
            let rotation = parse_rotation(&direction)?;
            position = rotate(position, &rotation);
            if position == 0 {
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
        let mut answer = 0;
        let mut position = 50; // Starting position
        for line in reader.lines() {
            let direction  = line?;
            let rotation = parse_rotation(&direction)?;
            let (new_position, cnt_zero_crossed)  = rotate2(position, &rotation);
            answer += cnt_zero_crossed;
            position = new_position;
        }
        Ok(answer as usize)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

fn parse_rotation(s: &str) -> Result<Rotation> {
    let (dir, delta_str) = s.split_at(1);
    let delta = delta_str.parse()?;
    match dir {
        "L" => Ok(Rotation::Left(delta)),
        "R" => Ok(Rotation::Right(delta)),
        _ => Err(anyhow!("Invalid rotation direction: {}", dir)),
    }
}

fn rotate(position: i32, rotation: &Rotation) -> i32 {
    match rotation {
        Rotation::Left(delta) => {
            let mut pos = position - delta;
            while pos < 0 {
                pos += 100;
            }
            pos
        }
        Rotation::Right(delta) => {
            let mut pos = position + delta;
            while pos >= 100 {
                pos -= 100;
            }
            pos
        }
    }
}

fn rotate2(position: i32, rotation: &Rotation) -> (i32, i32) {
    match rotation {
        Rotation::Left(delta) => {
            let mut pos = position - delta;
            while pos < 0 {
                pos += 100;
            }
            let cnt_zero_crossed= if *delta >= position {
                if position != 0 {
                    1 + (delta - position) / 100
                } else {
                    delta / 100
                }
            } else {
                0
            };
            (pos, cnt_zero_crossed)
        }
        Rotation::Right(delta) => {
            let mut pos = position + delta;
            while pos >= 100 {
                pos -= 100;
            }
            let cnt_zero_crossed= if position + *delta >= 100 {
                1 + (delta + position - 100) / 100
            } else {
                0
            };
            (pos, cnt_zero_crossed)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate2() {
        let (new_pos, cnt) = rotate2(50, &Rotation::Right(1000));
        assert_eq!(new_pos, 50);
        assert_eq!(cnt, 10);

        let (new_pos, cnt) = rotate2(50, &Rotation::Right(50));
        assert_eq!(new_pos, 0);
        assert_eq!(cnt, 1);

        let (new_pos, cnt) = rotate2(50, &Rotation::Left(50));
        assert_eq!(new_pos, 0);
        assert_eq!(cnt, 1);
    }
}


