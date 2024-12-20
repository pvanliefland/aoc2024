use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

const INPUT_TEST: &str = include_str!("../input_test.txt");
// const INPUT: &str = include_str!("../input.txt");

type Position = (isize, isize);
type Input = (HashMap<Position, char>, Position);

fn main() {
    let start = Instant::now();
    let test_input = parse(INPUT_TEST);
    // let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input));
    // println!("         validation    {} ", part_1(&input));
    // println!("Part 2   test          {} ", part_2(&test_input));
    // println!("         validation    {} ", part_2(&input));
    println!("Duration: {:?}", start.elapsed());
}

fn part_1(input: &Input) -> usize {
    let (map, start) = input;
    let mut stack = vec![(*start, 0, 2)];
    let mut explored = HashSet::new();
    let mut exit = None;
    while let Some((pos, time, cheats)) = stack.pop() {
        if !explored.contains(&pos) {
            let at_pos = map.get(&pos).unwrap();
            if at_pos == &'#' && cheats == 0 {
                continue;
            }
            if at_pos == &'E' {
                dbg!(cheats);
                exit = Some(time);
                break;
            }
            explored.insert(pos);
            for mov in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let adj_pos = (pos.0 + mov.0, pos.1 + mov.1);
                let at_adj_pos = map.get(&adj_pos);
                if let Some(at_adj_pos) = at_adj_pos {
                    if at_adj_pos != &'#' {
                        stack.push((adj_pos, time + 1, if cheats == 1 { 0 } else { cheats }));
                    } else if at_adj_pos == &'#' && cheats > 0 {
                        stack.push((adj_pos, time + 1, cheats - 1));
                    }
                }
            }
        }
    }

    exit.unwrap()
}

// fn part_2(input: &Input) -> usize {
//     input.trim().parse::<usize>().unwrap()
// }

fn parse(input: &str) -> Input {
    let mut start = None;
    (
        input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start = Some((x as isize, y as isize))
                        };
                        ((x as isize, y as isize), c)
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
        start.unwrap(),
    )
}
