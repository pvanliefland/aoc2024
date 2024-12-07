const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = Vec<(usize, Vec<usize>)>;

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input, false));
    println!("         validation    {} ", part_1(&input, false));
    println!("Part 2   test          {} ", part_1(&test_input, true));
    println!("         validation    {} ", part_1(&input, true));
}

fn part_1(input: &Input, concat: bool) -> usize {
    input
        .iter()
        .filter_map(|(left, right)| {
            let combinations = combinations(right.len() - 1, concat);
            if combinations.iter().any(|combo| {
                let mut result = right[0];
                for (index, op) in combo.iter().enumerate() {
                    match op {
                        '+' => {
                            result += right[index + 1];
                        }
                        '*' => {
                            result *= right[index + 1];
                        }
                        '|' => {
                            result = (result.to_string() + &right[index + 1].to_string())
                                .parse()
                                .unwrap();
                        }
                        _ => panic!("oops"),
                    }
                }
                left == &result
            }) {
                Some(left)
            } else {
                None
            }
        })
        .sum()
}

fn combinations(size: usize, concat: bool) -> Vec<Vec<char>> {
    let pools = if concat {
        vec![vec!['+', '*', '|']; size]
    } else {
        vec![vec!['+', '*']; size]
    };
    let mut result = vec![vec![]];
    for pool in pools {
        let mut combinations = vec![];
        for x in result {
            for y in &pool {
                combinations.push([x.clone(), vec![*y]].concat());
            }
        }
        result = combinations;
    }
    result
}

// fn part_2(input: &Input) -> u32 {
//     input.trim().parse::<u32>().unwrap()
// }

fn parse(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_once(": ")
                .map(|(a, b)| {
                    (
                        a.parse().unwrap(),
                        b.split(' ').map(|b| b.parse().unwrap()).collect(),
                    )
                })
                .unwrap()
        })
        .collect()
}
