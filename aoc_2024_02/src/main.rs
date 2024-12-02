use std::fs;

struct ReportAnalyzer {
    prev: u32,
    is_compromised: bool,
    direction: i8,
}

impl ReportAnalyzer {
    fn compromised(prev: u32) -> ReportAnalyzer {
        ReportAnalyzer {
            prev,
            is_compromised: true,
            direction: 0,
        }
    }

    fn init() -> ReportAnalyzer {
        ReportAnalyzer {
            prev: 0,
            is_compromised: false,
            direction: 0,
        }
    }

    fn validate(self, value: u32) -> ReportAnalyzer {
        if self.is_compromised {
            return Self::compromised(value);
        }

        if self.prev == 0 && self.direction == 0 {
            return ReportAnalyzer {
                prev: value,
                is_compromised: false,
                direction: 0,
            };
        }

        let difference = value as i64 - self.prev as i64;
        let direction_is_different = (difference < 0 && self.direction > 0) || (difference > 0 && self.direction < 0);

        if direction_is_different {
            Self::compromised(value)
        } else if difference == 0 {
            Self::compromised(value)
        } else if difference.abs() > 3 {
            Self::compromised(value)
        } else {
            ReportAnalyzer {
                prev: value,
                is_compromised: false,
                direction: difference.min(100).max(-100) as i8,
            }
        }
    }
}

fn read_input() -> Vec<Vec<u32>> {
    let input_file_string = fs::read_to_string("input")
        .expect("Input file should be available");

    Vec::from_iter(input_file_string.split("\n").map(|line| {
        Vec::from_iter(line.split(" ").map(|number| {
            number.parse().expect("Lines must be composed of u64 numbers")
        }))
    }))
}

fn part_one(reports: Vec<Vec<u32>>) {
    let validated_reports = reports.iter().map(|line| {
        line.iter().fold(ReportAnalyzer::init(), |analyzer, value| {
            analyzer.validate(value.clone())
        })
    });

    let valid_report_count = validated_reports.filter(|report| {
        !report.is_compromised
    }).count();

    println!("Out of {} reports, {} reports seem to be valid", reports.len(), valid_report_count)
}

fn main() {
    let reports = read_input();

    part_one(reports);
}
