use std::fs;

fn read_input() -> (Vec<u32>, Vec<u32>) {
    let input_file_string = fs::read_to_string("input").expect("Input file should be available");

    let (mut left_column, mut right_column) = (Vec::new(), Vec::new());

    input_file_string.split("\n").for_each(|l| {
        if l.trim().is_empty() {
            return;
        }
        let (left, right) = l.split_once("   ").expect("Input file should have two columns separated by three spaces");
        left_column.push(
            left.parse().expect("left value should be an unsigned int < u32::MAX")
        );
        right_column.push(right.parse().expect("left value should be an unsigned int < u32::MAX"));
    });

    left_column.sort();
    right_column.sort();

    (left_column, right_column)
}

fn main() {
    let (left_column, right_column) = read_input();
    
    if left_column.len() != right_column.len() {
        println!("left and right column must have same item count but did not!");
        return;
    }
    
    let mut distance = 0u64;

    for index in 0..left_column.len() {
        distance = distance.checked_add(left_column[index].abs_diff(right_column[index]) as u64)
            .expect("Sum must not exceed u64::MAX");
    }

    println!("The distance for all {} pairs is {}", left_column.len(), distance)
}
