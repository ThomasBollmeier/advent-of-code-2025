use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, n: usize) -> Result<usize> {
        let coordinates = parse_input(reader)?;
        let distances = calc_distances(&coordinates);
        let group_sizes = build_groups_from_first_n(coordinates.len(), &distances, n);
        Ok(group_sizes[0] * group_sizes[1] * group_sizes[2])
    }

    assert_eq!(40, part1(BufReader::new(TEST.as_bytes()), 10)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 1000)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let coordinates = parse_input(reader)?;
        let distances = calc_distances(&coordinates);
        let (coord_a, coord_b) = unite_all_groups(&coordinates, &distances)?;
        Ok(coord_a.0 as usize * coord_b.0 as usize)
    }

    assert_eq!(25272, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

type Coordinate = (i64, i64, i64);

fn unite_all_groups(
    coords: &Vec<Coordinate>,
    distances: &Vec<(i64, usize, usize)>,
) -> Result<(Coordinate, Coordinate)> {
    let num_coords = coords.len();
    let mut groups = init_groups(num_coords);

    for (_dist, a, b) in distances {
        merge(&mut groups, *a, *b);
        if groups.len() == 1 {
            let coord_a = coords[*a];
            let coord_b = coords[*b];
            return Ok((coord_a, coord_b));
        }
    }

    Err(anyhow!("Cannot unite all groups"))
}

fn build_groups_from_first_n(
    num_coords: usize,
    distances: &Vec<(i64, usize, usize)>,
    n: usize,
) -> Vec<usize> {
    let mut groups = init_groups(num_coords);

    let mut cnt = 0;
    for (_dist, a, b) in distances {
        if cnt == n {
            break;
        }
        cnt += 1;
        merge(&mut groups, *a, *b);
    }

    let mut group_sizes = groups.iter().map(|g| g.len()).collect::<Vec<usize>>();
    group_sizes.sort();
    group_sizes.reverse();

    group_sizes
}

fn merge(groups: &mut Vec<HashSet<usize>>, a: usize, b: usize) {
    let idx_a = groups
        .iter()
        .position(|g| g.contains(&a))
        .expect("Group for a not found");
    let idx_b = groups
        .iter()
        .position(|g| g.contains(&b))
        .expect("Group for b not found");

    if idx_a != idx_b {
        let group_a = groups.remove(idx_a);
        let mut group_b = groups.remove(if idx_b > idx_a { idx_b - 1 } else { idx_b });
        for item in group_a {
            group_b.insert(item);
        }
        groups.push(group_b);
    }
}

fn init_groups(num_coords: usize) -> Vec<HashSet<usize>> {
    (0..num_coords)
        .map(|i| {
            let mut hs = HashSet::new();
            hs.insert(i);
            hs
        })
        .collect::<Vec<HashSet<usize>>>()
}

fn calc_distances(coords: &Vec<Coordinate>) -> Vec<(i64, usize, usize)> {
    let mut ret = vec![];
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let dist = square_distance(&coords[i], &coords[j]);
            ret.push((dist, i, j));
        }
    }
    ret.sort();
    ret
}

fn square_distance(a: &Coordinate, b: &Coordinate) -> i64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    dx * dx + dy * dy + dz * dz
}

fn parse_input<R: BufRead>(reader: R) -> Result<Vec<Coordinate>> {
    let mut ret = vec![];

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.trim().split(',').collect();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid line: {}", line));
        }
        let x: i64 = parts[0].parse()?;
        let y: i64 = parts[1].parse()?;
        let z: i64 = parts[2].parse()?;
        ret.push((x, y, z));
    }

    Ok(ret)
}
