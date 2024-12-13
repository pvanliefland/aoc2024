const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Machine = [[isize; 2]; 3];
type Input = Vec<Machine>;

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input, 0));
    println!("         validation    {} ", part_1(&input, 0));
    println!(
        "Part 2   test          {} ",
        part_1(&test_input, 10000000000000)
    );
    println!("         validation    {} ", part_1(&input, 10000000000000));
}

fn part_1(input: &Input, dp: isize) -> isize {
    input
        .iter()
        .map(|machine| {
            let (x1, x2) = (machine[0][0] as f64, machine[1][0] as f64);
            let (y1, y2) = (machine[0][1] as f64, machine[1][1] as f64);
            let (px, py) = ((machine[2][0] + dp) as f64, (machine[2][1] + dp) as f64);
            // we now that a * x1 + b * x2 = px
            //    and that a * y1 + b * y2 = py
            // so b = (px - a * x1) / x2
            // so a * y1 + ((px - a * x1) / x2) * y2 = py
            // so a * y1 / y2 + ((px - a * x1) / x2) = py / y2
            // so a * y1 / y2 + px / x2 - a * x1 / x2 = py / y2
            // so a * y1 / y2 - a * x1 / x2 = py / y2 - px / x2
            // so a * (y1 / y2 - x1 / x2) = py / y2 - px / x2
            // so a = (py / y2 - px / x2) / (y1 / y2 - x1 / x2)
            let a = (py / y2 - px / x2) / (y1 / y2 - x1 / x2);
            let b = (px - a * x1) / x2;
            if (a.fract() < 0.01 || a.fract() > 0.99) && (b.fract() < 0.01 || b.fract() > 0.99) {
                let really_ok = (a.round() * x1 + b.round() * x2 == px)
                    && (a.round() * y1 + b.round() * y2 == py);
                if really_ok {
                    return 3 * a.round() as isize + b.round() as isize;
                } else {
                    dbg!("here");
                    return 0;
                }
            }
            0
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
