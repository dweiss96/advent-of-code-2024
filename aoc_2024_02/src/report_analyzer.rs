enum AnalyzerState {
    New, Initialized,
    Increasing, Decreasing,
    Failed
}

pub struct ReportAnalyzer {
    input: Vec<u32>,
    state: AnalyzerState,
    previous_value: u32,
}

impl ReportAnalyzer {
    pub fn new(input: &Vec<u32>) -> ReportAnalyzer {
        ReportAnalyzer {
            state: AnalyzerState::New,
            previous_value: 0,
            input: input.clone(),
        }
    }

    pub fn get_input(&self) -> Vec<u32> {
        self.input.clone()
    }

    pub fn is_failed(&self) -> bool {
        match &self.state {
            AnalyzerState::Failed => true,
            _ => false,
        }
    }

    pub fn is_valid(&self) -> bool {
        match &self.state {
            AnalyzerState::Failed => false,
            _ => true,
        }
    }

    fn initialize(self, initial_value: u32) -> ReportAnalyzer {
        ReportAnalyzer {
            state: AnalyzerState::Initialized,
            previous_value: initial_value,
            input: self.input,
        }
    }

    fn increasing(self, value: u32) -> ReportAnalyzer {
        ReportAnalyzer {
            state: AnalyzerState::Increasing,
            previous_value: value,
            input: self.input,
        }
    }

    fn decreasing(self, value: u32) -> ReportAnalyzer {
        ReportAnalyzer {
            state: AnalyzerState::Decreasing,
            previous_value: value,
            input: self.input,
        }
    }

    fn failed(self) -> ReportAnalyzer {
        ReportAnalyzer {
            state: AnalyzerState::Failed,
            previous_value: 0,
            input: self.input,
        }
    }

    pub fn analyze(self, value: u32) -> ReportAnalyzer {
        let values_are_close_enough = value.abs_diff(self.previous_value) < 4;

        let res = match self.state {
            AnalyzerState::New => self.initialize(value),
            AnalyzerState::Initialized if values_are_close_enough && self.previous_value < value => self.increasing(value),
            AnalyzerState::Initialized if values_are_close_enough && self.previous_value > value => self.decreasing(value),
            AnalyzerState::Increasing if values_are_close_enough && self.previous_value < value => self.increasing(value),
            AnalyzerState::Decreasing if values_are_close_enough && self.previous_value > value => self.decreasing(value),
            _ => self.failed()
        };

        res
    }
}