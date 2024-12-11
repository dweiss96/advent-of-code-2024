mod solver;

use std::fs;
use crate::solver::Solver;

#[derive(Clone)]
struct InputLine {
    checksum: u128,
    numbers: Vec<u64>,
}

fn read_input() -> Vec<InputLine> {
    let input_file_string = fs::read_to_string("input")
        .expect("Input file should be available");

    input_file_string.split("\n").fold(Vec::new(), |inputs, line| {
        let (checksum, numbers) = line.split_once(": ").unwrap();
        let mut resulting_inputs = inputs;

        resulting_inputs.push(InputLine {
            checksum: checksum.parse().unwrap_or(u128::MAX),
            numbers: numbers.split(" ").map(|number| number.parse().unwrap()).collect()
        });

        resulting_inputs
    })
}

fn main() {
    let input = read_input();

    println!("The sum of checksums for all valid equations is {}. [538191549061]", Solver::new(&input).calculate_plus_mul_checksum_sum());
}
