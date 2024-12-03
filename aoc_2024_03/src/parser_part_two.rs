enum ParserStatePartTwo {
    LetterM,LetterU,LetterL,BracketOpen,BracketClose,NumberA,NumberB,Comma,Empty,

    LetterD,LetterO,LetterN,Apostrophe,LetterT,
    BlockInstructionBracketOpen,

    Blocked, BlockedLetterD,BlockedLetterO,
    UnblockInstructionBracketOpen,}

pub struct ParserPartTwo {
    state: ParserStatePartTwo,

    storage_num_a: String,
    storage_num_b: String,

    sum_of_all_mults: u128,
}

impl ParserPartTwo {
    pub fn new() -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::Empty,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: 0
        }
    }

    fn move_to_empty(self) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::Empty,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_letter_m(self) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::LetterM,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_letter_u(self) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::LetterU,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_letter_l(self) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::LetterL,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_bracket_open(self) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::BracketOpen,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_number_a(self, digit: char) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::NumberA,
            storage_num_a: format!("{}{}", self.storage_num_a, digit),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_comma(self) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::Comma,
            storage_num_a: self.storage_num_a,
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_number_b(self, digit: char) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::NumberB,
            storage_num_a: self.storage_num_a,
            storage_num_b: format!("{}{}", self.storage_num_b, digit),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn add_result_and_move_to_empty(self) -> ParserPartTwo {
        let num_a: u128 = self.storage_num_a.parse().expect("Number A should be read as valid u128");
        let num_b: u128 = self.storage_num_b.parse().expect("Number B should be read as valid u128");

        ParserPartTwo {
            state: ParserStatePartTwo::Empty,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults + (num_a * num_b)
        }
    }

    fn move_to_letter_d(self) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::LetterD,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_letter_o(self) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::LetterO,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_letter_n(self) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::LetterN,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_apostrophe(self) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::Apostrophe,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_letter_t(self) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::LetterT,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_block_instruction_bracket_open(self) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::BlockInstructionBracketOpen,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_blocked(self) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::Blocked,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_blocked_letter_d(self) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::BlockedLetterD,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_blocked_letter_o(self) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::BlockedLetterO,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    fn move_to_unblock_instruction_bracket_open(self) -> ParserPartTwo {
        ParserPartTwo {
            state: ParserStatePartTwo::UnblockInstructionBracketOpen,
            storage_num_a: String::new(),
            storage_num_b: String::new(),
            sum_of_all_mults: self.sum_of_all_mults
        }
    }

    pub fn analyze_character(self, character: char) -> ParserPartTwo {
        match self.state {
            ParserStatePartTwo::Blocked if character == 'd' => self.move_to_blocked_letter_d(),
            ParserStatePartTwo::Blocked => self.move_to_blocked(),
            ParserStatePartTwo::BlockedLetterD if character == 'o' => self.move_to_blocked_letter_o(),
            ParserStatePartTwo::BlockedLetterD => self.move_to_blocked(),
            ParserStatePartTwo::BlockedLetterO if character == '(' => self.move_to_unblock_instruction_bracket_open(),
            ParserStatePartTwo::BlockedLetterO => self.move_to_blocked(),
            ParserStatePartTwo::UnblockInstructionBracketOpen if character == ')' => self.move_to_empty(),
            ParserStatePartTwo::UnblockInstructionBracketOpen => self.move_to_blocked(),

            ParserStatePartTwo::LetterD if character == 'o'  => self.move_to_letter_o(),
            ParserStatePartTwo::LetterO if character == 'n'  => self.move_to_letter_n(),
            ParserStatePartTwo::LetterN if character == '\''  => self.move_to_apostrophe(),
            ParserStatePartTwo::Apostrophe if character == 't'  => self.move_to_letter_t(),
            ParserStatePartTwo::LetterT if character == '('  => self.move_to_block_instruction_bracket_open(),
            ParserStatePartTwo::BlockInstructionBracketOpen if character == ')'  => self.move_to_blocked(),

            ParserStatePartTwo::LetterM if character == 'u' => self.move_to_letter_u(),
            ParserStatePartTwo::LetterU if character == 'l' => self.move_to_letter_l(),
            ParserStatePartTwo::LetterL if character == '(' => self.move_to_bracket_open(),
            ParserStatePartTwo::BracketOpen if character.is_ascii_digit() => self.move_to_number_a(character),
            ParserStatePartTwo::NumberA if character.is_ascii_digit() => self.move_to_number_a(character),
            ParserStatePartTwo::NumberA if character == ',' => self.move_to_comma(),
            ParserStatePartTwo::Comma if character.is_ascii_digit() => self.move_to_number_b(character),
            ParserStatePartTwo::NumberB if character.is_ascii_digit() => self.move_to_number_b(character),
            ParserStatePartTwo::NumberB if character == ')' => self.add_result_and_move_to_empty(),
            _ if character == 'd'  => self.move_to_letter_d(),
            _ if character == 'm'  => self.move_to_letter_m(),
            _ => self.move_to_empty()
        }
    }

    pub fn get_sum(self) -> u128 {
        self.sum_of_all_mults
    }
}
