mod solver;

use std::collections::{HashMap, HashSet};
use std::fs;
use crate::solver::Solver;

#[derive(Clone)]
struct Input {
    requirements: HashMap<u8, HashSet<u8>>,
    orders: Vec<Vec<u8>>,
}

fn read_input() -> Input {
    let input_file_string = fs::read_to_string("input")
        .expect("Input file should be available");

    input_file_string.split("\n").fold( Input{
        requirements: HashMap::new(),
        orders: Vec::new(),
    },|inputs, line| {
        if !line.trim().is_empty() {
            if line.contains("|") {
                let (read_required_page, read_required_for) = line.split_once('|').unwrap();
                let required_page = read_required_page.parse().unwrap_or_default();
                let required_for = read_required_for.parse().unwrap_or_default();

                let mut requirements = inputs.requirements.clone();

                let mut page_requirements = inputs.requirements.get(&required_for).cloned().unwrap_or(HashSet::new());
                let _ = page_requirements.insert(required_page);

                let _ = requirements.insert(required_for, page_requirements);

                Input {
                    requirements,
                    orders: inputs.orders
                }
            } else {
                let order = Vec::from_iter(line.split(',').map(|num| {
                   num.parse().unwrap_or_default()
                }));

                let mut orders = inputs.orders.clone();

                orders.push(order);

                Input {
                    requirements: inputs.requirements,
                    orders,
                }
            }
        } else {
            inputs
        }
    })
}

fn main() {
    let input = read_input();

    println!("The middle number checksum for all valid orders is {}. [7074]", Solver::new(input.clone()).calculate_valid_order_checksum());
    // println!("Within the Input {} X-MAS could be found. [2046]", Solver::new(input).count_x_mas());
}
