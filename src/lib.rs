use std::{fs::File, io::BufRead, io::BufReader};

pub fn read_file_to_vec(file_name: &str) -> Vec<String> {
    BufReader::new(File::open(file_name).unwrap())
        .lines()
        .filter_map(|line| line.ok())
        .collect()
}
