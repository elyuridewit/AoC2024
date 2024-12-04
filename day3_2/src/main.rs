use std::{collections::BTreeMap, env, fs, usize};
use std::ops::Range;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let filecontent: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening file. Error: {:?}", error),
    };

    let mul_pat: Regex = Regex::new(r"(mul\()\d{1,3},\d{1,3}\)").unwrap();
    let do_dont_pat: Regex = Regex::new(r"(do\(\))|(don't\(\))").unwrap();

    // Get all the do() and don't() operations
    let mut do_donts: BTreeMap<usize, &str> = do_dont_pat.find_iter(&filecontent).map(|c| (c.end(), filecontent[c.start()..c.end()].strip_suffix("()").unwrap())).collect();
    do_donts.insert(0, "do");
    // println!("{:?}", do_donts);

    let mut ranges: Vec<Range<usize>> = Vec::new();
    let mut prev_idx: usize = 0;
    let mut prev_op: &str = "do";
    // Get all ranges of indexes between the end of a do() to a don't()
    // This is purely to get the last range if the final op is a "do()"
    let binding = do_donts.clone();
    let last_elem = binding.iter().next_back().unwrap();
    if *last_elem.1 == "do" {
        ranges.push(Range{start:*last_elem.0,end:filecontent.len()})
    }
    for (idx, op) in do_donts.into_iter().skip(1) {
        // if op != prev_op {
        //     ranges.push(Range{start:prev_idx, end:idx});
        //     prev_op = op;
        //     prev_idx = idx;
        // }
        if prev_op == "do" && op == "don't" {
            ranges.push(Range{start:prev_idx, end:idx});
            prev_op = op;
            prev_idx = idx;
        } else if prev_op == "don't" && op == "do" {
            prev_op = op;
            prev_idx = idx
        }
    }
    // println!("{:?}", ranges);

    let mut total: usize = 0;
    for range in ranges {
        println!("{:?}", range);
        for mat in mul_pat.find_iter(&filecontent[range]) {
            println!("{:?}", mat);
            if !mat.is_empty() {
                let (num1_str, num2_str) = mat.as_str().strip_prefix("mul(").unwrap().strip_suffix(")").unwrap().split_once(",").unwrap();
                let num1: usize = num1_str.parse().unwrap();
                let num2: usize = num2_str.parse().unwrap();

                total += num1 * num2;
            }
        }
    }

    println!("Total value: {:?}", total)
}
