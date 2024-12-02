mod report_analyzer;

use std::fs;
use report_analyzer::ReportAnalyzer;

fn read_input() -> Vec<Vec<u32>> {
    let input_file_string = fs::read_to_string("input")
        .expect("Input file should be available");

    Vec::from_iter(input_file_string.split("\n").map(|line| {
        Vec::from_iter(line.split(" ").map(|number| {
            number.parse().expect("Lines must be composed of u64 numbers")
        }))
    }))
}

fn get_filtered_reports(reports: &Vec<Vec<u32>>) -> (Vec<ReportAnalyzer>,Vec<ReportAnalyzer>) {
    let validated_reports = reports.iter().map(|line| {
        line.iter().fold(ReportAnalyzer::new(line), |analyzer, value| {
            analyzer.analyze(value.clone())
        })
    });

    validated_reports.partition(|report| {
        report.is_valid()
    })
}

fn deep_analysis_of_reports(reports: &Vec<ReportAnalyzer>) -> Vec<ReportAnalyzer> {
    Vec::from_iter(reports.iter().map(|r| {
        let old_input = r.get_input();
        for i in 0..old_input.len() {
            let mut new_line = old_input.clone();
            let _ = new_line.remove(i);

            let new_result = new_line.clone()
                .iter().fold(ReportAnalyzer::new(new_line.clone().as_ref()),
                      |analyzer, value| {
                              analyzer.analyze(value.clone())
                    });

            if new_result.is_valid() {
                return new_result
            }
        }
        ReportAnalyzer::new(Vec::from([1, 1, 1]).as_ref()).analyze(1).analyze(1).analyze(1)
    }))
}

fn part_one(reports: &Vec<Vec<u32>>) {
    let valid_report_count = get_filtered_reports(&reports).0.len();

    println!("Out of {} reports, {} reports seem to be valid. [432]", reports.len(), valid_report_count)
}

fn part_two(reports: &Vec<Vec<u32>>) {
    let filtered_reports = get_filtered_reports(&reports);
    let valid_report_count = filtered_reports.0.len();

    let valid_report_count_retries = deep_analysis_of_reports(&filtered_reports.1)
        .iter().filter(|ra| ra.is_valid()).count();

    println!("Out of {} reports, {}+{}={} reports seem to be valid when dampened. [488]", reports.len(), valid_report_count,valid_report_count_retries,valid_report_count+valid_report_count_retries)
}

fn main() {
    let reports = read_input();

    part_one(&reports);
    part_two(&reports)
}
