use std::fs;
use parser_part_one::ParserPartOne;

fn read_input() -> String {
    fs::read_to_string("input")
        .expect("Input file should be available")
}

fn part_one(input: String) -> u128 {
    input.as_str().chars().fold(Parser::new(), |fsm, character| {
        fsm.analyze_character(character)
    }).sum_of_all_mults
}

fn main() {
    let input = read_input();

    println!("Sum of all valid `mul` instructions is {}", part_one(input));
}
