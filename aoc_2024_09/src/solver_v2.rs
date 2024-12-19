use std::iter::{Enumerate, FilterMap, Map};
use std::slice::Iter;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone)]
pub struct File {
    id: i128,
    size: u8,
}

impl File {
    fn to_string(&self) -> String {
        let id_string = if self.id == -1 {
            ".".to_string()
        } else {
            self.id.to_string()
        };
        vec![id_string; self.size as usize].join("")
    }
}

#[derive(Clone)]
pub struct SolverV2 {
    input: Vec<u8>,
    disk: Vec<File>,
    debug_mode: bool,
}

impl SolverV2 {
    fn initialize_disk(input: &Vec<u8>) -> Vec<File> {
        input.iter().fold((Vec::new(), 0, false), |(disk, current_id, needs_space), number| {
            let mut new_disk = disk.clone();
            let next_id = if needs_space {
                new_disk.push(File {
                        id: -1,
                        size: number.clone(),
                    });
                current_id
            } else {
                new_disk.push(File {
                    id: current_id,
                    size: number.clone(),
                });
                current_id+1
            };
            (new_disk, next_id, !needs_space)
        }).0
    }

    pub fn new_with_debug(input: &Vec<u8>) -> SolverV2 {
        SolverV2 {
            input: input.clone(),
            disk: Self::initialize_disk(input),
            debug_mode: true,
        }
    }

    pub fn new(input: &Vec<u8>) -> SolverV2 {
        SolverV2 {
            input: input.clone(),
            disk: Self::initialize_disk(input),
            debug_mode: false,
        }
    }

    fn defragment_disk(&self, disk: Vec<File>) -> Option<Vec<File>> {
        disk.iter().enumerate()
            .filter(|(_, file)| file.id > -1)
            .filter_map(|(fpos, file)| {
                disk.iter().enumerate().find(|(spos, space)| {
                    space.id == -1 && spos < &fpos && space.size >= file.size
                }).map(|space| ((fpos, file), space))
            }).last().map(|(file, space)| {
                let mut new_disk = disk.clone();
                let mut remaining_space = space.1.size - file.1.size;

                if remaining_space > 0 {
                    new_disk.push(File {
                        id: -1,
                        size: remaining_space,
                    });
                    new_disk.swap_remove(space.0);

                    new_disk.push(File {
                        id: -1,
                        size: file.1.size,
                    });
                    new_disk.swap_remove(file.0);

                    new_disk.insert(space.0, file.1.clone())
                } else {
                    new_disk.swap(file.0, space.0);
                }

                new_disk
            })
    }

    fn print_disk(disk: &Vec<File>) {
        println!("Disk : {}", disk.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(""));
    }

    pub fn calculate_checksum(self) -> u128 {
        if self.debug_mode {
            println!("Input: {}", self.input.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(""));
            Self::print_disk(&self.disk);
        }

        let mut current_disk = self.disk.clone();
        while let Some(disk) = self.defragment_disk(current_disk.clone()) {
            current_disk = disk;

            if self.debug_mode {
                sleep(Duration::from_secs(1));
                Self::print_disk(&current_disk);
            }
        }

        current_disk.iter().flat_map(|file| {
            vec![file.id; file.size as usize]
        }).enumerate().filter(|(_, block)| 0.le(&block.clone())).fold(0, |sum, (index, block)| {
            sum + (index as u128 * block.abs_diff(0))
        })
    }
}
