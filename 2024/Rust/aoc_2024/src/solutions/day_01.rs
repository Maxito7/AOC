use crate::utils;

fn part1(data: &str) -> u64 {
    let mut left_row: Vec<u64> = vec![];
    let mut right_row: Vec<u64> = vec![];
    let _: Vec<_> = data
        .lines()
        .map(|line| {
            let temp_arr: (&str, &str) = line
                .trim()
                .split_once("   ")
                .expect("Could not split the values found");
            left_row.push(
                temp_arr
                    .0
                    .parse::<u64>()
                    .expect("Cannot parse the left row number"),
            );
            right_row.push(
                temp_arr
                    .1
                    .parse::<u64>()
                    .expect("Cannot parse the right row number"),
            );
        })
        .collect();
    left_row.sort();
    right_row.sort();

    let mut sum: u64 = 0;
    for (x, y) in left_row.iter().zip(right_row.iter()) {
        sum += x.abs_diff(*y);
    }

    sum
}

fn part2(data: &str) -> u64 {
    let mut left_row: Vec<u64> = vec![];
    let mut right_row: Vec<u64> = vec![];
    let _: Vec<_> = data
        .lines()
        .map(|line| {
            let temp_arr: Vec<&str> = line.trim().split("   ").collect();
            left_row.push(
                temp_arr[0]
                    .parse::<u64>()
                    .expect("Cannot parse the left row number"),
            );
            right_row.push(
                temp_arr[1]
                    .parse::<u64>()
                    .expect("Cannot parse the right row number"),
            );
        })
        .collect();

    let mut total_similarity: u64 = 0;
    let mut total_times;
    for &number_l in &left_row {
        total_times = 0;
        for &number_r in &right_row {
            if number_r == number_l {
                total_times += 1;
            }
        }
        total_similarity += number_l * total_times;
    }
    total_similarity
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
