const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = Vec<(usize, Vec<usize>)>;

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    let (test_part_1, test_part_2) = part_1_2(&test_input);
    let (part_1, part_2) = part_1_2(&input);
    println!("Part 1   test          {test_part_1}");
    println!("         validation    {part_1}");
    println!("Part 2   test          {test_part_2}");
    println!("         validation    {part_2}");
}

fn part_1_2(input: &Input) -> (usize, usize) {
    let mut without_concat = 0;
    let mut with_concat = 0;
    for (left, right) in input {
        if validate(*left, right, false) {
            without_concat += left;
        } else if validate(*left, right, true) {
            with_concat += left;
        }
    }
    (without_concat, with_concat + without_concat)
}

fn validate(left: usize, right: &[usize], concat: bool) -> bool {
    let operators = if concat {
        vec!['+', '*', '|']
    } else {
        vec!['+', '*']
    };
    let combinations = combinations(&operators, right.len() - 1);
    combinations.iter().any(|combo| {
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
        left == result
    })
}

fn combinations<T: Copy + Clone>(items: &[T], size: usize) -> Vec<Vec<T>> {
    if size == 1 {
        items.iter().map(|item| vec![*item]).collect()
    } else {
        let mut result = vec![];
        for item in items {
            for combination in combinations(items, size - 1) {
                result.push([vec![*item], combination].concat());
            }
        }
        result
    }
}

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
