enum ParserStatePartOne {
    LetterM,LetterU,LetterL,BracketOpen,BracketClose,NumberA,NumberB,Comma,Empty
}

pub struct ParserPartOne {
    state: ParserStatePartOne,

    storage_num_a: String,
    storage_num_b: String,

    sum_of_all_mults: u128,
}

impl ParserPartOne {
    pub fn new() -> ParserPartOne {
        ParserPartOne {
            state: ParserStatePartOne::Empty,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: 0
        }
    }

    fn move_to_empty(self) -> ParserPartOne {
        ParserPartOne {
            state: ParserStatePartOne::Empty,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_letter_m(self) -> ParserPartOne {
        ParserPartOne {
            state: ParserStatePartOne::LetterM,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_letter_u(self) -> ParserPartOne {
        ParserPartOne {
            state: ParserStatePartOne::LetterU,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_letter_l(self) -> ParserPartOne {
        ParserPartOne {
            state: ParserStatePartOne::LetterL,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_bracket_open(self) -> ParserPartOne {
        ParserPartOne {
            state: ParserStatePartOne::BracketOpen,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_number_a(self, digit: char) -> ParserPartOne {
        ParserPartOne {
            state: ParserStatePartOne::NumberA,
            storage_num_a: format!("{}{}", self.storage_num_a, digit),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_comma(self) -> ParserPartOne {
        ParserPartOne {
            state: ParserStatePartOne::Comma,
            storage_num_a: self.storage_num_a,
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_number_b(self, digit: char) -> ParserPartOne {
        ParserPartOne {
            state: ParserStatePartOne::NumberB,
            storage_num_a: self.storage_num_a,
            storage_num_b: format!("{}{}", self.storage_num_b, digit),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_bracket_close(self) -> ParserPartOne {
        let num_a: u128 = self.storage_num_a.parse().expect("Number A should be read as valid u128");
        let num_b: u128 = self.storage_num_b.parse().expect("Number B should be read as valid u128");

        ParserPartOne {
            state: ParserStatePartOne::BracketClose,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults + (num_a * num_b)
        }
    }

    pub fn analyze_character(self, character: char) -> ParserPartOne {
        match self.state {
            ParserStatePartOne::LetterM if character == 'u' => self.move_to_letter_u(),
            ParserStatePartOne::LetterU if character == 'l' => self.move_to_letter_l(),
            ParserStatePartOne::LetterL if character == '(' => self.move_to_bracket_open(),
            ParserStatePartOne::BracketOpen if character.is_ascii_digit() => self.move_to_number_a(character),
            ParserStatePartOne::NumberA if character.is_ascii_digit() => self.move_to_number_a(character),
            ParserStatePartOne::NumberA if character == ',' => self.move_to_comma(),
            ParserStatePartOne::Comma if character.is_ascii_digit() => self.move_to_number_b(character),
            ParserStatePartOne::NumberB if character.is_ascii_digit() => self.move_to_number_b(character),
            ParserStatePartOne::NumberB if character == ')' => self.move_to_bracket_close(),
            ParserStatePartOne::BracketClose if character == 'm'  => self.move_to_letter_m(),
            ParserStatePartOne::Empty if character == 'm'  => self.move_to_letter_m(),
            _ if character == 'm'  => self.move_to_letter_m(),
            _ => self.move_to_empty()
        }
    }

    pub fn get_sum(self) -> u128 {
        self.sum_of_all_mults
    }
}
