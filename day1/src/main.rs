use std::{collections::VecDeque, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let filecontent: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening file. Error: {:?}", error),
    };

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
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

    left.sort();
    right.sort();

    let mut leftd: VecDeque<u32> = VecDeque::from(left);
    let mut rightd: VecDeque<u32> = VecDeque::from(right);
    println!("{:?}", leftd);
    println!("{:?}", rightd);

    let mut total_dist: u32 = 0;
    while leftd.len() != 0 {
        let leftn = match leftd.pop_front() {
            Some(num) => num,
            None => panic!("NO NUMBER AAAAAAAAAAA"),
        };
        let rightn = match rightd.pop_front() {
            Some(num) => num,
            None => panic!("NO NUMBER AAAAAAAAAAA"),
        };

        if leftn >= rightn {
            total_dist += leftn - rightn
        } else {
            total_dist += rightn - leftn
        };
    }
    println!("Total distance = {:?}", total_dist)
}
