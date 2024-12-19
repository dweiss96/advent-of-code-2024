mod solver;
mod solver_v2;

use std::fs;
use crate::solver::Solver;
use crate::solver_v2::SolverV2;


fn read_input() -> Vec<u8> {
    let input_file_string = fs::read_to_string("input")
        .expect("Input file should be available");

    input_file_string.chars().map(|c| c.to_string().parse().unwrap()).collect()
}

fn main() {
    let input = read_input();

    // println!("The checksum is {}. [?]", Solver::new_with_debug(&input).calculate_checksum());
    println!("The checksum is {}. [6366665108136]", Solver::new(&input).calculate_checksum());
    println!("The checksum is {} when defrag whole files. [6398065450842]", SolverV2::new(&input).calculate_checksum());
}
