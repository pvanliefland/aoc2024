const INPUT_TEST_1: &str = include_str!("../input_test_1.txt");
const INPUT_TEST_2: &str = include_str!("../input_test_2.txt");
const INPUT: &str = include_str!("../input.txt");

type Input = Vec<String>;

fn main() {
    let test_input_1 = parse(INPUT_TEST_1);
    let test_input_2 = parse(INPUT_TEST_2);
    let input = parse(INPUT);
    println!("Part 1   test          {} ", part_1(test_input_1));
    // println!("         validation    {} ", part_1(input));
    // println!("Part 2   test          {} ", part_2(test_input_2));
    // println!("         validation    {} ", part_2(input));
}

enum State {
    Unknown,
    DoOrDont(char),
    Do,
    Dont(char),
    Mul(Option<String>, Option<String>, bool),
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
            let mut state = State::Unknown;

            let mut cursor = 0;

            loop {
                // smallest valid thing is mul(1,1) so 8 chars
                if cursor > line.len() - 8 {
                    break;
                }

                match state {
                    State::Unknown | State::Do => {
                        if &line[cursor..cursor + 4] == "mul(" {
                            state = State::Mul(None, None, false);
                            cursor += 4;
                        } else {
                            cursor += 1;
                        }
                    }
                    State::DoOrDont(_) => todo!(),
                    State::Dont(_) => todo!(),
                    State::Mul(None, None, false) => {
                        if line[cursor..cursor + 1]
                            .chars()
                            .next()
                            .unwrap()
                            .is_ascii_digit()
                        {
                            state =
                                State::Mul(Some(line[cursor..cursor + 1].to_string()), None, false);
                            cursor += 1;
                        } else {
                            state = State::Unknown;
                            cursor += 1;
                        }
                    }
                    State::Mul(Some(left), None, false) => {
                        if line[cursor..cursor + 1]
                            .chars()
                            .next()
                            .unwrap()
                            .is_ascii_digit()
                        {
                            state = State::Mul(Some(left + &line[cursor..cursor + 1]), None, false);
                            cursor += 1;
                        } else if &line[cursor..cursor + 1] == "," {
                            state = State::Mul(Some(left), None, true);
                            cursor += 1;
                        } else {
                            state = State::Unknown;
                            cursor += 1;
                        }
                    }
                    State::Mul(Some(left), None, true) => {
                        if line[cursor..cursor + 1]
                            .chars()
                            .next()
                            .unwrap()
                            .is_ascii_digit()
                        {
                            state = State::Mul(
                                Some(left),
                                Some(line[cursor..cursor + 1].to_string()),
                                true,
                            );
                            cursor += 1;
                        } else {
                            state = State::Unknown;
                            cursor += 1;
                        }
                    }
                    State::Mul(Some(left), Some(right), true) => {
                        if line[cursor..cursor + 1]
                            .chars()
                            .next()
                            .unwrap()
                            .is_ascii_digit()
                        {
                            state = State::Mul(
                                Some(left),
                                Some(right + &line[cursor..cursor + 1]),
                                true,
                            );
                            cursor += 1;
                        } else if &line[cursor..cursor + 1] == ")" {
                            dbg!("r");
                            valid += left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
                            state = State::Unknown;
                            cursor += 1;
                        } else {
                            state = State::Unknown;
                            cursor += 1;
                        }
                    }
                    _ => {
                        state = State::Unknown;
                        cursor += 1;
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
