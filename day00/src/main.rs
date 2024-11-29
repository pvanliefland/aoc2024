const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1   test          {} ", part_1(INPUT_TEST));
    println!("         validation    {} ", part_1(INPUT));
    println!("Part 2   test          {} ", part_2(INPUT_TEST));
    println!("         validation    {} ", part_2(INPUT_TEST));
}

fn part_1(input: &str) -> u32 {
    input.trim().parse::<u32>().unwrap()
}

fn part_2(input: &str) -> u32 {
    input.trim().parse::<u32>().unwrap()
}
