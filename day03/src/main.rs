const INPUT_TEST_1: &str = include_str!("../input_test_1.txt");
const INPUT_TEST_2: &str = include_str!("../input_test_2.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = Vec<String>;

fn main() {
    let test_input_1 = parse(INPUT_TEST_1);
    let test_input_2 = parse(INPUT_TEST_2);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(test_input_1));
    println!("         validation    {} ", part_1(input));
    println!("Part 2   test          {} ", part_2(test_input_2));
    // println!("         validation    {} ", part_2(input));
}

enum State {
    Unknown,
    DoOrDont(char),
    Do(char),
    Dont(char),
    Mul(Option<char>, String, String),
}

fn part_1(input: Input) -> u32 {
    process(input, false)
}

fn part_2(input: Input) -> u32 {
    process(input, true)
}

fn process(input: Input, dos_and_donts: bool) -> u32 {
    input
        .iter()
        .map(|line| {
            let mut valid = 0u32;
            let mut char_iterator = line.chars();
            let mut state = State::Unknown;

            loop {
                let next = match char_iterator.next() {
                    Some(char) => char,
                    None => {
                        break;
                    }
                };

                match (state, next) {
                    (State::Unknown, 'm') => {
                        state = State::Mul(Some('u'), "".to_string(), "".to_string());
                    }
                    (State::Unknown, 'd') => {
                        state = State::DoOrDont('o');
                    }
                    (State::Mul(Some('u'), left, right), 'u') => {
                        state = State::Mul(Some('l'), left, right);
                    }
                    (State::Mul(Some('l'), left, right), 'l') => {
                        state = State::Mul(Some('('), left, right);
                    }
                    (State::Mul(Some('('), left, right), '(') => {
                        state = State::Mul(None, left, right);
                    }
                    (Some('('), '(', _, _) => {
                        expected_char = None;
                        left = Some("".to_string());
                    }
                    (None, next, Some(prev_left), None) if next.is_digit(10) => {
                        left.replace(prev_left.clone() + &next.to_string());
                    }
                    (_, next, Some(prev_left), None) if prev_left != "" && next == ',' => {
                        right = Some("".to_string());
                    }
                    (_, next, Some(_), Some(prev_right)) if next.is_digit(10) => {
                        right.replace(prev_right.clone() + &next.to_string());
                    }
                    (_, next, Some(prev_left), Some(prev_right))
                        if prev_right != "" && next == ')' =>
                    {
                        valid +=
                            prev_left.parse::<u32>().unwrap() * prev_right.parse::<u32>().unwrap();
                        left = None;
                        right = None;
                        expected_char = Some('m');
                    }
                    _ => {
                        left = None;
                        right = None;
                        expected_char = Some('m');
                        continue;
                    }
                }
            }
            valid
        })
        .sum()
}

fn parse(input: &str) -> Input {
    input.trim().lines().map(String::from).collect()
}
