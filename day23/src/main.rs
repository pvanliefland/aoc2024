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
    // println!("Part 2   test          {} ", part_2(&test_input));
    // println!("         validation    {} ", part_2(&input));
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
    let relevant_sets = network_map
        .iter()
        .filter_map(|(name, nodes)| {
            if !name.starts_with('t') || nodes.len() < 2 {
                return None;
            }
            let connected = nodes
                .iter()
                .filter(|node| network_map.get(*node).unwrap().contains(name))
                .copied()
                .collect::<Vec<_>>();
            if connected.len() < 2 {
                return None;
            }
            let combs = combinations(&connected, 2)
                .into_iter()
                .filter(|comb| {
                    network_map.get(comb[0]).unwrap().contains(&comb[1])
                        && network_map.get(comb[1]).unwrap().contains(&comb[0])
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

    // relevant_sets.iter().for_each(|set| {
    //     println!("{}", set.join(","));
    // });

    relevant_sets.len()
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

// fn part_2(input: &Input) -> usize {
//     input.trim().parse::<usize>().unwrap()
// }

fn parse(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect()
}
