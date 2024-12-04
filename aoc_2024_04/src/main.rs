mod solver;

use std::fs;
use crate::solver::Solver;

fn read_input() -> Vec<Vec<char>> {
    let input_file_string = fs::read_to_string("input")
        .expect("Input file should be available");

    Vec::from_iter(input_file_string.split("\n").map(|line| {
        Vec::from_iter(line.chars())
    }))
}

fn main() {
    let input = read_input();

    println!("Within the Input {} XMAS could be found. [2718]", Solver::new(input.clone()).count_xmas());
    println!("Within the Input {} X-MAS could be found. [2046]", Solver::new(input).count_x_mas());
}
