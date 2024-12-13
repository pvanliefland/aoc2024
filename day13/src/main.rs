use std::collections::HashSet;

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Machine = [[isize; 2]; 3];
type Input = Vec<Machine>;

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input));
    println!("         validation    {} ", part_1(&input));
    // println!("Part 2   test          {} ", part_2(&test_input));
    // println!("         validation    {} ", part_2(&input));
}

fn part_1(input: &Input) -> isize {
    input
        .iter()
        .map(|machine| {
            let x_values = (0..100)
                .filter_map(|x1| {
                    let dividend = -machine[0][0] * x1 + machine[2][0];
                    let divisor = machine[1][0];
                    if dividend % divisor == 0 {
                        let x2 = dividend / divisor;
                        if (0..=100).contains(&x2) {
                            Some((x1, x2))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<HashSet<(isize, isize)>>();
            let y_values = (0..100)
                .filter_map(|y1| {
                    let dividend = -machine[0][1] * y1 + machine[2][1];
                    let divisor = machine[1][1];
                    if dividend % divisor == 0 {
                        let y2 = dividend / divisor;
                        if (0..=100).contains(&y2) {
                            Some((y1, y2))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<HashSet<(isize, isize)>>();
            let cheapest = x_values
                .intersection(&y_values)
                .map(|(a, b)| ((a, b), 3 * a + b))
                .min_by_key(|(_, c)| *c);
            cheapest.map(|(_, c)| c).unwrap_or(0)
        })
        .sum()
}

// fn part_2(input: &Input) -> u32 {
//     input.trim().parse::<u32>().unwrap()
// }

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
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect()
}
