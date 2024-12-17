mod solver;

use std::fs;
use std::time::SystemTime;
use crate::solver::Solver;

fn read_input() -> Vec<Vec<char>> {
    let input_file_string = fs::read_to_string("input")
        .expect("Input file should be available");

    let mut actual_lines: Vec<Vec<char>> = input_file_string.split("\n").map(|s| {
        let mut line: Vec<char> = "  ".chars().collect();

        let mut actual_line = s.chars().collect::<Vec<char>>();

        line.append(&mut actual_line);

        line.push(' ');
        line.push(' ');

        line
    }).collect();

    let max_line_length = actual_lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut lines = vec![
        vec![' '; max_line_length],
        vec![' '; max_line_length],
    ];

    lines.append(&mut actual_lines);

    lines.push(vec![' '; max_line_length]);
    lines.push(vec![' '; max_line_length]);

    lines
}

fn time_command<R, F>(x: F) -> (R, u128) where F : FnOnce() -> R {
    let t_now = SystemTime::now();
    let result = x();
    let dt = t_now.elapsed().map(|d| d.as_millis()).unwrap_or(u128::MAX);

    (result, dt)
}

fn main() {
    let (input, dt_input) = time_command(|| {read_input()});
    println!("Read input in {dt_input}ms.");

    let (unique_positions, dt_calculation) = time_command(|| {
        Solver::new_with_debug(&input).get_unique_visited_positions()
    });

    println!("The guard visited {unique_positions} unique locations [4977]. Result calculated in {dt_calculation}ms");
}
