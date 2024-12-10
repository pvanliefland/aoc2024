use std::collections::HashMap;

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Position = (isize, isize);
type Map = HashMap<Position, u8>;

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input));
    println!("         validation    {} ", part_1(&input));
    println!("Part 2   test          {} ", part_2(&test_input));
    println!("         validation    {} ", part_2(&input));
}

fn part_1(input: &Map) -> u32 {
    input
        .iter()
        .filter_map(|(pos, height)| {
            if height == &0 {
                Some(explore(*pos, input))
            } else {
                None
            }
        })
        .sum()
}
fn part_2(input: &Map) -> u32 {
    input
        .iter()
        .filter_map(|(pos, height)| {
            if height == &0 {
                Some(explore_2(*pos, input))
            } else {
                None
            }
        })
        .sum()
}

fn explore(root: Position, map: &Map) -> u32 {
    let mut total = 0;
    let mut explored = vec![root];
    let mut queue = vec![(root, 0)];
    while !queue.is_empty() {
        let (pos, height) = queue.remove(0);
        if height == 9 {
            total += 1;
        }
        for step in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let edge_pos = (pos.0 + step.0, pos.1 + step.1);
            if let Some(edge_height) = map.get(&edge_pos) {
                if *edge_height == height + 1 && !explored.contains(&edge_pos) {
                    explored.push(edge_pos);
                    queue.push((edge_pos, *edge_height));
                }
            }
        }
    }
    total
}

fn explore_2(root: Position, map: &Map) -> u32 {
    let mut total = 0;
    let mut explored = vec![root];
    let mut queue = vec![(root, 0)];
    while !queue.is_empty() {
        let (pos, height) = queue.remove(0);
        if height == 9 {
            total += 1;
        }
        for step in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let edge_pos = (pos.0 + step.0, pos.1 + step.1);
            if let Some(edge_height) = map.get(&edge_pos) {
                if *edge_height == height + 1 {
                    explored.push(edge_pos);
                    queue.push((edge_pos, *edge_height));
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
