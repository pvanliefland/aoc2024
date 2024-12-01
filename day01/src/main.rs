use std::iter::zip;

const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let (mut test_left, mut test_right) = left_and_right(INPUT_TEST);
    let (mut validation_left, mut validation_right) = left_and_right(INPUT);
    println!(
        "Part 1   test          {} ",
        part_1(&mut test_left, &mut validation_right)
    );
    println!(
        "         validation    {} ",
        part_1(&mut validation_left, &mut validation_right)
    );
    println!(
        "Part 2   test          {} ",
        part_2(&mut test_left, &mut test_right)
    );
    println!(
        "         validation    {} ",
        part_2(&mut validation_left, &mut validation_right)
    );
}

fn part_1(left: &mut [u32], right: &mut [u32]) -> u32 {
    left.sort();
    right.sort();
    zip(left, right)
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

fn part_2(left: &mut [u32], right: &mut [u32]) -> u32 {
    left.iter()
        .map(|left| left * right.iter().filter(|&right| left == right).count() as u32)
        .sum()
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
