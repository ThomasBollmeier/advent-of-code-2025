use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

const TEST2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let graph = parse_input(reader)?;
        let answer = count_paths("you", "out", &graph);
        Ok(answer)
    }

    assert_eq!(5, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let graph = parse_input(reader)?;
        let mut cache: Cache = HashMap::new();
        let answer = count_paths2("svr", "out", &graph, false, false, &mut cache);
        Ok(answer)
    }

    assert_eq!(2, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

type Cache = HashMap<(String, bool, bool), usize>;

fn count_paths2(
    start: &str,
    end: &str,
    graph: &HashMap<String, Vec<String>>,
    dac_seen: bool,
    fft_seen: bool,
    cache: &mut Cache,
) -> usize {
    if start == end {
        return if dac_seen && fft_seen { 1 } else { 0 };
    }
    if let Some(cached) = cache.get(&(start.to_string(), dac_seen, fft_seen)) {
        return *cached;
    }

    let mut ret = 0;
    if let Some(neighbors) = graph.get(start) {
        for neighbor in neighbors {
            let new_dac_seen = dac_seen || neighbor == "dac";
            let new_fft_seen = fft_seen || neighbor == "fft";
            ret += count_paths2(neighbor, end, graph, new_dac_seen, new_fft_seen, cache);
        }
    }
    cache.insert((start.to_string(), dac_seen, fft_seen), ret);
    ret
}

fn count_paths(start: &str, end: &str, graph: &HashMap<String, Vec<String>>) -> usize {
    let mut ret = 0;

    let mut todo = VecDeque::new();
    todo.push_back(start.to_string());

    while let Some(node) = todo.pop_front() {
        if node == end {
            ret += 1;
            continue;
        }
        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                todo.push_back(neighbor.to_string());
            }
        }
    }

    ret
}

fn parse_input<R: BufRead>(reader: R) -> Result<HashMap<String, Vec<String>>> {
    let mut ret = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let parts = line
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let node = parts[0].trim_end_matches(':').to_string();
        let neighbors = parts[1..].to_vec();
        ret.insert(node, neighbors);
    }

    Ok(ret)
}
