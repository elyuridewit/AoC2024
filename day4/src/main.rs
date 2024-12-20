use std::{env, fs};
use ndarray::Array2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let filecontent: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening file. Error: {:?}", error),
    };

    let mut jagged_vec: Vec<Vec<char>> = Vec::new();
    for line in filecontent.lines() {
        let line_vec: Vec<char> = line.chars().collect();
        jagged_vec.push(line_vec);
    }
    
    let xbound = jagged_vec[0].len();
    let mut array: Array2<char> = Array2::from_elem((jagged_vec.len(), xbound), ' ');

    for (i, row) in jagged_vec.iter().enumerate() { 
        for (j, &val) in row.iter().enumerate() { 
            array[(i, j)] = val; 
        } 
    }

    // Get horizontals
    let mut horizontals: Vec<Vec<char>> = (0..array.nrows()).map(|i| {
        array.row(i).to_vec()
    }).collect();

    // Get verticals
    let mut verticals: Vec<Vec<char>> = (0..array.ncols()).map(|j| {
        array.column(j).to_vec()
    }).collect();

    // Get diagonals (from top-left to bottom-right and top-right to bottom-left)
    let mut diag1: Vec<Vec<char>> = Vec::new();
    let mut diag2: Vec<Vec<char>> = Vec::new();
    
    for k in 0..(array.nrows() + array.ncols() - 1) {
        let mut d1 = Vec::new();
        let mut d2 = Vec::new();
        
        for i in 0..array.nrows() {
            let j = k as isize - i as isize;
            if j >= 0 && j < array.ncols() as isize {
                d1.push(array[(i, j as usize)]);
            }

            let j = i as isize + array.ncols() as isize - 1 - k as isize;
            if j >= 0 && j < array.ncols() as isize {
                d2.push(array[(i, j as usize)]);
            }
        }

        if !d1.is_empty() {
            diag1.push(d1);
        }
        if !d2.is_empty() {
            diag2.push(d2);
        }
    }

    let mut all_windows: Vec<Vec<char>> = Vec::new();
    all_windows.append(&mut horizontals);
    all_windows.append(&mut verticals);
    all_windows.append(&mut diag1);
    all_windows.append(&mut diag2);

    let mut total: usize = 0;
    
    let xmas = "XMAS";
    let samx = "SAMX";
    for window in all_windows {
        let line_str: String = window.iter().collect();
        if line_str.contains(xmas) || line_str.contains(samx) {
            total += line_str.as_bytes().windows(xmas.len()).filter(|&w| w == xmas.as_bytes()).count();
            total += line_str.as_bytes().windows(samx.len()).filter(|&w| w == samx.as_bytes()).count();
        }
    }
    println!("Total occurences: {:?}", total)

}
