use std::thread::sleep;
use std::time::Duration;

pub struct Position {
    x: usize,
    y: usize
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
    pub fn min() -> Position {
        Self::new(usize::MIN, usize::MIN)
    }
    pub fn max() -> Position {
        Self::new(usize::MAX, usize::MAX)
    }
    pub fn viewport(&self) -> (Position, Position) {
        (
            Position {
                x: self.x.checked_sub(2).unwrap_or(0),
                y: self.y.checked_sub(2).unwrap_or(0),
            },
            Position {
                x: self.x.checked_add(2).unwrap_or(usize::MAX),
                y: self.y.checked_add(2).unwrap_or(usize::MAX),
            }
        )
    }

    pub fn move_up(self) -> Position {
        Position {
            x: self.x,
            y: self.y.checked_sub(1).unwrap_or(0),
        }
    }

    pub fn move_down(self) -> Position {
        Position {
            x: self.x,
            y: self.y.checked_add(1).unwrap_or(usize::MAX),
        }
    }

    pub fn move_left(self) -> Position {
        Position {
            x: self.x.checked_sub(1).unwrap_or(0),
            y: self.y,
        }
    }

    pub fn move_right(self) -> Position {
        Position {
            x: self.x.checked_add(1).unwrap_or(usize::MAX),
            y: self.y,
        }
    }
}

pub struct RelevantStateInformation {
    input: Vec<Vec<char>>,
    guard_mode: char,
    char_in_front: char,
    current_position: Position,
}

impl RelevantStateInformation {
    fn new(input: &Vec<Vec<char>>) -> RelevantStateInformation {
        let (guard, guard_position) = input.iter().enumerate().fold(
            (' ', Position::min()),
            |(guard, position), (y, line)| {
                line.iter().enumerate().find(|(_, c)| {
                    '^'.eq_ignore_ascii_case(c) || 'v'.eq_ignore_ascii_case(c) || '<'.eq_ignore_ascii_case(c) || '>'.eq_ignore_ascii_case(c)
                }).map(|(x,c)| {
                    (c.clone(), Position::new(x, y))
                }).unwrap_or((guard, position))
        });

        let focused_input: Vec<Vec<char>> = input.split_at(input.len().min(guard_position.y + 2)).0
            .split_at(guard_position.y.checked_sub(2).unwrap_or(0)).1.iter().map(
            |line| {
                line.split_at(line.len().min(guard_position.x + 2)).0
                    .split_at(guard_position.x.checked_sub(2).unwrap_or(0)).1
                    .iter().cloned().collect()
            }
        ).collect();

        let focused_y_pos: usize = match guard {
            '^' => 1,
            'v' => 3,
            _ => 2,
        };

        let focused_x_pos: usize = match guard {
            '<' => 1,
            '>' => 3,
            _ => 2,
        };

        RelevantStateInformation {
            input: focused_input.clone(),
            guard_mode: guard,
            current_position: guard_position,
            char_in_front: focused_input[focused_y_pos][focused_x_pos]
        }
    }

    fn rotate_guard(self) -> RelevantStateInformation {
        let new_guard_mode = match self.guard_mode {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            c => c,
        };

        let new_char_in_front = match new_guard_mode {
            '^' => self.input[1][2],
            '>' => self.input[2][3],
            'v' => self.input[3][2],
            '<' => self.input[2][1],
            _ => self.char_in_front
        };

        RelevantStateInformation {
            input: self.input,
            current_position: self.current_position,
            guard_mode: new_guard_mode,
            char_in_front: new_char_in_front,
        }
    }

    fn move_guard(self, new_focused_input: Vec<Vec<char>>) -> RelevantStateInformation {
        let new_position = match self.guard_mode {
            '^' => self.current_position.move_up(),
            '>' => self.current_position.move_right(),
            'v' => self.current_position.move_down(),
            '<' => self.current_position.move_left(),
            _ => self.current_position,
        };

        let new_char_in_front = match self.guard_mode {
            '^' => new_focused_input[1][2],
            '>' => new_focused_input[2][3],
            'v' => new_focused_input[3][2],
            '<' => new_focused_input[2][1],
            _ => self.char_in_front
        };

        RelevantStateInformation {
            input: new_focused_input,
            current_position: new_position,
            guard_mode: self.guard_mode,
            char_in_front: new_char_in_front,
        }
    }
}

pub struct Solver {
    input: Vec<Vec<char>>,
    state: RelevantStateInformation, // mainly created to not calculate/search position each tick
    debug_mode: bool,
}

impl Solver {
    pub fn new_with_debug(input: &Vec<Vec<char>>) -> Solver {
        Solver {
            input: input.clone(),
            state: RelevantStateInformation::new(input),
            debug_mode: true,
        }
    }

    pub fn new(input: &Vec<Vec<char>>) -> Solver {
        Solver {
            input: input.clone(),
            state: RelevantStateInformation::new(input),
            debug_mode: false,
        }
    }

    fn print_state(&self) {
        sleep(Duration::from_millis(10));
        if self.debug_mode {
            print!("\x1B[2J\x1B[1;1H");
            println!("CURRENT STATE WITH POSITION {},{}", self.state.current_position.x, self.state.current_position.y);
            for l in &self.state.input {
                println!("{}", l.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(""))
            }
            println!();
        }
    }

    fn calculate_unique_positions(&self) -> u128 {
        self.input.iter().fold(0, |xs, line| {
            xs + line.iter().fold(0, |line_xs, c| {
                match c.to_ascii_lowercase() {
                    'x' | '-' | '+' | '|' => line_xs+1,
                    _ => line_xs
                }
            })
        })
    }

    fn exit_guard(self) -> u128 {
        self.calculate_unique_positions()+1 // +1 for the current guard position

    }

    fn trim_input_line(min_x: usize, max_x: usize, input: &Vec<char>) -> Vec<char> {
        input.split_at(input.len().min(max_x)).0
            .split_at(min_x).1
            .iter().cloned().collect()
    }

    fn calculate_movement_char(new_char: char, existing_char: &char) -> char {
        match new_char {
            '|' if '-'.eq_ignore_ascii_case(existing_char) => '+',
            '-' if '|'.eq_ignore_ascii_case(existing_char) => '+',
            c => c,
        }
    }

    fn move_guard(self) -> Solver {
        let (min_pos, max_pos) = self.state.current_position.viewport();

        let mut new_viewport: Vec<Vec<char>> = Vec::new();

        let new_input: Vec<Vec<char>> = self.input.iter().enumerate().map(|(y, line)| {
            let y_in_new_viewport = match self.state.guard_mode {
                '^' => {
                    let y_big_enough = y >= min_pos.y.checked_sub(1).unwrap_or(0);
                    let y_small_enough = y <= max_pos.y.checked_sub(1).unwrap_or(0);

                    y_big_enough && y_small_enough
                }
                'v' => {
                    let y_big_enough = y >= min_pos.y.checked_add(1).unwrap_or(usize::MAX);
                    let y_small_enough = y <= max_pos.y.checked_add(1).unwrap_or(usize::MAX);

                    y_big_enough && y_small_enough
                }
                _ => y >= min_pos.y && y <= max_pos.y,
            };

            match self.state.guard_mode {
                '^' | 'v' if y_in_new_viewport => {
                    let new_y = match self.state.guard_mode {
                        '^' => self.state.current_position.y.checked_sub(1).unwrap_or(0),
                        'v' => self.state.current_position.y.checked_add(1).unwrap_or(usize::MAX),
                        _ => self.state.current_position.y,
                    };

                    let new_line = if y == self.state.current_position.y {
                        line.iter().enumerate().map(|(x, c)| {
                            if x == self.state.current_position.x {
                                Self::calculate_movement_char('|', c)
                            } else {
                                c.clone()
                            }
                        }).collect()
                    } else if y == new_y {
                        // write guard
                        line.iter().enumerate().map(|(x, c)| {
                            if x == self.state.current_position.x {
                                self.state.guard_mode
                            } else {
                                c.clone()
                            }
                        }).collect()
                    } else {
                        line.clone()
                    };
                    new_viewport.push(Self::trim_input_line(min_pos.x, max_pos.x, &new_line));

                    new_line
                }
                '<' | '>' if y_in_new_viewport => {
                    let new_line = if y == self.state.current_position.y {
                        line.iter().enumerate().map(|(x, c)| {
                            let new_x = match self.state.guard_mode {
                                '<' => self.state.current_position.x.checked_sub(1).unwrap_or(0),
                                '>' => self.state.current_position.x.checked_add(1).unwrap_or(usize::MAX),
                                _ => self.state.current_position.x,
                            };

                            if x == self.state.current_position.x {
                                Self::calculate_movement_char('-', c)
                            } else if x == new_x {
                                self.state.guard_mode
                            } else {
                                c.clone()
                            }
                        }).collect()
                    } else {
                        line.clone()
                    };


                    match self.state.guard_mode {
                        '<' => {
                            new_viewport.push(Self::trim_input_line(
                                min_pos.x.checked_sub(1).unwrap_or(0),
                                max_pos.x.checked_sub(1).unwrap_or(0),
                                &new_line
                            ));
                        }
                        '>' => {
                            new_viewport.push(Self::trim_input_line(
                                min_pos.x.checked_add(1).unwrap_or(usize::MAX),
                                max_pos.x.checked_add(1).unwrap_or(usize::MAX),
                                &new_line
                            ));
                        }
                        _ => {}
                    };

                    new_line
                }
                _ => line.clone()
            }
        }).collect();

        Solver {
            input: new_input,
            state: self.state.move_guard(new_viewport),
            debug_mode: self.debug_mode,
        }
    }

    fn rotate_guard(self) -> Solver {
        Solver {
            input: self.input,
            debug_mode: self.debug_mode,
            state: self.state.rotate_guard()
        }
    }

    fn do_tick(self) -> Result<Solver, u128> {
        if self.debug_mode {
            self.print_state()
        }

        match self.state.char_in_front {
            '.' | 'X' | '-' | '|' | '+' => Ok(self.move_guard()),
            '#' => Ok(self.rotate_guard()),
            ' ' => Err(self.exit_guard()),
            c =>  panic!("Unknown char {c}")
        }
    }

    pub(crate) fn get_unique_visited_positions(self) -> u128 {
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
