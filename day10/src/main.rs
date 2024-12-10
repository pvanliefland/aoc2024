use std::collections::{HashMap, HashSet, VecDeque};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Position = (isize, isize);
type Map = HashMap<Position, u8>;

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", ratings(&test_input, false));
    println!("         validation    {} ", ratings(&input, false));
    println!("Part 2   test          {} ", ratings(&test_input, true));
    println!("         validation    {} ", ratings(&input, true));
}

fn ratings(input: &Map, distinct: bool) -> u32 {
    input
        .iter()
        .filter_map(|(pos, height)| {
            if height == &0 {
                Some(explore(*pos, input, distinct))
            } else {
                None
            }
        })
        .sum()
}

fn explore(root: Position, map: &Map, distinct: bool) -> u32 {
    let mut total = 0;
    let mut explored: HashSet<Position> = HashSet::from_iter(vec![root]);
    let mut queue = VecDeque::from(vec![(root, 0)]);
    while !queue.is_empty() {
        let (pos, height) = queue.pop_front().unwrap();
        if height == 9 {
            total += 1;
        }
        for step in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let edge_pos = (pos.0 + step.0, pos.1 + step.1);
            if let Some(edge_height) = map.get(&edge_pos) {
                if *edge_height == height + 1 && (distinct || !explored.contains(&edge_pos)) {
                    explored.insert(edge_pos);
                    queue.push_back((edge_pos, *edge_height));
                }
            }
        }
    }
    total
}

fn parse(input: &str) -> Map {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| ((x as isize, y as isize), c.to_digit(10).unwrap() as u8))
                .collect::<Vec<_>>()
        })
        .collect()
}
