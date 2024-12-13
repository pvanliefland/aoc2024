use std::time::Instant;

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Machine = Vec<Vec<isize>>;
type Input = Vec<Machine>;
const YOLO: isize = 10000000000000;

fn main() {
    let start = Instant::now();
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", solve(&test_input, 0));
    println!("         validation    {} ", solve(&input, 0));
    println!("Part 2   test          {} ", solve(&test_input, YOLO));
    println!("         validation    {} ", solve(&input, YOLO));
    println!("Duration: {:?}", start.elapsed());
}

fn solve(input: &Input, dp: isize) -> isize {
    input
        .iter()
        .map(|machine| {
            let (x1, x2) = (machine[0][0] as f64, machine[1][0] as f64);
            let (y1, y2) = (machine[0][1] as f64, machine[1][1] as f64);
            let (px, py) = ((machine[2][0] + dp) as f64, (machine[2][1] + dp) as f64);
            let a = (py / y2 - px / x2) / (y1 / y2 - x1 / x2);
            let b = (px - a * x1) / x2;
            if (a.fract() < 0.01 || a.fract() > 0.99) && (b.fract() < 0.01 || b.fract() > 0.99) {
                let (a, b) = (a.round(), b.round());
                if (a * x1 + b * x2 == px) && (a * y1 + b * y2 == py) {
                    return 3 * a as isize + b as isize;
                } else {
                    return 0;
                }
            }
            0
        })
        .sum()
}

fn parse(input: &str) -> Input {
    input
        .trim()
        .split("\n\n")
        .map(|machine| {
            machine
                .lines()
                .map(|line| {
                    line.split(", ")
                        .map(|part| {
                            part.split_once(['+', '='])
                                .unwrap()
                                .1
                                .parse::<isize>()
                                .unwrap()
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
