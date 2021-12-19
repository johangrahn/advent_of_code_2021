fn main() {}

fn part1(input: Vec<String>) -> u64 {
    let mut depth = 0;
    let mut hos = 0;

    input.iter().for_each(|line| {
        let instruction: Vec<&str> = line.split(' ').collect();
        let step: u64 = instruction[1].parse().unwrap();
        match instruction[0] {
            "forward" => hos += step,
            "down" => depth += step,
            "up" => depth -= step,
            _ => panic!("Crap"),
        }
    });

    depth * hos
}

fn part2(input: Vec<String>) -> u64 {
    let mut depth = 0;
    let mut hos = 0;
    let mut aim = 0;

    input.iter().for_each(|line| {
        let instruction: Vec<&str> = line.split(' ').collect();
        let step: u64 = instruction[1].parse().unwrap();
        match instruction[0] {
            "forward" => {
                hos += step;
                depth += aim * step
            }
            "down" => aim += step,
            "up" => aim -= step,
            _ => panic!("Crap"),
        }
    });

    depth * hos
}

#[cfg(test)]
mod day2_tests {
    mod part1 {
        use crate::part1;

        #[test]
        fn test_example() {
            let input = vec![
                "forward 5",
                "down 5",
                "forward 8",
                "up 3",
                "down 8",
                "forward 2",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect();
            let result = part1(input);
            assert_eq!(result, 150)
        }

        #[test]
        fn test_input() {
            let input = aoc::read_file_to_vec("input/day2.txt");
            let expected = 2070300;
            let result = part1(input);
            assert_eq!(result, expected);
        }
    }

    mod part2 {
        use crate::part2;
        #[test]
        fn test_example() {
            let input = vec![
                "forward 5",
                "down 5",
                "forward 8",
                "up 3",
                "down 8",
                "forward 2",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect();
            let result = part2(input);
            assert_eq!(result, 900)
        }

        #[test]
        fn test_input() {
            let input = aoc::read_file_to_vec("input/day2.txt");
            let expected = 2078985210;
            let result = part2(input);
            assert_eq!(result, expected);
        }
    }
}
