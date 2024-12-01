use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let filecontent: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening file. Error: {:?}", error),
    };

    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();
    for (pos, line) in filecontent.lines().enumerate() {
        let num_strs: Vec<&str> = line.split_whitespace().collect();
        if num_strs.len() != 2 {
            panic!("More than 2 values found on line: {:?}", pos)
        }
        left.push(num_strs[0].parse().unwrap());
        right.push(num_strs[1].parse().unwrap());
    }

    if left.len() != right.len() {
        panic!("Something fucky this way blows.")
    }

    let mut sim: usize = 0;
    for lnum in left {
        let occurence: usize = right.iter().filter(|&n| *n == lnum).count();
        sim += lnum * occurence
    }
    println!("Similarity score: {:?}", sim)
}
