use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Position = (isize, isize);
type Map = HashMap<Position, char>;
type Input = Vec<Position>;

fn main() {
    let start = Instant::now();
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input, 6, 12));
    println!("         validation    {} ", part_1(&input, 70, 1024));
    // println!("Part 2   test          {} ", part_2(&test_input));
    // println!("         validation    {} ", part_2(&input));
    println!("Duration: {:?}", start.elapsed());
}

fn part_1(input: &Input, max_xy: isize, iterations: usize) -> usize {
    let mut map: Map = (0..=max_xy)
        .flat_map(|y| (0..=max_xy).map(|x| ((x, y), '.')).collect::<Vec<_>>())
        .collect();
    let falling = input.clone();
    (0..iterations).for_each(|i| {
        map.insert(falling[i], '#');
    });
    escape(&map, (0, 0), (max_xy, max_xy)).1
}

fn escape(map: &Map, start: Position, target: Position) -> (Position, usize) {
    let mut queue = VecDeque::from_iter([(start, 0)]);
    let mut explored: HashSet<Position> = HashSet::from_iter([start]);
    let mut found = None;
    while let Some((pos, steps)) = queue.pop_front() {
        if pos == target {
            found = Some((pos, steps));
            break;
        }
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let adj_pos = (pos.0 + dx, pos.1 + dy);
            if !explored.contains(&adj_pos) {
                explored.insert(adj_pos);
                if map
                    .get(&adj_pos)
                    .is_some_and(|at_adj_pos| at_adj_pos == &'.')
                {
                    queue.push_back((adj_pos, steps + 1));
                }
            }
        }
    }
    found.unwrap()
}

// fn part_2(input: &Input) -> usize {
//     input.trim().parse::<usize>().unwrap()
// }

fn parse(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}
