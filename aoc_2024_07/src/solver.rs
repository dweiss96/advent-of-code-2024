use crate::InputLine;


pub struct Solver {
    input: Vec<InputLine>
}

impl Solver {
    pub fn new(input: &Vec<InputLine>) -> Solver {
        Solver {
            input: input.clone()
        }
    }

    fn possible_results(input: &Vec<u64>, max_value: u128) -> Vec<u128> {
        let (initial, rest) = input.split_at(1);

        rest.iter().fold(initial.iter().map(|n| n.clone().into()).collect(), |possible_checksums, number| {
            possible_checksums.iter().flat_map(|cs| {
                vec![
                    cs.saturating_mul(number.clone().into()),
                    cs.saturating_add(number.clone().into()),
                ]
            }).filter(|psc| {
                psc <= &max_value
            }).collect()
        })
    }

    pub fn calculate_plus_mul_checksum_sum(self) -> u128 {
        self.input.iter().map(|il| {
            if Self::possible_results(&il.numbers, il.checksum).contains(&il.checksum) {
                il.checksum
            } else {
                0
            }
        }).sum()
    }
}
