type Report = Vec<u8>;

fn report_is_safe(report: &&Report) -> bool {
    (report.is_sorted() || report.iter().rev().is_sorted())
        && report
            .windows(2)
            .all(|lr| lr[0] != lr[1] && lr[0].abs_diff(lr[1]) <= 3)
}

fn part1(reports: &[Report]) {
    let num_safe_reports = reports.iter().filter(report_is_safe).count();

    println!("{num_safe_reports}");
}

// Sub-optimal solution as its complexity is O(n^2) even though it can be done in O(n)
fn part2(reports: &[Report]) {
    let num_safe_reports = reports
        .iter()
        .filter(|&report| {
            if report_is_safe(&report) {
                return true;
            }
            let mut report_cloned = report.clone();

            for i in 0..report.len() {
                report_cloned.remove(i);
                if report_is_safe(&&report_cloned) {
                    return true;
                }
                report_cloned.clone_from(report);
            }
            false
        })
        .count();
    println!("{num_safe_reports}");
}

pub fn day2() {
    let string = include_str!("../inputs/day2");

    let reports: Vec<Report> = string
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    part1(&reports);
    part2(&reports);
    // part2(&first_group, &second_group);
}
