use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input<'n> = Vec<(&'n str, &'n str)>;

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
    let mut network_map: HashMap<&str, Vec<&str>> = HashMap::new();
    input.iter().for_each(|(n1, n2)| {
        network_map
            .entry(n1)
            .and_modify(|nodes| {
                nodes.push(*n2);
            })
            .or_insert(vec![n2]);
        network_map
            .entry(n2)
            .and_modify(|nodes| {
                nodes.push(*n1);
            })
            .or_insert(vec![n1]);
    });
    find_connected_nodes(&network_map, 2).len()
}

fn part_2(input: &Input) -> String {
    let mut network_map: HashMap<&str, Vec<&str>> = HashMap::new();
    input.iter().for_each(|(n1, n2)| {
        network_map
            .entry(n1)
            .and_modify(|nodes| {
                nodes.push(*n2);
            })
            .or_insert(vec![n2]);
        network_map
            .entry(n2)
            .and_modify(|nodes| {
                nodes.push(*n1);
            })
            .or_insert(vec![n1]);
    });
    let mut size = 3;
    let mut largest_connected = None;
    loop {
        let connected = find_connected_nodes(&network_map, size);
        if !connected.is_empty() {
            largest_connected = Some(connected);
            size += 1;
        } else {
            break;
        }
    }
    dbg!(size);
    let as_vec = largest_connected
        .unwrap()
        .into_iter()
        .map(|yolo| yolo.into_iter().map(String::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut lan_party = as_vec[0].clone();
    lan_party.sort();
    lan_party.join(",")
}

fn find_connected_nodes<'n>(
    network_map: &HashMap<&'n str, Vec<&'n str>>,
    size: usize,
) -> HashSet<Vec<&'n str>> {
    let relevant_sets = network_map
        .iter()
        .filter_map(|(name, nodes)| {
            if (!name.starts_with('t') && size == 2) || nodes.len() < size {
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
    input
        .trim()
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect()
}
