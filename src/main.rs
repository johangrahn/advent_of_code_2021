use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    //let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    let input = io::BufReader::new(File::open("src/input.txt").unwrap())
        .lines()
        .into_iter()
        .filter_map(|x| x.ok())
        .filter_map(|s| s.parse::<i32>().ok());

    let something: i64 = part2(input.collect());
    //let something = part2(input);

    println!("Sum: {}", something)
}

fn part1(input: Vec<i32>) -> i64 {
    let mut prev_value = 0;
    input
        .into_iter()
        .map(|x| {
            let mut found = 0;
            if prev_value == 0 {
                prev_value = x;
            };

            if x > prev_value {
                found = 1;
            }

            prev_value = x;
            found
        })
        .sum()
}

fn part2(input: Vec<i32>) -> i64 {
    let mut prev_value = 0;
    let s: i64 = input[..]
        .windows(3)
        .map(|w| {
            let mut bigger = 0;
            let result = w.into_iter().sum();
            if prev_value != 0 {
                if result > prev_value {
                    bigger = 1;
                }
            }
            prev_value = result;
            bigger
        })
        .sum::<i64>();
    s
}
