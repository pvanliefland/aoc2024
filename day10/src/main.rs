use std::collections::{HashMap, HashSet};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Position = (isize, isize);
type Map = HashMap<Position, u8>;
type Graph = HashMap<Position, Vec<Position>>;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", ratings(&test_input, false));
    println!("         validation    {} ", ratings(&input, false));
    println!("Part 2   test          {} ", ratings(&test_input, true));
    println!("         validation    {} ", ratings(&input, true));

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn ratings(input: &(Map, Graph), distinct: bool) -> u32 {
    let (map, graph) = input;
    map.iter()
        .filter_map(|(pos, height)| {
            if height == &0 {
                Some(explore(*pos, graph, distinct))
            } else {
                None
            }
        })
        .sum()
}

fn explore(root: Position, graph: &Graph, distinct: bool) -> u32 {
    let mut total = 0;
    let mut explored: HashSet<Position> = HashSet::new();
    let mut stack = vec![(root, 0)];
    while let Some((pos, height)) = stack.pop() {
        if distinct || !explored.contains(&pos) {
            if height == 9 {
                total += 1;
            }
            explored.insert(pos);
            for adj in graph.get(&pos).unwrap() {
                stack.push((*adj, height + 1));
            }
        }
    }
    total
}

fn parse(input: &str) -> (Map, Graph) {
    let map = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| ((x as isize, y as isize), c.to_digit(10).unwrap() as u8))
                .collect::<Vec<_>>()
        })
        .collect::<Map>();
    let graph = map
        .iter()
        .map(|(pos, height)| {
            (
                *pos,
                [(1, 0), (0, 1), (-1, 0), (0, -1)]
                    .iter()
                    .filter_map(|step| {
                        map.get_key_value(&(pos.0 + step.0, pos.1 + step.1))
                            .filter(|&(_, n_height)| *n_height == height + 1)
                            .map(|(pos, _)| *pos)
                    })
                    .collect(),
            )
        })
        .collect();
    (map, graph)
}
