pub struct Solver {
    input: Vec<Vec<char>>
}

trait Direction {
    fn move_one(&self, index: usize) -> usize;
    fn move_two(&self, index: usize) -> usize;
    fn move_three(&self, index: usize) -> usize;
}

enum DirectionX { Right, Stay, Left }

impl Direction for DirectionX {
    fn move_one(&self, index: usize) -> usize {
        match self {
            DirectionX::Right => index.checked_add(1).unwrap_or(usize::MAX),
            DirectionX::Stay => index,
            DirectionX::Left => index.checked_sub(1).unwrap_or(usize::MAX),
        }
    }

    fn move_two(&self, index: usize) -> usize {
        match self {
            DirectionX::Right => index.checked_add(2).unwrap_or(usize::MAX),
            DirectionX::Stay => index,
            DirectionX::Left => index.checked_sub(2).unwrap_or(usize::MAX),
        }
    }

    fn move_three(&self, index: usize) -> usize {
        match self {
            DirectionX::Right => index.checked_add(3).unwrap_or(usize::MAX),
            DirectionX::Stay => index,
            DirectionX::Left => index.checked_sub(3).unwrap_or(usize::MAX),
        }
    }
}

enum DirectionY { Down, Stay, Up }

impl Direction for DirectionY {
    fn move_one(&self, index: usize) -> usize {
        match self {
            DirectionY::Down => index.checked_add(1).unwrap_or(usize::MAX),
            DirectionY::Stay => index,
            DirectionY::Up => index.checked_sub(1).unwrap_or(usize::MAX),
        }
    }

    fn move_two(&self, index: usize) -> usize {
        match self {
            DirectionY::Down => index.checked_add(2).unwrap_or(usize::MAX),
            DirectionY::Stay => index,
            DirectionY::Up => index.checked_sub(2).unwrap_or(usize::MAX),
        }
    }

    fn move_three(&self, index: usize) -> usize {
        match self {
            DirectionY::Down => index.checked_add(3).unwrap_or(usize::MAX),
            DirectionY::Stay => index,
            DirectionY::Up => index.checked_sub(3).unwrap_or(usize::MAX),
        }
    }
}

impl Solver {
    pub fn new(input: Vec<Vec<char>>) -> Solver {
        Solver {
            input
        }
    }

    fn get_safe_input_value(&self, x: usize, y: usize) -> Option<char> {
        match self.input.get(y) {
            None => None,
            Some(chars_in_line) => chars_in_line.get(x).map(|c| c.clone())
        }
    }
    pub fn count_x_mas(self) -> u128 {
        let mut x_mas_count = 0;
        for y in 1..self.input.len()-1 {
            for x in 1..self.input[y].len()-1 {
                if self.input[y][x].eq_ignore_ascii_case(&'a') {
                    match (
                        self.input[y-1][x-1].to_ascii_lowercase(),
                        self.input[y+1][x+1].to_ascii_lowercase(),
                        self.input[y-1][x+1].to_ascii_lowercase(),
                        self.input[y+1][x-1].to_ascii_lowercase(),
                    ) {
                        ('m', 's', 'm', 's') => x_mas_count += 1,
                        ('m', 's', 's', 'm') => x_mas_count += 1,
                        ('s', 'm', 'm', 's') => x_mas_count += 1,
                        ('s', 'm', 's', 'm') => x_mas_count += 1,
                        _ => {}
                    }
                }
            }
        }
        x_mas_count
    }

    pub fn count_xmas(self) -> u128 {
        let mut xmas_count = 0;
        for y in 0..self.input.len() {
            for x in 0..self.input[y].len() {
                if self.get_word(x,y, DirectionX::Right, DirectionY::Stay).eq_ignore_ascii_case("xmas") {
                    xmas_count += 1;
                }
                if self.get_word(x,y, DirectionX::Left, DirectionY::Stay).eq_ignore_ascii_case("xmas") {
                    xmas_count += 1;
                }

                if self.get_word(x,y, DirectionX::Stay, DirectionY::Up).eq_ignore_ascii_case("xmas") {
                    xmas_count += 1;
                }
                if self.get_word(x,y, DirectionX::Stay, DirectionY::Down).eq_ignore_ascii_case("xmas") {
                    xmas_count += 1;
                }

                if self.get_word(x,y, DirectionX::Right, DirectionY::Up).eq_ignore_ascii_case("xmas") {
                    xmas_count += 1;
                }
                if self.get_word(x,y, DirectionX::Right, DirectionY::Down).eq_ignore_ascii_case("xmas") {
                    xmas_count += 1;
                }
                if self.get_word(x,y, DirectionX::Left, DirectionY::Up).eq_ignore_ascii_case("xmas") {
                    xmas_count += 1;
                }
                if self.get_word(x,y, DirectionX::Left, DirectionY::Down).eq_ignore_ascii_case("xmas") {
                    xmas_count += 1;
                }
            }

        }
        xmas_count
    }

    fn get_word(&self, start_x: usize, start_y: usize, direction_x: DirectionX, direction_y: DirectionY) -> String {
        let maybe_x = self.get_safe_input_value(start_x, start_y);
        let maybe_m = self.get_safe_input_value(direction_x.move_one(start_x), direction_y.move_one(start_y));
        let maybe_a = self.get_safe_input_value(direction_x.move_two(start_x), direction_y.move_two(start_y));
        let maybe_s = self.get_safe_input_value(direction_x.move_three(start_x), direction_y.move_three(start_y));

        format!("{}{}{}{}",
                maybe_x.unwrap_or('.'),
                maybe_m.unwrap_or('.'),
                maybe_a.unwrap_or('.'),
                maybe_s.unwrap_or('.'),
        )
    }
}