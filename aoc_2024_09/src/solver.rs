pub struct Solver {
    input: Vec<u8>,
    disk: Vec<i128>,
    debug_mode: bool,
}

enum DefragState {
    Fine, Triggered, Required
}

impl Solver {
    fn initialize_disk(input: &Vec<u8>) -> Vec<i128> {
        input.iter().fold((Vec::new(), 0, false), |(disk, current_id, needs_space), number| {
            let mut new_disk = disk.clone();
            let next_id = if needs_space {
                for _ in 0..number.clone() {
                    new_disk.push(-1);
                }
                current_id
            } else {
                for _ in 0..number.clone() {
                    new_disk.push(current_id);
                }
                current_id+1
            };
            (new_disk, next_id, !needs_space)
        }).0
    }

    pub fn new_with_debug(input: &Vec<u8>) -> Solver {
        Solver {
            input: input.clone(),
            disk: Self::initialize_disk(input),
            debug_mode: true,
        }
    }

    pub fn new(input: &Vec<u8>) -> Solver {
        Solver {
            input: input.clone(),
            disk: Self::initialize_disk(input),
            debug_mode: false,
        }
    }

    fn needs_defragmentation(disk: Vec<i128>) -> bool {
        let disk_state = disk.iter().fold(DefragState::Fine, |state, disk_value| {
            match state {
                DefragState::Fine if disk_value.clone() >= 0 => DefragState::Fine,
                DefragState::Fine if disk_value.clone() < 0 => DefragState::Triggered,
                DefragState::Triggered if disk_value.clone() < 0 => DefragState::Triggered,
                _ => DefragState::Required
            }
        });

        match disk_state {
            DefragState::Fine | DefragState::Triggered => false,
            DefragState::Required => true,
        }
    }

    fn print_disk(disk: &Vec<i128>) {
        println!("Disk : {}", disk.iter().map(|n| {
            if *n < 0 {
                ".".to_string()
            } else {
                n.to_string()
            }
        }).collect::<Vec<String>>().join(""));
    }

    pub fn calculate_checksum(self) -> u128 {
        if self.debug_mode {
            println!("Input: {}", self.input.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(""));
            Self::print_disk(&self.disk);
        }

        let mut current_disk = self.disk.clone();
        while Self::needs_defragmentation(self.disk.clone()) {
            let first_space = current_disk.iter().enumerate().find(|(_, num)| num.clone().eq(&-1));
            let last_block = current_disk.iter().enumerate().rfind(|(_, num)| num.clone().ne(&-1));

            match (first_space, last_block) {
                (Some((a, _)), Some((b, _))) if a < b => {
                    current_disk.swap(a, b)
                }
                _ => break,
            }

            if self.debug_mode {
                Self::print_disk(&current_disk);
            }
        }

        current_disk.iter().enumerate().filter(|(_, block)| 0.le(block.clone())).fold(0, |sum, (index, block)| {
            sum + (index as u128 * block.abs_diff(0))
        })
    }
}
