use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let day_input: Vec<String> = io::BufReader::new(File::open("src/input.txt").unwrap())
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .collect();

    let mut numbers: Vec<[usize; 12]> = day_input
        .iter()
        .map(|row| {
            let mut number = [0; 12];
            row.chars()
                .enumerate()
                .for_each(|(index, item)| match item {
                    '1' => number[index] = 1,
                    '0' => number[index] = 0,
                    _ => panic!("Shit"),
                });
            number
        })
        .collect();

    println!("{:#?}", numbers);
    let backup = numbers.clone();

    for it in 0..12 {
        if numbers.len() == 1 {
            break;
        }
        let mut zero = 0;
        let mut one = 0;
        numbers.iter().for_each(|x| {
            let bit = x[it];

            if bit == 1 {
                one += 1;
            } else {
                zero += 1;
            }
        });

        let compare = if one >= zero { 1 } else { 0 };
        numbers = numbers
            .into_iter()
            .filter(|f| f[it] == compare)
            .collect::<Vec<[usize; 12]>>();
    }

    let n = numbers[0];
    println!("{:?}", n);
    let s: String = n.iter().map(|x| x.to_string()).collect();

    let n2 = isize::from_str_radix(&s, 2).unwrap();
    println!("{:?}", n2);

    numbers = backup;
    for it in 0..12 {
        if numbers.len() == 1 {
            break;
        }
        let mut zero = 0;
        let mut one = 0;
        numbers.iter().for_each(|x| {
            let bit = x[it];

            if bit == 1 {
                one += 1;
            } else {
                zero += 1;
            }
        });

        let compare = if one >= zero { 0 } else { 1 };
        numbers = numbers
            .into_iter()
            .filter(|f| f[it] == compare)
            .collect::<Vec<[usize; 12]>>();
    }

    let n = numbers[0];
    println!("{:?}", n);
    let s: String = n.iter().map(|x| x.to_string()).collect();

    let n3 = isize::from_str_radix(&s, 2).unwrap();
    println!("{:?}", n2 * n3);

    // let mut counter: [i64; 12] = [0; 12];

    // let num_rows = input.len() as i64;

    // input.iter().for_each(|num| {
    //     num.chars().enumerate().for_each(|(index, item)| {
    //         match item {
    //             '1' => counter[index] += 1,
    //             '0' => (),
    //             _ => panic!("Shit"),
    //         }
    //     });
    // });

    // let number: String = counter
    //     .iter()
    //     .map(|&x| if x > (num_rows / 2) { '1' } else { '0' })
    //     .collect();

    // let result1 = isize::from_str_radix(&number, 2).unwrap();

    // println!("Number1: {}", number);

    // let number2: String = counter
    //     .iter()
    //     .map(|&x| if x > (num_rows / 2) { '0' } else { '1' })
    //     .collect();

    // let result2 = isize::from_str_radix(&number2, 2).unwrap();
    // println!("Number2: {}", number2);

    // println!("Result: {}", result1 * result2);
}

fn day1_part1(input: Vec<i32>) -> i64 {
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

fn day1_part2(input: Vec<i32>) -> i64 {
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
