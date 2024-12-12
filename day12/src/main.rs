use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::RandomState,
    ops::ControlFlow,
};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Position = (isize, isize);
type Map = HashMap<Position, char>;
type Graph = HashMap<Position, Vec<Position>>;

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    // println!("Part 1   test          {} ", part_1(&test_input).0);
    // println!("         validation    {} ", part_1(&input));
    println!("Part 2   test          {} ", part_1(&test_input).1);
    println!("         validation    {} ", part_1(&input).1);
}

fn part_1(input: &(Map, Graph)) -> (usize, usize) {
    let (mut price, mut discount_price) = (0, 0);
    let (map, graph) = input;
    let mut all_explored: HashSet<Position> = HashSet::new();
    for (pos, c) in map {
        if !all_explored.contains(pos) {
            // pos is a root
            let (area, perimeter, sides) = explore(map, graph, (*pos, *c), &mut all_explored);
            println!("{}: {}", c, sides);
            price += area * perimeter;
            discount_price += area * sides;
        }
    }
    (price, discount_price)
}

// fn part_2(input: &Input) -> u32 {
//     input.trim().parse::<u32>().unwrap()
// }

fn explore(
    map: &Map,
    graph: &Graph,
    root: (Position, char),
    all_explored: &mut HashSet<Position>,
) -> (usize, usize, usize) {
    let (root_pos, root_c) = root;
    let (mut min_x, mut max_x, mut min_y, mut max_y) =
        (root_pos.0, root_pos.0, root_pos.1, root_pos.1);
    let mut perimeter = 0;
    let mut queue = VecDeque::from_iter(vec![root_pos]);
    let mut explored: HashSet<(isize, isize), RandomState> = HashSet::from_iter(vec![root_pos]);
    while let Some(pos) = queue.pop_front() {
        let neighbors = graph.get(&pos).unwrap();
        perimeter += 4 - neighbors.len();
        for adj in graph.get(&pos).unwrap() {
            if !explored.contains(adj) {
                min_x = min_x.min(adj.0);
                max_x = max_x.max(adj.0);
                min_y = min_y.min(adj.1);
                max_y = max_y.max(adj.1);
                explored.insert(*adj);
                all_explored.insert(*adj);
                queue.push_back(*adj);
            }
        }
    }

    // let's cast some rays now lol
    let mut sides = 0;
    let mut x_collisions = vec![];
    for y in min_y..=max_y {
        let mut inside = false;
        for x in min_x..=max_x + 1 {
            let next_inside = explored.contains(&(x, y)) && map.get(&(x, y)) == Some(&root_c);
            if next_inside != inside {
                inside = next_inside;
                x_collisions.push((x, y, inside));
                if !x_collisions.contains(&(x, y - 1, inside)) {
                    println!("x coll at {},{}", x, y);
                    sides += 1;
                }
            }
        }
    }
    let mut y_collisions = vec![];
    for x in min_x..=max_x {
        let mut inside = false;
        for y in min_y..=max_y + 1 {
            let next_inside = explored.contains(&(x, y)) && map.get(&(x, y)) == Some(&root_c);
            if next_inside != inside {
                inside = next_inside;
                y_collisions.push((x, y, inside));
                if !y_collisions.contains(&(x - 1, y, inside)) {
                    println!("y coll at {},{}", x, y);
                    sides += 1;
                }
            }
        }
    }
    (explored.len(), perimeter, sides)
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
