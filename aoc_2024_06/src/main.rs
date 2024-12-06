mod solver;

use std::fs;
use crate::solver::Solver;

fn read_input() -> Vec<String> {
    let input_file_string = fs::read_to_string("input")
        .expect("Input file should be available");

    input_file_string.split("\n").map(|s| s.to_string()).collect()
}

fn main() {
    let input = read_input();

    println!("The guard visited {} unique locations. [4977]", Solver::new(&input).get_unique_visited_positions());
}
