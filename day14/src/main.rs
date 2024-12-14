use std::{
    collections::HashMap,
    thread::sleep,
    time::{Duration, Instant},
};

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug)]
struct Robot {
    p: Position,
    v: Velocity,
}
type Position = (isize, isize);
type Velocity = (isize, isize);
type Map<'r> = HashMap<Position, Vec<Robot>>;

fn main() {
    let start = Instant::now();
    let (test_input, tw, th) = (parse(INPUT_TEST), 11, 7);
    let (input, w, h) = (parse(INPUT), 101, 103);
    // println!("Part 1   test          {} ", part_1(&test_input, tw, th));
    println!("         validation    {} ", part_1(&input, w, h));
    // println!("Part 2   test          {} ", part_2(&test_input));
    // println!("         validation    {} ", part_2(&input));
    println!("Duration: {:?}", start.elapsed());
}

fn part_1(input: &(Vec<Robot>, Map), width: isize, height: isize) -> usize {
    let (mut robots, _p) = input.clone();
    // more than 5717 (last: 232)
    for i in 1..20000 {
        step(&mut robots, width, height);
        let map = map_robots(&robots);
        if map
            .iter()
            .filter(|(p, _)| {
                map.contains_key(&(p.0 - 1, p.1 + 1))
                    && map.contains_key(&(p.0 - 2, p.1 + 2))
                    && map.contains_key(&(p.0 - 3, p.1 + 3))
            })
            .count()
            > 10
        {
            print_map(&map, width, height);
            println!("Iteration {}", i);
            break;
        }
    }

    let quadrants = [
        ((0, 0), (width - 1) / 2, (height - 1) / 2),
        (((width - 1) / 2 + 1, 0), (width - 1) / 2, (height - 1) / 2),
        ((0, (height - 1) / 2 + 1), (width - 1) / 2, (height - 1) / 2),
        (
            ((width - 1) / 2 + 1, (height - 1) / 2 + 1),
            (width - 1) / 2,
            (height - 1) / 2,
        ),
    ];
    quadrants
        .iter()
        .map(|q| {
            map_robots(
                &robots
                    .iter()
                    .filter(|r| {
                        r.p.0 >= q.0 .0
                            && r.p.0 < q.0 .0 + q.1
                            && r.p.1 >= q.0 .1
                            && r.p.1 < q.0 .1 + q.2
                    })
                    .copied()
                    .collect::<Vec<_>>(),
            )
            .values()
            .sum::<usize>()
        })
        .product()
}

fn map_robots(robots: &Vec<Robot>) -> HashMap<Position, usize> {
    let mut map = HashMap::new();
    for robot in robots {
        map.entry(robot.p)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    map
}

fn print_map(map: &HashMap<Position, usize>, width: isize, height: isize) {
    for y in 0..height {
        for x in 0..width {
            print!(
                "{}",
                map.get(&(x, y))
                    .map(|_| "\x1b[93mX\x1b[0m".to_string())
                    .unwrap_or(".".to_string())
            );
        }
        println!();
    }
    println!();
}

// fn part_2(input: &Input) -> u32 {
//     input.trim().parse::<u32>().unwrap()
// }

fn step(robots: &mut Vec<Robot>, width: isize, height: isize) {
    for robot in robots {
        let (mut new_x, mut new_y) = (robot.p.0 + robot.v.0, robot.p.1 + robot.v.1);
        if new_x < 0 {
            new_x += width;
        } else if new_x >= width {
            new_x -= width;
        }
        if new_y < 0 {
            new_y += height;
        } else if new_y >= height {
            new_y -= height;
        }
        robot.p = (new_x, new_y);
    }
}

fn parse(input: &str) -> (Vec<Robot>, Map) {
    let robots = input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" ").unwrap();
            let (x, y) = p.split_once("=").unwrap().1.split_once(",").unwrap();
            let (dx, dy) = v.split_once("=").unwrap().1.split_once(",").unwrap();
            Robot {
                p: (x.parse().unwrap(), y.parse().unwrap()),
                v: (dx.parse().unwrap(), dy.parse().unwrap()),
            }
        })
        .collect::<Vec<_>>();
    let mut map: Map = HashMap::new();
    for robot in &robots {
        map.entry(robot.p)
            .and_modify(|list: &mut _| list.push(*robot))
            .or_insert(vec![*robot]);
    }
    (robots, map)
}
