use std::iter::zip;

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1   test          {} ", part_1(INPUT_TEST));
    println!("         validation    {} ", part_1(INPUT));
    println!("Part 2   test          {} ", part_2(INPUT_TEST));
    println!("         validation    {} ", part_2(INPUT));
}

fn part_1(input: &str) -> u32 {
    let (mut left_list, mut right_list) = left_and_right(&input);
    left_list.sort();
    right_list.sort();

    zip(left_list, right_list).fold(0, |acc, (left, right)| acc + left.abs_diff(right))
}

fn part_2(input: &str) -> u32 {
    let (left_list, right_list): (Vec<u32>, Vec<u32>) = left_and_right(&input);
    left_list.iter().fold(0, |acc, left| {
        acc + (left * right_list.iter().filter(|&right| left == right).count() as u32)
    })
}

fn left_and_right(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left_list = vec![];
    let mut right_list = vec![];
    input.trim().lines().for_each(|line| {
        let (left, right) = line.split_once("   ").unwrap();
        left_list.push(left.parse::<u32>().unwrap());
        right_list.push(right.parse::<u32>().unwrap());
    });
    (left_list, right_list)
}
