use std::io::{self, BufRead};
use std::{error::Error, fs::File};

#[derive(Debug)]
struct Position {
    num: usize,
    marked: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let day = std::env::args()
        .nth(1)
        .expect("No date is given")
        .parse::<usize>()?;

    let day_input: Vec<String> = io::BufReader::new(File::open(format!("input/{}.txt", day))?)
        .lines()
        //.into_iter()
        .filter_map(|line| line.ok())
        .collect();

    println!("{:?}", day_input);
    Ok(())
}

// let day_input: Vec<String> = io::BufReader::new(File::open("src/input.txt").unwrap())
//     .lines()
//     .into_iter()
//     .filter_map(|line| line.ok())
//     .collect();

// let numbers: Vec<usize> = day_input[0]
//     .split('\n')
//     .map(|s| s.parse::<usize>())
//     .filter_map(|l| l.ok())
//     .collect();

// let mut boards: Vec<Vec<Vec<Position>>> = day_input[2..]
//     .windows(6)
//     .map(|window| {
//         window
//             .iter()
//             .map(|row| {
//                 row.split(" ")
//                     .map(|s| s.parse::<usize>())
//                     .filter_map(|l| l.ok())
//                     .map(|n| Position {
//                         num: n,
//                         marked: false,
//                     })
//                     .collect::<Vec<Position>>()
//             })
//             .filter(|l| !l.is_empty())
//             .collect::<Vec<Vec<Position>>>()
//     })
//     .collect();

// for num in numbers {
//     for board in boards.iter_mut() {
//         let mut found = false;

//         board.iter_mut().for_each(|row| {
//             row.iter_mut().for_each(|column| {
//                 if column.num == num {
//                     column.marked = true;
//                     found = true;
//                 }
//             })
//         });

//         if found == true {}
//     }
// }

// println!("{:?}", boards);
