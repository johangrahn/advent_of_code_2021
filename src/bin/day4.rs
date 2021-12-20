fn main() {}

#[derive(Debug)]
struct Position {
    num: usize,
    marked: bool,
}

fn part1(input: Vec<String>) -> usize {
    let numbers: Vec<usize> = input[0].split(',').map(|s| s.parse().unwrap()).collect();

    let mut boards: Vec<Vec<Vec<Position>>> = input[2..]
        .chunks(6)
        .map(|window| {
            window
                .iter()
                .map(|row| {
                    row.split(" ")
                        .map(|s| s.parse::<usize>())
                        .filter_map(|l| l.ok())
                        .map(|n| Position {
                            num: n,
                            marked: false,
                        })
                        .collect::<Vec<Position>>()
                })
                .filter(|l| !l.is_empty())
                .collect::<Vec<Vec<Position>>>()
        })
        .collect();

    let mut score = 0;
    for number in &numbers {
        if score != 0 {
            break;
        }
        boards.iter_mut().for_each(|board| {
            if score != 0 {
                return;
            }

            board.iter_mut().for_each(|row| {
                for pos in row {
                    if *number == pos.num {
                        pos.marked = true;
                    }
                }
            });

            if board_wins(board) {
                score = board_get_score(board, *number);
                return;
            }
        });
    }

    score
}

fn board_wins(board: &Vec<Vec<Position>>) -> bool {
    for row in board {
        let len = row.len();
        let num = row.iter().filter(|f| f.marked == true).count();
        if num == len {
            return true;
        }
    }

    false
}

fn board_get_score(board: &Vec<Vec<Position>>, win_number: usize) -> usize {
    let sum: usize = board
        .iter()
        .map(|row| {
            row.iter()
                .filter(|r| r.marked == false)
                .map(|r| r.num)
                .sum::<usize>()
        })
        .sum();
    win_number * sum
}

#[cfg(test)]
mod day4_tests {
    mod part1 {
        use crate::part1;

        #[test]
        fn test_example() {
            let input = vec![
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
                "",
                "22 13 17 11  0",
                " 8  2 23  4 24",
                "21  9 14 16  7",
                " 6 10  3 18  5",
                " 1 12 20 15 19",
                "",
                " 3 15  0  2 22",
                " 9 18 13 17  5",
                "19  8  7 25 23",
                "20 11 10 24  4",
                "14 21 16 12  6",
                "",
                "14 21 17 24  4",
                "10 16 15  9 19",
                "18  8 23 26 20",
                "22 11 13  6  5",
                " 2  0 12  3  7",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect();
            let result = part1(input);
            assert_eq!(result, 4512)
        }

        #[test]
        fn test_input() {
            let input = aoc::read_file_to_vec("input/day4.txt");
            let expected = 1071734;
            let result = part1(input);
            assert_eq!(result, expected);
        }
    }
}
