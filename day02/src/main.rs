const INPUT_TEST: &str = include_str!("../input_test.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = Vec<Vec<u32>>;

fn main() {
    let test_input = parse(INPUT_TEST);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(&test_input));
    println!("         validation    {} ", part_1(&input));
    println!("Part 2   test          {} ", part_2(&test_input));
    println!("         validation    {} ", part_2(&input));
}

fn part_1(input: &Input) -> usize {
    valid_reports(input, false)
}

fn part_2(input: &Input) -> usize {
    valid_reports(input, true)
}

fn valid_reports(input: &Input, dampener: bool) -> usize {
    input
        .into_iter()
        .filter(|&report| {
            validate_report(report)
                || dampener
                    && (0..report.len()).any(|index| {
                        let mut alternate_report = report.clone();
                        alternate_report.remove(index);
                        validate_report(&alternate_report)
                    })
        })
        .count()
}

fn validate_report(report: &[u32]) -> bool {
    let mut windows = report.windows(2).peekable();
    let first_pair = windows.peek().unwrap();
    let ascending = first_pair[1] > first_pair[0];
    windows.all(|pair| {
        pair[1] != pair[0] && (pair[1] > pair[0]) == ascending && pair[0].abs_diff(pair[1]) <= 3
    })
}

fn parse(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|number| number.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}
