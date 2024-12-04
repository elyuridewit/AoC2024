use std::{env, fs, usize};
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let filecontent: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening file. Error: {:?}", error),
    };

    let mul_pat: Regex = Regex::new(r"(mul\()\d{1,3},\d{1,3}\)").unwrap();
    let mut total: usize = 0;
    for mat in mul_pat.find_iter(&filecontent) {
        let mul = &filecontent[mat.start()..mat.end()];
        let (num1_str, num2_str) = mul.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap().split_once(",").unwrap();
        let num1: usize = num1_str.parse().unwrap();
        let num2: usize = num2_str.parse().unwrap();
        total += num1 * num2
    }
    println!("Total value: {:?}", total)
}
