use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input<'n> = HashMap<&'n str, Vec<&'n str>>;

fn main() {
    let start = Instant::now();
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input));
    println!("         validation    {} ", part_1(&input));
    println!("Part 2   test          {} ", part_2(&test_input));
    println!("         validation    {} ", part_2(&input));
    println!("Duration: {:?}", start.elapsed());
}

fn part_1(input: &Input) -> usize {
    find_connected_nodes(input, 2, true).len()
}

fn part_2(input: &Input) -> String {
    let mut size = input
        .iter()
        .map(|(_, connected)| connected.len())
        .max()
        .unwrap();
    let largest_connected = loop {
        let connected = find_connected_nodes(input, size, false);
        if !connected.is_empty() {
            break connected;
        }
        size -= 1;
    };
    largest_connected
        .into_iter()
        .next()
        .map(|nodes| nodes.join(","))
        .unwrap()
}

fn find_connected_nodes<'n>(
    network_map: &HashMap<&'n str, Vec<&'n str>>,
    size: usize,
    limit_to_chief: bool,
) -> HashSet<Vec<&'n str>> {
    let relevant_sets = network_map
        .iter()
        .filter_map(|(name, nodes)| {
            if (limit_to_chief && !name.starts_with('t')) || nodes.len() < size {
                return None;
            }
            let connected = nodes
                .iter()
                .filter(|node| network_map.get(*node).unwrap().contains(name))
                .copied()
                .collect::<Vec<_>>();
            if connected.len() < size {
                return None;
            }
            let combs = combinations(&connected, size)
                .into_iter()
                .filter(|comb| {
                    comb.iter().all(|n1| {
                        comb.iter()
                            .filter(|n2| n2 != &n1)
                            .all(|n2| network_map.get(n1).unwrap().contains(n2))
                    })
                })
                .collect::<Vec<_>>();
            Some(
                combs
                    .into_iter()
                    .map(|comb| {
                        let mut set = [vec![*name], comb].concat();
                        set.sort();
                        set
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .flatten()
        .collect::<HashSet<_>>();
    relevant_sets
}

fn combinations<T: Copy + Clone>(items: &[T], size: usize) -> Vec<Vec<T>> {
    if size == 1 {
        items.iter().map(|item| vec![*item]).collect()
    } else {
        let mut result = vec![];
        let mut items = items.to_vec();
        while !items.is_empty() {
            let item = items.remove(0);
            for combination in combinations(&items, size - 1) {
                result.push([vec![item], combination].concat());
            }
        }
        result
    }
}

fn parse(input: &str) -> Input {
    let mut network_map: Input = HashMap::new();
    input
        .trim()
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .for_each(|(n1, n2)| {
            network_map
                .entry(n1)
                .and_modify(|nodes| {
                    nodes.push(n2);
                })
                .or_insert(vec![n2]);
            network_map
                .entry(n2)
                .and_modify(|nodes| {
                    nodes.push(n1);
                })
                .or_insert(vec![n1]);
        });
    network_map
}
