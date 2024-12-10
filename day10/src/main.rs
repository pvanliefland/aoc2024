const INPUT_TEST: &str = include_str!("../input_test.txt");
// const INPUT: &str = include_str!("../input.txt");

type Input = u32;

fn main() {
    let test_input = parse(INPUT_TEST);
    // let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input));
    // println!("         validation    {} ", part_1(&input));
    // println!("Part 2   test          {} ", part_2(&test_input));
    // println!("         validation    {} ", part_2(&input));
}

fn part_1(input: &Input) -> u32 {
    dbg!(input);
    42
}

// fn part_2(input: &Input) -> u32 {
//     input.trim().parse::<u32>().unwrap()
// }

fn parse(input: &str) -> Input {
    input.trim().parse::<u32>().unwrap()
}
