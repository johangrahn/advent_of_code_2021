fn main() {
    //let input = aoc::read_file_to_vec("input/day1.txt");
}

fn string_to_usize(input: String) -> usize {
    input.to_string().parse::<_>().unwrap()
}
fn part1(input: Vec<String>) -> u64 {
    let mut prev = 0;
    input
        .iter()
        .map(|s| string_to_usize(s.to_string()))
        .map(|x| {
            if prev == 0 {
                prev = x;
            }

            let result = if x > prev { 1 } else { 0 };

            prev = x;
            result
        })
        .sum()
}

fn part2(input: Vec<String>) -> u64 {
    let mut prev = 0;
    input[..]
        .windows(3)
        .map(|window| {
            let sum = window
                .into_iter()
                .map(|s| string_to_usize(s.to_string()))
                .sum();
            if prev == 0 {
                prev = sum;
            }

            let result = if sum > prev { 1 } else { 0 };

            prev = sum;
            result
        })
        .sum()
}

#[cfg(test)]
mod day1 {
    mod part1 {
        use crate::part1;
        #[test]
        fn test_example() {
            let input = vec![
                "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect();
            let expected = 7;
            let result = part1(input);
            assert_eq!(result, expected);
        }

        #[test]
        fn test_input() {
            let input = aoc::read_file_to_vec("input/day1.txt");
            let expected = 1233;
            let result = part1(input);
            assert_eq!(result, expected);
        }
    }

    mod part2 {
        use crate::part2;
        #[test]
        fn test_example() {
            let input = vec![
                "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect();
            let expected = 5;
            let result = part2(input);
            assert_eq!(result, expected);
        }

        #[test]
        fn test_input() {
            let input = aoc::read_file_to_vec("input/day1.txt");
            let expected = 1275;
            let result = part2(input);
            assert_eq!(result, expected);
        }
    }
}
