pub struct Solver {
    input: Vec<String>,
    position_of_guard: (usize,usize), // mainly created to not calculate/search position each tick
    debug_mode: bool,
}

impl Solver {
    fn calculate_position_of_guard(input: &Vec<String>) -> (usize,usize) {
        input.iter().enumerate().fold((0, 0), |position, (y, line)| {
            let maybe_found_guard =  line.find('^')
                .or_else(|| line.find('>'))
                .or_else(|| line.find('<'))
                .or_else(|| line.find('v'));

            match maybe_found_guard {
                Some(x) => (x, y),
                None => position
            }
        })
    }

    pub fn new_with_debug(input: &Vec<String>) -> Solver {
        Solver {
            input: input.clone(),
            position_of_guard: Self::calculate_position_of_guard(input),
            debug_mode: true,
        }
    }

    pub fn new(input: &Vec<String>) -> Solver {
        Solver {
            input: input.clone(),
            position_of_guard: Self::calculate_position_of_guard(input),
            debug_mode: false,
        }
    }

    fn print_state(&self) {
        if self.debug_mode {
            print!("\x1B[2J\x1B[1;1H");
            println!("CURRENT STATE WITH POSITION {},{}", self.position_of_guard.0, self.position_of_guard.1);
            for l in &self.input {
                println!("{l}")
            }
            println!();
        }
    }

    fn calculate_unique_positions(&self) -> u128 {
        self.input.iter().fold(0, |xs, line| {
            xs + line.chars().fold(0, |line_xs, c| {
                if c.eq_ignore_ascii_case(&'X') {
                    line_xs+1
                } else {
                    line_xs
                }
            })
        })
    }

    fn safe_get_char_at(&self, x: usize, y: usize) -> Option<char> {
        self.input.get(y)
            .and_then(|s| {
                s.chars().collect::<Vec<char>>().get(x).cloned()
            })
    }

    fn get_square_in_front_of_guard(&self) -> Option<char> {
        match self.safe_get_char_at(self.position_of_guard.0, self.position_of_guard.1) {
            Some('^') => self.safe_get_char_at(self.position_of_guard.0, self.position_of_guard.1.wrapping_sub(1)),
            Some('v') => self.safe_get_char_at(self.position_of_guard.0, self.position_of_guard.1 + 1),
            Some('>') => self.safe_get_char_at(self.position_of_guard.0 + 1, self.position_of_guard.1),
            Some('<') => self.safe_get_char_at(self.position_of_guard.0.wrapping_sub(1), self.position_of_guard.1),
            Some('X') | None => None,
            Some(c) => panic!("POSITION IS INVALID, READ CHAR {c}"),
        }
    }

    fn move_guard(self) -> Solver  {
        self.print_state();
        let (current_x, current_y) = self.position_of_guard;
        let guard = self.input[current_y].as_bytes()[current_x] as char;

        let (new_x, new_y) = match guard {
            '^' => (current_x, current_y.wrapping_sub(1)),
            '>' => (current_x+1, current_y),
            'v' => (current_x, current_y+1),
            '<' => (current_x.wrapping_sub(1), current_y),
            _ => panic!("NO GUARD FOUND TO MOVE")
        };

        let new_input: Vec<String> = self.input.iter().enumerate().map(|(y, line)| {
            line.chars().enumerate().map(|(x, char)| {
                if x == new_x && y == new_y {
                    format!("{guard}")
                } else if x == current_x && y == current_y {
                    "X".to_string()
                } else {
                    format!("{char}")
                }
            }).collect::<Vec<String>>().join("")
        }).collect::<Vec<String>>();

        Solver{
            input: new_input,
            position_of_guard: (new_x, new_y),
            debug_mode: self.debug_mode,
        }
    }

    fn rotate_guard(self) -> Solver  {
        self.print_state();

        let new_input = self.input.iter().enumerate().map(|(y, line)| {
            if y == self.position_of_guard.1 {
                line.chars().enumerate().map(|(x, char)| {
                    if x == self.position_of_guard.0 {
                        match char {
                            '^' => '>',
                            '>' => 'v',
                            'v' => '<',
                            '<' => '^',
                            _ => char
                        }
                    } else {
                        char
                    }
                }).map(|c| format!("{c}")).collect::<Vec<String>>().join("")
            } else {
                line.clone()
            }
        }).collect::<Vec<String>>();

        Solver {
            input: new_input,
            position_of_guard: self.position_of_guard,
            debug_mode: self.debug_mode,
        }
    }

    fn exit_guard(self) -> u128 {
        self.calculate_unique_positions()+1 // +1 for the current guard position
    }

    fn do_tick(self) -> Result<Solver, u128> {
        match self.get_square_in_front_of_guard() {
            Some('.') | Some('X') => Ok(self.move_guard()),
            Some('#') => Ok(self.rotate_guard()),
            None => Err(self.exit_guard()),
            Some(c) => panic!("Unknown char {c}")
        }
    }
    pub fn get_unique_visited_positions(self) -> u128 {
        let mut result = Ok(self);

        loop {
            match result {
                Ok(solver) => {
                    result = solver.do_tick();
                },
                Err(unique_positions) => return unique_positions,
            }
        }
    }
}
