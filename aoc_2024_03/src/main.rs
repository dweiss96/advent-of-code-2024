mod parser_part_one;

use std::fs;
use parser_part_one::ParserPartOne;

fn read_input() -> String {
    fs::read_to_string("input")
        .expect("Input file should be available")
}

fn part_one(input: String) -> u128 {
    input.as_str().chars().fold(ParserPartOne::new(), |fsm, character| {
        fsm.analyze_character(character)
    }).get_sum()
}

fn main() {
    let input = read_input();

    println!("Sum of all valid `mul` instructions is [159892596] {}", part_one(input.clone()));
}
