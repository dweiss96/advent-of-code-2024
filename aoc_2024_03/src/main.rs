mod parser_part_one;
mod parser_part_two;

use std::fs;
use parser_part_one::ParserPartOne;
use parser_part_two::ParserPartTwo;

fn read_input() -> String {
    fs::read_to_string("input")
        .expect("Input file should be available")
}

fn part_one(input: String) -> u128 {
    input.as_str().chars().fold(ParserPartOne::new(), |fsm, character| {
        fsm.analyze_character(character)
    }).get_sum()
}

fn part_two(input: String) -> u128 {
    input.as_str().chars().fold(ParserPartTwo::new(), |fsm, character| {
        fsm.analyze_character(character)
    }).get_sum()
}

fn main() {
    let input = read_input();

    println!("Sum of all valid `mul` instructions is [159892596] {}", part_one(input.clone()));
    println!("Sum of all valid `mul` with blocking instructions is [92626942] {}", part_two(input));
}
