fn main() {}

fn part1(input: Vec<String>) -> u64 {
    let numbers: Vec<Vec<u64>> = input
        .iter()
        .map(|row| {
            let numbers: Vec<u64> = row
                .chars()
                .map(|x| match x {
                    '1' => 1,
                    '0' => 0,
                    _ => panic!("Crap"),
                })
                .collect();

            numbers
        })
        .collect();

    let len = numbers[0].len();
    let number_entries = numbers.len();

    let mut gamma_rate_numbers = Vec::new();
    let mut epsilon_rate_numbers = Vec::new();

    for index in 0..len {
        let ones = numbers.iter().filter(|num| num[index] == 1).count();
        let gamma_result = if ones >= (number_entries / 2) { 1 } else { 0 };
        let epsilon_result = if ones >= (number_entries / 2) { 0 } else { 1 };

        gamma_rate_numbers.push(gamma_result);
        epsilon_rate_numbers.push(epsilon_result);
    }

    vec_binary_to_decimal(&gamma_rate_numbers) * vec_binary_to_decimal(&epsilon_rate_numbers)
}

fn part2(input: Vec<String>) -> u64 {
    let numbers: Vec<Vec<u64>> = input
        .iter()
        .map(|row| {
            let numbers: Vec<u64> = row
                .chars()
                .map(|x| match x {
                    '1' => 1,
                    '0' => 0,
                    _ => panic!("Crap"),
                })
                .collect();

            numbers
        })
        .collect();

    let len = numbers[0].len();
    let number_entries = numbers.len();

    let mut nums = numbers.clone();

    for index in 0..len {
        if nums.len() == 1 {
            break;
        }
        let ones = nums.iter().filter(|num| num[index] == 1).count();
        let zeros = nums.len() - ones;

        let filter = if ones >= zeros { 1 } else { 0 };

        nums = nums.into_iter().filter(|n| n[index] == filter).collect();
    }

    let mut nums2 = numbers.clone();

    for index in 0..len {
        if nums2.len() == 1 {
            break;
        }

        let ones = nums2.iter().filter(|num| num[index] == 1).count();
        let zeros = nums2.len() - ones;

        let filter = if ones >= zeros { 0 } else { 1 };

        nums2 = nums2.into_iter().filter(|n| n[index] == filter).collect();
    }

    vec_binary_to_decimal(&nums[0]) * vec_binary_to_decimal(&nums2[0])
}

fn vec_binary_to_decimal<T>(input: &Vec<T>) -> u64
where
    T: ToString,
{
    let s: String = input.iter().map(|x| x.to_string()).collect();

    u64::from_str_radix(&s, 2).unwrap()
}

#[cfg(test)]
mod day3_tests {
    mod part1 {
        use crate::part1;

        #[test]
        fn test_example() {
            let input = vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect();
            let result = part1(input);
            assert_eq!(result, 198)
        }

        #[test]
        fn test_input() {
            let input = aoc::read_file_to_vec("input/day3.txt");
            let expected = 1071734;
            let result = part1(input);
            assert_eq!(result, expected);
        }
    }

    mod part2 {
        use crate::part2;

        #[test]
        fn test_example() {
            let input = vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect();
            let result = part2(input);
            assert_eq!(result, 230)
        }

        #[test]
        fn test_input() {
            let input = aoc::read_file_to_vec("input/day3.txt");
            let expected = 6124992;
            let result = part2(input);
            assert_eq!(result, expected);
        }
    }
}
