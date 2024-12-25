use std::time::Instant;

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = (Vec<Schematic>, Vec<Schematic>);
type Schematic = [usize; 5];

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
    let (locks, keys) = input;
    keys.iter()
        .map(|k| {
            locks
                .iter()
                .filter(|l| (0..5).all(|i| k[i] + l[i] <= 5))
                .count()
        })
        .sum()
}

// fn part_2(input: &Input) -> usize {
//     input.trim().parse::<usize>().unwrap()
// }

fn parse(input: &str) -> Input {
    let schematics = input
        .trim()
        .split("\n\n")
        .map(|schematic_data| {
            let lock = schematic_data.lines().next().unwrap() == "#####";
            let mut heights = [0; 5];
            schematic_data
                .lines()
                .skip(if lock { 1 } else { 0 })
                .take(6)
                .for_each(|line| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, c)| c == &'#')
                        .for_each(|(i, _)| {
                            heights[i] += 1;
                        });
                });
            (lock, heights)
        })
        .collect::<Vec<_>>();
    (
        schematics
            .iter()
            .filter_map(|(l, s)| if *l { Some(*s) } else { None })
            .collect(),
        schematics
            .iter()
            .filter_map(|(l, s)| if *l { None } else { Some(*s) })
            .collect(),
    )
}
