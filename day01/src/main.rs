use std::iter::zip;

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = (Vec<u32>, Vec<u32>);

fn main() {
    let mut test_input = parse(INPUT_TEST);
    let mut input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&mut test_input));
    println!("         validation    {} ", part_1(&mut input));
    println!("Part 2   test          {} ", part_2(&mut test_input));
    println!("         validation    {} ", part_2(&mut input));
}

fn part_1(input: &mut Input) -> u32 {
    input.0.sort();
    input.1.sort();
    zip(&input.0, &input.1)
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

fn part_2(input: &mut Input) -> u32 {
    input
        .0
        .iter()
        .map(|left| left * input.1.iter().filter(|&right| left == right).count() as u32)
        .sum()
}

fn parse(input: &str) -> Input {
    let (mut left_list, mut right_list) = (vec![], vec![]);
    input.trim().lines().for_each(|line| {
        let (left, right) = line.split_once("   ").unwrap();
        left_list.push(left.parse::<u32>().unwrap());
        right_list.push(right.parse::<u32>().unwrap());
    });
    (left_list, right_list)
}
