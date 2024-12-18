mod solver;

use std::fs;
use crate::solver::Solver;


fn read_input() -> Vec<Vec<char>> {
    let input_file_string = fs::read_to_string("input")
        .expect("Input file should be available");

    input_file_string
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn main() {
    let input = read_input();

    println!("The antennas create {} unique antinodes inside the bounds of the map. [254]", Solver::new(&input).calculate_distinct_antinode_positions())
}
