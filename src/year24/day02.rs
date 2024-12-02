use crate::include_input;

pub fn input() -> &'static str {
    include_input!(2024 / 02)
}

pub fn part1(input: &str) -> String {
    let reports = parse_input(input);
    let mut safe_reports = 0;
    for report in &reports {
        if is_safe(report) {
            safe_reports += 1;
        }
    }

    safe_reports.to_string()
}

pub fn part2(input: &str) -> String {
    let reports = parse_input(input);
    let mut safe_reports = 0;
    for report in &reports {
        if is_safe_with_tolerance(report) {
            safe_reports += 1;
        }
    }

    safe_reports.to_string()
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(" ")
                .map(|v| v.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect()
}

fn is_safe(report: &Vec<i64>) -> bool {
    let diffs: Vec<i64> = report.windows(2).map(|v| v[0] - v[1]).collect();
    let (min, max) = (diffs.iter().min().unwrap(), diffs.iter().max().unwrap());

    (*min >= -3 && *max <= -1) || (*min >= 1 && *max <= 3)
}

fn is_safe_with_tolerance(report: &Vec<i64>) -> bool {
    let mut is_report_safe = is_safe(report);

    if !is_report_safe {
        for i in 0..report.len() {
            let mut new_report = report.clone();
            new_report.remove(i);
            if is_safe(&new_report) {
                is_report_safe = true;
                break;
            }
        }
    }

    is_report_safe
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn test_preview_part1() {
        let preview_input = "
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
            ";
        let safe_reports = part1(preview_input);
        assert_eq!("2", safe_reports);
    }

    #[test]
    fn test_preview_part2() {
        let preview_input = "
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
            ";
        let safe_reports = part2(preview_input);
        assert_eq!("4", safe_reports);
    }
}
