use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let filecontent: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening file. Error: {:?}", error),
    };

    let mut reports: Vec<Vec<isize>> = Vec::new();
    for line in filecontent.lines() {
        let report: Vec<isize> = line.split_whitespace().flat_map(|n| n.parse()).collect();
        reports.push(report);
    }

    let mut safe_reports: usize = 0;
    let mut damp_safe: usize = 0;
    for report in reports {
        // println!("NEW REPORT");
        let safe: bool = report_checker(report.clone());
        if safe {
            safe_reports += 1;
        } else {
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                // println!("Calling report_checker with vec {:?}", new_report);
                let safe_con = report_checker(new_report.clone());
                if safe_con {
                    damp_safe += 1;
                    break
                }
            }
        }
    }

    // Part 1
    println!("Found {:?} safe reports.", safe_reports);
    // Part 2
    println!("Found {:?} safe reports with the Problem Dampener enabled.", safe_reports + damp_safe)
}

fn report_checker(report: Vec<isize>) -> bool {
    let mut increase: Option<bool> = None;
    for (pos, value) in report.clone().into_iter().skip(1).enumerate() {
        // Due to .skip(1), pos can reference the previous element in the report
        if let Some(incr) = increase {
            if report[pos] > value && incr || 
               report[pos] < value && !incr || 
               report[pos] == value {
                return false;
            }
        } else {
            if report[pos] < value {
                increase = Some(true);
            } else if report[pos] > value {
                increase = Some(false);
            } else if report[pos] == value {
                return false;
            } else {
                panic!("Some shit happened: {:?}", (report, value, increase))
            }
        }
        
        if report[pos] > value && report[pos] - value >= 4 {
            return false;
        } else if report[pos] < value && value - report[pos] >= 4 {
            return false;
        }

    }
    
    return true
}
