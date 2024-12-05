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

    fn get_valid_or_invalid_orders(&self, invalid_orders: bool) -> Vec<Vec<u8>> {
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
            }).0 ^ invalid_orders
        }).cloned())
    }

    fn get_valid_orders(&self) -> Vec<Vec<u8>> {
        self.get_valid_or_invalid_orders(false)
    }
    fn get_invalid_orders(&self) -> Vec<Vec<u8>> {
        self.get_valid_or_invalid_orders(true)
    }

    pub fn calculate_valid_order_checksum(self) -> u128 {
        let valid_orders = self.get_valid_orders();

        valid_orders.iter().map(|order| {
            order[order.len().div(2)]
        }).fold(0, |sum, middle| { sum + middle as u128 })
    }

    pub fn calculate_fixed_invalid_order_checksum(self) -> u128 {
        let invalid_orders = self.get_invalid_orders();

        let mut sum: u128 = 0;

        for order in invalid_orders {
            let mut old_order: Vec<u8> = order.clone();
            let mut new_order: Vec<u8> = Vec::new();

            while new_order.len() < order.len() {
                let open_order_pages: Vec<(usize, HashSet<u8>)> = old_order.iter().enumerate().map(|(index, number)| {
                    let requirements: HashSet<u8> = self.input.requirements.get(number).cloned().unwrap_or(HashSet::new());

                    let requirements_in_order: HashSet<u8> = requirements.iter().filter(|n| {
                        order.contains(n)
                    }).cloned().collect();

                    let requirements_not_yet_fulfilled: HashSet<u8> = requirements_in_order.iter().filter(|n| {
                        !new_order.contains(n)
                    }).cloned().collect();

                    (
                        index,
                        requirements_not_yet_fulfilled
                    )
                }).collect();

                open_order_pages.iter().filter(|(_, p)| {
                    p.is_empty()
                }).for_each(|(index, _)| {
                    new_order.push(old_order.remove(index.clone()))
                });
            }

            sum += new_order[new_order.len().div(2)] as u128;
        }

        sum
    }
}
