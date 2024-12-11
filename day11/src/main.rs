const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = Vec<String>;

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input));
    println!("         validation    {} ", part_1(&input));
    // println!("Part 2   test          {} ", part_2(&test_input));
    // println!("         validation    {} ", part_2(&input));
}

fn part_1(input: &Input) -> usize {
    let mut transformed = input.clone();
    for _ in 0..25 {
        transformed = step(&transformed);
        // println!("{}", transformed.join(" "));
    }
    transformed.len()
}

// fn part_2(input: &Input) -> u32 {
//     input.trim().parse::<u32>().unwrap()
// }

fn step(input: &Input) -> Input {
    input
        .iter()
        .flat_map(|stone| match stone.as_str() {
            "0" => vec!["1".to_string()],
            stone if stone.len() % 2 == 0 => {
                let (left, right) = stone.split_at(stone.len() / 2);
                vec![left.to_string(), right.to_string()]
                    .into_iter()
                    .map(|s| {
                        if s.parse::<usize>().unwrap() == 0 {
                            "0".to_string()
                        } else {
                            s.trim_start_matches('0').to_string()
                        }
                    })
                    .collect()
            }
            _ => vec![(stone.parse::<usize>().unwrap() * 2024).to_string()],
        })
        .collect()
}

fn parse(input: &str) -> Input {
    input.trim().split(" ").map(|s| s.to_string()).collect()
}
