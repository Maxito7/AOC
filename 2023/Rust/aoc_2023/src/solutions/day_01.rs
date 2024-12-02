use crate::utils;

fn part1(data: &str) -> u32 {
    let digits = data
        .lines()
        .map(|line| {
            line.chars()
                .filter(|&c| ('0'..='9').contains(&c))
                .map(|c| &line[line.find(c).unwrap()..=line.find(c).unwrap()])
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();
    let mut sum: u32 = 0;
    for digit in digits {
        let mut number = 0;
        let digit_len = digit.len();
        let first_digit = digit[0];
        let last_digit = digit[digit_len - 1];
        let mut number_str = String::new();
        for digit_small in digit {
            if digit_len == 1 {
                number_str = format!("{}{}", digit_small, digit_small);
                number = number_str.parse::<u32>().unwrap();
            } else {
                number_str = format!("{}{}", first_digit, last_digit);
                number = number_str.parse::<u32>().unwrap();
            }
        }
        println!("{}", number);
        sum += number;
    }
    println!("{}", sum);
    sum
}

pub enum NumbersStr {
    One(String),
    Two(String),
    Three(String),
    Four(String),
    Five(String),
    Six(String),
    Seven(String),
    Eight(String),
    Nine(String),
}

fn part2(data: &str) -> u32 {
    let words = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let data_line = data
        .lines()
        .map(|line| {
            line.chars()
                .peekable()
                .filter(|&c| ('0'..='9').contains(&c))

            /*
                            .map(|c| {
                                let number = 0;
                                if ('0'..='9').contains(&c) {
                                } else {
                                    for word in words {
                                        word.0 & c.nex
                                    }

                                }
                            })
                            .collect::<Vec<u32>>()
            */
        })
        .collect::<Vec<Vec<u32>>>();

    let mut sum: u32 = 0;
    /*
        for digit in digits {
            let mut number = 0;
            let digit_len = digit.len();
            let first_digit = digit[0];
            let last_digit = digit[digit_len - 1];
            let mut number_str = String::new();
            for digit_small in digit {
                if digit_len == 1 {
                    number_str = format!("{}{}", digit_small, digit_small);
                    number = number_str.parse::<u32>().unwrap();
                } else {
                    number_str = format!("{}{}", first_digit, last_digit);
                    number = number_str.parse::<u32>().unwrap();
                }
            }
            println!("{}", number);
            sum += number;
        }
    */
    println!("{}", sum);
    sum
}

pub fn day01(day: u8, suffix: Option<&str>, part: u8) {
    let data = utils::load_file(day, suffix);
    match part {
        1 => println!("Output of part #1: {}", part1(&data)),
        2 => println!("Output of part #2: {}", part2(&data)),
        3 => println!("Output of both parts will show!"),
        _ => println!("Wrong input, cannot show output of any part!"),
    }
}
