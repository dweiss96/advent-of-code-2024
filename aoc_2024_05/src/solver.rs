use std::collections::HashSet;
use std::ops::Div;
use crate::Input;
use std::iter::FromIterator;


pub struct Solver {
    input: Input
}

impl Solver {
    pub fn new(input: Input) -> Solver {
        Solver {
            input
        }
    }

    fn get_valid_orders(&self) -> Vec<Vec<u8>> {
        Vec::from_iter(self.input.orders.iter().filter(|order| {
            order.iter().fold((true, HashSet::<u8>::new()), |(is_valid, pages_printed_until_now), page_number| {
                if !is_valid {
                    return (is_valid, pages_printed_until_now);
                }

                let requirements = self.input.requirements.get(page_number).cloned().unwrap_or(HashSet::new());

                let has_unmatched_requirements = requirements.iter().fold(false, |has_unmatched_requirements, req| {
                    if order.contains(req) {
                        !pages_printed_until_now.contains(req) || has_unmatched_requirements
                    } else {
                        has_unmatched_requirements
                    }
                });

                if has_unmatched_requirements {
                    (false, pages_printed_until_now)
                } else {
                    let mut pages_printed = pages_printed_until_now.clone();
                    let _ = pages_printed.insert(page_number.clone());

                    (true, pages_printed)
                }
            }).0
        }).cloned())
    }

    pub fn calculate_valid_order_checksum(self) -> u128 {
        let valid_orders = self.get_valid_orders();

        valid_orders.iter().map(|order| {
            order[order.len().div(2)]
        }).fold(0, |sum, middle| { sum + middle as u128 })
    }

}