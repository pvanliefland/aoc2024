use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::RandomState,
};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Position = (isize, isize);
type Map = HashMap<Position, char>;
type Graph = HashMap<Position, Vec<Position>>;

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input));
    println!("         validation    {} ", part_1(&input));
    // println!("Part 2   test          {} ", part_2(&test_input));
    // println!("         validation    {} ", part_2(&input));
}

fn part_1(input: &(Map, Graph)) -> usize {
    let mut price = 0;
    let (map, graph) = input;
    let mut all_explored: HashSet<Position> = HashSet::new();
    for pos in map.keys() {
        if !all_explored.contains(pos) {
            let (area, perimeter) = explore(map, graph, *pos, &mut all_explored);
            price += area * perimeter;
        }
    }
    price
}

// fn part_2(input: &Input) -> u32 {
//     input.trim().parse::<u32>().unwrap()
// }

fn explore(
    map: &Map,
    graph: &Graph,
    root: Position,
    all_explored: &mut HashSet<Position>,
) -> (usize, usize) {
    let mut perimeter = 0;
    let mut queue = VecDeque::from_iter(vec![root]);
    let mut explored: HashSet<(isize, isize), RandomState> = HashSet::from_iter(vec![root]);
    while let Some(pos) = queue.pop_front() {
        let neighbors = graph.get(&pos).unwrap();
        perimeter += 4 - neighbors.len();
        for adj in graph.get(&pos).unwrap() {
            if !explored.contains(adj) {
                explored.insert(*adj);
                all_explored.insert(*adj);
                queue.push_back(*adj);
            }
        }
    }
    (explored.len(), perimeter)
}

fn parse(input: &str) -> (Map, Graph) {
    let map = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| ((x as isize, y as isize), c))
                .collect::<Vec<_>>()
        })
        .collect::<Map>();
    let graph = map
        .iter()
        .map(|(pos, c)| {
            (
                *pos,
                [(1, 0), (0, 1), (-1, 0), (0, -1)]
                    .iter()
                    .filter_map(|step| {
                        map.get_key_value(&(pos.0 + step.0, pos.1 + step.1))
                            .filter(|&(_, nc)| c == nc)
                            .map(|(pos, _)| *pos)
                    })
                    .collect(),
            )
        })
        .collect();
    (map, graph)
}
