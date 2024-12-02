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
    
    let mut similarity = 0u64;
    let location_count = left_column.len();

    for location_id in left_column {
        let occurences = right_column.iter()
            .filter(|bad_writing| location_id.eq(bad_writing))
            .count();

        similarity = similarity.checked_add(
            (location_id as u64) * (occurences as u64)
        ).expect("Sum must not exceed u64::MAX");
    }

    println!("The similarity score for all {} locations is {}", location_count, similarity)
}