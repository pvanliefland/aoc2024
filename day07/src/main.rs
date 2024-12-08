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
    input.iter().for_each(|(left, right)| {
        if computations(right, false)
            .into_iter()
            .any(|value| left == &value)
        {
            without_concat += left;
        } else if computations(right, true)
            .into_iter()
            .any(|value| left == &value)
        {
            with_concat += left;
        }
    });
    (without_concat, with_concat + without_concat)
}

fn computations(operands: &[usize], concat: bool) -> Vec<usize> {
    if operands.len() == 1 {
        operands.to_vec()
    } else {
        let mut results = vec![];
        let (last, others) = operands.split_last().unwrap();
        for other in computations(others, concat) {
            results.push(other + last);
            results.push(other * last);
            if concat {
                results.push((other.to_string() + &last.to_string()).parse().unwrap());
            }
        }
        results
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
